mod address;
mod bgp_path;
mod common;
mod error;

pub(crate) use crate::common::*;

fn main() -> Result<(), Error> {
    let path = "./data/dump_01_2019-12-28";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut prefix_to_as_paths: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

    for line in buffered.lines() {
        let l = line.unwrap();
        let bgp_line = BGPPath::from_str(&l)?;
        prefix_to_as_paths.entry(bgp_line.addr)
            .or_insert_with(HashSet::new)
            .insert(bgp_line.as_path);
    }

    let mut prefix_to_common_suffix: HashMap<&Address, Vec<u32>> = HashMap::new();
    for (prefix, as_paths) in prefix_to_as_paths.iter() {
        let mut as_paths_sorted: Vec<&Vec<u32>> = as_paths
            .iter()
            .collect();
        as_paths_sorted.sort_by(|a, b| a.len().cmp(&b.len())); // descending
        let mut rev_common_suffix: Vec<u32> = as_paths_sorted[0].to_vec();
        rev_common_suffix.reverse();
        for as_path in as_paths_sorted
            .iter()
            .skip(1) { // first one is already in rev_common_suffix
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
        prefix_to_common_suffix.entry(prefix).or_insert(rev_common_suffix);
    }
    Ok(())
}
