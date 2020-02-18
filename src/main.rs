mod address;
mod as_path_parser;
mod common;
mod error;
mod mrt_parse;
mod opt;
mod subcommand;

use crate::common::*;
fn main() -> Result<()> {
    pretty_env_logger::init();
    trace!("tracing");
    Opt::from_args().run()
}
