pub(crate) use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt::{self, Display, Formatter},
    fs::OpenOptions,
    io::prelude::*,
    net::IpAddr,
    path::{Path, PathBuf},
    str::FromStr,
    time::SystemTime,
};

pub(crate) use flate2::read::GzDecoder;
pub(crate) use mrt_rs::{tabledump::TABLE_DUMP_V2, Reader, Record};
pub(crate) use structopt::StructOpt;
pub(crate) use url::Url;

pub(crate) use crate::{
    address::Address, as_path_parser::AsPathParser, data_op, error::Error, helper, mrt_parse,
    opt::Opt, subcommand::Subcommand,
};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) use std::{fs::File, io::BufReader};
