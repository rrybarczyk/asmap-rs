mod address;
mod bgp_path;
mod common;
mod error;

pub(crate) use crate::common::*;

fn main() -> Result<(), Error> {
    // Import quagga file and store
    let path = "./data/dump_01_2019-12-28";
    let input = File::open(path).map_err(|error| Error::IoError { io_error: error })?;
    let mut buffered = BufReader::new(input);
    let bgp_path_vec = BGPPath::load(&mut buffered)?;

    let prefix_to_as_paths = get_prefix_as_paths(bgp_path_vec)?;

    let mut prefix_to_common_suffix: HashMap<&Address, Vec<u32>> = HashMap::new();
    for (prefix, as_paths) in prefix_to_as_paths.iter() {
        let mut as_paths_sorted: Vec<&Vec<u32>> = as_paths.iter().collect();
        as_paths_sorted.sort_by(|a, b| a.len().cmp(&b.len())); // descending
        let mut rev_common_suffix: Vec<u32> = as_paths_sorted[0].to_vec();
        rev_common_suffix.reverse();
        for as_path in as_paths_sorted.iter().skip(1) {
            // first one is already in rev_common_suffix
            let mut rev_as_path: Vec<u32> = as_path.to_vec();
            rev_as_path.reverse();

            // every IP should always belong to only one AS
            assert!(rev_common_suffix.first() == rev_as_path.first());

            // first element is already checked
            for i in 1..rev_common_suffix.len() {
                if rev_as_path[i] != rev_common_suffix[i] {
                    rev_common_suffix.truncate(i);
                    break;
                }
            }
        }
        rev_common_suffix.reverse();
        prefix_to_common_suffix
            .entry(prefix)
            .or_insert(rev_common_suffix);
    }
    Ok(())
}

fn get_prefix_as_paths(
    bgp_path_vec: Vec<BGPPath>,
) -> Result<HashMap<Address, HashSet<Vec<u32>>>, Error> {
    let mut prefix_to_as_paths: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();
    for bgp_line in bgp_path_vec {
        prefix_to_as_paths
            .entry(bgp_line.addr)
            .or_insert_with(HashSet::new)
            .insert(bgp_line.as_path);
    }

    Ok(prefix_to_as_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_prefix_to_as_paths_from_bgppath_vec() -> Result<(), Error> {
        // Build expected prefix_as_path return value
        let mut want: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

        let ip = IpAddr::from_str("223.255.245.0").unwrap();
        let mask = 24;
        let addr_entry = Address { ip, mask };

        let mut asn_paths_entry = HashSet::new();
        asn_paths_entry.insert(vec![31742, 174, 6453, 4755, 45820, 45954]);
        asn_paths_entry.insert(vec![35266, 2914, 6453, 4755, 45820, 45954]);

        want.insert(addr_entry, asn_paths_entry);

        let ip = IpAddr::from_str("223.255.246.0").unwrap();
        let mask = 24;
        let addr_entry = Address { ip, mask };

        let mut asn_paths_entry = HashSet::new();
        asn_paths_entry.insert(vec![8607, 3356, 6453, 4755, 45820, 45954]);
        asn_paths_entry.insert(vec![2914, 6453, 4755, 45820, 45954]);
        asn_paths_entry.insert(vec![286, 6453, 4755, 45820, 45954]);

        want.insert(addr_entry, asn_paths_entry);

        // Build actual prefix_as_path return value from a vec of BGPPath's
        let mut bgp_path_vec: Vec<BGPPath> = Vec::new();

        bgp_path_vec.push(BGPPath::from_str(
            "223.255.245.0/24|31742 174 6453 4755 45820 45954",
        )?);

        bgp_path_vec.push(BGPPath::from_str(
            "223.255.245.0/24|35266 2914 6453 4755 45820 45954",
        )?);

        bgp_path_vec.push(BGPPath::from_str(
            "223.255.246.0/24|8607 3356 6453 4755 45820 45954",
        )?);

        bgp_path_vec.push(BGPPath::from_str(
            "223.255.246.0/24|2914 6453 4755 45820 45954",
        )?);

        bgp_path_vec.push(BGPPath::from_str(
            "223.255.246.0/24|286 6453 4755 45820 45954",
        )?);

        let have = get_prefix_as_paths(bgp_path_vec)?;

        assert_eq!(want, have);

        Ok(())
    }
}
