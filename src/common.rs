pub(crate) use crate::{address::Address, error::Error, mrt_parse};
pub(crate) use std::collections::{HashMap, HashSet};
pub(crate) use std::fs::{File, OpenOptions};
pub(crate) use std::io::{prelude::*, BufRead, BufReader};
pub(crate) use std::{
    fmt::{self, Display, Formatter},
    net::IpAddr,
    str::FromStr,
};

pub(crate) use flate2::read::GzDecoder;
pub(crate) use mrt_rs::{
    records::tabledump::PEER_INDEX_TABLE,
    tabledump::{PeerEntry, RIB_AFI, TABLE_DUMP_V2},
    Header, Reader, Record,
};
