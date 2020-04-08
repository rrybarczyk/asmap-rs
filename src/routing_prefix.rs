use crate::common::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct RoutingPrefix {
    pub(crate) ip: IpAddr,
    pub(crate) mask: u8,
}

impl FromStr for RoutingPrefix {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.find('/') {
            Some(_) => (),
            None => {
                return Err(Error::NoSlash {
                    bad_prefix: text.to_owned(),
                })
            }
        };

        let ip_mask_vec: Vec<&str> = text.split('/').collect();

        let ip_str = ip_mask_vec[0];
        let ip = IpAddr::from_str(ip_str).map_err(|addr_parse| Error::AddrParse {
            addr_parse,
            bad_addr: ip_str.to_string(),
        })?;
        let mask = ip_mask_vec[1].parse::<u8>().unwrap();
        Ok(RoutingPrefix { ip, mask })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_prefix_from_str_ipv4() -> Result<(), Error> {
        let ip = "127.0.0.1";
        let mask = 23;
        let ip_and_mask = format!("{}/{}", ip, mask);
        let want = RoutingPrefix {
            ip: IpAddr::from_str(ip).unwrap(),
            mask,
        };

        let have = RoutingPrefix::from_str(&ip_and_mask).unwrap();

        assert_eq!(have, want);

        Ok(())
    }

    #[test]
    fn routing_prefix_from_str_ipv6() -> Result<(), Error> {
        let ip = "2001::";
        let mask = 32;
        let ip_and_mask = format!("{}/{}", ip, mask);
        let want = RoutingPrefix {
            ip: IpAddr::from_str(ip).unwrap(),
            mask,
        };

        let have = RoutingPrefix::from_str(&ip_and_mask).unwrap();

        assert_eq!(have, want);

        Ok(())
    }
}
