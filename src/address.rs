pub(crate) use crate::common::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Address {
    pub(crate) ip: IpAddr,
    pub(crate) mask: u8,
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.find('/') {
            Some(_) => (),
            None => {
                return Err(Error::NoSlash {
                    bad_addr: text.to_owned(),
                })
            }
        };

        let ip_mask_vec: Vec<&str> = text.split('/').collect();
        dbg!(&ip_mask_vec);

        let ip_str = ip_mask_vec[0];
        let ip = IpAddr::from_str(ip_str).map_err(|addr_parse| Error::AddrParse {
            addr_parse,
            bad_addr: ip_str.to_string(),
        })?;
        let mask = ip_mask_vec[1].parse::<u8>().unwrap();
        Ok(Address { ip, mask })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_from_str() -> Result<(), Error> {
        let ip = "127.0.0.1";
        let mask = 23;
        let ip_and_mask = format!("{}/{}", ip, mask);
        let want = Address {
            ip: IpAddr::from_str(ip).unwrap(),
            mask: mask,
        };

        let have = Address::from_str(&ip_and_mask).unwrap();

        assert_eq!(have, want);

        Ok(())
    }
}
