#[derive(PartialEq, Eq, Debug)]
pub enum EndianType {
    BigEndian,
    LittleEndian,
    UnknownEndian,
}

pub fn get_endian_type() -> EndianType {
    let uint32 = 1u32;
    let bytes = uint32.to_be_bytes();
    match bytes[0] {
        1u8 => EndianType::BigEndian,
        0u8 => EndianType::LittleEndian,
        _ => EndianType::UnknownEndian,
    }
}

#[cfg(test)]
mod endian_check {
    use super::*;

    #[test]
    fn test_get_endian_type() {
        let target_endian = if cfg!(target_endian = "little") {
            EndianType::LittleEndian
        } else if cfg!(target_endian = "big") {
            EndianType::BigEndian
        } else {
            EndianType::UnknownEndian
        };
        assert_eq!(get_endian_type(), target_endian)
    }
}
