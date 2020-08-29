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
    fn ioactive_from_pdf() {
        assert_eq!((0x02 + 0xe4 + 0x05 + 0xf8) & 0xff, 0xe3);
    }

    #[test]
    fn ioactive_rust_impl() {
        assert_eq!(
            ioactive_checksum(0x02, 0xe4, 0x05, &[0xf8, 0, 0, 0]),
            0xe3
        );
    }
}
