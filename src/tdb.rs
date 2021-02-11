use crate::bitfield::*;
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

        let type_instance_count = file.read_u32()?;
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
        let array_table_len = file.read_u32()?;

        let a_offset = file.read_u64()?;
        let type_instance_offset = file.read_u64()?;
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
        let array_table_offset = file.read_u64()?;
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

        struct TypeInstance {
            base_type_instance_index: u64,
            parent_type_instance_index: u64,
            af: u64,
            arrayize_type_instance_index: u64,
            dearrayize_type_instance_index: u64,
            type_index: u64,
            bf: u64,
            b: u32,
            c: u32,
            interface_list_offset: u32,
            field_membership_start_index: u32,
            template_argument_list_offset: u32,
        }
        file.seek_assert_align_up(type_instance_offset, 16)?;
        let type_instances = (0..type_instance_count)
            .map(|i| {
                let (index, base_type_instance_index, parent_type_instance_index, af) =
                    file.read_u64()?.bit_split((18, 18, 18, 10));

                if index != u64::from(i) {
                    bail!("Unexpected index");
                }

                let (arrayize_type_instance_index, dearrayize_type_instance_index, type_index, bf) =
                    file.read_u64()?.bit_split((18, 18, 18, 10));

                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;

                file.read_u32()?;
                let b = file.read_u32()?;
                file.read_u32()?;
                let field_membership_start_index = file.read_u32()?;

                file.read_u32()?;
                let c = file.read_u32()?;
                let interface_list_offset = file.read_u32()?;
                let template_argument_list_offset = file.read_u32()?;

                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                Ok(TypeInstance {
                    base_type_instance_index,
                    parent_type_instance_index,
                    af,
                    arrayize_type_instance_index,
                    dearrayize_type_instance_index,
                    type_index,
                    bf,
                    b,
                    c,
                    interface_list_offset,
                    field_membership_start_index,
                    template_argument_list_offset,
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
            type_instance_index: u64,
            field_index: u64,
            position: u64,
        }
        file.seek_assert_align_up(field_membership_offset, 16)?;
        let field_memberships = (0..field_membership_count)
            .map(|_| {
                let (type_instance_index, field_index, position) =
                    file.read_u64()?.bit_split((18, 20, 26));
                Ok(FieldMembership {
                    type_instance_index,
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

            field_count: u32,

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
                let field_count = file.read_u32()?;
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
                    field_count,
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
            type_instance_index: u32,
            b_upper: u32,
            name_offset: u32,
        }
        file.seek_assert_align_up(field_offset, 16)?;
        let fields = (0..field_count)
            .map(|_| {
                let a1 = file.read_u16()?;
                let a2 = file.read_u16()?;
                let (type_instance_index, b_upper) = file.read_u32()?.bit_split((18, 14));
                let name_offset = file.read_u32()?;
                Ok(Field {
                    a1,
                    a2,
                    type_instance_index,
                    b_upper,
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

        file.seek_assert_align_up(array_table_offset, 16)?;
        let mut array_table = vec![0; array_table_len.try_into()?];
        file.read_exact(&mut array_table)?;

        file.seek_assert_align_up(q_offset, 16)?;
        let qs = (0..q_count)
            .map(|_| Ok(file.read_u32()?))
            .collect::<Result<Vec<_>>>()?;

        #[derive(Clone)]
        struct Mono {
            full_name: String,
        }

        let mut monos: Vec<Option<Mono>> = vec![None; type_instances.len()];

        fn build_mono(
            monos: &mut [Option<Mono>],
            index: usize,
            type_instances: &[TypeInstance],
            types: &[Type],
            array_table: &[u8],
            read_string: impl Fn(u32) -> Result<String> + Copy,
        ) -> Result<()> {
            if index > monos.len() {
                bail!("Index out of bound: {}", index);
            }
            if monos[index].is_some() {
                return Ok(());
            }

            if index == 0 {
                monos[index] = Some(Mono {
                    full_name: "".to_string(),
                });
                return Ok(());
            }

            let ti = &type_instances[index];
            let ty = types
                .get(ti.type_index as usize)
                .context("Type index out of bound")?;

            if ti.dearrayize_type_instance_index != 0 {
                build_mono(
                    monos,
                    ti.dearrayize_type_instance_index as usize,
                    type_instances,
                    types,
                    array_table,
                    read_string,
                )
                .context(format!("Build dearrayize mono for {}", index))?;
                let full_name = monos[ti.dearrayize_type_instance_index as usize]
                    .as_ref()
                    .unwrap()
                    .full_name
                    .clone()
                    + "[]";

                monos[index] = Some(Mono { full_name });

                return Ok(());
            }

            let parent = if ti.parent_type_instance_index != 0 {
                build_mono(
                    monos,
                    ti.parent_type_instance_index as usize,
                    type_instances,
                    types,
                    array_table,
                    read_string,
                )
                .context(format!("Build parent mono for {}", index))?;
                Some(
                    monos[ti.parent_type_instance_index as usize]
                        .as_ref()
                        .unwrap()
                        .clone(),
                )
            } else {
                None
            };

            let namespace = read_string(ty.namespace_offset)?;

            if !namespace.is_empty() && parent.is_some() {
                bail!("Parent collision");
            }

            let parent_string = if let Some(parent) = parent {
                parent.full_name
            } else {
                namespace
            };

            let mut full_name = parent_string + "." + &read_string(ty.name_offset)?;

            if ti.template_argument_list_offset != 0 {
                let mut template_argument_list =
                    &array_table[ti.template_argument_list_offset as usize..];
                let (template_type_instance_index, targ_count) =
                    template_argument_list.read_u32()?.bit_split((18, 14));

                if template_type_instance_index as usize != index {
                    let mut targs = vec![];
                    for _ in 0..targ_count {
                        let (targ_type_instance_id, _) =
                            template_argument_list.read_u32()?.bit_split((18, 14));
                        build_mono(
                            monos,
                            targ_type_instance_id as usize,
                            type_instances,
                            types,
                            array_table,
                            read_string,
                        )
                        .context(format!("Build targ mono for {}", index))?;
                        targs.push(
                            monos[targ_type_instance_id as usize]
                                .as_ref()
                                .unwrap()
                                .clone(),
                        );
                    }

                    let mut targs = targs.into_iter();

                    let insert_points: Vec<_> =
                        full_name.match_indices('`').map(|(i, _)| i).collect();
                    if insert_points.is_empty() {
                        bail!("Non template: {}", index);
                    }
                    let mut inserted = full_name[0..insert_points[0]].to_owned();

                    for (i, &insert_point) in insert_points.iter().enumerate() {
                        let digit_begin = insert_point + 1;
                        let mut digit_end = digit_begin;
                        while digit_end < full_name.len()
                            && full_name.as_bytes()[digit_end].is_ascii_digit()
                        {
                            digit_end += 1;
                        }
                        let count: usize = full_name[digit_begin..digit_end].parse()?;
                        let args: Vec<_> =
                            targs.by_ref().take(count).map(|a| a.full_name).collect();
                        if args.len() != count {
                            bail!("Not enough targ: {}", index);
                        }
                        inserted += "<";
                        inserted += &args.join(",");
                        inserted += ">";
                        let next_insert_point = if i == insert_points.len() - 1 {
                            full_name.len()
                        } else {
                            insert_points[i + 1]
                        };
                        inserted += &full_name[digit_end..next_insert_point];
                    }
                    if targs.next().is_some() {
                        bail!("Excessive targ: {}", index);
                    }
                    full_name = inserted;
                }
            }

            monos[index] = Some(Mono { full_name });

            Ok(())
        }

        for i in 0..type_instances.len() {
            build_mono(
                &mut monos,
                i,
                &type_instances,
                &types,
                &array_table,
                &read_string,
            )?;
        }

        for (i, type_instance) in type_instances.iter().enumerate() {
            println!("#################################\n$TI[{}]", i);

            println!("{{ {} }}", &monos[i].as_ref().unwrap().full_name);

            println!(
                "base type instance: {{ {} }} TI[{}]",
                &monos[type_instance.base_type_instance_index as usize]
                    .as_ref()
                    .unwrap()
                    .full_name,
                type_instance.base_type_instance_index
            );
            println!(
                "parent type instance: {{ {} }} TI[{}]",
                &monos[type_instance.parent_type_instance_index as usize]
                    .as_ref()
                    .unwrap()
                    .full_name,
                type_instance.parent_type_instance_index
            );
            println!("type: Type[{}]", type_instance.type_index);

            let ty = types
                .get(type_instance.type_index as usize)
                .context("Type index out of bound")?;

            println!("  -> name: {}", read_string(ty.name_offset)?);
            println!("  -> field_count: {}", ty.field_count);

            println!(
                "TemplateArgList: [{}]",
                type_instance.template_argument_list_offset
            );
            let mut template_argument_list =
                &array_table[type_instance.template_argument_list_offset as usize..];
            let (template_type_instance_id, targ_count) =
                template_argument_list.read_u32()?.bit_split((18, 14));
            println!(
                " --> Template = TypeInstance[{}]",
                template_type_instance_id
            );
            for _ in 0..targ_count {
                let (targ_type_instance_id, targ_high) =
                    template_argument_list.read_u32()?.bit_split((18, 14));
                println!(
                    " --> TypeInstance[{}], {}",
                    targ_type_instance_id, targ_high
                );
            }

            println!("InterfaceList[{}]", type_instance.interface_list_offset);
            let mut interface_list = &array_table[type_instance.interface_list_offset as usize..];
            let interface_count = interface_list.read_u32()?;
            for _ in 0..interface_count {
                let (interface_type_instance_id, interface_high) =
                    interface_list.read_u32()?.bit_split((18, 14));
                println!(
                    "  ->  {{ {} }} TI[{}], {}",
                    &monos[interface_type_instance_id as usize]
                        .as_ref()
                        .unwrap()
                        .full_name,
                    interface_type_instance_id,
                    interface_high,
                );
            }

            println!(
                "Field membership start index: {}",
                type_instance.field_membership_start_index
            );
            println!("Fields:");
            for j in 0..ty.field_count {
                let field_membership_index = type_instance.field_membership_start_index + j;
                let field_membership = field_memberships
                    .get(field_membership_index as usize)
                    .context("Field membership index out of bound")?;
                if field_membership.type_instance_index != i as u64 {
                    bail!("field_membership.type_instance_index mismatch")
                }
                println!(
                    "Membership[{}]: at {}",
                    field_membership_index, field_membership.position
                );
                let field = fields
                    .get(field_membership.field_index as usize)
                    .context("Field index out of bound")?;

                println!(
                    "  -> Field[{}]: {}, {}, {{ {} }} TI[{}], {}, {}",
                    field_membership.field_index,
                    field.a1,
                    field.a2,
                    &monos[field.type_instance_index as usize]
                        .as_ref()
                        .unwrap()
                        .full_name,
                    field.type_instance_index,
                    field.b_upper,
                    read_string(field.name_offset)?
                );
            }
        }

        /*for field_membership in field_memberships {
            let b = &type_instances[field_membership.b_index as usize];
            let type_name = read_string(types[b.type_index as usize].name_offset)?;
            let field = &fields[field_membership.field_index as usize];
            let field_name = read_string(field.name_offset)?;
            let sub_b = &type_instances[field.b_index as usize];
            let sub_type_name = read_string(types[sub_b.type_index as usize].name_offset)?;
            println!(
                "{} + {}: ({}) {}",
                type_name, field_membership.position, sub_type_name, field_name
            );
        }*/

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
