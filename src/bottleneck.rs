use crate::common::*;

/// Contains the mapping of each prefix to its bottleneck asn.
#[derive(Debug, PartialEq)]
pub(crate) struct FindBottleneck {
    prefix_asn: HashMap<Address, u32>,
}

impl FindBottleneck {
    /// Creates a new `FindBottleneck`, reads and parses mrt files, locates prefix and asn bottleneck
    pub(crate) fn locate(dump: &[PathBuf]) -> Result<Self> {
        let mut mrt_hm = HashMap::new();

        for path in dump {
            let buffer = BufReader::new(File::open(path).map_err(|io_error| Error::IoError {
                io_error,
                path: path.into(),
            })?);

            let mut decoder = GzDecoder::new(buffer);
            Self::parse_mrt(&mut decoder, &mut mrt_hm)?;
        }

        let mut bottleneck = FindBottleneck {
            prefix_asn: HashMap::new(),
        };
        bottleneck.find_as_bottleneck(&mut mrt_hm)?;

        Ok(bottleneck)
    }

    /// Creates a mapping between a prefix and all of its asn paths, gets the common asns from
    /// those paths, and considers the last asn (the asn farthest from the originating hop) from
    /// the common asns to be the bottleneck.
    fn find_as_bottleneck(
        &mut self,
        mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
    ) -> Result<(), Error> {
        // In the vector value, the first element is the final AS (so the actual AS of the IP,
        // not some AS on the path). The last element is the critical AS on the path that
        // determines the bottleneck.
        let mut prefix_to_common_suffix: HashMap<Address, Vec<u32>> = HashMap::new();

        Self::find_common_suffix(mrt_hm, &mut prefix_to_common_suffix)?;

        for (addr, mut as_path) in prefix_to_common_suffix {
            let asn = match as_path.pop() {
                Some(a) => a,
                None => panic!("ahhh! no asn :( TODO: handle this error"),
            };
            self.prefix_asn.insert(addr, asn);
        }

        Ok(())
    }

    /// Logic that finds the mapping of each prefix and the asns common to all of the prefix's asn
    /// paths.
    fn find_common_suffix(
        mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
        prefix_to_common_suffix: &mut HashMap<Address, Vec<u32>>,
    ) -> Result<(), Error> {
        for (prefix, as_paths) in mrt_hm.iter() {
            let mut as_paths_sorted: Vec<&Vec<u32>> = as_paths.iter().collect();

            as_paths_sorted.sort_by(|a, b| a.len().cmp(&b.len())); // descending

            let mut rev_common_suffix: Vec<u32> = as_paths_sorted[0].to_vec();
            rev_common_suffix.reverse();

            for as_path in as_paths_sorted.iter().skip(1) {
                // first one is already in rev_common_suffix
                let mut rev_as_path: Vec<u32> = as_path.to_vec();
                rev_as_path.reverse();

                // Every IP should always belong to only one AS
                if rev_common_suffix.first() != rev_as_path.first() {
                    error!(
                        "bn: Every IP belongs to one AS: rev_common_suffix {:?}, rev_as_path: {:?}",
                        &rev_common_suffix, &rev_as_path
                    );
                    debug!("bn: prefix: {:?}", &prefix);
                    debug!("bn: as_paths_sorted: {:?}", &as_paths_sorted);
                    debug!("bn: rev_common_suffix: {:?}", &rev_common_suffix);
                    debug!(
                        "bn: as_path in as_paths_sorted.iter().skip(1): {:?}",
                        &as_path
                    );
                    debug!("bn: rev_as_path: {:?}", &rev_as_path);
                    continue;
                }

                // first element is already checked
                for i in 1..rev_common_suffix.len() {
                    if rev_as_path[i] != rev_common_suffix[i] {
                        rev_common_suffix.truncate(i);
                        break;
                    }
                }
            }
            // rev_common_suffix.reverse();
            prefix_to_common_suffix
                .entry(*prefix)
                .or_insert(rev_common_suffix);
        }

        Ok(())
    }

