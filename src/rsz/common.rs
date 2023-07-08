use super::*;
use nalgebra_glm::*;
use serde::*;

#[macro_export]
macro_rules! rsz_inner {
    ($rsz:ident, $($field_name:ident : $field_type:ty,)*) => {
        Ok(Self {
            $(
                $field_name: <$field_type>::field_from_rsz($rsz).context(stringify!($field_name))?,
            )*
        })
    }
}

#[macro_export]
macro_rules! rsz_inner_trait {
    (rsz($symbol:tt $(,path=$singleton:literal)? $(,$vhash:literal=$version:literal)*),
        $struct_name:ident, $($field_name:ident : $field_type:ty,)*) => {
        impl $crate::rsz::FromRsz for $struct_name {
            const SYMBOL: &'static str = $symbol;
            const VERSIONS: &'static [(u32, u32)] = &[$(($vhash, $version)),*];
            #[allow(unused_variables)]
            fn from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                $crate::rsz_inner!(rsz, $($field_name : $field_type,)*)
            }
        }

        $(impl $crate::rsz::SingletonUser for $struct_name {
            const PATH: &'static str = $singleton;
            type RszType = Self;
            fn from_rsz(value: Self::RszType) -> Self {
                value
            }
        })?
    };

    (rsz(), $struct_name:ident, $($field_name:ident : $field_type:ty,)*) => {
        impl $crate::rsz::FieldFromRsz for $struct_name {
            #[allow(unused_variables)]
            fn field_from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                $crate::rsz_inner!(rsz, $($field_name : $field_type,)*)
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_struct {
    (
        #[rsz($($symbol:tt $(,path=$singleton:literal)? $(,$vhash:literal=$version:literal)* $(,)?)?)]
        $(#[$outer_meta:meta])*
        $outer_vis:vis struct $struct_name:ident {
            $(
                $(#[$inner_meta:meta])*
                $inner_vis:vis $field_name:ident : $field_type:ty
            ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])*
        $outer_vis struct $struct_name {
            $(
                $(#[$inner_meta])* #[allow(dead_code)]
                $inner_vis $field_name : $field_type,
            )*
        }

        $crate::rsz_inner_trait!(rsz($($symbol $(,path=$singleton)? $(,$vhash=$version)*)?),
            $struct_name, $($field_name : $field_type,)*);
    };
}

#[macro_export]
macro_rules! rsz_enum_arm {
    ($enum_name:ident, $variant:ident, $raw:ident, $value:literal, $end_value:literal) => {
        $enum_name::$variant($raw - $value)
    };
    ($enum_name:ident, $variant:ident, $raw:ident, $value:literal) => {
        $enum_name::$variant
    };
}

#[macro_export]
macro_rules! rsz_enum_arm_rev_left {
    ($i: ident, $enum_name:ident, $variant:ident, $value:literal, $end_value:literal) => {
        $enum_name::$variant($i)
    };
    ($i: ident, $enum_name:ident, $variant:ident, $value:literal) => {
        $enum_name::$variant
    };
}

#[macro_export]
macro_rules! rsz_enum_arm_rev_right {
    ($i: ident, $value:literal, $end_value:literal) => {
        $i + $value
    };
    ($i: ident, $value:literal) => {
        $value
    };
}

#[macro_export]
macro_rules! rsz_enum {
    (
        #[rsz($base:ty)]
        $(#[$outer_meta:meta])*
        $outer_vis:vis enum $enum_name:ident {
            $( $variant:ident $(($field:ty))? = $value:literal $(..= $end_value:literal)? ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])* #[allow(clippy::enum_variant_names)]
        $outer_vis enum $enum_name {
            $( $variant $(($field))?, )*
        }

        impl $enum_name {
            pub fn from_raw(raw: $base) -> Result<Self> {
                Ok(#[allow(unreachable_patterns)] match raw {
                    $(
                        $value $(..=$end_value)? =>
                        $crate::rsz_enum_arm!($enum_name, $variant, raw, $value $(, $end_value)?),
                    )*
                    x => bail!("Unknown value {} for enum {}", x, stringify!($enum_name))
                })
            }

            #[allow(dead_code)]
            pub fn into_raw(self) -> $base {
                match self {
                    $(
                        $crate::rsz_enum_arm_rev_left!(i,$enum_name, $variant, $value $(, $end_value)?)
                        => $crate::rsz_enum_arm_rev_right!(i, $value $(, $end_value)?),
                    )*
                }
            }
        }

        impl $crate::rsz::FieldFromRsz for $enum_name {
            fn field_from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                let raw = <$base>::field_from_rsz(rsz)?;
                Self::from_raw(raw)
            }
        }
    };
}

#[macro_export]
macro_rules! rsz_sumtype {
    (
        $(#[$outer_meta:meta])*
        $outer_vis:vis enum $enum_name:ident {
            $( $variant:ident($variant_type:ty) ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])* #[allow(clippy::enum_variant_names)]
        $outer_vis enum $enum_name {
            $( $variant($variant_type), )*
        }

        impl $crate::rsz::FieldFromRsz for $enum_name {
            fn field_from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                let current = rsz.cursor.stream_position()?;
                $(
                    if let Ok(v) = <$variant_type>::field_from_rsz(rsz) {
                        return Ok($enum_name::$variant(v))
                    }
                    rsz.cursor.seek(SeekFrom::Start(current))?;
                )*
                bail!("No matching type for sum type {}", stringify!($enum_name))
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_sumuser {
    (
        $(#[$outer_meta:meta])*
        $outer_vis:vis enum $enum_name:ident {
            $( $variant:ident($variant_type:ty) ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])* #[allow(clippy::enum_variant_names)]
        $outer_vis enum $enum_name {
            $( $variant($variant_type), )*
        }

        impl $crate::rsz::FromUser for $enum_name {
            fn from_any(any: AnyRsz) -> Result<Self> {
                $(
                    if any.symbol() == <$variant_type>::SYMBOL {
                        return Ok($enum_name::$variant(Rc::try_unwrap(any.downcast().unwrap()).map_err(|_| anyhow!("Shared user node"))?))
                    }
                )*
                bail!("No matching type for {}", any.symbol())
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_bitflags {
    (
        $(#[$outer_meta:meta])*
        pub struct $name:ident : $base:ty {
            $( const $field_name:ident = $field_value:literal; )*
        }
    ) => {
        bitflags! {
            $(#[$outer_meta])*
            #[derive(Serialize, Clone, Debug, Hash, PartialEq, Eq)]
            #[serde(into = "Vec<&'static str>")]
            pub struct $name : $base {
                $( const $field_name = $field_value; )*
            }
        }
        impl $crate::rsz::FieldFromRsz for $name {
            fn field_from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                let value = <$base>::field_from_rsz(rsz)?;
                <$name>::from_bits(value).with_context(|| {
                    format!("Unknown bit flag {:08X} for {}", value, stringify!($name))
                })
            }
        }

        impl From<$name> for Vec<&'static str> {
            fn from(v: $name) -> Vec<&'static str> {
                let mut result = vec![];
                $( if v.contains($name::$field_name) {
                    result.push(stringify!($field_name))
                } )*
                result
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_newtype {
    (
        #[rsz_offset($offset:literal)]
        $(#[$outer_meta:meta])*
        $outer_vis:vis struct $name:ident($inner_vis:vis $base:ty);
    ) => (
        $(#[$outer_meta])*
        $outer_vis struct $name($inner_vis $base);

        impl $crate::rsz::FieldFromRsz for $name {
            fn field_from_rsz(rsz: &mut $crate::rsz::RszDeserializer) -> Result<Self> {
                let raw = <$base>::field_from_rsz(rsz)?;
                Ok($name(raw + $offset))
            }
        }
    )
}

#[macro_export]
macro_rules! rsz_with_singleton {
    (
        $(
            #[path($path:literal)]$
            outer_vis:vis struct $name:ident($t:ty);
        )*
    ) => {
        $(
            #[derive(Serialize, Debug)]
            pub struct $name {
                #[serde(flatten)]
                inner: $t,
            }

            impl Deref for $name {
                type Target = $t;

                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }
            impl $crate::rsz::SingletonUser for $name {
                const PATH: &'static str = $path;
                type RszType = $t;
                fn from_rsz(value: Self::RszType) -> Self {
                    Self {inner: value}
                }
            }
        )*
    };
}

impl FieldFromRsz for bool {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        match rsz.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => bail!("Invalid bool"),
        }
    }
}

impl FieldFromRsz for u8 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.read_u8()
    }
}

impl FieldFromRsz for u16 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(2)?;
        rsz.read_u16()
    }
}

impl FieldFromRsz for u32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_u32()
    }
}

impl FieldFromRsz for u64 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(8)?;
        rsz.read_u64()
    }
}

impl FieldFromRsz for i8 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.read_i8()
    }
}

impl FieldFromRsz for i16 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(2)?;
        rsz.read_i16()
    }
}

impl FieldFromRsz for i32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_i32()
    }
}

impl FieldFromRsz for i64 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(8)?;
        rsz.read_i64()
    }
}

