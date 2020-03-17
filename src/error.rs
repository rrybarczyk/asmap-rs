use crate::common::*;

#[derive(Debug)]
pub enum Error {
    IoError {
        io_error: std::io::Error,
        path: PathBuf,
    },
    AddrParse {
        addr_parse: std::net::AddrParseError,
        bad_addr: String,
    },
    NoSlash {
        bad_addr: String,
    },
    Reqwest {
        url: String,
        reqwest_error: reqwest::Error,
    },
    MissingPathAttribute {
        missing_attribute: String,
    },
    UnknownAsValue {
        unknown_as_value: u8,
    },
    UnexpectedEndOfBuffer,
    MultipleAsPaths,
    NoAsPathInAttributePath,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            AddrParse {
                addr_parse,
                bad_addr,
            } => write!(f, "Invalid address, {}: {}", addr_parse, bad_addr),
            NoSlash { bad_addr } => write!(
                f,
                "Invalid IP and mask: {}. Missing `/`, expected format `IP/mask`",
                bad_addr
            ),
            IoError { io_error, path } => {
                write!(f, "I/O error at `{}`: {}", path.display(), io_error)
            }
            Reqwest { url, reqwest_error } => {
                write!(f, "Failed request for {}. {}", url, reqwest_error)
            }
            MissingPathAttribute { missing_attribute } => {
                write!(f, "Invalid mrt entry. Missing {}.", missing_attribute)
            }
            UnknownAsValue { unknown_as_value } => write!(
                f,
                "Did not recognize as path value `{}`, expected AS_SET (1) or AS_SEQUENCE (2).",
                unknown_as_value
            ),
            UnexpectedEndOfBuffer => write!(f, "Expected another byte but buffer is exhausted."),
            NoAsPathInAttributePath => {
                write!(f, "Expected an AS_PATH attribute in BGP Attribute Path.")
            }
            MultipleAsPaths => write!(
                f,
                "Expected one AS_PATH attribute in BGP Attribute Path, found multiple."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_parse_display() {
        let ip = "bad_address";
        let mask = 23;
        let text = format!("{}/{}", ip, mask);
        let err = Address::from_str(&text).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid address, invalid IP address syntax: bad_address"
        );
    }

    #[test]
    fn no_slash_display() {
        let err = Error::NoSlash {
            bad_addr: String::from("INVALID_IP_AND_MASK"),
        };

        assert_eq!(
            err.to_string(),
            "Invalid IP and mask: INVALID_IP_AND_MASK. Missing `/`, expected format `IP/mask`",
        );
    }
}
