use crate::bitfield::*;
use crate::file_ext::*;
use crate::hash::*;
use anyhow::{bail, Context, Result};
use bitflags::*;
use std::collections::*;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{Read, Seek, Write};

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
        const EXPOSE_MEMBER            = 0x4000;
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

bitflags! {
    struct TypeFlag: u32 {
        const NOT_PUBLIC           = 0x00000000;
        const PUBLIC               = 0x00000001;
        const NESTED_PUBLIC        = 0x00000002;
        const NESTED_PRIVATE       = 0x00000003;
        const NESTED_FAMILY        = 0x00000004;
        const NESTED_ASSEMBLY      = 0x00000005;
        const NESTED_FAMANDASSEM   = 0x00000006;
        const NESTED_FAMORASSEM    = 0x00000007;
        const VISIBILITY_MASK      = 0x00000007;

        const AUTO_LAYOUT          = 0x00000000;
        const SEQUENTIAL_LAYOUT    = 0x00000008;
        const EXPLICIT_LAYOUT      = 0x00000010;
        const LAYOUT_MASK          = 0x00000018;

        const INTERFACE            = 0x00000020;
        // no 0x0040
        const ABSTRACT             = 0x00000080;
        const SEALED               = 0x00000100;
        // no 0x0200
        const SPECIAL_NAME         = 0x00000400;
        const RT_SPECIAL_NAME      = 0x00000800;
        const IMPORT               = 0x00001000;
        const SERIALIZABLE         = 0x00002000;
        const WINDOWS_RUNTIME      = 0x00004000;
        // no 0x8000

        const ANSI_CLASS           = 0x00000000;
        const UNICODE_CLASS        = 0x00010000;
        const AUTO_CLASS           = 0x00020000;
        const CUSTOM_FORMAT_CLASS  = 0x00030000;
        const STRING_FORMAT_MASK   = 0x00030000;

        const HAS_SECURITY         = 0x00040000;
        // no 0x080000
        const BEFORE_FIELD_INIT    = 0x00100000;
        // no 0x200000
        const CUSTOM_FORMAT_MASK   = 0x00C00000;
        const LOCAL_HEAP           = 0x01000000;
        const FINALIZE             = 0x02000000;
        const NATIVE_TYPE          = 0x04000000;
        const UNK_08000000         = 0x08000000;
        const NATIVE_CTOR          = 0x10000000;
        // no 0x20000000
        const MANAGED_VTABLE       = 0x40000000;
    }
}

bitflags! {
    struct MethodImplFlag: u16 {
        const CODE_TYPE_MASK              = 0x0003;
        const IL                          = 0x0000;
        const NATIVE                      = 0x0001;
        const OPTIL                       = 0x0002;
        const RUNTIME                     = 0x0003;
        const UNMANAGED                   = 0x0004;
        const NO_INLINING                 = 0x0008;
        const FORWARD_REF                 = 0x0010;
        const SYNCHRONIZED                = 0x0020;
        const NO_OPTMIZATION              = 0x0040;
        const PRESERVE_SIG                = 0x0080;
        const AGGRESSIVE_INLINING         = 0x0100;
        const HAS_RET_VAL                 = 0x0200;
        const EXPOSE_MEMBER               = 0x0400;
        const EMPTY_CTOR                  = 0x0800;
        const INTERNAL_CALL               = 0x1000;
        const CONTAINS_GENERIC_PARAMETERS = 0x2000;
        const HAS_THIS                    = 0x4000;
        const THREAD_SAFE                 = 0x8000;
    }
}

bitflags! {
    struct PropertyFlag: u16 {
        const SPECIAL_NAME    = 0x0200;
        const RT_SPECIAL_NAME = 0x0400;
        const HAS_DEFAULT     = 0x1000;
        const EXPOSE_MEMBER   = 0x4000;
    }
}

fn display_property_flag(flags: PropertyFlag) -> String {
    let mut s = String::new();
    if flags.contains(PropertyFlag::SPECIAL_NAME) {
        s += "[special]";
    }
    if flags.contains(PropertyFlag::RT_SPECIAL_NAME) {
        s += "[rt_special]";
    }
    if flags.contains(PropertyFlag::HAS_DEFAULT) {
        s += "[default]";
    }
    if flags.contains(PropertyFlag::EXPOSE_MEMBER) {
        s += "[expose]";
    }
    s
}

fn display_param_modifier(param_modifier: u32, return_pos: bool) -> String {
    let return_pos = if return_pos { "return:" } else { "" };
    let tag = match param_modifier {
        0 => return "".to_string(),
        1 => "ptr",
        2 => "ref",
        _ => "unknown-mod",
    };
    format!("[{}{}]", return_pos, tag)
}

