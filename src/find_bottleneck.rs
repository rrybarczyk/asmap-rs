use crate::common::*;

/// Contains the mapping of each prefix to its bottleneck asn.
#[derive(Debug, PartialEq)]
pub(crate) struct FindBottleneck {
    prefix_asn: HashMap<Address, u32>,
}

impl FindBottleneck {
    /// Creates a new `FindBottleneck`, reads and parses mrt files, locates prefix and asn bottleneck
    pub(crate) fn locate(dir: &PathBuf) -> Result<()> {
        if !dir.is_dir() {
            Error::NotDirError{path: dir.to_path_buf()};
        }

        let mut file_decoders = Vec::new();

        // Walk the directory and read its contents
        for entry in fs::read_dir(dir).map_err(|io_error| Error::IoError {
            io_error,
            path: "path".into(),
        })? {
            let entry = entry.map_err(|io_error| Error::IoError {
                io_error,
                path: "path".into(),
            })?;
            let path = entry.path();
            println!("Acquiring a reader for file`{}`", &path.display());
            let buffer =
                BufReader::new(File::open(&path).map_err(|io_error| Error::IoError {
                    io_error,
                    path: path.into(),
                })?);
            
            let decoder = GzDecoder::new(buffer);
            file_decoders.push(decoder);
        }

        let step: u8 = 16;
        assert!(step.is_power_of_two());
        // Assuming the files are sorted by first octet
        // TODO check this
        for current_start_high_octet in (0..u8::max_value()).step_by(step as usize) {
            let current_end_high_octet: u8 = current_start_high_octet + step;

            let mut mrt_hm = HashMap::new();

            for i in 0..file_decoders.len() {
                println!("Processing from {}.*.*.* to {}.*.*.* (both ipv4 and ipv6) in file {}/{}",
                    current_start_high_octet, current_end_high_octet, i, file_decoders.len());
                Self::parse_mrt(&mut file_decoders[i], &mut mrt_hm, current_end_high_octet)?;
            }

            let mut bottleneck = FindBottleneck {
                prefix_asn: HashMap::new(),
            };
            bottleneck.find_as_bottleneck(&mut mrt_hm)?;
            bottleneck.write_intermediate(current_end_high_octet)?;
        }
        Ok(())
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
                None => panic!("ERROR: No ASN"), // TODO: Handle error
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
        'outer: for (prefix, as_paths) in mrt_hm.iter() {
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
                    warn!(
                            "Every IP should belong to one AS. Prefix: `{:?}` has anomalous AS paths: `{:?}`.",
                            &prefix, &as_paths
                        );
                    continue 'outer;
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
        current_end_high_octet: u8,
    ) -> Result<()> {
        let mut reader = Reader { stream: reader };

        loop {
            match reader.read() {
                Ok(header_record) => match header_record {
                    Some((_, record)) => match record {
                        Record::TABLE_DUMP_V2(tdv2_entry) => match tdv2_entry {
                            TABLE_DUMP_V2::RIB_IPV4_UNICAST(entry) => {
                                let ip = Self::format_ip(&entry.prefix, true)?;
                                if let IpAddr::V4(ipv4) = ip {
                                    if ipv4.octets()[0] > current_end_high_octet {
                                        break
                                    }                                    
                                }
                                let mask = entry.prefix_length;
                                Self::match_rib_entry(entry.entries, ip, mask, mrt_hm)?;
                            }
                            TABLE_DUMP_V2::RIB_IPV6_UNICAST(entry) => {
                                let ip = Self::format_ip(&entry.prefix, false)?;
                                if let IpAddr::V6(ipv6) = ip {
                                    if ipv6.octets()[0] >= current_end_high_octet {
                                        break
                                    }
                                }                                    
                                let mask = entry.prefix_length;
                                Self::match_rib_entry(entry.entries, ip, mask, mrt_hm)?;
                            }
                            _ => continue,
                            
                        },
                        _ => continue,
                    },
                    None => break,
                },
                Err(e) => match e.kind() {
                    std::io::ErrorKind::InvalidInput => {
                        println!("Invalid gzip header. Skipping file.")
                    }
                    other_error => println!(
                        "Problem with gzip mrt file. `{:?}`. Skipping file.",
                        other_error
                    ),
                },
            }
        }
        Ok(())
    }

