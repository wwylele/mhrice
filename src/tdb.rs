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
        let f_count = file.read_u32()?;
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
        let f_offset = file.read_u64()?;
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

        file.seek_assert_align_up(b_offset, 16)?;
        (0..b_count)
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
                Ok(())
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

        file.seek_assert_align_up(f_offset, 16)?;
        (0..f_count)
            .map(|_| {
                file.read_u64()?;
                Ok(())
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

        file.seek_assert_align_up(k_offset, 16)?;
        (0..k_count)
            .map(|_| {
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                Ok(())
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

        let read_string = move |offset: u32| {
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
        (0..q_count)
            .map(|_| {
                file.read_u32()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        for type_info in types {
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
        }
        for field in fields {
            println!("{}", read_string(field.name_offset)?);
            println!(" - {},{} - {}", field.a1, field.a2, field.b);
        }

        Ok(Tdb {})
    }
}
