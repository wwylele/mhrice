use std::ops::*;

pub fn align_up<T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>>(
    value: T,
    align: T,
) -> T {
    value + (align - value % align) % align
}