    /// Parses the mrt formatted data, extracting the pertinent `PEER_INDEX_TABLE` values
    /// containing the prefix and associated as paths.
    fn parse_mrt(
        reader: &mut dyn Read,
        mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
    ) -> Result<()> {
        let mut reader = Reader { stream: reader };

        while let Some((_, record)) = reader.read().map_err(|io_error| Error::IoError {
            io_error,
            path: PathBuf::from("path to mrt data"),
        })? {
            match record {
                Record::TABLE_DUMP_V2(tdv2_entry) => match tdv2_entry {
                    TABLE_DUMP_V2::RIB_IPV4_UNICAST(entry) => {
                        let mask = entry.prefix_length;
                        let mut ip = entry.prefix;
                        while ip.len() < 4 {
                            ip.push(0);
                        }
                        let text = format!("{}.{}.{}.{}/{}", ip[0], ip[1], ip[2], ip[3], mask);
                        Self::match_rib_entry(entry.entries, &text, mrt_hm)?;
                    }
                    TABLE_DUMP_V2::RIB_IPV6_UNICAST(entry) => {
                        let mask = entry.prefix_length;
                        let mut ip = entry.prefix;
                        while ip.len() < 8 {
                            ip.push(0);
                        }
                        let mut ipv6 = Vec::new();
                        ip.reverse();
                        while ipv6.len() < 4 {
                            let a = match ip.pop() {
                                Some(x) => x,
                                None => 0,
                            };
                            let b = match ip.pop() {
                                Some(x) => x,
                                None => 0,
                            };
                            if a == 0 && b == 0 {
                                ipv6.push(':'.to_string());
                                break;
                            } else {
                                let n = format!("{:x}", u16::from_be_bytes([a, b]));
                                ipv6.push(n);
                            }
                        }
                        let ipv6_str = ipv6.join(":");
                        let text = format!("{}/{}", ipv6_str, mask);
                        Self::match_rib_entry(entry.entries, &text, mrt_hm)?;
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
        info!("mrt_hm: {:?}", &mrt_hm);
        Ok(())
    }

    /// Parse each RIB Entry.
    fn match_rib_entry(
        entries: Vec<mrt_rs::records::tabledump::RIBEntry>,
        text: &str,
        mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
    ) -> Result<()> {
        let addr = Address::from_str(&text)?;

        for rib_entry in entries {
            match AsPathParser::parse(&rib_entry.attributes) {
                Ok(mut as_path) => {
                    as_path.dedup();
                    mrt_hm
                        .entry(addr)
                        .or_insert_with(HashSet::new)
                        .insert(as_path);
                }
                Err(e) => error!("ERROR: {:?}. TODO: handle error.", e),
            };
        }
        Ok(())
    }
    /// Writes the asn bottleneck result to a stdout or a time stamped file
    pub(crate) fn write(self, out: Option<&Path>) -> Result<()> {
        if let Some(path) = out {
            let epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let now = epoch.as_secs();
            let dst = path.join(format!("bottleneck.{}.txt", now));
            let mut file = File::create(&dst).map_err(|io_error| Error::IoError {
                io_error,
                path: dst.to_path_buf(),
            })?;

            self.write_bottleneck(&mut file)?;
        } else {
            self.write_bottleneck(&mut io::stdout())?;
        };

        Ok(())
    }

    /// Helper write function
    fn write_bottleneck(self, out: &mut dyn Write) -> Result<(), Error> {
        for (key, value) in self.prefix_asn {
            let text = format!("{}/{}|{:?}", key.ip, key.mask, value);
            writeln!(out, "{:?}", &text).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mrt_hm() -> Result<HashMap<Address, HashSet<Vec<u32>>>, Error> {
        let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();
        let ip_str = "1.0.139.0";
        let addr = Address {
            ip: IpAddr::from_str(ip_str).map_err(|addr_parse| Error::AddrParse {
                addr_parse,
                bad_addr: ip_str.to_string(),
            })?,
            mask: 24,
        };

        let mut asn_paths = HashSet::new();
        asn_paths.insert(vec![2497, 38040, 23969]);
        asn_paths.insert(vec![25152, 6939, 4766, 38040, 23969]);
        asn_paths.insert(vec![4777, 6939, 4766, 38040, 23969]);
        mrt_hm.insert(addr, asn_paths);

        let ip_str = "1.0.204.0";
        let addr = Address {
            ip: IpAddr::from_str(ip_str).map_err(|addr_parse| Error::AddrParse {
                addr_parse,
                bad_addr: ip_str.to_string(),
            })?,
            mask: 22,
        };
        let mut asn_paths = HashSet::new();
        asn_paths.insert(vec![2497, 38040, 23969]);
        asn_paths.insert(vec![4777, 6939, 4766, 38040, 23969]);
        asn_paths.insert(vec![25152, 2914, 38040, 23969]);
        mrt_hm.insert(addr, asn_paths);

        let ip_str = "1.0.6.0";
        let addr = Address {
            ip: IpAddr::from_str(ip_str).map_err(|addr_parse| Error::AddrParse {
                addr_parse,
                bad_addr: ip_str.to_string(),
            })?,
            mask: 24,
        };
        let mut asn_paths = HashSet::new();
        asn_paths.insert(vec![2497, 4826, 38803, 56203]);
        asn_paths.insert(vec![25152, 6939, 4826, 38803, 56203]);
        asn_paths.insert(vec![4777, 6939, 4826, 38803, 56203]);
        mrt_hm.insert(addr, asn_paths);

        Ok(mrt_hm)
    }

    #[test]
    fn finds_common_suffix_from_mrt_hashmap() -> Result<(), Error> {
        let mut want: HashMap<Address, Vec<u32>> = HashMap::new();
        want.insert(Address::from_str("1.0.139.0/24")?, vec![23969, 38040]);
        want.insert(Address::from_str("1.0.204.0/22")?, vec![23969, 38040]);
        want.insert(Address::from_str("1.0.6.0/24")?, vec![56203, 38803, 4826]);

        let mut mrt_hm = setup_mrt_hm()?;
        let mut have: HashMap<Address, Vec<u32>> = HashMap::new();

        assert_eq!(
            FindBottleneck::find_common_suffix(&mut mrt_hm, &mut have)?,
            ()
        );
        assert_eq!(have, want);

        Ok(())
    }

    #[test]
    fn finds_as_bottleneck_from_mrt_hashmap() -> Result<(), Error> {
        let mut want = FindBottleneck {
            prefix_asn: HashMap::new(),
        };
        want.prefix_asn
            .insert(Address::from_str("1.0.139.0/24")?, 38040);
        want.prefix_asn
            .insert(Address::from_str("1.0.204.0/22")?, 38040);
        want.prefix_asn
            .insert(Address::from_str("1.0.6.0/24")?, 4826);

        let mut have = FindBottleneck {
            prefix_asn: HashMap::new(),
        };
        let mut mrt_hm = setup_mrt_hm()?;
        have.find_as_bottleneck(&mut mrt_hm)?;

        assert_eq!(have, want);

        Ok(())
    }
}
