use super::*;
use serde::*;

#[derive(Debug, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(into = "String")]
pub struct Guid {
    pub bytes: [u8; 16],
}

impl FieldFromRsz for Guid {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        let mut bytes = [0; 16];
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
