use crate::file_ext::*;
use anyhow::*;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek};

pub struct Tdb {}

impl Tdb {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Tdb> {
        if &file.read_magic()? != b"TDB\0" {
            bail!("Wrong magic forTDB file");
        }

        if file.read_u32()? != 0x45 {
            bail!("Wrong version for RSZ block");
        }

        if file.read_u32()? != 0 {
            bail!("Expected 0");
        }

        let b_count = file.read_u32()?;
        let d_count = file.read_u32()?;
        let field_membership_count = file.read_u32()?;
        let type_count = file.read_u32()?;
        let field_count = file.read_u32()?;
        let method_count = file.read_u32()?;
        let property_count = file.read_u32()?;
        let h_count = file.read_u32()?;
        let j_count = file.read_u32()?;
        let k_count = file.read_u32()?;
        let l_count = file.read_u32()?;
        let m_count = file.read_u32()?;
        let n_count = file.read_u32()?;
        let q_count = file.read_u32()?;
        let a_count = file.read_u32()?;

        if file.read_u32()? != 0 {
            bail!("Expected 0");
        }

        let unknown = file.read_u32()?;
        let string_table_len = file.read_u32()?;
        let p_len = file.read_u32()?;

        let a_offset = file.read_u64()?;
        let b_offset = file.read_u64()?;
        let type_offset = file.read_u64()?;
        let d_offset = file.read_u64()?;
        let method_offset = file.read_u64()?;
        let field_membership_offset = file.read_u64()?;
        let field_offset = file.read_u64()?;
        let h_offset = file.read_u64()?;
        let property_offset = file.read_u64()?;
        let j_offset = file.read_u64()?;
        let k_offset = file.read_u64()?;
        let l_offset = file.read_u64()?;
        let m_offset = file.read_u64()?;
        let n_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;
        let p_offset = file.read_u64()?;
        let q_offset = file.read_u64()?;

