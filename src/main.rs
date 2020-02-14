mod address;
mod as_path_parser;
mod common;
mod error;
mod helper;
mod mrt_parse;
mod opt;
mod subcommand;

use crate::common::*;

fn main() -> Result<()> {
    Opt::from_args().run()
}
