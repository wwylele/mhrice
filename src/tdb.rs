use crate::bitfield::*;
use crate::file_ext::*;
use anyhow::*;
use bitflags::*;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek};

bitflags! {
    struct FieldAttribute: u16 {
        const PRIVATE_SCOPE            = 0x0000;
        const PRIVATE                  = 0x0001;
        const FAM_AND_ASSEM            = 0x0002;
        const ASSEMBLY                 = 0x0003;
        const FAMILY                   = 0x0004;
        const FAM_OR_ASSEM             = 0x0005;
        const PUBLIC                   = 0x0006;
        const MEMBER_ACCESS_MASK       = 0x0007;

        const STATIC                   = 0x0010;
        const READONLY                 = 0x0020;
        const LITERAL                  = 0x0040;
        const NO_SERIALIZE             = 0x0080;
        const HAS_RVA                  = 0x0100;
        const SPECIAL                  = 0x0200;
        const RT_SPECIAL               = 0x0400;
        const POINTER                  = 0x0800;
        const MARSHAL                  = 0x1000;
        const PINVOKE                  = 0x2000;
        const UNK                      = 0x4000;
        const DEFAULT                  = 0x8000;
    }
}

bitflags! {
    struct ParamAttribute: u16 {
        const IN                = 0x0001;
        const OUT               = 0x0002;
        const LCID              = 0x0004;
        const RETVAL            = 0x0008;
        const OPTIONAL          = 0x0010;
        const HAS_DEFAULT       = 0x1000;
        const HAS_FIELD_MARSHAL = 0x2000;
    }
}

bitflags! {
    struct MethodAttribute: u16 {
        const PRIVATE_SCOPE            = 0x0000;
        const PRIVATE                  = 0x0001;
        const FAM_AND_ASSEM            = 0x0002;
        const ASSEMBLY                 = 0x0003;
        const FAMILY                   = 0x0004;
        const FAM_OR_ASSEM             = 0x0005;
        const PUBLIC                   = 0x0006;
        const MEMBER_ACCESS_MASK       = 0x0007;

        const UNMANAGED_EXPORT         = 0x0008;
        const STATIC                   = 0x0010;
        const FINAL                    = 0x0020;
        const VIRTUAL                  = 0x0040;
        const HIDE_BY_SIG              = 0x0080;
        const NEW_SLOT                 = 0x0100;
        const CHECK_ACCESS_ON_OVERRIDE = 0x0200;
        const ABSTRACT                 = 0x0400;
        const SPECIAL_NAME             = 0x0800;
        const RT_SPECIAL_NAME          = 0x1000;
        const PINVOKE_IMPL             = 0x2000;
        const HAS_SECURITY             = 0x4000;
        const REQUIRE_SEC_OBJECT       = 0x8000;
    }
}

fn display_field_attributes(attributes: FieldAttribute) -> String {
    let mut s = String::new();

    if attributes.contains(FieldAttribute::LITERAL) {
        s += "[literal]"
    }

    if attributes.contains(FieldAttribute::NO_SERIALIZE) {
        s += "[no-serialize]"
    }

    if attributes.contains(FieldAttribute::HAS_RVA) {
        s += "[has-rva]"
    }

    if attributes.contains(FieldAttribute::SPECIAL) {
        s += "[special]"
    }

    if attributes.contains(FieldAttribute::RT_SPECIAL) {
        s += "[rt-special]"
    }

    if attributes.contains(FieldAttribute::POINTER) {
        s += "[pointer]"
    }

    if attributes.contains(FieldAttribute::MARSHAL) {
        s += "[marshal]"
    }

    if attributes.contains(FieldAttribute::PINVOKE) {
        s += "[pinvoke]"
    }

    if attributes.contains(FieldAttribute::UNK) {
        s += "[4000]"
    }

    if attributes.contains(FieldAttribute::DEFAULT) {
        s += "[default]"
    }

    s += match attributes & FieldAttribute::MEMBER_ACCESS_MASK {
        FieldAttribute::PRIVATE_SCOPE => "[hidden]private ",
        FieldAttribute::PRIVATE => "private ",
        FieldAttribute::FAM_AND_ASSEM => "private protected ",
        FieldAttribute::ASSEMBLY => "internal ",
        FieldAttribute::FAMILY => "protected ",
        FieldAttribute::FAM_OR_ASSEM => "protected internal ",
        FieldAttribute::PUBLIC => "public ",
        FieldAttribute::MEMBER_ACCESS_MASK => "[public?] ",
        _ => panic!(),
    };

    if attributes.contains(FieldAttribute::STATIC) {
        s += "static "
    }

    if attributes.contains(FieldAttribute::READONLY) {
        s += "readonly "
    }

    s
}

