use crate::bitfield::*;
use crate::file_ext::*;
use anyhow::*;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek};

fn display_field_attributes(attributes: u16) -> String {
    let mut s = match attributes & 7 {
        0 => "private",
        1 => "private-parent",
        2 => "protected-asm",
        3 => "asm",
        4 => "protected",
        5 => "protected-or-asm",
        6 => "public",
        7 => "access?",
        _ => panic!(),
    }
    .to_owned();

    if attributes & 0x10 != 0 {
        s += " static"
    }

    if attributes & 0x20 != 0 {
        s += " readonly"
    }

    if attributes & 0x40 != 0 {
        s += " literal"
    }

    if attributes & 0x80 != 0 {
        s += " no-serialize"
    }

    if attributes & 0x100 != 0 {
        s += " has-rva"
    }

    if attributes & 0x200 != 0 {
        s += " special"
    }

    if attributes & 0x400 != 0 {
        s += " rt-special"
    }

    if attributes & 0x800 != 0 {
        s += " 800"
    }

    if attributes & 0x1000 != 0 {
        s += " marshal"
    }

    if attributes & 0x2000 != 0 {
        s += " pinvoke"
    }

    if attributes & 0x4000 != 0 {
        s += " 4000"
    }

    if attributes & 0x8000 != 0 {
        s += " default"
    }

    s
}

pub struct Tdb {}

impl Tdb {
    #[allow(unused_variables, dead_code)]
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
        let method_membership_count = file.read_u32()?;
        let field_membership_count = file.read_u32()?;
        let type_count = file.read_u32()?;
        let field_count = file.read_u32()?;
        let method_count = file.read_u32()?;
        let property_count = file.read_u32()?;
        let h_count = file.read_u32()?;
        let j_count = file.read_u32()?;
        let param_count = file.read_u32()?;
        let l_count = file.read_u32()?;
        let constant_count = file.read_u32()?;
        let n_count = file.read_u32()?;
        let q_count = file.read_u32()?;
        let assembly_count = file.read_u32()?;

        if file.read_u32()? != 0 {
            bail!("Expected 0");
        }

        let unknown = file.read_u32()?;
        let string_table_len = file.read_u32()?;
        let heap_len = file.read_u32()?;

        let assembly_offset = file.read_u64()?;
        let type_instance_offset = file.read_u64()?;
        let type_offset = file.read_u64()?;
        let method_membership_offset = file.read_u64()?;
        let method_offset = file.read_u64()?;
        let field_membership_offset = file.read_u64()?;
        let field_offset = file.read_u64()?;
        let h_offset = file.read_u64()?;
        let property_offset = file.read_u64()?;
        let j_offset = file.read_u64()?;
        let param_offset = file.read_u64()?;
        let l_offset = file.read_u64()?;
        let constant_offset = file.read_u64()?;
        let n_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;
        let heap_offset = file.read_u64()?;
        let q_offset = file.read_u64()?;

        file.seek_noop(assembly_offset)?;
        (0..assembly_count)
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
            base_type_instance_index: usize,
            parent_type_instance_index: usize,
            af: u64,
            arrayize_type_instance_index: usize,
            dearrayize_type_instance_index: usize,
            type_index: usize,
            bf: u64,
            b: u32,
            c: u32,
            interface_list_offset: usize,
            method_membership_start_index: usize,
            field_membership_start_index: usize,
            template_argument_list_offset: usize,
            hash: u32,
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
                let hash = file.read_u32()?;
                file.read_u32()?;

                file.read_u32()?;
                let b = file.read_u32()?;
                let method_membership_start_index = file.read_u32()?;
                let field_membership_start_index = file.read_u32()?;

                file.read_u32()?;
                let c = file.read_u32()?;
                let interface_list_offset = file.read_u32()?;
                let template_argument_list_offset = file.read_u32()?;

