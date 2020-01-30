pub(crate) use crate::common::*;

pub(crate) fn parse_mrt_from_file(
    path: &str,
    mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
) -> Result<(), Error> {
    let mut addresses: Vec<Address> = Vec::new();

    let mut buffer =
        BufReader::new(File::open(path).map_err(|error| Error::IoError { io_error: error })?);

    let mut reader = Reader { stream: buffer };

    while let Ok(Some((_, record))) = reader.read() {
        match record {
            Record::TABLE_DUMP_V2(tdv2_entry) => match tdv2_entry {
                TABLE_DUMP_V2::PEER_INDEX_TABLE(entry) => {
                    for peer_entry in entry.peer_entries {
                        let addr = Address {
                            ip: peer_entry.peer_ip_address,
                            mask: None,
                        };
                        addresses.push(addr);
                    }
                    break;
                }
                TABLE_DUMP_V2::RIB_IPV4_UNICAST(entry) => println!("tdv2_rib_ipv4_unicast"),
                _ => continue,
            },
            _ => continue,
        }
    }
    println!("{:?}", addresses);

    Ok(())
}

pub(crate) fn parse_mrt_from_gz_url(
    url: &str,
    mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
) -> Result<(), Error> {
    todo!();
    let res = reqwest::blocking::get(url).map_err(|reqwest_error| Error::Reqwest {
        url: url.to_string(),
        reqwest_error,
    })?;

    let decoder = GzDecoder::new(res);

    let mut reader = Reader { stream: decoder };

    Ok(())
}
pub(crate) fn parse_mrt_from_gz_file(
    path: &str,
    mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
) -> Result<(), Error> {
    todo!();
    let mut buffer =
        BufReader::new(File::open(path).map_err(|error| Error::IoError { io_error: error })?);

    let decoder = GzDecoder::new(buffer);

    let mut reader = Reader { stream: decoder };

    Ok(())
}

pub(crate) fn find_as_bottleneck(
    mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
) -> Result<HashMap<Address, u32>, Error> {
    todo!();
}

pub(crate) fn write_bottleneck(mrt_hm: HashMap<Address, u32>) -> Result<(), Error> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_mrt_from_file() -> Result<(), Error> {
        let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();
        let path = "data/latest-bview-2020-01-28-160000";
        assert_eq!(parse_mrt_from_file(path, &mut mrt_hm)?, ());
        Ok(())
    }
    //
    // #[ignore]
    // #[test]
    // fn test_gunzip_and_parse() -> Result<(), Error> {
    //     let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();
    //     let path = "data/dump_latest-bview-compressed.gz";
    //     assert_eq!(gunzip_mrt_and_parse(path, &mut mrt_hm)?, ());
    //     Ok(())
    // }
}