    /// Format IPV4 and IPV6 from slice.
    fn format_ip(ip: &[u8], is_ipv4: bool) -> Result<IpAddr> {
        let pad = &[0; 17];
        let ip = [ip, pad].concat();
        if is_ipv4 {
            Ok(IpAddr::V4(std::net::Ipv4Addr::new(
                ip[0], ip[1], ip[2], ip[3],
            )))
        } else {
            Ok(IpAddr::V6(std::net::Ipv6Addr::new(
                u16::from_be_bytes([ip[0], ip[1]]),
                u16::from_be_bytes([ip[2], ip[3]]),
                u16::from_be_bytes([ip[4], ip[5]]),
                u16::from_be_bytes([ip[7], ip[8]]),
                u16::from_be_bytes([ip[9], ip[10]]),
                u16::from_be_bytes([ip[11], ip[12]]),
                u16::from_be_bytes([ip[13], ip[14]]),
                u16::from_be_bytes([ip[15], ip[16]]),
            )))
        }
    }

    /// Parse each RIB Entry.
    fn match_rib_entry(
        entries: Vec<mrt_rs::records::tabledump::RIBEntry>,
        ip: IpAddr,
        mask: u8,
        mrt_hm: &mut HashMap<Address, HashSet<Vec<u32>>>,
    ) -> Result<()> {
        let addr = Address { ip, mask };

        for rib_entry in entries {
            match AsPathParser::parse(&rib_entry.attributes) {
                Ok(mut as_path) => {
                    as_path.dedup();
                    mrt_hm
                        .entry(addr)
                        .or_insert_with(HashSet::new)
                        .insert(as_path);
                }
                Err(e) => info!("ERROR: {:?}. ", e), // TODO: Handle error
            };
        }
        Ok(())
    }

    /// Writes the asn bottleneck result to a stdout or a time stamped file
    fn write_intermediate(self, end_high_octet: u8) -> Result<()> {
        let file_name = format!("tmp/bottleneck.up_to_{}.txt", end_high_octet);
        let dst = PathBuf::from(file_name);
        let mut file = File::create(&dst).map_err(|io_error| Error::IoError {
            io_error,
            path: dst.to_path_buf(),
        })?;
        self.write_bottleneck(&mut file)?;
        Ok(())
    }

    /// Writes the asn bottleneck result to a stdout or a time stamped file
    pub(crate) fn write_result(out: Option<&Path>) -> Result<()> {
        let epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let now = epoch.as_secs();
        let file_name = format!("bottleneck.{}.txt", now);
        let dst;
        if let Some(path) = out {
            dst = path.join(file_name);
        } else {
            dst = PathBuf::from(file_name);
        };
        let mut file = File::create(&dst).map_err(|io_error| Error::IoError {
            io_error,
            path: dst.to_path_buf(),
        })?;

        // TODO Here combine files from "tmp" directory we wrote with write_bottleneck
        // and merge them together them to the new file created above.
        Ok(())
    }

    /// Helper write function
    fn write_bottleneck(self, out: &mut dyn Write) -> Result<(), Error> {
        for (key, value) in self.prefix_asn {
            let text = format!("{}/{}|{:?}", key.ip, key.mask, value);
            writeln!(out, "{}", &text).unwrap();
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

    #[test]
    fn ipaddr_from_ipv6_short() -> Result<(), Error> {
        let have = FindBottleneck::format_ip(&[32, 1, 3, 24], false)?;
        assert_eq!("2001:318::".parse(), Ok(have));

        Ok(())
    }

    #[test]
    fn ipaddr_from_ipv6_long() -> Result<(), Error> {
        let have = FindBottleneck::format_ip(&[32, 1, 2, 248, 16, 8], false)?;
        assert_eq!("2001:2f8:1008::".parse(), Ok(have));

        Ok(())
    }
}
