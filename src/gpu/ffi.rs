use std::convert::{TryFrom, TryInto};

#[link(name = "ffi-shim")]
extern "C" {
    fn astc_decompress_block_ffi(
        astc_data: *const u8,
        block_width: u8,
        block_height: u8,
        out_buffer: *mut u8,
    );

    fn bc7_decompress_block_ffi(in_buffer: *const u8, out_buffer: *mut u8);
}

pub fn atsc_decompress_block<F: FnMut(usize, usize, [u8; 4])>(
    in_buf: &[u8; 16],
    block_width: usize,
    block_height: usize,
    mut writer: F,
) {
    let mut buffer = vec![0; block_width * block_height * 4];
    unsafe {
        astc_decompress_block_ffi(
            in_buf.as_ptr(),
            u8::try_from(block_width).unwrap(),
            u8::try_from(block_height).unwrap(),
            buffer.as_mut_ptr(),
        );
    }
    for y in 0..block_height {
        for x in 0..block_width {
            let i = (x + y * block_width) * 4;
            writer(x, y, buffer[i..][..4].try_into().unwrap())
        }
    }
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
