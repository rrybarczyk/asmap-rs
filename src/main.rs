mod address;
mod as_path_parser;
mod bottleneck;
mod common;
mod error;
mod opt;
mod subcommand;

use crate::common::*;
fn main() -> Result<()> {
    pretty_env_logger::init();
    Opt::from_args().run()
}
