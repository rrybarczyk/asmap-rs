use crate::common::*;

/// Convert two u8's into one u16
pub(crate) fn read_be_u16(input: &mut &[u8]) -> Result<u16, Error> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;

    let result = u16::from_be_bytes(int_bytes.try_into().map_err(|error| Error::TryFromSlice {
        bad_input: input.to_vec(),
        num_type: String::from("u16"),
        error,
    })?);

    Ok(result)
}

/// Convert four u8's into one u32
pub(crate) fn read_be_u32(input: &mut &[u8]) -> Result<u32, Error> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;

    let result = u32::from_be_bytes(int_bytes.try_into().map_err(|error| Error::TryFromSlice {
        bad_input: input.to_vec(),
        num_type: String::from("u32"),
        error,
    })?);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_bytes_to_u16() -> Result<(), Error> {
        let length_bytes = vec![0, 10];
        let have = read_be_u16(&mut length_bytes.as_slice())?;
        let want = 10u16;

        assert_eq!(have, want);

        Ok(())
    }

    #[test]
    fn convert_bytes_to_u32() -> Result<(), Error> {
        let length_bytes = vec![0, 0, 0, 10];
        let have = read_be_u32(&mut length_bytes.as_slice())?;
        let want = 10u32;

        assert_eq!(have, want);

        Ok(())
    }
}
