mod address;
mod bgp_path;
mod common;
mod error;

pub(crate) use crate::common::*;

fn main() -> Result<(), Error> {
    let path = "./data/dump_01_2019-12-28";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut bgp_input_data: Vec<BGPPath>;

    // for line in buffered.lines() {
    //     let l = line.unwrap();
    //     let bgp_line = BGPPath::from_str(&l)?;
    //     bgp_input_data.push(bgp_line);
    // }

    Ok(())
}
