mod address;
mod as_path_parser;
mod common;
mod error;
mod find_bottleneck;
mod opt;
mod subcommand;

use crate::common::*;
fn main() -> Result<()> {
    let mut cause_a_warning = 1;
    pretty_env_logger::init();
    Opt::from_args().run()
}