fn display_method_attributes(attributes: MethodAttribute) -> String {
    let mut s = String::new();

    if attributes.contains(MethodAttribute::UNMANAGED_EXPORT) {
        s += "[export]"
    }

    if attributes.contains(MethodAttribute::HIDE_BY_SIG) {
        s += "[hid_by_sig]";
    }

    if attributes.contains(MethodAttribute::NEW_SLOT) {
        s += "[new_slot]";
    }

    if attributes.contains(MethodAttribute::CHECK_ACCESS_ON_OVERRIDE) {
        s += "[check_access_override]";
    }

    if attributes.contains(MethodAttribute::SPECIAL_NAME) {
        s += "[special]";
    }

    if attributes.contains(MethodAttribute::RT_SPECIAL_NAME) {
        s += "[rt_special]";
    }

    if attributes.contains(MethodAttribute::PINVOKE_IMPL) {
        s += "[pinvoke]";
    }

    if attributes.contains(MethodAttribute::HAS_SECURITY) {
        s += "[has_security]";
    }

    if attributes.contains(MethodAttribute::REQUIRE_SEC_OBJECT) {
        s += "[require_sec_object]";
    }

    s += match attributes & MethodAttribute::MEMBER_ACCESS_MASK {
        MethodAttribute::PRIVATE_SCOPE => "[hidden]private ",
        MethodAttribute::PRIVATE => "private ",
        MethodAttribute::FAM_AND_ASSEM => "private protected ",
        MethodAttribute::ASSEMBLY => "internal ",
        MethodAttribute::FAMILY => "protected ",
        MethodAttribute::FAM_OR_ASSEM => "protected internal ",
        MethodAttribute::PUBLIC => "public ",
        MethodAttribute::MEMBER_ACCESS_MASK => "[public?] ",
        _ => panic!(),
    };

    if attributes.contains(MethodAttribute::STATIC) {
        s += "static ";
    }

    if attributes.contains(MethodAttribute::FINAL) {
        s += "sealed ";
    }

    if attributes.contains(MethodAttribute::VIRTUAL) {
        s += "virtual ";
    }

    if attributes.contains(MethodAttribute::ABSTRACT) {
        s += "abstract ";
    }

    s
}