impl FieldFromRsz for f32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_f32()
    }
}

// A wrapper of f32 that has bit-equality semantics
#[derive(Clone, Copy)]
pub struct MeqF32(pub f32);

impl FieldFromRsz for MeqF32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        Ok(MeqF32(rsz.read_f32()?))
    }
}

impl std::cmp::PartialEq for MeqF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl std::cmp::Eq for MeqF32 {}

impl std::hash::Hash for MeqF32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl std::fmt::Display for MeqF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for MeqF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Serialize for MeqF32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Aligner<const ALIGN: u64>;

impl<const ALIGN: u64> FieldFromRsz for Aligner<ALIGN> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(ALIGN)?;
        Ok(Aligner)
    }
}

impl<T: FromRsz + 'static> FieldFromRsz for T {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.get_child()
    }
}

impl<T: FromRsz + 'static> FieldFromRsz for Option<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.get_child_opt()
    }
}

impl<T: FromRsz + 'static> FieldFromRsz for Rc<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.get_child_rc()
    }
}

impl<T: FieldFromRsz + 'static> FieldFromRsz for Vec<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let count = rsz.read_u32()?;
        (0..count)
            .map(|_| T::field_from_rsz(rsz))
            .collect::<Result<Vec<_>>>()
    }
}

impl FieldFromRsz for Vec<()> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let count = rsz.read_u32()?;
        if count != 0 {
            bail!("Placeholder array not empty")
        }
        Ok(vec![])
    }
}

