use crate::bitfield::*;
use crate::file_ext::*;
use crate::hash::*;
use anyhow::{bail, Context, Result};
use bitflags::*;
use serde::*;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{Read, Seek, Write};

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u16")]
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

impl From<FieldAttribute> for u16 {
    fn from(f: FieldAttribute) -> Self {
        f.bits()
    }
}

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u16")]
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

impl From<ParamAttribute> for u16 {
    fn from(f: ParamAttribute) -> Self {
        f.bits()
    }
}

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u16")]
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

impl From<MethodAttribute> for u16 {
    fn from(f: MethodAttribute) -> Self {
        f.bits()
    }
}

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u32")]
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

impl From<TypeFlag> for u32 {
    fn from(f: TypeFlag) -> Self {
        f.bits()
    }
}

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u16")]
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

impl From<MethodImplFlag> for u16 {
    fn from(f: MethodImplFlag) -> Self {
        f.bits()
    }
}

bitflags! {
    #[derive(Serialize)]
    #[serde(into = "u16")]
    struct PropertyFlag: u16 {
        const SPECIAL_NAME    = 0x0200;
        const RT_SPECIAL_NAME = 0x0400;
        const HAS_DEFAULT     = 0x1000;
        const EXPOSE_MEMBER   = 0x4000;
    }
}
impl From<PropertyFlag> for u16 {
    fn from(f: PropertyFlag) -> Self {
        f.bits()
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

#[derive(Serialize, Clone)]
enum TypeParent {
    Namespace(String),
    OuterType(usize),
}

#[derive(Serialize, Clone)]
struct GenericParam {
    name: String,
    ti_constraint: Option<usize>,

    #[serde(rename = "System.Reflection.GenericParameterAttributes")]
    flags: u32,
}

#[derive(Serialize, Clone)]
enum Generics {
    Template {
        params: Vec<GenericParam>,
    },
    Constructed {
        ti_template: usize,
        ti_args: Vec<usize>,
    },
}

#[derive(Serialize)]
struct Interface {
    ti: usize,
    vtable_slot_start: u32,
}

#[derive(Serialize)]
enum ValueInfo {
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Serialize)]
struct FieldInfo {
    name: String,
    ti: usize,
    #[serde(rename = "via.clr.FieldFlag")]
    flags: FieldAttribute,
    position: u32,
    value: Option<ValueInfo>,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize)]
struct ParamInfo {
    name: String,
    ti: usize,
    default: Option<ValueInfo>,
    #[serde(rename = "via.clr.ParamModifier")]
    modifier: u32,
    #[serde(rename = "via.clr.ParamFlag")]
    flags: ParamAttribute,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize)]
struct MethodInfo {
    name: String,
    address: u32,
    vtable_slot: i16,
    #[serde(rename = "via.clr.MethodFlag")]
    flags: MethodAttribute,
    #[serde(rename = "via.clr.MethodImplFlag")]
    impl_flags: MethodImplFlag,
    signature: u16,
    attributes: Vec<AttributeInfo>,
    ret: ParamInfo,
    params: Vec<ParamInfo>,
}

#[derive(Serialize)]
struct PropertyInfo {
    name: String,
    mi_get: Option<usize>,
    mi_set: Option<usize>,
    #[serde(rename = "via.clr.PropertyFlag")]
    flags: PropertyFlag,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize)]
struct EventInfo {
    name: String,
    mi_add: usize,
    mi_remove: usize,
}

#[derive(Serialize)]
enum Arg {
    Byte(u8),
    SByte(i8),
    Char(u16),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Single(f32),
    Double(f64),
    Boolean(bool),
    String(Option<String>),
    Type(Option<String>),
    Array(Vec<Arg>),
}

#[derive(Serialize)]
struct NamedArg {
    name: String,
    arg: Arg,
}

#[derive(Serialize)]
struct AttributeInfo {
    pub ti_attribute: usize,
    pub mi_default_ctor: usize,
    pub positional_args: Vec<Arg>,
    pub named_args: Vec<NamedArg>,
}

#[derive(Serialize)]
struct TypeInfo {
    name: String,
    full_name: Option<String>,
    parent: Option<TypeParent>,
    len: usize,
    static_len: usize,
    ti_base: Option<usize>,
    ti_array: Option<usize>,
    ti_dearray: Option<usize>,
    array_rank: u64,
    vtable_size: u64,
    native_vtable_size: u16,

    #[serde(rename = "via.clr.SystemType")]
    system_type: u64,

    #[serde(rename = "via.clr.ElementType")]
    element_type: u8,

    #[serde(rename = "via.clr.VMObjType")]
    vmobj_type: u64,

    #[serde(rename = "via.clr.TypeFlag")]
    flags: TypeFlag,

    hash: u32,
    assembly: usize,
    mi_default_ctor: Option<usize>,
    attributes: Vec<AttributeInfo>,
    generics: Option<Generics>,
    interfaces: Vec<Interface>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    properties: Vec<PropertyInfo>,
    events: Vec<EventInfo>,

    // A bit map indicating which 8-byte chunks are references
    // used for GC to build reference graph
    // If the type is very large (> 512 bytes?), this is an address to the map instead
    ref_map: u64,

    // Maybe a similar map as ref_map, but this marks where it can have cycle?
    cycle_map: u64,
}

#[derive(Serialize)]
struct AssemblyInfo {
    name: String,
    full_path: String,
    dll_name: String,
}

#[derive(Serialize)]
struct Tdb {
    types: Vec<TypeInfo>,
    intern_strings: Vec<String>,
    assemblies: Vec<AssemblyInfo>,
}

