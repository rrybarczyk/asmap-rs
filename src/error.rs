pub(crate) use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    AddrParse {
        addr_parse: std::net::AddrParseError,
        bad_addr: String,
    },
    NoSlash {
        bad_addr: String,
    },
    NoPipe {
        bad_quagga: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::AddrParse {
                addr_parse,
                bad_addr,
            } => write!(f, "Invalid address, {}: {}", addr_parse, bad_addr),
            Self::NoSlash { bad_addr } => write!(
                f,
                "Invalid IP and mask: {}. Missing `/`, expected format `IP/mask`",
                bad_addr
            ),
            Self::NoPipe { bad_quagga } => write!(
                f,
                "Invalid quagga data line: {}. Missing `|`, expected format `IP/mask|<asn list>`",
                bad_quagga
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
