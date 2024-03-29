pub fn copy_paste_input(address: u16, data_string: &str) -> u8 {
    let decoded_bytes = hex::decode(data_string).unwrap();
    comma_input(address, &decoded_bytes)
}

pub fn comma_input(address: u16, data: &[u8]) -> u8 {
    let address_bytes = address.to_be_bytes();
    let idh = address_bytes[0];
    let idl = address_bytes[1];
    let len = data.len() as u8;
    let data_trimmed = &data[..((len - 1) as usize)];
    ioactive_checksum(idh, idl, len, data_trimmed)
}

pub fn copy_paste_nelson_input(address: u16, data_string: &str) -> u64 {
    let decoded_bytes = hex::decode(data_string).unwrap();
    nelson_input(address, &decoded_bytes)
}

pub fn nelson_input(address: u16, data: &[u8]) -> u64 {
    let address_bytes = address.to_be_bytes();
    let idh = address_bytes[0];
    let idl = address_bytes[1];
    let len = data.len() as u8;
    let data_trimmed = &data[..((len - 4) as usize)];
    ioactive_checksum_unwrapped(idh, idl, len, data_trimmed)
}

pub fn ioactive_checksum(idh: u8, idl: u8, len: u8, data: &[u8]) -> u8 {
    let data_sum: u8 = data
        .iter()
        .fold(0u8, |state, value| state.wrapping_add(*value));
    idh.wrapping_add(idl)
        .wrapping_add(len)
        .wrapping_add(data_sum)
}

pub fn ioactive_checksum_unwrapped(idh: u8, idl: u8, len: u8, data: &[u8]) -> u64 {
    let data_sum = data.iter().fold(0, |state, value| state + *value as u64);
    idh as u64 + idl as u64 + len as u64 + data_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    mod oldalg {
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
        fn test_toto_lta() {
            assert_eq!(copy_paste_input(0x190, "02fc00dc595cf0"), 0xf0)
        }

        #[test]
        fn test_nelson_lta() {
            assert_eq!(copy_paste_input(0x191, "f600003064640088"), 0x88)
        }


        #[test]
        fn test_rohang88_lka() {
            assert_eq!(copy_paste_input(0x2e4, "c2000000ad"), 0xad)
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
        fn test_jamil() {
            assert_eq!(copy_paste_input(0x262, "0000000200ff"), 0x6c)
        }

        #[test]
        fn test_comma_prime_lka_same_data_1() {
            assert_eq!(
                copy_paste_input(0x2e4, "a20000007f66e2e6"),
                0xe6
            );
        }

        #[test]
        fn test_comma_prime_lka_same_data_2() {
            assert_eq!(
                copy_paste_input(0x2e4, "a2000000a7ea6089"),
                0x89
            );
        }

        #[test]
        fn test_comma_prime_lka_same_data_2_first() {
            assert_eq!(
                copy_paste_input(0x2e4, "a2000000a7ea6089"),
                0xa7
            );
        }

        #[test]
        fn test_comma_prime_lta() {
            assert_eq!(
                copy_paste_input(0x191, "800000000005001f"),
                0x1f
            );
        }

        #[test]
        fn test_comma_prime_lta_alt() {
            assert_eq!(
                copy_paste_input(0x191, "df00000000000079"),
                0x79
            );
        }
    }

    #[test]
    fn test_comma_prime_lka_128() {
        assert_eq!(copy_paste_input(0x2e4, "ca000000a5"), 0xb5);
    }

    #[test]
    fn test_comma_matty_prime_lka_2() {
        assert_eq!(copy_paste_input(0x2e4, "d8000000022d38ec"), 0xec);
    }

    #[test]
    fn test_comma_matty_prime_lka_0_last() {
        assert_eq!(copy_paste_input(0x2e4, "9200000000000077"), 0x77);
    }

    #[test]
    fn test_comma_matty_prime_lka_0() {
        assert_eq!(copy_paste_input(0x2e4, "920000004d2ac577"), 0x77);
    }


    #[test]
    fn test_comma_matty_prime_pc_0() {
        assert_eq!(copy_paste_input(0x283, "0000000093264705"), 0x05);
    }

    #[test]
    fn test_comma_md1000_sienna_lka_0() {
        assert_eq!(copy_paste_input(0x2e4, "a800000075e20a7d"), 0x7d);
    }

    #[test]
    fn test_comma_some_chr_hybrid_lka_0() {
        assert_eq!(copy_paste_input(0x2e4, "a6000000dbe07e5a"), 0x5a);
    }

    #[test]
    fn test_comma_prime_lka_nelson_0() {
        assert_eq!(
            copy_paste_nelson_input(0x2e4, "920000004d2ac577"),
            0x4d2ac577
        );
    }

    #[test]
    fn test_comma_prime_lka_nelson_crc32() {
        assert_eq!(
            copy_paste_nelson_input(0x2e4, "920000004d2ac577"),
            0x4d2ac577
        );
    }



    #[test]
    fn test_crc32_just_data() {
        // 9e00000022714b01
        assert_eq!(
            format!("{:02x}", crc::crc32::checksum_ieee(&[0x02, 0xe4, 0x08, 0x9e, 0x00, 0x00, 0x00])),
            format!("{:02x}", 0x4d2ac577)
        );
    }
}
