pub trait BitField<Bits> {
    type Output;
    fn bit_split(self, bits: Bits) -> Self::Output;
}

macro_rules! smash {
    ($a:ident) => {
        usize
    };
}

macro_rules! smash2 {
    ($a:ident, $b:ty) => {
        $b
    };
}

macro_rules! impl_bit {
    ($base_type:ty, $($a:ident),+) => {
        impl BitField<($(smash!($a)),*)> for $base_type {
            type Output = ($(smash2!($a, $base_type)),*);
            #[allow(unused_assignments)]
            fn bit_split(mut self, ($($a),*): ($(smash!($a)),*)) -> Self::Output {
                if 0 $(+$a)* != std::mem::size_of::<$base_type>() * 8 {
                    panic!();
                }
                $(
                    let t = self & ((1 << $a) - 1);
                    self >>= $a;
                    let $a = t;
                )*

                ($($a),*)
            }
        }
    };
}

impl_bit!(u8, a, b);
impl_bit!(u8, a, b, c);
impl_bit!(u8, a, b, c, d);
impl_bit!(u8, a, b, c, d, e);

impl_bit!(u16, a, b);
impl_bit!(u16, a, b, c);
impl_bit!(u16, a, b, c, d);
impl_bit!(u16, a, b, c, d, e);

impl_bit!(u32, a, b);
impl_bit!(u32, a, b, c);
impl_bit!(u32, a, b, c, d);
impl_bit!(u32, a, b, c, d, e);

impl_bit!(u64, a, b);
impl_bit!(u64, a, b, c);
impl_bit!(u64, a, b, c, d);
impl_bit!(u64, a, b, c, d, e);
