pub(crate) use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) struct BGPPath {
    pub addr: Address,
    pub as_path: Vec<u32>,
}

impl FromStr for BGPPath {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.find('|') {
            Some(_) => (),
            None => {
                return Err(Error::NoPipe {
                    bad_quagga: text.to_owned(),
                })
            }
        };

        //
        // TODO: Count number of fields and if num fields is wrong throw error instead of splitting
        // first and then counting vector length

        // Get Address from IP and mask str
        let record_vec: Vec<&str> = text.split('|').collect();
        if record_vec.len() != 2 {
            panic!("TODO: handle err for could not parse line correctly");
        }

        let addr = Address::from_str(record_vec[0])?;

        // Get BGPPath from ASN array
        let as_vec_str: Vec<&str> = record_vec[1].split(' ').collect();
        let mut as_path: Vec<u32> = as_vec_str
            .into_iter()
            .map(|s| {
                s.parse().map_err(|e| Error::ParseInt {
                    bad_num: s.to_string(),
                    error: e,
                })
            })
            .collect::<Result<Vec<u32>, Error>>()?;
        as_path.dedup();

        Ok(BGPPath { addr, as_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // "223.255.245.0/24|31742 174 6453 4755 45820 45954 45954 45954 45954"
    fn bgp_path_from_str() -> Result<(), Error> {
        let ip = "223.255.245.0";
        let mask = 24;
        let asn_list = "31742 174 6453 4755 45820 45954 45954 45954 45954";
        let text = format!("{}/{}|{}", ip, mask, asn_list);
        let have = BGPPath::from_str(&text).unwrap();

        let addr = Address {
            ip: IpAddr::from_str(ip).unwrap(),
            mask: mask,
        };

        let asn_list_dedup: Vec<u32> = vec![31742, 174, 6453, 4755, 45820, 45954];
        let want = BGPPath {
            addr: addr,
            as_path: asn_list_dedup,
        };

        assert_eq!(have, want);

        Ok(())
    }

    #[test]
    fn parse_int() {
        let text = "223.255.245.0/24|31742 174 4294967296 4755 45820 45954 45954 45954 45954";
        let have = BGPPath::from_str(&text).unwrap_err();

        match have {
            Error::ParseInt { .. } => {}
            _ => panic!("Expected BGPPath ParseInt Error type"),
        };
    }
}
