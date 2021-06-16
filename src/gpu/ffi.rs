use std::convert::TryInto;

#[link(name = "ffi-shim")]
extern "C" {
    fn bc7_decompress_block_ffi(in_buffer: *const u8, out_buffer: *mut u8);
}

pub fn bc7_decompress_block<F: FnMut(usize, usize, [u8; 4])>(in_buf: &[u8; 16], mut writer: F) {
    let mut buffer = vec![0; 4 * 4 * 4];
    unsafe {
        bc7_decompress_block_ffi(in_buf.as_ptr(), buffer.as_mut_ptr());
    }
    for y in 0..4 {
        for x in 0..4 {
            let i = (x + y * 4) * 4;
            writer(x, y, buffer[i..][..4].try_into().unwrap())
        }
    }
}