        file.seek_noop(a_offset)?;
        (0..a_count)
            .map(|_| {
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                file.read_u64()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        struct B {
            base_index: u64,
            parent_index: u64,
            af: u64,
            prev_index: u64,
            next_index: u64,
            type_index: u64,
            bf: u64,
            b: u32,
            c: u32,
            d: u32,
        }
        file.seek_assert_align_up(b_offset, 16)?;
        let bs = (0..b_count)
            .map(|i| {
                let index_a = file.read_u64()?;
                let index = index_a & 0x3FFFF;
                let base_index = (index_a >> 18) & 0x3FFFF;
                let parent_index = (index_a >> 36) & 0x3FFFF;
                let af = index_a >> 54;

                if index != u64::from(i) {
                    bail!("Unexpected index");
                }

                let index_b = file.read_u64()?;
                let prev_index = index_b & 0x3FFFF;
                let next_index = (index_b >> 18) & 0x3FFFF;
                let type_index = (index_b >> 36) & 0x3FFFF;
                let bf = index_b >> 54;

                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;

                file.read_u32()?;
                let b = file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;

                file.read_u32()?;
                let c = file.read_u32()?;
                let d = file.read_u32()?;
                file.read_u32()?;

                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                Ok(B {
                    base_index,
                    parent_index,
                    af,
                    prev_index,
                    next_index,
                    type_index,
                    bf,
                    b,
                    c,
                    d,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(d_offset, 16)?;
        (0..d_count)
            .map(|_| {
                file.read_u64()?;
                file.read_u64()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        struct FieldMembership {
            b_index: u64,
            field_index: u64,
            position: u64,
        }
        file.seek_assert_align_up(field_membership_offset, 16)?;
        let field_memberships = (0..field_membership_count)
            .map(|_| {
                let data = file.read_u64()?;
                let b_index = data & 0x3FFFF;
                let field_index = (data >> 18) & 0xFFFFF;
                let position = data >> 38;
                Ok(FieldMembership {
                    b_index,
                    field_index,
                    position,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Type {
            name_offset: u32,
            namespace_offset: u32,
            len: u32,
            s2: u32,

            n0: u16,
            n1: u16,
            n2: u16,
            n3: u16,
            n4: u16,
            n5: u16,
            n6: u16,
            n7: u16,

            flag_a: u64,
            flag_b: u64,
        }

        file.seek_assert_align_up(type_offset, 16)?;
        let types = (0..type_count)
            .map(|_| {
                let name_offset = file.read_u32()?;
                let namespace_offset = file.read_u32()?;
                let len = file.read_u32()?;
                let s2 = file.read_u32()?;

                let n0 = file.read_u16()?;
                let n1 = file.read_u16()?;
                let n2 = file.read_u16()?;
                let n3 = file.read_u16()?;
                let n4 = file.read_u16()?;
                let n5 = file.read_u16()?;
                let n6 = file.read_u16()?;
                let n7 = file.read_u16()?;

                let flag_a = file.read_u64()?;
                let flag_b = file.read_u64()?;
                Ok(Type {
                    name_offset,
                    namespace_offset,
                    len,
                    s2,
                    n0,
                    n1,
                    n2,
                    n3,
                    n4,
                    n5,
                    n6,
                    n7,
                    flag_a,
                    flag_b,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Method {
            a: u32,
            b: u32,
            name_offset: u32,
        }
        file.seek_assert_align_up(method_offset, 16)?;
        let methods = (0..method_count)
            .map(|_| {
                let a = file.read_u32()?;
                let b = file.read_u32()?;
                let name_offset = file.read_u32()?;
                Ok(Method { a, b, name_offset })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Field {
            a1: u16,
            a2: u16,
            b: u32,
            name_offset: u32,
        }
        file.seek_assert_align_up(field_offset, 16)?;
        let fields = (0..field_count)
            .map(|_| {
                let a1 = file.read_u16()?;
                let a2 = file.read_u16()?;
                let b = file.read_u32()?;
                let name_offset = file.read_u32()?;
                Ok(Field {
                    a1,
                    a2,
                    b,
                    name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(h_offset, 16)?;
        (0..h_count)
            .map(|_| {
                file.read_u64()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        struct Property {
            a: u16,
            b: u16,
            name_offset: u32,
        }
        file.seek_assert_align_up(property_offset, 16)?;
        let properties = (0..property_count)
            .map(|_| {
                let a = file.read_u16()?;
                let b = file.read_u16()?;
                let name_offset = file.read_u32()?;
                Ok(Property { a, b, name_offset })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(j_offset, 16)?;
        (0..j_count)
            .map(|_| {
                file.read_u64()?;
                file.read_u64()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(l_offset, 16)?;
        (0..l_count)
            .map(|_| {
                file.read_u64()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        struct Param {
            a: u32,
            name_offset_sometimes: u32,
            c: u32,
        }
        file.seek_assert_align_up(k_offset, 16)?;
        let params = (0..k_count)
            .map(|_| {
                let a = file.read_u32()?;
                let name_offset_sometimes = file.read_u32()?;
                let c = file.read_u32()?;
                Ok(Param {
                    a,
                    name_offset_sometimes,
                    c,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(m_offset, 16)?;
        (0..m_count)
            .map(|_| {
                file.read_u32()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(n_offset, 16)?;
        (0..n_count)
            .map(|_| {
                file.read_u32()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(string_table_offset, 16)?;
        let mut string_table = vec![0; string_table_len.try_into()?];
        file.read_exact(&mut string_table)?;

        let string_all: std::collections::HashSet<u32> = string_table[0..string_table.len() - 1]
            .iter()
            .enumerate()
            .filter(|&(index, &c)| c == 0)
            .map(|(index, _)| index as u32 + 1)
            .collect();
        let string_all = std::rc::Rc::new(std::cell::RefCell::new(string_all));
        let string_all_ref = string_all.clone();

        let mut read_string = move |offset: u32| {
            string_all_ref.borrow_mut().remove(&offset);
            let offset = usize::try_from(offset)?;
            if offset >= string_table.len() {
                bail!("offset out of bount");
            }
            let mut end = offset;
            while string_table[end] != 0 {
                end += 1;
                if end >= string_table.len() {
                    bail!("end out of bound");
                }
            }
            Ok(std::str::from_utf8(&string_table[offset..end])?.to_owned())
        };

        file.seek_assert_align_up(p_offset, 16)?;
        let mut p = vec![0; p_len.try_into()?];
        file.read_exact(&mut p)?;

        file.seek_assert_align_up(q_offset, 16)?;
        let qs = (0..q_count)
            .map(|_| Ok(file.read_u32()?))
            .collect::<Result<Vec<_>>>()?;

        for field_membership in field_memberships {
            let b = &bs[field_membership.b_index as usize];
            let type_name = read_string(types[b.type_index as usize].name_offset)?;
            let field_name =
                read_string(fields[field_membership.field_index as usize].name_offset)?;
            println!(
                "{} + {}: {}",
                type_name, field_membership.position, field_name
            );
        }

        /*let mut parent_ref = vec![0; types.len()];
        for (i, b) in bs.iter().enumerate() {
            if b.parent_index != 0 {
                let self_index = b.type_index as usize;
                let parent_index = bs[b.parent_index as usize].type_index as usize;
                /*if parent_ref[self_index] != 0 && parent_ref[self_index] != parent_index {
                    bail!("Collision at B[{}]", i)
                }*/
                parent_ref[self_index] = parent_index;
            }
        }

        for i in 0..types.len() {
            if (types[i].namespace_offset == 0) == (parent_ref[i] == 0) {
                //bail!("huh {}", i)
                continue;
            }
            let mut cur = i;
            let mut name = "".to_owned();
            loop {
                name = ".".to_owned() + &read_string(types[cur].name_offset)? + &name;
                if parent_ref[cur] == 0 {
                    name = read_string(types[cur].namespace_offset)? + &name;
                    break;
                }
                cur = parent_ref[cur];
            }
            println!(
                "{:08X}, {}",
                murmur3::murmur3_32(&mut name.as_bytes(), 0xFFFFFFFF)?,
                name
            );
        }*/

        /*for (i, b) in bs.into_iter().enumerate() {
            println!(
                "*B[{:8}] base=B[{:8}] parent=B[{:8}] prev==B[{:8}] next=B[{:8}] type=C[{:8}] af={:4} bf={:4}",
                i, b.base_index, b.parent_index, b.prev_index, b.next_index, b.type_index, b.af, b.bf
            );
        }

        for (i, type_info) in types.into_iter().enumerate() {
            let name = read_string(type_info.name_offset)?;
            let namespace = read_string(type_info.namespace_offset)?;
            println!("*C[{:8}] {}.{}", i, namespace, name);
        }*/

        /*for type_info in types {
            let name = read_string(type_info.name_offset)?;
            let namespace = read_string(type_info.namespace_offset)?;
            println!("{}.{} - {} bytes", namespace, name, type_info.len);
            println!(
                "{:08X}/{}, {}, {}, {}, {}, {}, {}, {},\n${:016X}^{:016X}",
                type_info.s2,
                type_info.n0,
                type_info.n1,
                type_info.n2,
                type_info.n3,
                type_info.n4,
                type_info.n5,
                type_info.n6,
                type_info.n7,
                type_info.flag_a,
                type_info.flag_b
            );
        }

        for property in properties {
            println!("{}", read_string(property.name_offset)?);
            println!("{:04X} - {:04X}", property.a, property.b);
        }
        for method in methods {
            println!("{}", read_string(method.name_offset)?);
            println!(" - {} - {}", method.a, method.b);
        }*/
        /* for field in fields {
            println!("{}", read_string(field.name_offset)?);
            println!(" - {},{} - {}", field.a1, field.a2, field.b);
        }

        for q in qs {
            println!("{}", read_string(q)?);
        }

        for param in params {
            println!(" - {:?}", read_string(param.name_offset_sometimes));
        }

        println!("#########");
        let k = string_all.borrow().clone();
        for unvisited in k {
            println!("{:08X} - {}", unvisited, read_string(unvisited)?);
        }*/

        Ok(Tdb {})
    }
}
