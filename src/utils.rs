use std::io::Write;

pub fn write_all_bytes<T: Write>(writer: &mut T, bytes: &[u8]) -> std::io::Result<()> {
    writer.write_all(bytes)
}
