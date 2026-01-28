use std::io::Write;

use crate::error::TalosResult;

pub fn write_all_bytes<T: Write>(writer: &mut T, bytes: &[u8]) -> TalosResult<()> {
    writer.write_all(bytes).map_err(Into::into)
}

pub fn u16_as_ascii_bytes(n: &u16) -> [u8; 2] {
    let mut bytes = [0u8; 2];

    bytes[0] = (n % 256) as u8;
    bytes[1] = (n / 256) as u8;

    bytes
}