fn display_type_flags(flags: TypeFlag) -> String {
    let mut s = String::new();

    s += match flags & TypeFlag::LAYOUT_MASK {
        TypeFlag::AUTO_LAYOUT => "[auto]",
        TypeFlag::SEQUENTIAL_LAYOUT => "[sequential]",
        TypeFlag::EXPLICIT_LAYOUT => "[explicit]",
        _ => "[unknown_layout]",
    };

    if flags.contains(TypeFlag::INTERFACE) {
        s += "[interface]"
    }
    if flags.contains(TypeFlag::ABSTRACT) {
        s += "[abstract]"
    }
    if flags.contains(TypeFlag::SEALED) {
        s += "[sealed]"
    }
    if flags.contains(TypeFlag::SPECIAL_NAME) {
        s += "[special]"
    }
    if flags.contains(TypeFlag::RT_SPECIAL_NAME) {
        s += "[rt_special]"
    }
    if flags.contains(TypeFlag::IMPORT) {
        s += "[import]"
    }
    if flags.contains(TypeFlag::SERIALIZABLE) {
        s += "[serializable]"
    }
    if flags.contains(TypeFlag::WINDOWS_RUNTIME) {
        s += "[windows_runtime]"
    }

    s += match flags & TypeFlag::STRING_FORMAT_MASK {
        TypeFlag::ANSI_CLASS => "[ansi]",
        TypeFlag::UNICODE_CLASS => "[unicode]",
        TypeFlag::AUTO_CLASS => "[auto_format]",
        TypeFlag::CUSTOM_FORMAT_CLASS => "[custom_format]",
        _ => panic!(),
    };
    if flags.contains(TypeFlag::HAS_SECURITY) {
        s += "[has_security]"
    }
    if flags.contains(TypeFlag::BEFORE_FIELD_INIT) {
        s += "[before_field_init]"
    }
    if flags.contains(TypeFlag::LOCAL_HEAP) {
        s += "[local_heap]"
    }
    if flags.contains(TypeFlag::FINALIZE) {
        s += "[finalize]"
    }
    if flags.contains(TypeFlag::NATIVE_TYPE) {
        s += "[native]"
    }
    if flags.contains(TypeFlag::UNK_08000000) {
        s += "[UNK_08000000]"
    }
    if flags.contains(TypeFlag::NATIVE_CTOR) {
        s += "[native_ctor]"
    }
    if flags.contains(TypeFlag::MANAGED_VTABLE) {
        s += "[managed_vtable]"
    }

    s += match flags & TypeFlag::VISIBILITY_MASK {
        TypeFlag::NOT_PUBLIC => "",
        TypeFlag::PUBLIC => "public ",
        TypeFlag::NESTED_PUBLIC => "[nested]public",
        TypeFlag::NESTED_PRIVATE => "[nested]private",
        TypeFlag::NESTED_FAMILY => "[nested]protected",
        TypeFlag::NESTED_ASSEMBLY => "[nested]internal",
        TypeFlag::NESTED_FAMANDASSEM => "[nested]private protected",
        TypeFlag::NESTED_FAMORASSEM => "[nested]protected internal",
        _ => panic!(),
    };

    s
}

