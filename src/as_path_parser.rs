use crate::common::*;

#[derive(PartialEq, Debug)]
pub(crate) struct AsPathParser<'buffer> {
    buffer: &'buffer [u8],
    next: usize,
}

impl<'buffer> AsPathParser<'buffer> {
    /// Given a `buffer` with lifetime `'buffer`, constructs a new `AsPathParser` and parses the
    /// attributes.
    pub(crate) fn parse(buffer: &'buffer [u8]) -> Result<Vec<u32>> {
        if buffer.is_empty() {
            info!("Error::MissingPathAttribute, buffer: {:?}", buffer);
            return Err(Error::MissingPathAttribute {
                missing_attribute: "all attributes".to_string(),
            });
        }
        Self::new(buffer).parse_attributes()
    }

    /// Given a `buffer` with lifetime `'buffer`, constructs a new `AsPathParser`
    fn new(buffer: &'buffer [u8]) -> AsPathParser {
        AsPathParser { next: 0, buffer }
    }

    /// Advances forward one in the buffer and returns that byte. Error if `buffer` is already exhausted.
    fn advance(&mut self) -> Result<u8> {
        if self.done() {
            info!("Error::UnexpectedEndOfBuffer {:?}", &self.buffer);
            Err(Error::UnexpectedEndOfBuffer)
        } else {
            let byte = self.buffer[self.next];
            self.next += 1;
            Ok(byte)
        }
    }

    /// Advances forward four in the buffer and returns the fours bytes as a u32.
    fn parse_u32(&mut self) -> Result<u32> {
        let a = self.advance()?;
        let b = self.advance()?;
        let c = self.advance()?;
        let d = self.advance()?;
        Ok(u32::from_be_bytes([a, b, c, d]))
    }

    /// Returns true if buffer is exhausted, false otherwise.
    fn done(&self) -> bool {
        self.next == self.buffer.len()
    }

    fn parse_attributes(mut self) -> Result<Vec<u32>> {
        let mut paths = Vec::new();

        while !self.done() {
            if let Some(path) = self.parse_attribute()? {
                // if there are no asn's in the as path
                if path.is_empty() {
                    info!("Error::NoAsPathInAttributePath {:?}", &self.buffer);
                    return Err(Error::NoAsPathInAttributePath);
                }
                paths.push(path);
            }
        }

        if paths.len() > 1 {
            // Too many asn paths in path attributes
            info!("Error::MultipleAsPaths {:?}", &self.buffer);
            Err(Error::MultipleAsPaths)
        } else if let Some(path) = paths.pop() {
            Ok(path)
        } else {
            info!("Error::NoAsPathInAttributePath{:?}", &self.buffer);
            Err(Error::NoAsPathInAttributePath)
        }
    }

    /// Parses all attributes. If attribute is AS_PATH, p
    fn parse_attribute(&mut self) -> Result<Option<Vec<u32>>> {
        let flag = self.advance()?;
        let type_code = self.advance()?;
        let mut attribute_length: u16 = self.advance()?.into();

        if (flag >> 4) & 1 == 1 {
            attribute_length <<= 8;
            attribute_length |= self.advance()? as u16;
        }

        if type_code == 2 {
            let asn_attr_position_end = &self.next + attribute_length as usize;

            let asn_path = self.parse_as_path();

            if asn_attr_position_end != self.next {
                let leftover_attr_count = asn_attr_position_end - self.next;
                for _ in 0..leftover_attr_count {
                    self.advance()?;
                }
            }

            asn_path
        } else {
            for _ in 0..attribute_length {
                self.advance()?;
            }
            Ok(None)
        }
    }

    /// Takes self and returns a vec of asn
    fn parse_as_path(&mut self) -> Result<Option<Vec<u32>>> {
        let as_set_indicator = self.advance()?;

        // Determine if asn's are listed as an unordered AS_SET (1) or an ordered AS_SEQUENCE (2)
        // Only add asn's to as_path vector if they are listed in an ordered AS_SEQUENCE
        match as_set_indicator {
            1 => {
                println!("AS_SET's are not factored into the bottleneck calculation.");
                let num_asn = self.advance()?;

                for _ in 0..num_asn {
                    self.parse_u32()?;
                }

                Ok(None)
            }
            2 => {
                let mut as_path = Vec::new();

                let num_asn = self.advance()?;

                for _ in 0..num_asn {
                    as_path.push(self.parse_u32()?);
                }

                Ok(Some(as_path))
            }
            _ => Err(Error::UnknownAsValue {
                unknown_as_value: as_set_indicator,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_as_path_parser() -> Result<()> {
        let buffer = &[0, 1, 2, 3, 4];

        let want = AsPathParser {
            buffer: buffer,
            next: 0,
        };

        let have = AsPathParser::new(buffer);

        assert_eq!(want, have);
        Ok(())
    }

    #[test]
    fn returns_true_if_buffer_exhausted() -> Result<()> {
        let want = true;

        let have = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 5,
        }
        .done();

        assert_eq!(want, have);
        Ok(())
    }

    #[test]
    fn returns_false_if_buffer_has_contents() -> Result<()> {
        let have = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 1,
        }
        .done();

        let want = false;

        assert_eq!(want, have);
        Ok(())
    }

    #[test]
    fn advances_buffer_one_forward() -> Result<()> {
        let want = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 1,
        };

        let mut have = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 0,
        };
        let next_byte = have.advance()?;

        assert_eq!(have, want);
        assert_eq!(0, next_byte);
        Ok(())
    }

