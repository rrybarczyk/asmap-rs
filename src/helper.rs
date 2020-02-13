use crate::common::*;

pub(crate) fn read_be_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

pub(crate) fn read_be_u16(input: &mut &[u8]) -> u16 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_bytes_to_u16() {
        let length_bytes = vec![0, 10];
        let have = read_be_u16(&mut length_bytes.as_slice());
        let want = 10u16;
        assert_eq!(have, want);
    }

    #[test]
    fn convert_bytes_to_u32() {
        let length_bytes = vec![0, 0, 0, 10];
        let have = read_be_u32(&mut length_bytes.as_slice());
        let want = 10u32;
        assert_eq!(have, want);
    }
}
