mod address;
mod common;
pub mod error;
mod mrt_parse;

pub(crate) use crate::common::*;

/// Reads mrt files defined by range, decompresses them, parses mrt output, finds bottleneck
pub fn run_mrt_file() -> Result<(), Error> {
    let range = [1, 2]; // [start, end]

    let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

    for i in range[0]..range[1] + 1 {
        let path = format!("gz-dumps/latest-bview-{}", i);
        mrt_parse::parse_mrt_from_file(&path, &mut mrt_hm)?;
    }

    let as_bottleneck: HashMap<Address, u32> = mrt_parse::find_as_bottleneck(&mut mrt_hm)?;
    mrt_parse::write_bottleneck(as_bottleneck)?;
    Ok(())
}

/// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, finds bottleneck
pub fn bottleneck_from_mrt_gz_url() -> Result<(), Error> {
    let range = [1, 2]; // [start, end]

    let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

    for i in range[0]..range[1] + 1 {
        let url = format!("http://data.ris.ripe.net/rrc0{}/latest-bview.gz", i);
        mrt_parse::parse_mrt_from_gz_url(&url, &mut mrt_hm)?;
    }

    let as_bottleneck: HashMap<Address, u32> = mrt_parse::find_as_bottleneck(&mut mrt_hm)?;
    mrt_parse::write_bottleneck(as_bottleneck)?;
    Ok(())
}
