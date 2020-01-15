mod address;
mod bgp_path;
mod common;
mod error;

pub(crate) use crate::common::*;

fn get_common_suffix() -> Result<HashMap<Address, HashSet<u32>>, Error> {
    let path = "./data/dump_01_2019-12-28";
    let input = File::open(path).map_err(|error| Error::IoError { io_error: error })?;
    let buffered = BufReader::new(input);

    let mut common_suffix: HashMap<Address, HashSet<u32>> = HashMap::new();

    for line in buffered.lines() {
        let l = line.unwrap();
        let bgp = BGPPath::from_str(&l)?;

        if common_suffix.contains_key(&bgp.addr) {
            let next_as_path: HashSet<_> = bgp.as_path.iter().cloned().collect();
            let current_as_path: HashSet<_> = common_suffix
                .get(&bgp.addr)
                .unwrap()
                .intersection(&next_as_path)
                .cloned()
                .collect();

            common_suffix.insert(bgp.addr, current_as_path);
        } else {
            let current_as_path: HashSet<_> = bgp.as_path.iter().cloned().collect();
            common_suffix.insert(bgp.addr, current_as_path);
        }
    }
    Ok(common_suffix)
}

fn print_common_suffix(common_suffix: HashMap<Address, HashSet<u32>>) {
    for (prefix, as_path) in common_suffix.iter() {
        println!("prefix: {:?}", prefix);
        println!("as_path: {:?}", as_path);
    }
}

fn main() -> Result<(), Error> {
    let common_suffix = get_common_suffix()?;
    print_common_suffix(common_suffix);
    Ok(())
}
