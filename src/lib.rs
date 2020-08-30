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
    fn test_copy_paste_input_invalid() {
        assert_eq!(copy_paste_input(0x2e4, "f800000000"), 0xe3);
    }

    #[test]
    fn test_copy_paste_input_valid() {
        assert_eq!(copy_paste_input(0x2e4, "f8000000e3"), 0xe3);
    }

    #[test]
    fn test_nelsons_corolla_2() {
        assert_eq!(copy_paste_input(0x2e4, "e2000000cd"), 0xcd)
    }


    #[test]
    fn test_nelsons_corolla_128() {
        assert_eq!(copy_paste_input(0x2e4, "e2000000cd"), 0xcd)
    }

    #[test]
    fn test_nelsons_corolla_130() {
        assert_eq!(copy_paste_input(0x2e4, "940000007f"), 0x7f)
    }

    #[test]
    fn test_nelsons_corolla_0() {
        assert_eq!(copy_paste_input(0x2e4, "940000007f"), 0x7f)
    }

    #[test]
    fn test_comma_prime_lka_128() {
        assert_eq!(copy_paste_input(0x2e4, "ba000000a5"), 0xa5);
    }

    #[test]
    fn test_comma_prime_lka_2() {
        assert_eq!(copy_paste_input(0x2e4, "a4000000fb3b64cd"), 0xcd);
    }

    #[test]
    fn test_comma_prime_lka_2_trim() {
        assert_eq!(copy_paste_input(0x2e4, "a4000000fb"), 0xfb);
    }

    #[test]
    fn test_comma_prime_lka_0() {
        assert_eq!(copy_paste_input(0x2e4, "920000004d2ac577"), 0x77);
    }
}