fn display_field_attributes(attributes: FieldAttribute) -> String {
    let mut s = String::new();

    if attributes.contains(FieldAttribute::LITERAL) {
        s += "[literal]"
    }

    if attributes.contains(FieldAttribute::NO_SERIALIZE) {
        s += "[no_serialize]"
    }

    if attributes.contains(FieldAttribute::HAS_RVA) {
        s += "[has_rva]"
    }

    if attributes.contains(FieldAttribute::SPECIAL) {
        s += "[special]"
    }

    if attributes.contains(FieldAttribute::RT_SPECIAL) {
        s += "[rt_special]"
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

    if attributes.contains(FieldAttribute::EXPOSE_MEMBER) {
        s += "[expose]"
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

fn display_method_impl_flag(attributes: MethodImplFlag) -> String {
    let mut s = String::new();

    s += match attributes & MethodImplFlag::CODE_TYPE_MASK {
        MethodImplFlag::IL => "[il]",
        MethodImplFlag::NATIVE => "[native]",
        MethodImplFlag::OPTIL => "[optil]",
        MethodImplFlag::RUNTIME => "[runtime]",
        _ => panic!(),
    };

    if attributes.contains(MethodImplFlag::UNMANAGED) {
        s += "[unmanaged]"
    }

    if attributes.contains(MethodImplFlag::NO_INLINING) {
        s += "[no_inline]"
    }

    if attributes.contains(MethodImplFlag::FORWARD_REF) {
        s += "[forward_ref]"
    }
    if attributes.contains(MethodImplFlag::SYNCHRONIZED) {
        s += "[synchronized]"
    }
    if attributes.contains(MethodImplFlag::NO_OPTMIZATION) {
        s += "[no_optimization]"
    }
    if attributes.contains(MethodImplFlag::PRESERVE_SIG) {
        s += "[preserve_sig]"
    }
    if attributes.contains(MethodImplFlag::AGGRESSIVE_INLINING) {
        s += "[inline]"
    }
    if attributes.contains(MethodImplFlag::HAS_RET_VAL) {
        s += "[ret]"
    }
    if attributes.contains(MethodImplFlag::EXPOSE_MEMBER) {
        s += "[expose]"
    }
    if attributes.contains(MethodImplFlag::EMPTY_CTOR) {
        s += "[empty_ctor]"
    }
    if attributes.contains(MethodImplFlag::INTERNAL_CALL) {
        s += "[internal_call]"
    }
    if attributes.contains(MethodImplFlag::CONTAINS_GENERIC_PARAMETERS) {
        s += "[generic]"
    }
    if attributes.contains(MethodImplFlag::HAS_THIS) {
        s += "[has_this]"
    }
    if attributes.contains(MethodImplFlag::THREAD_SAFE) {
        s += "[thread_safe]"
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

/*
fn print_bytes(b: &[u8]) {
    for b in b {
        print!("{:02x} ", b);
    }
    println!()
}

fn analyze_bits<F: Read + Seek>(mut file: F, offset: u64, count: u32, item_byte_len: usize) {
    println!();
    println!("========Analyzing {offset}===========");
    file.seek_assert_align_up(offset, 16).unwrap();
    let mut counter = vec![0; item_byte_len * 8];
    for _ in 0..count {
        let mut buf = vec![0; item_byte_len];
        file.read_exact(&mut buf).unwrap();
        for i in 0..item_byte_len * 8 {
            if (buf[i / 8] >> (i % 8)) & 1 != 0 {
                counter[i] += 1;
            }
        }
    }

    for (i, c) in counter.into_iter().enumerate() {
        if i % 64 == 0 {
            println!();
        } else if i % 8 == 0 {
            print!("  ");
        }

        let mut freq = ((c as f32 / count as f32) * 100.0).round() as u32;
        if freq > 99 {
            freq = 99;
        }
        if freq == 0 && c > 1 {
            freq = 1;
        }
        print!("{freq:02} ");
    }
    println!()
}
*/

#[allow(unused_variables, dead_code)]
pub fn print<F: Read + Seek>(
    mut file: F,
    base_address: u64,
    map: Option<String>,
    options: crate::TdbOptions,
) -> Result<()> {
    if &file.read_magic()? != b"TDB\0" {
        bail!("Wrong magic for TDB file");
    }

    if file.read_u32()? != 0x47 {
        bail!("Wrong version for TDB file");
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
    let (attribute_list_count, data_attribute_list_count) = file.read_u32()?.bit_split((16, 16));
    let q_count = file.read_u32()?;
    let assembly_count = file.read_u32()?;
    if file.read_u32()? != 0 {
        bail!("Expected 0");
    }
    let unknown = file.read_u32()?;
    let string_table_len = file.read_u32()?;
    let heap_len = file.read_u32()?;

    println!("// type_instance_count = {}", type_instance_count);
    println!("// method_membership_count = {}", method_membership_count);
    println!("// field_membership_count = {}", field_membership_count);
    println!("// type_count = {}", type_count);
    println!("// field_count = {}", field_count);
    println!("// method_count = {}", method_count);
    println!("// property_count = {}", property_count);
    println!(
        "// property_membership_count = {}",
        property_membership_count
    );
    println!("// event_count = {}", event_count);
    println!("// param_count = {}", param_count);
    println!("// attribute_count = {}", attribute_count);
    println!("// constant_count = {}", constant_count);
    println!("// attribute_list_count = {}", attribute_list_count);
    println!(
        "// data_attribute_list_count = {}",
        data_attribute_list_count
    );
    println!("// q_count = {}", q_count);
    println!("// assembly_count = {}", assembly_count);
    println!("// string_table_len = {}", string_table_len);
    println!("// heap_len = {}", heap_len);

    let assembly_offset = file.read_u64()? - base_address;
    let type_instance_offset = file.read_u64()? - base_address;
    let type_offset = file.read_u64()? - base_address;
    let method_membership_offset = file.read_u64()? - base_address;
    let method_offset = file.read_u64()? - base_address;
    let field_membership_offset = file.read_u64()? - base_address;
    let field_offset = file.read_u64()? - base_address;
    let property_membership_offset = file.read_u64()? - base_address;
    let property_offset = file.read_u64()? - base_address;
    let event_offset = file.read_u64()? - base_address;
    let param_offset = file.read_u64()? - base_address;
    let attribute_offset = file.read_u64()? - base_address;
    let constant_offset = file.read_u64()? - base_address;
    let attribute_list_offset = file.read_u64()? - base_address;
    let data_attribute_list_offset = file.read_u64()? - base_address;
    let string_table_offset = file.read_u64()? - base_address;
    let heap_offset = file.read_u64()? - base_address;
    let q_offset = file.read_u64()? - base_address;
    let _ = file.read_u64()?;

    println!("// type_instance_offset = {type_instance_offset}");
    println!("// type_offset = {type_offset}");
    println!("// method_membership_offset = {method_membership_offset}");
    println!("// method_offset = {method_offset}");
    println!("// field_membership_offset = {field_membership_offset}");
    println!("// field_offset = {field_offset}");
    println!("// property_membership_offset = {property_membership_offset}");
    println!("// property_offset = {property_offset}");
    println!("// event_offset = {event_offset}");
    println!("// param_offset = {param_offset}");
    println!("// attribute_offset = {attribute_offset}");
    println!("// constant_offset = {constant_offset}");
    println!("// attribute_list_offset = {attribute_list_offset}");
    println!("// data_attribute_list_offset = {data_attribute_list_offset}");
    println!("// string_table_offset = {string_table_offset}");
    println!("// heap_offset = {heap_offset}");
    println!("// q_offset = {q_offset}");

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

    #[derive(Debug)]
    struct TypeInstance {
        base_type_instance_index: usize,
        parent_type_instance_index: usize,
        // a3
        arrayize_type_instance_index: usize,
        dearrayize_type_instance_index: usize,
        type_index: usize,
        special_type_id: u64,

        flags: TypeFlag,
        hash: u32,

        ctor_method_membership_index: usize,
        method_membership_start_index: usize,
        field_membership_start_index: usize,

        property_count: usize,
        property_membership_start_index: usize,
        d2_into_heap: usize,
        d3: u64,
        array_rank: u64,

        interface_list_offset: usize,
        template_argument_list_offset: usize,
        e2: u64,
    }
    file.seek_assert_align_up(type_instance_offset, 16)?;
    let type_instances = (0..type_instance_count)
        .map(|i| {
            let (index, base_type_instance_index, parent_type_instance_index, a3) =
                file.read_u64()?.bit_split((19, 19, 19, 7));

            if index != u64::from(i) {
                bail!("Unexpected index");
            }

            if base_type_instance_index >= type_instance_count.into() {
                bail!("base_type_instance_index out of bound")
            }
            if parent_type_instance_index >= type_instance_count.into() {
                bail!("parent_type_instance_index out of bound")
            }

            let (
                arrayize_type_instance_index,
                dearrayize_type_instance_index,
                type_index,
                special_type_id,
            ) = file.read_u64()?.bit_split((19, 19, 18, 8));

            if arrayize_type_instance_index >= type_instance_count.into() {
                bail!("arrayize_type_instance_index out of bound")
            }
            if dearrayize_type_instance_index >= type_instance_count.into() {
                bail!("dearrayize_type_instance_index out of bound")
            }

            if type_index >= type_count.into() {
                bail!("type_index out of bound")
            }

            let flags = file.read_u32()?;
            let _ = file.read_u32()?;
            let hash = file.read_u32()?;
            let crc = file.read_u32()?;

            let (
                ctor_method_membership_index,
                method_membership_start_index,
                field_membership_start_index,
            ) = file.read_u64()?.bit_split((22, 22, 20));

            if ctor_method_membership_index >= method_membership_count.into() {
                bail!("ctor_method_membership_index out of bound")
            }

            if method_membership_start_index >= method_membership_count.into() {
                bail!("method_membership_start_index out of bound")
            }

            if field_membership_start_index >= field_membership_count.into() {
                bail!("field_membership_start_index out of bound")
            }

            let (property_count, property_membership_start_index, d2_into_heap, d3, array_rank) =
                file.read_u64()?.bit_split((12, 20, 26, 3, 3));
            if property_membership_start_index + property_count > property_membership_count.into() {
                bail!("property_membership_index out of bound")
            }

            let (interface_list_offset, template_argument_list_offset, e2) =
                file.read_u64()?.bit_split((26, 26, 12));

            if template_argument_list_offset >= heap_len.into() {
                bail!("template_argument_list_offset out of bound")
            }

            if interface_list_offset >= heap_len.into() {
                bail!("interface_list_offset out of bound")
            }

            let _ = file.read_u64()?;
            let _ = file.read_u64()?;

            Ok(TypeInstance {
                base_type_instance_index: base_type_instance_index.try_into()?,
                parent_type_instance_index: parent_type_instance_index.try_into()?,

                arrayize_type_instance_index: arrayize_type_instance_index.try_into()?,
                dearrayize_type_instance_index: dearrayize_type_instance_index.try_into()?,
                type_index: type_index.try_into()?,
                special_type_id,

                flags: TypeFlag::from_bits(flags).context("Unknown type flag")?,
                hash,

                ctor_method_membership_index: ctor_method_membership_index.try_into()?,
                method_membership_start_index: method_membership_start_index.try_into()?,
                field_membership_start_index: field_membership_start_index.try_into()?,

                property_count: property_count.try_into()?,
                property_membership_start_index: property_membership_start_index.try_into()?,
                d2_into_heap: d2_into_heap.try_into()?,
                d3,
                array_rank,

                interface_list_offset: interface_list_offset.try_into()?,
                template_argument_list_offset: template_argument_list_offset.try_into()?,
                e2,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    struct MethodMembership {
        type_instance_index: usize,
        method_index: usize,
        param_list_offset: usize,
        address: u32,
    }
    file.seek_assert_align_up(method_membership_offset, 16)?;
    let method_memberships = (0..method_membership_count)
        .map(|_| {
            let (type_instance_index, param_list_offset_lo) = file.read_u32()?.bit_split((19, 13));
            let (method_index, param_list_offset_hi) = file.read_u32()?.bit_split((19, 13));
            let address = file.read_u32()?;
            let param_list_offset = param_list_offset_lo | (param_list_offset_hi << 13);

            if type_instance_index >= type_instance_count {
                bail!("type_instance_index out of bound");
            }
            if method_index >= method_count {
                bail!("method index out of bound");
            }
            if param_list_offset >= heap_len {
                bail!("param_list_offset out of bound")
            }

            Ok(MethodMembership {
                type_instance_index: type_instance_index.try_into()?,
                method_index: method_index.try_into()?,
                param_list_offset: param_list_offset.try_into()?,
                address,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    struct FieldMembership {
        parent_type_instance_index: usize,
        field_type_instance_index: usize,
        field_index: usize,
        constant_index_hi: u32,
        attribute_hi: FieldAttribute,
    }
    file.seek_assert_align_up(field_membership_offset, 16)?;
    let field_memberships = (0..field_membership_count)
        .map(|_| {
            let (
                parent_type_instance_index,
                field_index,
                field_type_instance_index,
                constant_index_hi,
                attribute_hi,
            ) = file.read_u64()?.bit_split((19, 19, 19, 6, 1));

            if parent_type_instance_index >= type_instance_count.into() {
                bail!("parent_type_instance_index out of bound");
            }

            if field_index >= field_count.into() {
                bail!("field_index out of bound");
            }

            if field_type_instance_index >= type_instance_count.into() {
                bail!("field_type_instance_index out of bound");
            }

            Ok(FieldMembership {
                parent_type_instance_index: parent_type_instance_index.try_into()?,
                field_index: field_index.try_into()?,
                field_type_instance_index: field_type_instance_index.try_into()?,
                constant_index_hi: constant_index_hi.try_into()?,
                attribute_hi: FieldAttribute::from_bits((attribute_hi << 15).try_into()?)
                    .context("Unknown field attr")?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    #[derive(Debug)]
    struct Type {
        name_offset: u32,
        namespace_offset: u32,
        len: usize,
        static_len: u32,

        method_count: usize,
        field_count: usize,
        attribute_list_index: usize,
        event_count: usize,
        event_start_index: usize,
    }

    file.seek_assert_align_up(type_offset, 16)?;
    let types = (0..type_count)
        .map(|_| {
            let name_offset = file.read_u32()?;
            let namespace_offset = file.read_u32()?;
            let len = file.read_u32()?;
            let static_len = file.read_u32()?;

            let (attribute_list_index, a, field_count, b) =
                file.read_u64()?.bit_split((17, 16, 24, 7));
            let method_count = file.read_u16()?;
            let _ = file.read_u16()?;
            let (types_event_count, event_start_index) = file.read_u32()?.bit_split((12, 20));

            if types_event_count + event_start_index > event_count {
                bail!("Event out of bound")
            }

            let _ = file.read_u64()?;
            let _ = file.read_u64()?;
            Ok(Type {
                name_offset,
                namespace_offset,
                len: len.try_into()?,
                static_len,
                method_count: method_count.try_into()?,
                field_count: field_count.try_into()?,
                attribute_list_index: attribute_list_index.try_into()?,
                event_count: types_event_count.try_into()?,
                event_start_index: event_start_index.try_into()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    struct Method {
        attribute_list_index: usize,
        vtable_slot: i16,
        attributes: MethodAttribute,
        impl_flag: MethodImplFlag,
        name_offset: u32,
    }
    file.seek_assert_align_up(method_offset, 16)?;
    let methods = (0..method_count)
        .map(|_| {
            let attribute_list_index = file.read_u16()?;
            let vtable_slot = file.read_i16()?;
            let attributes = file.read_u16()?;
            let impl_flag = file.read_u16()?;
            let name_offset = file.read_u32()?;
            if u32::from(attribute_list_index) >= attribute_list_count {
                bail!("attribute_list_index out of bound")
            }
            if name_offset >= string_table_len {
                bail!("name_offset out of bound")
            }
            Ok(Method {
                attribute_list_index: attribute_list_index.try_into()?,
                vtable_slot,
                attributes: MethodAttribute::from_bits(attributes)
                    .context("Unknown method attr")?,
                impl_flag: MethodImplFlag::from_bits(impl_flag)
                    .context("Unknown method impl flag")?,
                name_offset,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    struct Field {
        attribute_list_index: usize,
        attributes: FieldAttribute,
        constant_index_lm: u32,
        name_offset: u32,
    }
    file.seek_assert_align_up(field_offset, 16)?;
    let fields = (0..field_count)
        .map(|_| {
            let (attribute_list_index, attributes) = file.read_u32()?.bit_split((17, 15));
            let (position, constant_index_low) = file.read_u32()?.bit_split((26, 6));
            let (name_offset, constant_index_mid) = file.read_u32()?.bit_split((28, 4));

            if attribute_list_index >= data_attribute_list_count {
                bail!("attribute_list_index out of bound")
            }

            if name_offset >= string_table_len {
                bail!("name_offset out of bound")
            }

            let constant_index_lm = constant_index_low | (constant_index_mid << 6);
            let attributes = attributes as u16; //?

            Ok(Field {
                attribute_list_index: attribute_list_index.try_into()?,
                attributes: FieldAttribute::from_bits(attributes)
                    .context("Unknown field attribute")?,
                constant_index_lm,
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
        flags: PropertyFlag,
        attribute_list_index: usize,
        name_offset: u32,
    }
    file.seek_assert_align_up(property_offset, 16)?;
    let properties = (0..property_count)
        .map(|_| {
            let flags = file.read_u16()?;
            let attribute_list_index = file.read_u16()?;
            let name_offset = file.read_u32()?;
            Ok(Property {
                flags: PropertyFlag::from_bits(flags).context("Unknown property flag")?,
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
        modifier: u32,
        type_instance_index: usize,
        attribute: ParamAttribute,
    }
    file.seek_assert_align_up(param_offset, 16)?;
    let params = (0..param_count)
        .map(|_| {
            let attribute_list_index = file.read_u16()?;
            let default_const_index = file.read_u16()?;
            let (name_offset, modifier) = file.read_u32()?.bit_split((30, 2));
            let (type_instance_index, attribute) = file.read_u32()?.bit_split((19, 13));

            if u32::from(attribute_list_index) >= attribute_list_count {
                bail!("attribute_list_index out of bound")
            }

            if u32::from(default_const_index) >= constant_count {
                bail!("defualt_const_index out of bound")
            }

            if name_offset >= string_table_len {
                bail!("name out of bound")
            }

            if type_instance_index >= type_instance_count {
                bail!("type_instance_index out of bound")
            }

            Ok(Param {
                attribute_list_index: attribute_list_index.try_into()?,
                default_const_index: default_const_index.try_into()?,
                name_offset,
                modifier,
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

    let read_constant = |file: &mut F| -> Result<Constant> {
        let raw = file.read_i32()?;
        Ok(if raw >= 0 {
            Constant::Integral(usize::try_from(raw)?)
        } else {
            Constant::String(u32::try_from(-raw)?)
        })
    };

    file.seek_assert_align_up(constant_offset, 16)?;
    let constants = (0..constant_count)
        .map(|_| read_constant(&mut file))
        .collect::<Result<Vec<_>>>()?;

    file.seek_assert_align_up(attribute_list_offset, 16)?;
    let attribute_lists = (0..attribute_list_count)
        .map(|_| file.read_u32())
        .collect::<Result<Vec<_>>>()?;

    file.seek_assert_align_up(data_attribute_list_offset, 16)?;
    let data_attribute_lists = (0..data_attribute_list_count)
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
        .map(|_| file.read_u32())
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
            for _ in 1..ti.array_rank {
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
                template_argument_list.read_u32()?.bit_split((19, 13));

            let template_type_instance_index: usize = template_type_instance_index.try_into()?;
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

    fn read_vint<F: ReadExt>(mut f: F) -> Result<usize> {
        let a = f.read_u8()?.into();
        if a < 128 {
            Ok(a)
        } else if a == 0xFF {
            // A weird case found in sunbreak demo. Not sure what it signifies
            // The containing attribute param list also looks broken
            Ok(0)
        } else {
            let b: usize = f.read_u8()?.into();
            Ok(((a - 128) << 8) + b)
        }
    }

    let print_attribute = |attribute_i: usize, return_pos: bool| -> Result<()> {
        if attribute_i == 0 {
            return Ok(());
        }
        let attribute = &attributes[attribute_i];
        let ctor = &method_memberships[attribute.ctor_method_index];
        let mut attribute_args = &heap[attribute.arguments_offset..];
        let args_len = read_vint(&mut attribute_args)?;
        let mut args_data = &attribute_args[0..args_len];
        let start = args_data.read_u16()?;
        if start != 1 {
            bail!("unexpected attribute arg start {}", start)
        }
        let symbol = symbols[ctor.type_instance_index].as_ref().unwrap();
        let return_pos = if return_pos { "return:" } else { "" };
        print!("[{}{}(", return_pos, symbol);

        let mut mp = &heap[ctor.param_list_offset..];
        let param_count = mp.read_u16()?;
        let _abi_id = mp.read_u16()?;
        let _return_value_index = usize::try_from(mp.read_u32()?)?;

        let print_arg = |primitive_type: &str, args_data: &mut &[u8]| -> Result<()> {
            match primitive_type {
                "System.UInt32" => {
                    print!("{}", args_data.read_u32()?);
                }
                "System.Int32" => {
                    print!("{}", args_data.read_i32()?);
                }
                "System.Boolean" => {
                    print!("{}", args_data.read_bool()?);
                }
                "System.Single" => {
                    print!("{}", args_data.read_f32()?);
                }
                "System.String" | "System.Type" => {
                    let len = read_vint(&mut *args_data)?;
                    let mut buf = vec![0; len];
                    args_data.read_exact(&mut buf)?;
                    let v = std::str::from_utf8(&buf)?;
                    print!("\"{}\"", v);
                }
                _ => {
                    // TODO: Other type
                    // And for enums, we should look up the base type
                    print!("{}", args_data.read_i32()?);
                }
            }
            Ok(())
        };

        for param_i in 0..param_count {
            let param_index = usize::try_from(mp.read_u32()?)?;
            let param = &params[param_index];
            let param_symbol = symbols[param.type_instance_index].as_ref().unwrap();

            if param_symbol.ends_with("[]") {
                let element_type = &param_symbol[0..param_symbol.len() - 2];
                print!("[");
                let len = args_data.read_u32()?;
                for _ in 0..len {
                    print_arg(element_type, &mut args_data)?;
                    print!(",");
                }
                print!("]");
            } else {
                print_arg(param_symbol, &mut args_data)?;
            }
            print!(",");
        }

        let positional_arg_count = args_data.read_u16()?;
        for _ in 0..positional_arg_count {
            let magic = args_data.read_u8()?;
            if magic != 84 {
                print!("unexpected magic {}", magic);
                break;
            }
            let arg_type = args_data.read_u8()?;
            let name_length = read_vint(&mut args_data)?;
            let mut name_buf = vec![0; name_length];
            args_data.read_exact(&mut name_buf)?;
            print!("{}=", std::str::from_utf8(&name_buf)?);
            match arg_type {
                2 => {
                    print!("{}", args_data.read_bool()?);
                }
                14 => {
                    let v_length = read_vint(&mut args_data)?;
                    let mut v_buf = vec![0; v_length];
                    args_data.read_exact(&mut v_buf)?;
                    print!("{}", std::str::from_utf8(&v_buf)?);
                }
                _ => break, //TODO: what else type? Probably via.clr.ElementType
            }

            print!(",");
        }

        if !args_data.is_empty() {
            print!("$%$ leftover {:?}", args_data);
        }
        print!(")]");
        Ok(())
    };

    let print_attributes = |attribute_list_offset: u32, return_pos: bool| -> Result<()> {
        let attribute_list_offset = attribute_list_offset.try_into()?;
        let mut attribute_list = &heap[attribute_list_offset..];
        let attribute_count = attribute_list.read_u32()?;
        let attribute_list = (0..attribute_count)
            .map(|_| attribute_list.read_u32())
            .collect::<Result<Vec<_>>>()?;
        for attribute_i in attribute_list {
            print_attribute(attribute_i.try_into()?, return_pos)?;
        }
        Ok(())
    };

    let print_constant = |constant_index: usize, type_instance_index: usize| -> Result<()> {
        let constant = constants[constant_index];
        match constant {
            Constant::Integral(offset) => {
                let field_type_instance = &type_instances[type_instance_index];
                let len = types[field_type_instance.type_index].len;
                let value = &heap[offset..][..len];
                match len {
                    1 => print!(" = 0x{:02X}", value[0]),
                    2 => print!(" = 0x{:04X}", u16::from_le_bytes(value.try_into().unwrap())),
                    4 => print!(" = 0x{:08X}", u32::from_le_bytes(value.try_into().unwrap())),
                    8 => print!(
                        " = 0x{:016X}",
                        u64::from_le_bytes(value.try_into().unwrap())
                    ),
                    _ => print!(" = {:?}", value),
                }
            }
            Constant::String(offset) => {
                let s = read_string(offset)?;
                print!(" = \"{}\"", s);
            }
        }

        Ok(())
    };

    let mut function_map: BTreeMap<u32, Vec<String>> = BTreeMap::new();

    let mut order: Vec<_> = (0..type_instances.len()).collect();
    order.sort_by_key(|&i| symbols[i].as_ref().unwrap());

    for i in order {
        let type_instance = &type_instances[i];
        let ty = types
            .get(type_instance.type_index)
            .context("Type index out of bound")?;

        // println!("/// $TI[{}]", i);
        let full_name = &symbols[i].as_ref().unwrap();

        #[allow(clippy::collapsible_if)]
        if options.no_compound {
            if type_instance.dearrayize_type_instance_index != 0
                || full_name.contains('!')
                || full_name.contains('<')
            {
                continue;
            }
        }

        #[allow(clippy::collapsible_if)]
        if options.no_system {
            if full_name.starts_with("System.") {
                continue;
            }
        }

        let calc_hash = hash_as_utf8(full_name);
        println!("/// % {:08X}", type_instance.hash);
        if ty.attribute_list_index != 0 {
            print_attributes(attribute_lists[ty.attribute_list_index], false)?;
            println!();
        }
        if !options.no_type_flag {
            println!("{}", display_type_flags(type_instance.flags));
        }
        println!(
            "class {}: {}",
            full_name,
            symbols[type_instance.base_type_instance_index]
                .as_ref()
                .unwrap()
        );

        if i != 0 && calc_hash != type_instance.hash {
            bail!("Mismatched hash for TI[{}]", i)
        }

        let mut interface_list = &heap[type_instance.interface_list_offset..];
        let interface_count = interface_list.read_u32()?;
        for _ in 0..interface_count {
            let (interface_type_instance_id, interface_vtable_slot_start) =
                interface_list.read_u32()?.bit_split((19, 13));
            let interface_type_instance_id: usize = interface_type_instance_id.try_into()?;
            println!(
                "    ,{} /* ^{} */",
                &symbols[interface_type_instance_id].as_ref().unwrap(),
                interface_vtable_slot_start,
            );
        }

        println!("{{");
        if type_instance.dearrayize_type_instance_index != 0 || full_name.contains('!') {
            println!("    // Omitted ");
            println!("}}");
            println!();
            continue;
        }

        println!("    // Special = {}", type_instance.special_type_id);

        if type_instance.template_argument_list_offset != 0 {
            let mut template_argument_list = &heap[type_instance.template_argument_list_offset..];
            let (template_type_instance_id, targ_count) =
                template_argument_list.read_u32()?.bit_split((19, 13));
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

        println!();
        println!("    /*** Method ***/");
        println!();
        if type_instance.method_membership_start_index + ty.method_count > method_memberships.len()
        {
            bail!("Method membership out of bound")
        }
        for method_membership in
            &method_memberships[type_instance.method_membership_start_index..][..ty.method_count]
        {
            if method_membership.type_instance_index != i {
                bail!("method_membership.type_instance_index mismatch");
            }

            let method = methods
                .get(method_membership.method_index)
                .context("Method index out of bound")?;

            if method.attribute_list_index != 0 {
                print!("    ");
                print_attributes(attribute_lists[method.attribute_list_index], false)?;
                println!();
            }

            let mut mp = &heap[method_membership.param_list_offset..];
            let param_count = mp.read_u16()?;
            let abi_id = mp.read_u16()?;
            let return_value_index = usize::try_from(mp.read_u32()?)?;
            let return_value = &params[return_value_index];
            if return_value.attribute_list_index != 0 {
                print!("    ");
                print_attributes(attribute_lists[return_value.attribute_list_index], true)?;
                println!();
            }

            let method_name = read_string(method.name_offset)?;

            println!(
                "    {}{}{}\n    {} {} (",
                display_param_modifier(return_value.modifier, true),
                display_method_impl_flag(method.impl_flag),
                display_method_attributes(method.attributes),
                symbols[return_value.type_instance_index].as_ref().unwrap(),
                method_name
            );

            for _ in 0..param_count {
                let param_index = usize::try_from(mp.read_u32()?)?;
                let param = &params[param_index];
                print!("        ");
                if param.attribute_list_index != 0 {
                    print_attributes(attribute_lists[param.attribute_list_index], false)?;
                }
                print!(
                    "{}{} {} {}",
                    display_param_modifier(param.modifier, false),
                    display_param_attributes(param.attribute),
                    symbols[param.type_instance_index].as_ref().unwrap(),
                    read_string(param.name_offset)?
                );

                if param.default_const_index != 0 {
                    print_constant(param.default_const_index, param.type_instance_index)?;
                }
                println!(",");
            }

            let address = if !options.no_runtime && method_membership.address != 0 {
                function_map
                    .entry(method_membership.address)
                    .or_default()
                    .push(format!("{}.{}", full_name, method_name));
                format!(" = 0x{:08X}", method_membership.address)
            } else {
                "".to_string()
            };

            println!("    ){};\n", address);
        }

        println!();
        println!("    /*** Field ***/");
        println!();

        if type_instance.field_membership_start_index + ty.field_count > field_memberships.len() {
            bail!("Field membership out of bound")
        }
        for field_membership in
            &field_memberships[type_instance.field_membership_start_index..][..ty.field_count]
        {
            if field_membership.parent_type_instance_index != i {
                bail!("field_membership.parent_type_instance_index mismatch")
            }

            let field = fields
                .get(field_membership.field_index)
                .context("Field index out of bound")?;

            if field.attribute_list_index != 0 {
                print!("    ");
                print_attributes(data_attribute_lists[field.attribute_list_index], false)?;
                println!();
            }

            let attributes = field.attributes | field_membership.attribute_hi;

            print!(
                "    {} {} {}",
                display_field_attributes(attributes),
                &symbols[field_membership.field_type_instance_index]
                    .as_ref()
                    .unwrap(),
                read_string(field.name_offset)?
            );

            let constant_index = usize::try_from(
                field.constant_index_lm | (field_membership.constant_index_hi << 10),
            )?;

            if constant_index != 0 {
                print_constant(constant_index, field_membership.field_type_instance_index)?;
            }

            println!(";");
        }

        println!();
        println!("    /*** Event ***/");
        println!();
        for event in &events[ty.event_start_index..][..ty.event_count] {
            println!("    public event {};", read_string(event.name_offset)?);
        }

        println!();
        println!("    /*** Property ***/");
        println!();
        for property_membership in &property_memberships
            [type_instance.property_membership_start_index..][..type_instance.property_count]
        {
            let property = &properties[property_membership.property_index];
            if property.attribute_list_index != 0 {
                print!("    ");
                print_attributes(data_attribute_lists[property.attribute_list_index], false)?;
                println!();
            }
            println!(
                "    {}public property {};",
                display_property_flag(property.flags),
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

    if let Some(map) = map {
        let mut map = File::create(map)?;
        for (address, names) in function_map {
            for name in names {
                writeln!(map, "{} {:016X} f", name, address)?
            }
        }
    }

    Ok(())
}
