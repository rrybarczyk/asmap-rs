pub(crate) use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::{self, prelude::*, BufReader, BufWriter},
    net::IpAddr,
    path::{Path, PathBuf},
    str::FromStr,
    time::SystemTime,
};

pub(crate) use flate2::read::GzDecoder;
pub(crate) use log::*;
pub(crate) use mrt_rs::{tabledump::TABLE_DUMP_V2, Reader, Record};
pub(crate) use structopt::StructOpt;

pub(crate) use crate::{
    as_path_parser::AsPathParser, error::Error, find_bottleneck::FindBottleneck, opt::Opt,
    routing_prefix::RoutingPrefix, subcommand::Subcommand,
};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
