pub fn hash_as_utf16(s: &str) -> u32 {
    fn iter(a: [u8; 2]) -> impl Iterator<Item = u8> {
        std::iter::once(a[0]).chain(std::iter::once(a[1]))
    }
    let bytes: Vec<u8> = s
        .encode_utf16()
        .flat_map(|u| iter(u16::to_le_bytes(u)))
        .collect();
    murmur3::murmur3_32(&mut &bytes[..], 0xFFFF_FFFF).unwrap()
}