fn display_param_attributes(attributes: ParamAttribute) -> String {
    let mut s = String::new();
    if attributes.contains(ParamAttribute::IN) {
        s += "in";
    }
    if attributes.contains(ParamAttribute::OUT) {
        s += "out";
    }
    if attributes.contains(ParamAttribute::LCID) {
        s += "[lcid]";
    }
    if attributes.contains(ParamAttribute::RETVAL) {
        s += "[ret]";
    }
    if attributes.contains(ParamAttribute::OPTIONAL) {
        s += "[opt]";
    }
    if attributes.contains(ParamAttribute::HAS_DEFAULT) {
        s += "[default]";
    }
    if attributes.contains(ParamAttribute::HAS_FIELD_MARSHAL) {
        s += "[marshal]";
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
        let property_membership_count = file.read_u32()?;
        let event_count = file.read_u32()?;
        let param_count = file.read_u32()?;
        let attribute_count = file.read_u32()?;
        let constant_count = file.read_u32()?;
        let attribute_list_count = file.read_u32()?;
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
        let property_membership_offset = file.read_u64()?;
        let property_offset = file.read_u64()?;
        let event_offset = file.read_u64()?;
        let param_offset = file.read_u64()?;
        let attribute_offset = file.read_u64()?;
        let constant_offset = file.read_u64()?;
        let attribute_list_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;
        let heap_offset = file.read_u64()?;
        let q_offset = file.read_u64()?;

        struct Assembly {
            name_offset: u32,
            full_path_offset: u32,
            dll_name_offset: u32,
        }
        file.seek_noop(assembly_offset)?;
        let assemblies = (0..assembly_count)
            .map(|_| {
                file.read_u64()?;
                file.read_u64()?;

                file.read_u64()?;
                file.read_u64()?;

                file.read_u32()?;
                let name_offset = file.read_u32()?;
                let full_path_offset = file.read_u32()?;
                file.read_u32()?;

                let dll_name_offset = file.read_u32()?;
                file.read_u32()?;
                file.read_u64()?;

                file.read_u64()?;
                file.read_u64()?;

                file.read_u64()?;
                Ok(Assembly {
                    name_offset,
                    full_path_offset,
                    dll_name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct TypeInstance {
            base_type_instance_index: usize,
            parent_type_instance_index: usize,
            af: u64,
            arrayize_type_instance_index: usize,
            dearrayize_type_instance_index: usize,
            type_index: usize,
            special_type_id: u64,
            b: u32,
            interface_list_offset: usize,
            method_membership_start_index: usize,
            field_membership_start_index: usize,
            template_argument_list_offset: usize,
            hash: u32,
            j: u32,
            event_start_index: usize,
            event_count: usize,
            property_membership_start_index: usize,
            property_count: usize,
            default_ctor_method_membership_index: usize,
        }
        file.seek_assert_align_up(type_instance_offset, 16)?;
        let type_instances = (0..type_instance_count)
            .map(|i| {
                let (index, base_type_instance_index, parent_type_instance_index, af) =
                    file.read_u64()?.bit_split((18, 18, 18, 10));

                if index != u64::from(i) {
                    bail!("Unexpected index");
                }

                let (
                    arrayize_type_instance_index,
                    dearrayize_type_instance_index,
                    type_index,
                    special_type_id,
                ) = file.read_u64()?.bit_split((18, 18, 18, 10));

                let j = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected 0: {}", index);
                }
                let hash = file.read_u32()?;
                file.read_u32()?;

                let default_ctor_method_membership_index = file.read_u32()?;
                let b = file.read_u32()?;
                let method_membership_start_index = file.read_u32()?;
                let field_membership_start_index = file.read_u32()?;

                let (property_count, property_membership_start_index) =
                    file.read_u32()?.bit_split((12, 20));
                let (event_count, event_start_index) = file.read_u32()?.bit_split((12, 20));
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
                    special_type_id,
                    b,
                    interface_list_offset: interface_list_offset.try_into()?,
                    method_membership_start_index: method_membership_start_index.try_into()?,
                    field_membership_start_index: field_membership_start_index.try_into()?,
                    template_argument_list_offset: template_argument_list_offset.try_into()?,
                    hash,
                    j,
                    event_start_index: event_start_index.try_into()?,
                    event_count: event_count.try_into()?,
                    property_count: property_count.try_into()?,
                    property_membership_start_index: property_membership_start_index.try_into()?,
                    default_ctor_method_membership_index: default_ctor_method_membership_index
                        .try_into()?,
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
            attribute_list_index: usize,
            vtable_slot: i16,
            attributes: MethodAttribute,
            b2: u16,
            name_offset: u32,
        }
        file.seek_assert_align_up(method_offset, 16)?;
        let methods = (0..method_count)
            .map(|_| {
                let attribute_list_index = file.read_u16()?;
                let vtable_slot = file.read_i16()?;
                let attributes = file.read_u16()?;
                let b2 = file.read_u16()?;
                let name_offset = file.read_u32()?;
                Ok(Method {
                    attribute_list_index: attribute_list_index.try_into()?,
                    vtable_slot,
                    attributes: MethodAttribute::from_bits(attributes)
                        .context("Unknown method attr")?,
                    b2,
                    name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Field {
            attribute_list_index: usize,
            attributes: FieldAttribute,
            type_instance_index: usize,
            constant_index: usize,
            name_offset: u32,
        }
        file.seek_assert_align_up(field_offset, 16)?;
        let fields = (0..field_count)
            .map(|_| {
                let attribute_list_index = file.read_u16()?;
                let attributes = file.read_u16()?;
                let (type_instance_index, constant_index) = file.read_u32()?.bit_split((18, 14));
                let name_offset = file.read_u32()?;
                Ok(Field {
                    attribute_list_index: attribute_list_index.try_into()?,
                    attributes: FieldAttribute::from_bits(attributes)
                        .context("Unknown field attribute")?,
                    type_instance_index: type_instance_index.try_into()?,
                    constant_index: constant_index.try_into()?,
                    name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct PropertyMembership {
            property_index: usize,
            get_method_membership_index: usize,
            set_method_membership_index: usize,
        }
        file.seek_assert_align_up(property_membership_offset, 16)?;
        let property_memberships = (0..property_membership_count)
            .map(|_| {
                let (property_index, get_method_membership_index, set_method_membership_index) =
                    file.read_u64()?.bit_split((20, 22, 22));
                Ok(PropertyMembership {
                    property_index: property_index.try_into()?,
                    get_method_membership_index: get_method_membership_index.try_into()?,
                    set_method_membership_index: set_method_membership_index.try_into()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Property {
            a: u16,
            attribute_list_index: usize,
            name_offset: u32,
        }
        file.seek_assert_align_up(property_offset, 16)?;
        let properties = (0..property_count)
            .map(|_| {
                let a = file.read_u16()?;
                if a != 0 && a != 0x4000 {
                    bail!("Unexpected flag")
                }
                let attribute_list_index = file.read_u16()?;
                let name_offset = file.read_u32()?;
                Ok(Property {
                    a,
                    attribute_list_index: attribute_list_index.try_into()?,
                    name_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Event {
            name_offset: u32,
            add_method_membership_index: usize,
            remove_method_membership_index: usize,
        }
        file.seek_assert_align_up(event_offset, 16)?;
        let events = (0..event_count)
            .map(|_| {
                let a = file.read_u32()?;
                if a != 0 {
                    bail!("expected 0")
                }
                let name_offset = file.read_u32()?;
                let add_method_membership_index = file.read_u32()?;
                let remove_method_membership_index = file.read_u32()?;
                Ok(Event {
                    name_offset,
                    add_method_membership_index: add_method_membership_index.try_into()?,
                    remove_method_membership_index: remove_method_membership_index.try_into()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Attribute {
            ctor_method_index: usize,
            arguments_offset: usize,
        }
        file.seek_assert_align_up(attribute_offset, 16)?;
        let attributes = (0..attribute_count)
            .map(|_| {
                let ctor_method_index = file.read_u32()?;
                let arguments_offset = file.read_u32()?;
                Ok(Attribute {
                    ctor_method_index: ctor_method_index.try_into()?,
                    arguments_offset: arguments_offset.try_into()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        struct Param {
            attribute_list_index: usize,
            default_const_index: usize,
            name_offset: u32,
            no_high: u32,
            type_instance_index: usize,
            attribute: ParamAttribute,
        }
        file.seek_assert_align_up(param_offset, 16)?;
        let params = (0..param_count)
            .map(|_| {
                let attribute_list_index = file.read_u16()?;
                let default_const_index = file.read_u16()?;
                let (name_offset, no_high) = file.read_u32()?.bit_split((30, 2));
                let (type_instance_index, attribute) = file.read_u32()?.bit_split((18, 14));
                Ok(Param {
                    attribute_list_index: attribute_list_index.try_into()?,
                    default_const_index: default_const_index.try_into()?,
                    name_offset,
                    no_high,
                    type_instance_index: type_instance_index.try_into()?,
                    attribute: ParamAttribute::from_bits(u16::try_from(attribute)?)
                        .context("Unknown param attr")?,
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

        file.seek_assert_align_up(attribute_list_offset, 16)?;
        let attribute_lists = (0..attribute_list_count)
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
        let qs = (0..q_count)
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
                        let targ_type_instance_id: usize =
                            template_argument_list.read_u32()?.try_into()?;
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

        let print_attributes = |attribute_list_index: usize| -> Result<()> {
            fn read_vint<F: ReadExt>(mut f: F) -> Result<usize> {
                let a = f.read_u8()?.into();
                if a < 128 {
                    Ok(a)
                } else {
                    let b: usize = f.read_u8()?.into();
                    Ok(((a - 128) << 8) + b)
                }
            }

            let attribute_list = attribute_lists[attribute_list_index] as usize;
            let mut attribute_list = &heap[attribute_list..];
            let attribute_count = attribute_list.read_u16()?;
            let attribute_list = (0..attribute_count)
                .map(|_| attribute_list.read_u16())
                .collect::<Result<Vec<_>>>()?;
            for attribute in attribute_list {
                let attribute = &attributes[attribute as usize];
                let ctor = &method_memberships[attribute.ctor_method_index];
                let mut attribute_args = &heap[attribute.arguments_offset..];
                let args_len = read_vint(&mut attribute_args)?;
                let args_data = &attribute_args[0..args_len];
                print!(
                    "[{}({:?})]",
                    symbols[ctor.type_instance_index].as_ref().unwrap(),
                    args_data,
                );
            }
            Ok(())
        };

        let mut order: Vec<_> = (0..type_instances.len()).collect();
        order.sort_by_key(|&i| symbols[i].as_ref().unwrap());

        for i in order {
            let type_instance = &type_instances[i];
            // println!("/// $TI[{}]", i);
            let full_name = &symbols[i].as_ref().unwrap();
            let calc_hash = murmur3::murmur3_32(&mut full_name.as_bytes(), 0xFFFF_FFFF)?;
            if i != 0 && calc_hash != type_instance.hash {
                bail!("Mismatched hash for TI[{}]", i)
            }
            println!("/// % {:08X}", calc_hash);
            println!(
                "class {}: {}",
                full_name,
                symbols[type_instance.base_type_instance_index]
                    .as_ref()
                    .unwrap()
            );

            let mut interface_list = &heap[type_instance.interface_list_offset..];
            let interface_count = interface_list.read_u32()?;
            for _ in 0..interface_count {
                let (interface_type_instance_id, interface_vtable_slot_start) =
                    interface_list.read_u32()?.bit_split((18, 14));
                let interface_type_instance_id: usize = interface_type_instance_id.try_into()?;
                println!(
                    "    ,{} /* ^{} */",
                    &symbols[interface_type_instance_id].as_ref().unwrap(),
                    interface_vtable_slot_start,
                );
            }

            println!("{{");

            println!("    // Special = {}", type_instance.special_type_id);

            let ty = types
                .get(type_instance.type_index)
                .context("Type index out of bound")?;

            println!("    // size: {}", ty.len);

            println!(
                "    // s2={}, n4={}, n5={}, n7={}",
                ty.s2, ty.n4, ty.n5, /*ty.n6,*/ ty.n7
            );

            if type_instance.template_argument_list_offset != 0 {
                let mut template_argument_list =
                    &heap[type_instance.template_argument_list_offset..];
                let (template_type_instance_id, targ_count) =
                    template_argument_list.read_u32()?.bit_split((18, 14));
                let template_type_instance_id: usize = template_type_instance_id.try_into()?;
                println!(
                    "    // Template = {}",
                    symbols[template_type_instance_id].as_ref().unwrap()
                );
                if template_type_instance_id == i {
                    for _ in 0..targ_count {
                        let flag = template_argument_list.read_u32()?;
                        let name_offset = template_argument_list.read_u32()?;
                        println!(
                            "     // param {}, 0x{:08X}",
                            read_string(name_offset)?,
                            flag
                        );
                    }
                } else {
                    println!("    // Omitted ");
                    println!("}}");
                    println!();
                    continue;
                }
            }

            if type_instance.dearrayize_type_instance_index != 0 {
                println!("    // Omitted ");
                println!("}}");
                println!();
                continue;
            }

            if full_name.contains('!') {
                println!("    // Omitted ");
                println!("}}");
                println!();
                continue;
            }

            println!();
            println!("    /*** Method ***/");
            println!();
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

                if method.attribute_list_index != 0 {
                    print!("    ");
                    print_attributes(method.attribute_list_index)?;
                    println!();
                }

                let mut mp = &heap[method_membership.param_list_offset..];
                let param_count = mp.read_u16()?;
                let abi_id = mp.read_u16()?;
                let return_value_index = usize::try_from(mp.read_u32()?)?;
                let return_value = &params[return_value_index];
                if return_value.attribute_list_index != 0 {
                    println!("/* returns */");
                    print!("    ");
                    print_attributes(return_value.attribute_list_index)?;
                    println!();
                }

                println!(
                    "    /* ^{} , {}, {}*/ {} {} {} (",
                    method.vtable_slot,
                    method.b2,
                    return_value.no_high,
                    display_method_attributes(method.attributes),
                    symbols[return_value.type_instance_index].as_ref().unwrap(),
                    read_string(method.name_offset)?
                );

                for _ in 0..param_count {
                    let param_index = usize::try_from(mp.read_u32()?)?;
                    let param = &params[param_index];
                    print!("        ");
                    if param.attribute_list_index != 0 {
                        print_attributes(param.attribute_list_index)?;
                    }
                    print!(
                        "/*{}*/ {} {} {}",
                        param.no_high,
                        display_param_attributes(param.attribute),
                        symbols[param.type_instance_index].as_ref().unwrap(),
                        read_string(param.name_offset)?
                    );

                    if param.default_const_index != 0 {
                        let constant = constants[param.default_const_index];
                        match constant {
                            Constant::Integral(offset) => {
                                let field_type_instance =
                                    &type_instances[param.type_instance_index];
                                let len = types[field_type_instance.type_index].len;
                                let value = &heap[offset..][..len];
                                print!(" = {:?}", value);
                            }
                            Constant::String(offset) => {
                                let s = read_string(offset)?;
                                print!(" = \"{}\"", s);
                            }
                        }
                    }
                    println!(",");
                }

                println!("    );");
            }

            println!();
            println!("    /*** Field ***/");
            println!();
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

                if field.attribute_list_index != 0 {
                    print!("    ");
                    print_attributes(field.attribute_list_index)?;
                    println!();
                }

                print!(
                    "    /* +{} */ {} {} {}",
                    field_membership.position,
                    display_field_attributes(field.attributes),
                    &symbols[field.type_instance_index].as_ref().unwrap(),
                    read_string(field.name_offset)?
                );

                if field.constant_index != 0 {
                    let constant = constants[field.constant_index];
                    match constant {
                        Constant::Integral(offset) => {
                            let field_type_instance = &type_instances[field.type_instance_index];
                            let len = types[field_type_instance.type_index].len;
                            let value = &heap[offset..][..len];
                            print!(" = {:?}", value);
                        }
                        Constant::String(offset) => {
                            let s = read_string(offset)?;
                            print!(" = \"{}\"", s);
                        }
                    }
                }

                println!(";");
            }

            println!();
            println!("    /*** Event ***/");
            println!();
            for j in 0..type_instance.event_count {
                let event = &events[type_instance.event_start_index + j];
                println!("    public event {};", read_string(event.name_offset)?);
            }

            println!();
            println!("    /*** Property ***/");
            println!();
            for j in 0..type_instance.property_count {
                let property_membership =
                    &property_memberships[type_instance.property_membership_start_index + j];
                let property = &properties[property_membership.property_index];
                if property.attribute_list_index != 0 {
                    print!("    ");
                    print_attributes(property.attribute_list_index)?;
                    println!();
                }
                println!(
                    "    public property {};",
                    read_string(property.name_offset)?
                );
            }

            println!("}}");
            println!();
        }

        for q in qs {
            println!("// ~ {}", read_string(q)?);
        }

        for assembly in assemblies {
            println!(
                "// <Asm> {}, {}, {}",
                read_string(assembly.name_offset)?,
                read_string(assembly.full_path_offset)?,
                read_string(assembly.dll_name_offset)?
            );
        }

        Ok(Tdb {})
    }
}