    #[test]
    fn advances_buffer_four_forward_returns_u32() -> Result<()> {
        let want = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 4,
        };

        let mut have = AsPathParser {
            buffer: &[0, 1, 2, 3, 4],
            next: 0,
        };
        let new_u32 = have.parse_u32()?;

        assert_eq!(have, want);
        assert_eq!(66051, new_u32);
        Ok(())
    }

    #[test]
    fn parses_attributes_0() -> Result<()> {
        let bgp_attributes = &[
            64, 1, 1, 0, 80, 2, 0, 10, 2, 2, 0, 0, 251, 15, 0, 0, 243, 32, 64, 3, 4, 195, 66, 225,
            77,
        ];
        let have = AsPathParser::parse(bgp_attributes)?;
        let want = &[64271u32, 62240u32];
        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn parses_attributes_1() -> Result<(), Error> {
        let bgp_attributes = &[
            64, 1, 1, 0, 64, 2, 14, 2, 3, 0, 0, 12, 231, 0, 0, 50, 74, 0, 3, 49, 30, 64, 3, 4, 195,
            66, 224, 110, 192, 8, 28, 12, 231, 3, 232, 12, 231, 3, 238, 12, 231, 3, 252, 12, 231,
            12, 21, 50, 74, 2, 188, 50, 74, 3, 243, 50, 74, 11, 210,
        ];
        let have = AsPathParser::parse(bgp_attributes)?;
        let want = &[3303, 12874, 209182];

        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn parses_attributes_2() -> Result<(), Error> {
        let bgp_attributes = &[
            64, 1, 1, 0, 64, 2, 10, 2, 2, 0, 0, 165, 233, 0, 0, 5, 19, 64, 3, 4, 195, 66, 226, 113,
            128, 4, 4, 0, 0, 0, 0, 192, 8, 24, 184, 43, 5, 222, 184, 43, 7, 208, 184, 43, 8, 64,
            184, 43, 8, 252, 184, 43, 9, 112, 184, 43, 10, 40,
        ];
        let have = AsPathParser::parse(bgp_attributes)?;
        let want = &[42473u32, 1299u32];

        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn parses_attributes_3() -> Result<(), Error> {
        let bgp_attributes = &[
            64, 1, 1, 0, 80, 2, 0, 10, 2, 2, 0, 2, 1, 149, 0, 0, 229, 255, 64, 3, 4, 103, 102, 5,
            1, 192, 16, 8, 2, 2, 0, 2, 1, 149, 0, 200,
        ];
        let have = AsPathParser::parse(bgp_attributes)?;
        let want = &[131477u32, 58879u32];

        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn parse_long_as_path_attr() -> Result<(), Error> {
        let bgp_attributes = &[
            64, 1, 1, 2, 64, 2, 24, 2, 4, 0, 0, 58, 59, 0, 0, 11, 98, 0, 0, 25, 53, 0, 0, 50, 49,
            1, 1, 0, 0, 50, 49, 64, 3, 4, 27, 111, 228, 186, 192, 7, 8, 0, 0, 50, 49, 213, 57, 3,
            1, 192, 8, 32, 11, 98, 1, 164, 11, 98, 5, 125, 11, 98, 9, 102, 11, 98, 13, 72, 25, 53,
            7, 208, 25, 53, 8, 52, 25, 53, 8, 57, 58, 59, 0, 4,
        ];

        // 0, 0, 58, 59, 0, 0, 11, 98, 0, 0, 25, 53, 0, 0, 50, 49,

        let have = AsPathParser::parse(bgp_attributes)?;
        let want = &[14907u32, 2914u32, 6453u32, 12849u32];

        assert_eq!(have, want);
        Ok(())
    }

    #[ignore]
    #[test]
    fn returns_err_if_buffer_empty() -> Result<()> {
        let _have = match AsPathParser::parse(&[]) {
            Ok(_) => panic!("exepceted errr"),
            Err(e) => e,
        };
        let _want = Error::MissingPathAttribute {
            missing_attribute: "all attributes".to_string(),
        };
        // assert_eq!(want, have);
        Ok(())
    }
}
