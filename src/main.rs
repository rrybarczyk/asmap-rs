mod address;
mod common;
mod data_op;
pub mod error;
mod helper;
mod mrt_parse;
mod opt;

pub(crate) use crate::common::*;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("opt: {:?}", &opt);

    match opt.cmd {
        opt::Command::Download { url, out, gunzip } => {
            println!("Download: url: {:?}, out: {:?}", url, out);
            data_op::download_gz(url, out, gunzip).unwrap()
        }
        opt::Command::Bottleneck { url, out, gunzip } => {
            bottleneck_from_mrt_gz_url(url, out, gunzip)?;
        }
    };
    Ok(())
}

/// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, finds bottleneck
fn bottleneck_from_mrt_gz_url(url: Vec<String>, out: String, gunzip: bool) -> Result<(), Error> {
    let mut mrt_hm = mrt_data_from_gz_url(url, out, gunzip)?;

    let as_bottleneck: HashMap<Address, u32> = mrt_parse::find_as_bottleneck(&mut mrt_hm)?;
    data_op::write_bottleneck(as_bottleneck)?;
    Ok(())
}

/// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, and writes
/// to file
fn mrt_data_from_gz_url(
    url: Vec<String>,
    out: String,
    gunzip: bool,
) -> Result<HashMap<Address, HashSet<Vec<u32>>>, Error> {
    let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

    for (i, u) in url.iter().enumerate() {
        mrt_parse::parse_mrt_from_gz_url(&u, &mut mrt_hm)?;
    }

    Ok(mrt_hm)
}