                let x = file.read_u64()?;
                if x != 0 {
                    bail!("Expected 0: {}", index);
                }
                let x = file.read_u64()?;
                if x != 0 {
                    bail!("Expected 0: {}", index);
                }
                Ok(TypeInstance {
                    base_type_instance_index: base_type_instance_index.try_into()?,
                    parent_type_instance_index: parent_type_instance_index.try_into()?,
                    af,
                    arrayize_type_instance_index: arrayize_type_instance_index.try_into()?,
                    dearrayize_type_instance_index: dearrayize_type_instance_index.try_into()?,
                    type_index: type_index.try_into()?,
                    bf,
                    b,
                    c,
                    interface_list_offset: interface_list_offset.try_into()?,
                    method_membership_start_index: method_membership_start_index.try_into()?,
                    field_membership_start_index: field_membership_start_index.try_into()?,
                    template_argument_list_offset: template_argument_list_offset.try_into()?,
                    hash,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct MethodMembership {
            type_instance_index: usize,
            method_index: usize,
            param_list_offset: usize,
        }
        file.seek_assert_align_up(method_membership_offset, 16)?;
        let method_memberships = (0..method_membership_count)
            .map(|_| {
                let (type_instance_index, method_index, param_list_offset) =
                    file.read_u64()?.bit_split((18, 20, 26));
                let zero = file.read_u64()?;
                if zero != 0 {
                    bail!("Expected 0")
                }
                Ok(MethodMembership {
                    type_instance_index: type_instance_index.try_into()?,
                    method_index: method_index.try_into()?,
                    param_list_offset: param_list_offset.try_into()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct FieldMembership {
            type_instance_index: usize,
            field_index: usize,
            position: u64,
        }
        file.seek_assert_align_up(field_membership_offset, 16)?;
        let field_memberships = (0..field_membership_count)
            .map(|_| {
                let (type_instance_index, field_index, position) =
                    file.read_u64()?.bit_split((18, 20, 26));
                Ok(FieldMembership {
                    type_instance_index: type_instance_index.try_into()?,
                    field_index: field_index.try_into()?,
                    position,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Type {
            name_offset: u32,
            namespace_offset: u32,
            len: usize,
            s2: u32,

            assembly_index: u8,
            array_dimension: u8,
            method_count: usize,

            field_count: usize,

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

                let assembly_index = file.read_u8()?;
                let array_dimension = file.read_u8()?;
                let method_count = file.read_u16()?;
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
                    len: len.try_into()?,
                    s2,
                    assembly_index,
                    array_dimension,
                    method_count: method_count.try_into()?,
                    field_count: field_count.try_into()?,
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
            a1: u16,
            vtable_slot: i16,
            b1: u16,
            b2: u16,
            name_offset: u32,
        }
        file.seek_assert_align_up(method_offset, 16)?;
        let methods = (0..method_count)
            .map(|_| {
                let a1 = file.read_u16()?;
                let vtable_slot = file.read_i16()?;
                let b1 = file.read_u16()?;
                let b2 = file.read_u16()?;
                let name_offset = file.read_u32()?;
                Ok(Method {
                    a1,
                    vtable_slot,
                    b1,
                    b2,
                    name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Field {
            a1: u16,
            attributes: u16,
            type_instance_index: usize,
            constant_index: usize,
            name_offset: u32,
        }
        file.seek_assert_align_up(field_offset, 16)?;
        let fields = (0..field_count)
            .map(|_| {
                let a1 = file.read_u16()?;
                let attributes = file.read_u16()?;
                let (type_instance_index, constant_index) = file.read_u32()?.bit_split((18, 14));
                let name_offset = file.read_u32()?;
                Ok(Field {
                    a1,
                    attributes,
                    type_instance_index: type_instance_index.try_into()?,
                    constant_index: constant_index.try_into()?,
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
            k: u16,
            a: u16,
            name_offset: u32,
            no_high: u32,
            type_instance_index: usize,
            ti_high: u32,
        }
        file.seek_assert_align_up(param_offset, 16)?;
        let params = (0..param_count)
            .map(|_| {
                let k = file.read_u16()?;
                let a = file.read_u16()?;
                let (name_offset, no_high) = file.read_u32()?.bit_split((30, 2));
                let (type_instance_index, ti_high) = file.read_u32()?.bit_split((18, 14));
                Ok(Param {
                    k,
                    a,
                    name_offset,
                    no_high,
                    type_instance_index: type_instance_index.try_into()?,
                    ti_high,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        #[derive(Clone, Copy)]
        enum Constant {
            Integral(usize),
            String(u32),
        }

        file.seek_assert_align_up(constant_offset, 16)?;
        let constants = (0..constant_count)
            .map(|_| {
                let raw = file.read_i32()?;
                Ok(if raw >= 0 {
                    Constant::Integral(usize::try_from(raw)?)
                } else {
                    Constant::String(u32::try_from(-raw)?)
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(n_offset, 16)?;
        let ns = (0..n_count)
            .map(|_| file.read_u32())
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

        file.seek_assert_align_up(heap_offset, 16)?;
        let mut heap = vec![0; heap_len.try_into()?];
        file.read_exact(&mut heap)?;

        file.seek_assert_align_up(q_offset, 16)?;
        let _ = (0..q_count)
            .map(|_| Ok(file.read_u32()?))
            .collect::<Result<Vec<_>>>()?;

        let mut symbols: Vec<Option<String>> = vec![None; type_instances.len()];

        fn build_symbol(
            symbols: &mut [Option<String>],
            index: usize,
            type_instances: &[TypeInstance],
            types: &[Type],
            heap: &[u8],
            read_string: impl Fn(u32) -> Result<String> + Copy,
        ) -> Result<String> {
            if index > symbols.len() {
                bail!("Index out of bound: {}", index);
            }
            if let Some(s) = &symbols[index] {
                return Ok(s.clone());
            }

            if index == 0 {
                symbols[index] = Some("".to_string());
                return Ok("".to_string());
            }

            let ti = &type_instances[index];
            let ty = types
                .get(ti.type_index)
                .context("Type index out of bound")?;

            if ti.dearrayize_type_instance_index != 0 {
                let element_name = build_symbol(
                    symbols,
                    ti.dearrayize_type_instance_index,
                    type_instances,
                    types,
                    heap,
                    read_string,
                )
                .context(format!("Build dearrayize symbol for {}", index))?;

                let mut suffix = "[".to_string();
                for _ in 1..ty.array_dimension {
                    suffix += ","
                }
                suffix += "]";

                let full_name = element_name + &suffix;

                symbols[index] = Some(full_name.clone());

                return Ok(full_name);
            }

            let parent = if ti.parent_type_instance_index != 0 {
                Some(
                    build_symbol(
                        symbols,
                        ti.parent_type_instance_index,
                        type_instances,
                        types,
                        heap,
                        read_string,
                    )
                    .context(format!("Build parent symbol for {}", index))?,
                )
            } else {
                None
            };

            let namespace = read_string(ty.namespace_offset)?;

            if !namespace.is_empty() && parent.is_some() {
                bail!("Parent collision");
            }

            let parent_string = if let Some(parent) = parent {
                parent + "."
            } else if !namespace.is_empty() {
                namespace + "."
            } else {
                "".to_string()
            };

            let mut full_name = parent_string + &read_string(ty.name_offset)?;

            if ti.template_argument_list_offset != 0 {
                let mut template_argument_list = &heap[ti.template_argument_list_offset..];
                let (template_type_instance_index, targ_count) =
                    template_argument_list.read_u32()?.bit_split((18, 14));

                let template_type_instance_index: usize =
                    template_type_instance_index.try_into()?;
                if template_type_instance_index != index {
                    let mut targs = vec![];
                    for _ in 0..targ_count {
                        let (targ_type_instance_id, _) =
                            template_argument_list.read_u32()?.bit_split((18, 14));
                        let targ_type_instance_id = targ_type_instance_id.try_into()?;
                        let targ = build_symbol(
                            symbols,
                            targ_type_instance_id,
                            type_instances,
                            types,
                            heap,
                            read_string,
                        )
                        .context(format!("Build targ symbol for {}", index))?;
                        targs.push(targ);
                    }

                    full_name += "<";
                    full_name += &targs.join(",");
                    full_name += ">";
                }
            }

            symbols[index] = Some(full_name.clone());

            Ok(full_name)
        }

        for i in 0..type_instances.len() {
            build_symbol(
                &mut symbols,
                i,
                &type_instances,
                &types,
                &heap,
                &read_string,
            )?;
        }

        for (i, type_instance) in type_instances.iter().enumerate() {
            println!("#################################\n$TI[{}]", i);
            let full_name = &symbols[i].as_ref().unwrap();
            let calc_hash = murmur3::murmur3_32(&mut full_name.as_bytes(), 0xFFFF_FFFF)?;
            if i != 0 && calc_hash != type_instance.hash {
                bail!("Mismatched hash for TI[{}]", i)
            }
            println!("% {:08X}", calc_hash);
            println!("${{ {} }}", full_name);

            println!(
                "extends {}",
                &symbols[type_instance.base_type_instance_index]
                    .as_ref()
                    .unwrap(),
            );
            println!(
                "belongs to {}",
                &symbols[type_instance.parent_type_instance_index]
                    .as_ref()
                    .unwrap(),
            );
            println!("type: Type[{}]", type_instance.type_index);

            let ty = types
                .get(type_instance.type_index)
                .context("Type index out of bound")?;

            println!("  -> name: {}", read_string(ty.name_offset)?);
            println!("  -> size: {}", ty.len);

            println!(
                "TemplateArgList: [{}]",
                type_instance.template_argument_list_offset
            );
            let mut template_argument_list = &heap[type_instance.template_argument_list_offset..];
            let (template_type_instance_id, targ_count) =
                template_argument_list.read_u32()?.bit_split((18, 14));
            println!(" --> Template = TI[{}]", template_type_instance_id);
            for _ in 0..targ_count {
                let (targ_type_instance_id, targ_high) =
                    template_argument_list.read_u32()?.bit_split((18, 14));
                println!(" --> TI[{}], {}", targ_type_instance_id, targ_high);
            }

            println!("implements[{}]", type_instance.interface_list_offset);
            let mut interface_list = &heap[type_instance.interface_list_offset..];
            let interface_count = interface_list.read_u32()?;
            for _ in 0..interface_count {
                let (interface_type_instance_id, interface_vtable_slot_start) =
                    interface_list.read_u32()?.bit_split((18, 14));
                let interface_type_instance_id: usize = interface_type_instance_id.try_into()?;
                println!(
                    "  ->  {}, ^{}",
                    &symbols[interface_type_instance_id].as_ref().unwrap(),
                    interface_vtable_slot_start,
                );
            }

            println!(
                "Method membership start index: {}",
                type_instance.method_membership_start_index
            );
            for j in 0..ty.method_count {
                let method_membership_index = type_instance.method_membership_start_index + j;
                let method_membership = method_memberships
                    .get(method_membership_index)
                    .context("Method membership index out of bound")?;
                if method_membership.type_instance_index != i {
                    bail!("method_membership.type_instance_index mismatch");
                }

                let method = methods
                    .get(method_membership.method_index)
                    .context("Method index out of bound")?;

                println!(
                    "-> {}, ^{}, {}, {}, {}",
                    method.a1,
                    method.vtable_slot,
                    method.b1,
                    method.b2,
                    read_string(method.name_offset)?
                );

                let mut mp = &heap[method_membership.param_list_offset..];
                let param_count = mp.read_u16()?;
                let hmm = mp.read_u16()?;
                let return_value_index = usize::try_from(mp.read_u32()?)?;
                let return_value = &params[return_value_index];
                println!(
                    "      !=> {} |return| [{}{}, {}{}, {}, {}] {} {}",
                    hmm,
                    return_value.k,
                    if return_value.k != 0 { "*$*" } else { "" },
                    return_value.a,
                    if return_value.a != 0 { "*%*" } else { "" },
                    return_value.no_high,
                    return_value.ti_high,
                    symbols[return_value.type_instance_index].as_ref().unwrap(),
                    read_string(return_value.name_offset)?
                );
                for _ in 0..param_count {
                    let param_index = usize::try_from(mp.read_u32()?)?;
                    let param = &params[param_index];
                    println!(
                        "      ==> [{}{}, {}{}, {}, {}] {} {}",
                        param.k,
                        if param.k != 0 { "*$*" } else { "" },
                        param.a,
                        if param.a != 0 { "*%*" } else { "" },
                        param.no_high,
                        param.ti_high,
                        symbols[param.type_instance_index].as_ref().unwrap(),
                        read_string(param.name_offset)?
                    );
                }
            }

            println!(
                "Field membership start index: {}",
                type_instance.field_membership_start_index
            );
            println!("Fields:");
            for j in 0..ty.field_count {
                let field_membership_index = type_instance.field_membership_start_index + j;
                let field_membership = field_memberships
                    .get(field_membership_index)
                    .context("Field membership index out of bound")?;
                if field_membership.type_instance_index != i {
                    bail!("field_membership.type_instance_index mismatch")
                }

                let field = fields
                    .get(field_membership.field_index)
                    .context("Field index out of bound")?;

                println!(
                    "@{} -> {}, {{ {} }}, {}",
                    field_membership.position,
                    display_field_attributes(field.attributes),
                    &symbols[field.type_instance_index].as_ref().unwrap(),
                    read_string(field.name_offset)?
                );

                if field.a1 != 0 {
                    let n = ns[field.a1 as usize] as usize;
                    let nn = &heap[n..][..4];
                    println!("    -> N[{}]: {:?}", n, nn);
                }

                if field.constant_index != 0 {
                    let constant = constants[field.constant_index];
                    match constant {
                        Constant::Integral(offset) => {
                            let field_type_instance = &type_instances[field.type_instance_index];
                            let len = types[field_type_instance.type_index].len;
                            let value = &heap[offset..][..len];
                            println!("    -> Value: {:?}", value);
                        }
                        Constant::String(offset) => {
                            let s = read_string(offset)?;
                            println!("    -> Value: \"{}\"", s);
                        }
                    }
                }
            }
        }

        Ok(Tdb {})
    }
}
