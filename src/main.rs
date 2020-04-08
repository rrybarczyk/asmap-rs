mod as_path_parser;
mod common;
mod error;
mod find_bottleneck;
mod opt;
mod routing_prefix;
mod subcommand;

use crate::common::*;
fn main() -> Result<()> {
    pretty_env_logger::init();
    Opt::from_args().run()
}
