pub fn hash_as_utf16(s: &str) -> u32 {
    let bytes: Vec<u8> = s.encode_utf16().flat_map(u16::to_le_bytes).collect();
    murmur3::murmur3_32(&mut &bytes[..], 0xFFFF_FFFF).unwrap()
}

pub fn hash_as_utf8(s: &str) -> u32 {
    murmur3::murmur3_32(&mut s.as_bytes(), 0xFFFF_FFFF).unwrap()
}