#[allow(dead_code)]
impl Tdb {
    pub fn new<F: Read + Seek>(mut file: F, base_address: u64) -> Result<Tdb> {
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
        let (attribute_list_count, data_attribute_list_count) =
            file.read_u32()?.bit_split((16, 16));
        let intern_string_count = file.read_u32()?;
        let assembly_count = file.read_u32()?;
        if file.read_u32()? != 0 {
            bail!("Expected 0");
        }
        let _unknown = file.read_u32()?;
        let string_table_len = file.read_u32()?;
        let heap_len = file.read_u32()?;

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
        let intern_string_offset = file.read_u64()? - base_address;
        let _ = file.read_u64()?;

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
            element_type: u8,
            arrayize_type_instance_index: usize,
            dearrayize_type_instance_index: usize,
            type_index: usize,
            system_type: u64,

            flags: TypeFlag,
            hash: u32,

            ctor_method_membership_index: usize,
            method_membership_start_index: usize,
            field_membership_start_index: usize,

            property_count: usize,
            property_membership_start_index: usize,
            d2_into_heap: usize,
            vmobj_type: u64,
            array_rank: u64,

            interface_list_offset: usize,
            template_argument_list_offset: usize,
            e2: u64,
        }
        file.seek_assert_align_up(type_instance_offset, 16)?;
        let type_instances = (0..type_instance_count)
            .map(|i| {
                let (index, base_type_instance_index, parent_type_instance_index, element_type) =
                    file.read_u64()?.bit_split((19, 19, 19, 7));

                if index != u64::from(i) {
                    bail!("Unexpected index");
                }

                let (
                    arrayize_type_instance_index,
                    dearrayize_type_instance_index,
                    type_index,
                    system_type,
                ) = file.read_u64()?.bit_split((19, 19, 18, 8));

                let flags = file.read_u32()?;
                let _ = file.read_u32()?;
                let hash = file.read_u32()?;
                let _crc = file.read_u32()?;

                let (
                    ctor_method_membership_index,
                    method_membership_start_index,
                    field_membership_start_index,
                ) = file.read_u64()?.bit_split((22, 22, 20));

                let (
                    property_count,
                    property_membership_start_index,
                    d2_into_heap,
                    vmobj_type,
                    array_rank,
                ) = file.read_u64()?.bit_split((12, 20, 26, 3, 3));

                let (interface_list_offset, template_argument_list_offset, e2) =
                    file.read_u64()?.bit_split((26, 26, 12));

                let _ = file.read_u64()?;
                let _ = file.read_u64()?;

                Ok(TypeInstance {
                    base_type_instance_index: base_type_instance_index.try_into()?,
                    parent_type_instance_index: parent_type_instance_index.try_into()?,
                    element_type: element_type.try_into()?,

                    arrayize_type_instance_index: arrayize_type_instance_index.try_into()?,
                    dearrayize_type_instance_index: dearrayize_type_instance_index.try_into()?,
                    type_index: type_index.try_into()?,
                    system_type,

                    flags: TypeFlag::from_bits(flags).context("Unknown type flag")?,
                    hash,

                    ctor_method_membership_index: ctor_method_membership_index.try_into()?,
                    method_membership_start_index: method_membership_start_index.try_into()?,
                    field_membership_start_index: field_membership_start_index.try_into()?,

                    property_count: property_count.try_into()?,
                    property_membership_start_index: property_membership_start_index.try_into()?,
                    d2_into_heap: d2_into_heap.try_into()?,
                    vmobj_type,
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
                let (type_instance_index, param_list_offset_lo) =
                    file.read_u32()?.bit_split((19, 13));
                let (method_index, param_list_offset_hi) = file.read_u32()?.bit_split((19, 13));
                let address = file.read_u32()?;
                let param_list_offset = param_list_offset_lo | (param_list_offset_hi << 13);

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
            native_vtable_size: u16,
            field_count: usize,
            attribute_list_index: usize,
            vtable_size: u64,
            event_count: usize,
            event_start_index: usize,
            assembly_index: usize,

            ref_map: u64,
            cycle_map: u64,
        }

        file.seek_assert_align_up(type_offset, 16)?;
        let types = (0..type_count)
            .map(|_| {
                let name_offset = file.read_u32()?;
                let namespace_offset = file.read_u32()?;
                let len = file.read_u32()?;
                let static_len = file.read_u32()?;

                let (attribute_list_index, vtable_size, field_count, assembly_index) =
                    file.read_u64()?.bit_split((17, 16, 24, 7));
                let method_count = file.read_u16()?;
                let native_vtable_size = file.read_u16()?;
                let (types_event_count, event_start_index) = file.read_u32()?.bit_split((12, 20));

                let ref_map = file.read_u64()?;
                let cycle_map = file.read_u64()?;

                Ok(Type {
                    name_offset,
                    namespace_offset,
                    len: len.try_into()?,
                    static_len,
                    method_count: method_count.try_into()?,
                    native_vtable_size,
                    field_count: field_count.try_into()?,
                    attribute_list_index: attribute_list_index.try_into()?,
                    vtable_size,
                    event_count: types_event_count.try_into()?,
                    event_start_index: event_start_index.try_into()?,
                    assembly_index: assembly_index.try_into()?,
                    ref_map,
                    cycle_map,
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
            position: u32,
        }
        file.seek_assert_align_up(field_offset, 16)?;
        let fields = (0..field_count)
            .map(|_| {
                let (attribute_list_index, attributes) = file.read_u32()?.bit_split((17, 15));
                let (position, constant_index_low) = file.read_u32()?.bit_split((26, 6));
                let (name_offset, constant_index_mid) = file.read_u32()?.bit_split((28, 4));

                let constant_index_lm = constant_index_low | (constant_index_mid << 6);
                let attributes = attributes as u16;

                Ok(Field {
                    attribute_list_index: attribute_list_index.try_into()?,
                    attributes: FieldAttribute::from_bits(attributes)
                        .context("Unknown field attribute")?,
                    constant_index_lm,
                    name_offset,
                    position,
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

        file.seek_assert_align_up(intern_string_offset, 16)?;
        let intern_strings = (0..intern_string_count)
            .map(|_| file.read_u32())
            .collect::<Result<Vec<_>>>()?;

        ///////////////////////

        let to_ti_opt = |index: usize| -> Result<Option<usize>> {
            if index >= type_instance_count as usize {
                bail!("TI out of bound")
            }
            Ok(index.checked_sub(1))
        };

        let to_ti = |index: usize| -> Result<usize> { to_ti_opt(index)?.context("Null TI") };

        let to_mi = |method_membership_index: usize| -> Result<(usize, usize)> {
            let mm = method_memberships
                .get(method_membership_index)
                .context("Method membership out of bound")?;
            let ti = type_instances
                .get(mm.type_instance_index)
                .context("Type instance out of bound")?;
            let ty = types.get(ti.type_index).context("Type out of bound")?;
            if method_membership_index < ti.method_membership_start_index
                || method_membership_index >= ti.method_membership_start_index + ty.method_count
            {
                bail!("Method doesn't belong to type")
            }
            Ok((
                to_ti(mm.type_instance_index)?,
                method_membership_index - ti.method_membership_start_index,
            ))
        };

        let to_mi_self = |method_membership_index, type_instance_index| -> Result<usize> {
            let (ti, mi) = to_mi(method_membership_index)?;
            if ti != to_ti(type_instance_index)? {
                bail!("Unexpected method parent")
            }
            Ok(mi)
        };

        fn read_element_arg(element_type: u8, args_data: &mut &[u8]) -> Result<Arg> {
            let arg = match element_type {
                2 => Arg::Boolean(args_data.read_bool()?),
                3 => Arg::Char(args_data.read_u16()?),
                4 => Arg::SByte(args_data.read_i8()?),
                5 => Arg::Byte(args_data.read_u8()?),
                6 => Arg::Int16(args_data.read_i16()?),
                7 => Arg::UInt16(args_data.read_u16()?),
                8 => Arg::Int32(args_data.read_i32()?),
                9 => Arg::UInt32(args_data.read_u32()?),
                10 => Arg::Int64(args_data.read_i64()?),
                11 => Arg::UInt64(args_data.read_u64()?),
                12 => Arg::Single(args_data.read_f32()?),
                13 => Arg::Double(args_data.read_f64()?),
                14 => {
                    let v_length = read_vint(&mut *args_data)?;
                    if let Some(v_length) = v_length {
                        let mut v_buf = vec![0; v_length];
                        args_data.read_exact(&mut v_buf)?;
                        Arg::String(Some(std::str::from_utf8(&v_buf)?.to_owned()))
                    } else {
                        Arg::String(None)
                    }
                }
                x => bail!("Unknown named argument type {x}"),
            };
            Ok(arg)
        }

        fn read_positional_arg(
            type_instances: &[TypeInstance],
            field_memberships: &[FieldMembership],
            pti_index: usize,
            args_data: &mut &[u8],
        ) -> Result<Arg> {
            let pti = type_instances.get(pti_index).context("PTI out of bound")?;
            let element = pti.dearrayize_type_instance_index;
            if element != 0 {
                let len = args_data.read_u32()?;
                Ok(Arg::Array(
                    (0..len)
                        .map(|_| {
                            read_positional_arg(
                                type_instances,
                                field_memberships,
                                element,
                                args_data,
                            )
                        })
                        .collect::<Result<_>>()?,
                ))
            } else if pti.system_type == 12 || pti.system_type == 2 {
                // System.Type || System.String
                let wrapper = if pti.system_type == 12 {
                    Arg::Type
                } else {
                    Arg::String
                };
                let v_length = read_vint(&mut *args_data)?;
                if let Some(v_length) = v_length {
                    let mut v_buf = vec![0; v_length];
                    args_data.read_exact(&mut v_buf)?;
                    Ok(wrapper(Some(std::str::from_utf8(&v_buf)?.to_owned())))
                } else {
                    Ok(wrapper(None))
                }
            } else {
                let arg = read_element_arg(pti.element_type, &mut *args_data)?;
                Ok(arg)
            }
        }

        let read_attributes = |attribute_list_offset: u32| -> Result<Vec<AttributeInfo>> {
            if attribute_list_offset == 0 {
                return Ok(vec![]);
            }
            let attribute_list_offset: usize = attribute_list_offset.try_into()?;
            let mut attribute_list = heap
                .get(attribute_list_offset..)
                .context("Attribute list out of bound")?;
            let attribute_count: usize = attribute_list.read_u32()?.try_into()?;
            let attributes: Vec<AttributeInfo> = attribute_list
                .get(0..attribute_count * 4)
                .context("Attribute list out of bound")?
                .chunks(4)
                .map(|c| {
                    let index: usize = u32::from_le_bytes(c.try_into().unwrap()).try_into()?;
                    let attribute = attributes.get(index).context("Attribute out of bound")?;
                    let (ti_attribute, mi_default_ctor) = to_mi(attribute.ctor_method_index)?;

                    let mut attribute_args = heap
                        .get(attribute.arguments_offset..)
                        .context("Attribute arguments out of bound")?;
                    let args_len = read_vint(&mut attribute_args)?.context("Null len")?;
                    let mut args_data = attribute_args
                        .get(0..args_len)
                        .context("Attribute arguments out of bound")?;
                    let start = args_data.read_u16()?;
                    if start != 1 {
                        bail!("unexpected attribute arg start {}", start)
                    }

                    let ctor = method_memberships
                        .get(attribute.ctor_method_index)
                        .context("Attribute ctor out of bound")?;
                    let mut mp = &heap[ctor.param_list_offset..];
                    let param_count = mp.read_u16()?;
                    let _signature = mp.read_u16()?;
                    let _ret = mp.read_u32()?;

                    let positional_args = (0..param_count)
                        .map(|_| {
                            let param_index = usize::try_from(mp.read_u32()?)?;
                            let param = params
                                .get(param_index)
                                .context("Attribute param out of bound")?;
                            let pti_index = param.type_instance_index;

                            read_positional_arg(
                                &type_instances,
                                &field_memberships,
                                pti_index,
                                &mut args_data,
                            )
                        })
                        .collect::<Result<_>>()?;

                    let named_count = args_data.read_u16()?;
                    let named_args = (0..named_count)
                        .map(|_| {
                            let magic = args_data.read_u8()?;
                            if magic != 84 {
                                bail!("Unexpected attribute magic")
                            }
                            let element_type = args_data.read_u8()?;
                            let name_length = read_vint(&mut args_data)?.context("Null len")?;
                            let mut name_buf = vec![0; name_length];
                            args_data.read_exact(&mut name_buf)?;
                            let name = std::str::from_utf8(&name_buf)?.to_owned();
                            let arg = read_element_arg(element_type, &mut args_data)?;

                            Ok(NamedArg { name, arg })
                        })
                        .collect::<Result<_>>()?;

                    if !args_data.is_empty() {
                        bail!("Left over attribute argument data")
                    }

                    Ok(AttributeInfo {
                        ti_attribute,
                        mi_default_ctor,
                        positional_args,
                        named_args,
                    })
                })
                .collect::<Result<_>>()?;

            Ok(attributes)
        };

        let get_constant =
            |constant_index: usize, type_instance_index: usize| -> Result<Option<ValueInfo>> {
                let value = if constant_index == 0 {
                    None
                } else {
                    let constant = constants
                        .get(constant_index)
                        .context("Constant out of bound")?;
                    Some(match constant {
                        Constant::Integral(offset) => {
                            let constant_type_instance = type_instances
                                .get(type_instance_index)
                                .context("constant type instance out of bound")?;
                            let constant_type = types
                                .get(constant_type_instance.type_index)
                                .context("constant type out of bound")?;
                            let data = heap
                                .get(*offset..*offset + constant_type.len)
                                .context("Constant out of bound")?;
                            ValueInfo::Bytes(data.to_vec())
                        }
                        Constant::String(offset) => ValueInfo::String(read_string(*offset)?),
                    })
                };
                Ok(value)
            };
        let get_param = |index: usize| -> Result<ParamInfo> {
            let param = params.get(index).context("Param out of bound")?;
            let attribute_list = *attribute_lists
                .get(param.attribute_list_index)
                .context("Attribute list out of bound")?;

            Ok(ParamInfo {
                name: read_string(param.name_offset)?,
                ti: to_ti(param.type_instance_index)?,
                default: get_constant(param.default_const_index, param.type_instance_index)?,
                modifier: param.modifier,
                flags: param.attribute,
                attributes: read_attributes(attribute_list)?,
            })
        };

        fn read_vint<F: ReadExt>(mut f: F) -> Result<Option<usize>> {
            let a = f.read_u8()?.into();
            if a < 128 {
                Ok(Some(a))
            } else if a == 0xFF {
                // This is theorized to represent null string
                Ok(None)
            } else {
                let b: usize = f.read_u8()?.into();
                Ok(Some(((a - 128) << 8) + b))
            }
        }

        let mut type_infos: Vec<TypeInfo> = type_instances
            .iter()
            .enumerate()
            .skip(1)
            .map(|(ti_index, ti)| {
                let ty = types
                    .get(ti.type_index)
                    .context("type index out of bound")?;
                let parent = if ty.namespace_offset != 0 && ti.parent_type_instance_index != 0 {
                    bail!("Type has both namespace and parent type")
                } else if ty.namespace_offset != 0 {
                    Some(TypeParent::Namespace(read_string(ty.namespace_offset)?))
                } else if ti.parent_type_instance_index != 0 {
                    Some(TypeParent::OuterType(to_ti(ti.parent_type_instance_index)?))
                } else {
                    None
                };
                let name = read_string(ty.name_offset)?;

                let generics = if ti.template_argument_list_offset != 0 {
                    let mut template_argument_list =
                        heap.get(ti.template_argument_list_offset..)
                            .context("template argument list offset out of bound")?;

                    let (template_type_instance_index, arg_count) =
                        template_argument_list.read_u32()?.bit_split((19, 13));

                    let template_type_instance_index: usize =
                        template_type_instance_index.try_into()?;
                    let arg_count: usize = arg_count.try_into()?;

                    if template_type_instance_index != ti_index {
                        let ti_template = to_ti(template_type_instance_index)?;
                        let ti_args: Vec<usize> = template_argument_list
                            .get(0..arg_count * 4)
                            .context("template argument list out of bound")?
                            .chunks(4)
                            .map(|chunk| {
                                let ti = u32::from_le_bytes(chunk.try_into().unwrap());
                                let ti: usize = ti.try_into()?;
                                let ti = to_ti(ti)?;
                                Ok(ti)
                            })
                            .collect::<Result<_>>()?;
                        Some(Generics::Constructed {
                            ti_template,
                            ti_args,
                        })
                    } else {
                        let params: Vec<GenericParam> = template_argument_list
                            .get(0..arg_count * 8)
                            .context("template argument list out of bound")?
                            .chunks(8)
                            .map(|chunk| {
                                let (type_instance_index, flags) =
                                    u32::from_le_bytes(chunk[0..4].try_into().unwrap())
                                        .bit_split((19, 13));
                                let type_instance_index: usize = type_instance_index.try_into()?;
                                let name_offset =
                                    u32::from_le_bytes(chunk[4..8].try_into().unwrap());
                                Ok(GenericParam {
                                    name: read_string(name_offset)?,
                                    ti_constraint: to_ti_opt(type_instance_index)?,
                                    flags,
                                })
                            })
                            .collect::<Result<_>>()?;
                        Some(Generics::Template { params })
                    }
                } else {
                    None
                };

                let interfaces = if ti.interface_list_offset != 0 {
                    let mut interface_list = heap
                        .get(ti.interface_list_offset..)
                        .context("Interface list out of bound")?;
                    let interface_count: usize = interface_list.read_u32()?.try_into()?;
                    interface_list
                        .get(0..interface_count * 4)
                        .context("Interface list out of bound")?
                        .chunks(4)
                        .map(|chunk| {
                            let (interface_type_instance_id, vtable_slot_start) =
                                u32::from_le_bytes(chunk.try_into().unwrap()).bit_split((19, 13));
                            let interface_type_instance_id: usize =
                                interface_type_instance_id.try_into()?;
                            let ti = to_ti(interface_type_instance_id)?;
                            Ok(Interface {
                                ti,
                                vtable_slot_start,
                            })
                        })
                        .collect::<Result<_>>()?
                } else {
                    vec![]
                };

                let field_memberships = field_memberships
                    .get(
                        ti.field_membership_start_index
                            ..ti.field_membership_start_index + ty.field_count,
                    )
                    .context("Field membership out of bound")?;

                let fields = field_memberships
                    .iter()
                    .map(|fm| {
                        if fm.parent_type_instance_index != ti_index {
                            bail!("Field parent mismatch")
                        }
                        let f = fields.get(fm.field_index).context("Field out of bound")?;
                        let constant_index =
                            usize::try_from(f.constant_index_lm | (fm.constant_index_hi << 10))?;
                        let value = get_constant(constant_index, fm.field_type_instance_index)?;
                        let attribute_list = *data_attribute_lists
                            .get(f.attribute_list_index)
                            .context("Attribute list out of bound")?;
                        Ok(FieldInfo {
                            name: read_string(f.name_offset)?,
                            ti: to_ti(fm.field_type_instance_index)?,
                            flags: f.attributes | fm.attribute_hi,
                            position: f.position,
                            value,
                            attributes: read_attributes(attribute_list)?,
                        })
                    })
                    .collect::<Result<_>>()?;

                let method_memberships = method_memberships
                    .get(
                        ti.method_membership_start_index
                            ..ti.method_membership_start_index + ty.method_count,
                    )
                    .context("Method membership out of bound")?;

                let methods = method_memberships
                    .iter()
                    .map(|mm| {
                        let m = methods
                            .get(mm.method_index)
                            .context("Method out of bound")?;
                        let mut mp = heap
                            .get(mm.param_list_offset..)
                            .context("Param list out of bound")?;
                        let param_count: usize = mp.read_u16()?.into();
                        let signature = mp.read_u16()?;
                        let attribute_list = *attribute_lists
                            .get(m.attribute_list_index)
                            .context("Attribute list out of bound")?;
                        let ret = get_param(usize::try_from(mp.read_u32()?)?)?;
                        let params: Vec<ParamInfo> = mp
                            .get(0..4 * param_count)
                            .context("Param list out of bound")?
                            .chunks(4)
                            .map(|chunk| {
                                let index = u32::from_le_bytes(chunk.try_into().unwrap());
                                get_param(index.try_into()?)
                            })
                            .collect::<Result<_>>()?;

                        Ok(MethodInfo {
                            name: read_string(m.name_offset)?,
                            address: mm.address,
                            vtable_slot: m.vtable_slot,
                            flags: m.attributes,
                            impl_flags: m.impl_flag,
                            signature,
                            attributes: read_attributes(attribute_list)?,
                            ret,
                            params,
                        })
                    })
                    .collect::<Result<_>>()?;

                let property_memberships = property_memberships
                    .get(
                        ti.property_membership_start_index
                            ..ti.property_membership_start_index + ti.property_count,
                    )
                    .context("Property membership out of bound")?;
                let properties = property_memberships
                    .iter()
                    .map(|pm| {
                        let p = properties
                            .get(pm.property_index)
                            .context("Property out of bound")?;
                        let get = pm.get_method_membership_index;
                        let set = pm.set_method_membership_index;
                        let attribute_list = *data_attribute_lists
                            .get(p.attribute_list_index)
                            .context("Attribute list out of bound")?;
                        Ok(PropertyInfo {
                            name: read_string(p.name_offset)?,
                            mi_get: (get != 0).then(|| to_mi_self(get, ti_index)).transpose()?,
                            mi_set: (set != 0).then(|| to_mi_self(set, ti_index)).transpose()?,
                            flags: p.flags,
                            attributes: read_attributes(attribute_list)?,
                        })
                    })
                    .collect::<Result<_>>()?;

                let events = events
                    .get(ty.event_start_index..ty.event_start_index + ty.event_count)
                    .context("Event out of bound")?;
                let events = events
                    .iter()
                    .map(|e| {
                        Ok(EventInfo {
                            name: read_string(e.name_offset)?,
                            mi_add: to_mi_self(e.add_method_membership_index, ti_index)?,
                            mi_remove: to_mi_self(e.remove_method_membership_index, ti_index)?,
                        })
                    })
                    .collect::<Result<_>>()?;

                let attribute_list = *attribute_lists
                    .get(ty.attribute_list_index)
                    .context("Attribute list out of bound")?;

                let ctor = ti.ctor_method_membership_index;

                Ok(TypeInfo {
                    name,
                    full_name: None,
                    parent,
                    len: ty.len,
                    static_len: ty.static_len as usize,
                    ti_base: to_ti_opt(ti.base_type_instance_index)?,
                    ti_array: to_ti_opt(ti.arrayize_type_instance_index)?,
                    ti_dearray: to_ti_opt(ti.dearrayize_type_instance_index)?,
                    array_rank: ti.array_rank,
                    vtable_size: ty.vtable_size,
                    native_vtable_size: ty.native_vtable_size,
                    system_type: ti.system_type,
                    element_type: ti.element_type,
                    flags: ti.flags,
                    hash: ti.hash,
                    assembly: ty.assembly_index,
                    mi_default_ctor: (ctor != 0)
                        .then(|| to_mi_self(ctor, ti_index))
                        .transpose()?,
                    attributes: read_attributes(attribute_list)?,
                    generics,
                    interfaces,
                    fields,
                    methods,
                    properties,
                    events,
                    vmobj_type: ti.vmobj_type,
                    ref_map: ty.ref_map,
                    cycle_map: ty.cycle_map,
                })
            })
            .collect::<Result<_>>()?;

        fn build_symbol(type_infos: &mut [TypeInfo], index: usize) -> String {
            if let Some(s) = &type_infos[index].full_name {
                return s.clone();
            }

            let type_info = &type_infos[index];

            let ti_dearray = type_info.ti_dearray;
            let array_rank = type_info.array_rank;
            let parent = type_info.parent.clone();
            let name = type_info.name.clone();
            let generics = type_info.generics.clone();

            if let Some(ti_dearray) = ti_dearray {
                let element_name = build_symbol(type_infos, ti_dearray);

                let mut suffix = "[".to_string();
                for _ in 1..array_rank {
                    suffix += ","
                }
                suffix += "]";

                let full_name = element_name + &suffix;

                type_infos[index].full_name = Some(full_name.clone());

                return full_name;
            }

            let parent_string = match parent {
                None => "".to_owned(),
                Some(TypeParent::Namespace(namespace)) => format!("{namespace}."),
                Some(TypeParent::OuterType(ti_outer)) => build_symbol(type_infos, ti_outer) + ".",
            };

            let mut full_name = parent_string + &name;
            if let Some(Generics::Constructed { ti_args, .. }) = generics {
                let args: Vec<String> = ti_args
                    .into_iter()
                    .map(|ti_arg| build_symbol(type_infos, ti_arg))
                    .collect();
                full_name += "<";
                full_name += &args.join(",");
                full_name += ">";
            }

            type_infos[index].full_name = Some(full_name.clone());

            full_name
        }

        for i in 0..type_infos.len() {
            build_symbol(&mut type_infos, i);
        }

        let mut order: Vec<_> = (0..type_infos.len()).collect();
        order.sort_by_key(|&i| type_infos[i].full_name.as_ref().unwrap());
        let mut remap: Vec<_> = vec![0; type_infos.len()];
        for (i, &o) in order.iter().enumerate() {
            remap[o] = i
        }

        fn ti_remap(ti: &mut usize, remap: &[usize]) {
            *ti = remap[*ti];
        }

        fn ti_remap_attributes(attributes: &mut [AttributeInfo], remap: &[usize]) {
            for attr in attributes {
                ti_remap(&mut attr.ti_attribute, remap);
            }
        }

        fn ti_remap_param(param: &mut ParamInfo, remap: &[usize]) {
            ti_remap(&mut param.ti, remap);
            ti_remap_attributes(&mut param.attributes, remap)
        }

        for type_info in &mut type_infos {
            if let Some(ti) = &mut type_info.ti_array {
                ti_remap(ti, &remap);
            }
            if let Some(ti) = &mut type_info.ti_dearray {
                ti_remap(ti, &remap);
            }
            if let Some(ti) = &mut type_info.ti_base {
                ti_remap(ti, &remap);
            }
            if let Some(TypeParent::OuterType(ti)) = &mut type_info.parent {
                ti_remap(ti, &remap);
            }
            if let Some(Generics::Constructed {
                ti_template,
                ti_args,
            }) = &mut type_info.generics
            {
                ti_remap(ti_template, &remap);
                for ti in ti_args {
                    ti_remap(ti, &remap);
                }
            } else if let Some(Generics::Template { params }) = &mut type_info.generics {
                for param in params {
                    if let Some(ti) = &mut param.ti_constraint {
                        ti_remap(ti, &remap);
                    }
                }
            }
            ti_remap_attributes(&mut type_info.attributes, &remap);
            for interface in &mut type_info.interfaces {
                ti_remap(&mut interface.ti, &remap);
            }
            for method in &mut type_info.methods {
                ti_remap_attributes(&mut method.attributes, &remap);
                ti_remap_param(&mut method.ret, &remap);
                for param in &mut method.params {
                    ti_remap_param(param, &remap);
                }
            }
            for field in &mut type_info.fields {
                ti_remap(&mut field.ti, &remap);
                ti_remap_attributes(&mut field.attributes, &remap);
            }
            for property in &mut type_info.properties {
                ti_remap_attributes(&mut property.attributes, &remap);
            }
        }

        type_infos.sort_by_key(|t| t.full_name.clone());

        let intern_strings = intern_strings
            .into_iter()
            .map(&read_string)
            .collect::<Result<_>>()?;

        let assembly_infos = assemblies
            .into_iter()
            .map(|a| {
                Ok(AssemblyInfo {
                    name: read_string(a.name_offset)?,
                    full_path: read_string(a.full_path_offset)?,
                    dll_name: read_string(a.dll_name_offset)?,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Tdb {
            types: type_infos,
            intern_strings,
            assemblies: assembly_infos,
        })
    }

    pub fn write_json(&self, path: &str) -> Result<()> {
        serde_json::to_writer_pretty(std::fs::File::create(path)?, &self)?;
        Ok(())
    }

    pub fn write_map(&self, path: &str) -> Result<()> {
        let mut function_map: Vec<(u32, String)> = self
            .types
            .iter()
            .flat_map(|t| {
                t.methods.iter().map(|method| {
                    let symbol = format!("{}.{}", t.full_name.as_ref().unwrap(), method.name);
                    (method.address, symbol)
                })
            })
            .collect();
        function_map.sort_by_key(|f| f.0);

        let mut map = File::create(path)?;
        for (address, name) in function_map {
            writeln!(map, "{} {:016X} f", name, address)?
        }

        Ok(())
    }

    pub fn write_cs(&self, path: &str, options: &crate::TdbOptions) -> Result<()> {
        let mut output = std::fs::File::create(path)?;

        let type_infos = &self.types;

        fn print_arg(arg: &Arg, output: &mut File) -> Result<()> {
            match arg {
                Arg::SByte(x) => write!(output, "{x}")?,
                Arg::Byte(x) => write!(output, "{x}")?,
                Arg::Char(x) => write!(output, "'\\u{x:04X}'")?,
                Arg::Int16(x) => write!(output, "{x}")?,
                Arg::UInt16(x) => write!(output, "{x}")?,
                Arg::Int32(x) => write!(output, "{x}")?,
                Arg::UInt32(x) => write!(output, "{x}")?,
                Arg::Int64(x) => write!(output, "{x}")?,
                Arg::UInt64(x) => write!(output, "{x}")?,
                Arg::Single(x) => write!(output, "{x}")?,
                Arg::Double(x) => write!(output, "{x}")?,
                Arg::Boolean(x) => write!(output, "{x}")?,
                Arg::String(x) | Arg::Type(x) => {
                    if let Some(x) = x {
                        write!(output, "\"{x}\"")?
                    } else {
                        write!(output, "null")?
                    }
                }
                Arg::Array(x) => {
                    write!(output, "[")?;
                    for a in x {
                        print_arg(a, output)?;
                        write!(output, ",")?
                    }
                    write!(output, "]")?;
                }
            }
            Ok(())
        }

        let print_attributes =
            |attributes: &[AttributeInfo], return_pos: bool, output: &mut File| -> Result<()> {
                for attribute in attributes {
                    let symbol = type_infos[attribute.ti_attribute]
                        .full_name
                        .as_ref()
                        .unwrap();
                    let return_pos = if return_pos { "return:" } else { "" };
                    write!(output, "[{}{}(", return_pos, symbol)?;
                    for positional in &attribute.positional_args {
                        print_arg(positional, output)?;
                        write!(output, ",")?;
                    }
                    for named in &attribute.named_args {
                        write!(output, "{}=", named.name)?;
                        print_arg(&named.arg, output)?;
                        write!(output, ",")?;
                    }
                    write!(output, ")]")?;
                }
                Ok(())
            };

        fn print_constant(value: &Option<ValueInfo>, output: &mut File) -> Result<()> {
            match value {
                None => (),
                Some(ValueInfo::String(s)) => write!(output, " = \"{}\"", s)?,
                Some(ValueInfo::Bytes(b)) => match b.len() {
                    1 => write!(output, " = 0x{:02X}", b[0])?,
                    2 => write!(
                        output,
                        " = 0x{:04X}",
                        u16::from_le_bytes(b.as_slice().try_into().unwrap())
                    )?,
                    4 => write!(
                        output,
                        " = 0x{:08X}",
                        u32::from_le_bytes(b.as_slice().try_into().unwrap())
                    )?,
                    8 => write!(
                        output,
                        " = 0x{:016X}",
                        u64::from_le_bytes(b.as_slice().try_into().unwrap())
                    )?,
                    _ => write!(output, " = {:?}", b)?,
                },
            }
            Ok(())
        }

        for type_info in type_infos {
            let full_name = type_info.full_name.as_ref().unwrap();

            #[allow(clippy::collapsible_if)]
            if options.no_compound {
                if type_info.ti_dearray.is_some()
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
            writeln!(output, "/// % {:08X}", type_info.hash)?;
            if !type_info.attributes.is_empty() {
                print_attributes(&type_info.attributes, false, &mut output)?;
                writeln!(output,)?;
            }
            if !options.no_type_flag {
                writeln!(output, "{}", display_type_flags(type_info.flags))?;
            }
            let base_name = if let Some(ti_base) = type_info.ti_base {
                type_infos[ti_base].full_name.as_ref().unwrap().as_str()
            } else {
                ""
            };
            writeln!(output, "class {}: {}", full_name, base_name)?;

            if calc_hash != type_info.hash {
                bail!("Mismatched hash for {}", full_name)
            }

            for interface in &type_info.interfaces {
                writeln!(
                    output,
                    "    ,{} /* ^{} */",
                    type_infos[interface.ti].full_name.as_ref().unwrap(),
                    interface.vtable_slot_start,
                )?;
            }

            writeln!(output, "{{")?;
            if type_info.ti_dearray.is_some() || full_name.contains('!') {
                writeln!(output, "    // Omitted ")?;
                writeln!(output, "}}")?;
                writeln!(output,)?;
                continue;
            }

            writeln!(output, "    // Special = {}", type_info.system_type)?;

            match &type_info.generics {
                None => (),
                Some(Generics::Template { params }) => {
                    writeln!(output, "    // Template = {}", full_name)?;
                    for param in params {
                        writeln!(
                            output,
                            "     // param {}, 0x{:08X}",
                            param.name, param.flags
                        )?;
                    }
                }
                Some(Generics::Constructed { ti_template, .. }) => {
                    writeln!(
                        output,
                        "    // Template = {}",
                        type_infos[*ti_template].full_name.as_ref().unwrap()
                    )?;
                    writeln!(output, "    // Omitted ")?;
                    writeln!(output, "}}")?;
                    writeln!(output,)?;
                    continue;
                }
            }

            writeln!(output,)?;
            writeln!(output, "    /*** Method ***/")?;
            writeln!(output,)?;

            for method in &type_info.methods {
                if !method.attributes.is_empty() {
                    write!(output, "    ")?;
                    print_attributes(&method.attributes, false, &mut output)?;
                    writeln!(output,)?;
                }

                if !method.ret.attributes.is_empty() {
                    write!(output, "    ")?;
                    print_attributes(&method.ret.attributes, true, &mut output)?;
                    writeln!(output,)?;
                }

                writeln!(
                    output,
                    "    {}{}{}\n    {} {} (",
                    display_param_modifier(method.ret.modifier, true),
                    display_method_impl_flag(method.impl_flags),
                    display_method_attributes(method.flags),
                    type_infos[method.ret.ti].full_name.as_ref().unwrap(),
                    method.name,
                )?;

                for param in &method.params {
                    write!(output, "        ")?;
                    if !param.attributes.is_empty() {
                        print_attributes(&param.attributes, false, &mut output)?;
                    }
                    write!(
                        output,
                        "{}{} {} {}",
                        display_param_modifier(param.modifier, false),
                        display_param_attributes(param.flags),
                        type_infos[param.ti].full_name.as_ref().unwrap(),
                        param.name
                    )?;

                    print_constant(&param.default, &mut output)?;
                    writeln!(output, ",")?;
                }

                let address = if !options.no_runtime && method.address != 0 {
                    format!(" = 0x{:08X}", method.address)
                } else {
                    "".to_string()
                };

                writeln!(output, "    ){};\n", address)?;
            }

            writeln!(output,)?;
            writeln!(output, "    /*** Field ***/")?;
            writeln!(output,)?;

            for field in &type_info.fields {
                if !field.attributes.is_empty() {
                    write!(output, "    ")?;
                    print_attributes(&field.attributes, false, &mut output)?;
                    writeln!(output,)?;
                }
                write!(
                    output,
                    "    {} {} {}",
                    display_field_attributes(field.flags),
                    type_infos[field.ti].full_name.as_ref().unwrap(),
                    field.name
                )?;

                print_constant(&field.value, &mut output)?;

                writeln!(output, ";")?;
            }

            writeln!(output,)?;
            writeln!(output, "    /*** Event ***/")?;
            writeln!(output,)?;
            for event in &type_info.events {
                writeln!(output, "    public event {};", event.name)?;
            }

            writeln!(output,)?;
            writeln!(output, "    /*** Property ***/")?;
            writeln!(output,)?;
            for property in &type_info.properties {
                if !property.attributes.is_empty() {
                    write!(output, "    ")?;
                    print_attributes(&property.attributes, false, &mut output)?;
                    writeln!(output,)?;
                }
                writeln!(
                    output,
                    "    {}public property {};",
                    display_property_flag(property.flags),
                    property.name
                )?;
            }

            writeln!(output, "}}")?;
            writeln!(output,)?;
        }

        for s in &self.intern_strings {
            writeln!(output, "// ~ {}", s)?;
        }

        for assembly in &self.assemblies {
            writeln!(
                output,
                "// <Asm> {}, {}, {}",
                assembly.name, assembly.full_path, assembly.dll_name
            )?;
        }

        Ok(())
    }
}

pub fn print<F: Read + Seek>(file: F, base_address: u64, options: crate::TdbOptions) -> Result<()> {
    if options.json.is_none() && options.map.is_none() && options.cs.is_none() {
        eprintln!("Please specify at least one of --json, --map, --cs");
        return Ok(());
    }

    let tdb = Tdb::new(file, base_address)?;

    if let Some(json) = &options.json {
        tdb.write_json(json)?;
    }
    if let Some(map) = &options.map {
        tdb.write_map(map)?;
    }
    if let Some(cs) = &options.cs {
        tdb.write_cs(cs, &options)?;
    }
    Ok(())
}
