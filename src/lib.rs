
fn toyota_checksum_existing(address: u16, data: u64, length: u8 ) -> u8 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn ioactive() {
        assert_eq!((0x02 + 0xe4 + 0x05 + 0xf8) & 0xff, 0xe3);
    }
}
