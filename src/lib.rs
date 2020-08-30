

fn copy_paste_input(address: u16, data_string: &str) -> u8 {
    let decoded_bytes = hex::decode(data_string).unwrap();
    comma_input(address, &decoded_bytes)
}


fn comma_input(address: u16, data: &[u8]) -> u8 {
    let address_bytes = address.to_be_bytes();
    let idh = address_bytes[0];
    let idl = address_bytes[1];
    let len = data.len() as u8;
    let data_trimmed = &data[..((len - 1) as usize)];
    ioactive_checksum(idh, idl, len, data_trimmed)
}

fn toyota_checksum_existing(address: u16, data: u64, length: u8) -> u8 {
    0
}

pub fn ioactive_checksum(idh: u8, idl: u8, len: u8, data: &[u8]) -> u8 {
    let data_sum: u8 = data
        .iter()
        .fold(0u8, |state, value| state.wrapping_add(*value));
    idh.wrapping_add(idl)
        .wrapping_add(len)
        .wrapping_add(data_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioactive_from_pdf() {
        assert_eq!((0x02 + 0xe4 + 0x05 + 0xf8) & 0xff, 0xe3);
    }

    #[test]
    fn test_ioactive_rust_impl() {
        assert_eq!(ioactive_checksum(0x02, 0xe4, 0x05, &[0xf8, 0, 0, 0]), 0xe3);
    }

    #[test]
    fn test_comma_input() {
        assert_eq!(comma_input(0x02e4, &[0xf8, 0, 0, 0, 0]), 0xe3);
    }

    #[test]
    fn test_copy_paste_input() {
        assert_eq!(copy_paste_input(0x2e4, "f800000000"), 0xe3);
    }
}