impl<T: FieldFromRsz + 'static, const N: usize> FieldFromRsz for [T; N] {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        Vec::<T>::field_from_rsz(rsz)?
            .try_into()
            .map_err(|v: Vec<T>| anyhow!("Expected array size {}, found {}", N, v.len()))
    }
}

impl FieldFromRsz for String {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        Option::<String>::field_from_rsz(rsz)?.context("Null String")
    }
}

impl FieldFromRsz for Option<String> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let count = rsz.read_u32()?;
        if count == 0 {
            return Ok(None);
        }
        let mut utf16 = (0..count)
            .map(|_| rsz.read_u16())
            .collect::<Result<Vec<_>>>()?;
        if utf16.pop() != Some(0) {
            bail!("String not null-terminated");
        }
        Ok(Some(String::from_utf16(&utf16)?))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Flatten<T>(pub T);

impl<T: FromRsz> FieldFromRsz for Flatten<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        Ok(Flatten(T::from_rsz(rsz)?))
    }
}

impl<T> Deref for Flatten<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Versioned<T, const MIN: u32, const MAX: u32 = 0xFFFFFFFF>(pub Option<T>);

impl<T: FieldFromRsz, const MIN: u32, const MAX: u32> FieldFromRsz for Versioned<T, MIN, MAX> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        let version = rsz.version();
        Ok(Versioned(if version >= MIN && version <= MAX {
            Some(T::field_from_rsz(rsz)?)
        } else {
            None
        }))
    }
}

#[macro_export]
macro_rules! rsz_versioned_choice {
    (
        $(#[$outer_meta:meta])*
        $outer_vis:vis enum $enum_name:ident {
            $( $variant:ident($field:ty) = $version:pat ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])*
        $outer_vis enum $enum_name {
            $( $variant($field),)*
        }

        impl FieldFromRsz for $enum_name {
            fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
                let version = rsz.version();
                Ok(match version {
                    $($version => $enum_name::$variant(<$field>::field_from_rsz(rsz)?),)*
                    _ => bail!("Unknown version for {}: {}", stringify!($enum_name), version)
                })
            }
        }
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum Zero {
        Zero = 0
    }
}

pub fn ser_arr<S, T: Serialize, const N: usize>(arr: &[T; N], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    arr[..].serialize(s)
}

#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(into = "String")]
pub struct Guid {
    pub bytes: [u8; 16],
}

impl FieldFromRsz for Guid {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        let mut bytes = [0; 16];
        rsz.cursor.seek_align_up(8)?;
        rsz.read_exact(&mut bytes)?;
        Ok(Guid { bytes })
    }
}

impl From<Guid> for String {
    fn from(guid: Guid) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            guid.bytes[3],
            guid.bytes[2],
            guid.bytes[1],
            guid.bytes[0],
            guid.bytes[5],
            guid.bytes[4],
            guid.bytes[7],
            guid.bytes[6],
            guid.bytes[8],
            guid.bytes[9],
            guid.bytes[10],
            guid.bytes[11],
            guid.bytes[12],
            guid.bytes[13],
            guid.bytes[14],
            guid.bytes[15],
        )
    }
}

impl FieldFromRsz for Quat {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(16)?;
        let v = rsz.read_f32vec4()?;
        Ok(Quat::from(v))
    }
}

impl FieldFromRsz for Vec4 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(16)?;
        let v = rsz.read_f32vec4()?;
        Ok(v)
    }
}

impl FieldFromRsz for Vec3 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(16)?;
        let v = rsz.read_f32vec3()?;
        rsz.cursor.seek_align_up(16)?;
        Ok(v)
    }
}

impl FieldFromRsz for IVec3 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let x = rsz.read_i32()?;
        let y = rsz.read_i32()?;
        let z = rsz.read_i32()?;
        Ok(vec3(x, y, z))
    }
}

impl FieldFromRsz for Vec2 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(16)?;
        let v = rsz.read_f32vec2()?;
        rsz.cursor.seek_align_up(16)?;
        Ok(v)
    }
}

impl FieldFromRsz for Mat4x4 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(16)?;
        let v = rsz.read_f32m4x4()?;
        Ok(v)
    }
}
