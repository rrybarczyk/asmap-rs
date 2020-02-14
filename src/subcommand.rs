use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum Subcommand {
    Download {
        #[structopt(name = "URL", long = "url", short = "u")]
        url: Vec<Url>,

        #[structopt(name = "OUT", long = "out", short = "o", default_value = "gz-dumps")]
        out: PathBuf,

        #[structopt(name = "GUNZIP", long = "gunzip")]
        gunzip: bool,
    },
    Bottleneck {
        #[structopt(name = "URL", long = "url", short = "u")]
        url: Vec<Url>,

        #[structopt(name = "OUT", long = "out", short = "o", default_value = "gz-dumps")]
        out: PathBuf,

        #[structopt(name = "GUNZIP", long = "gunzip")]
        gunzip: bool,
    },
}

impl Subcommand {
    pub(crate) fn run(self) -> Result<(), Error> {
        match self {
            Self::Download { url, out, gunzip } => Self::download(&url, &out, gunzip),
            Self::Bottleneck { url, out, gunzip } => Self::bottleneck(&url, &out, gunzip),
        }
    }

    fn download(_urls: &[Url], _out: &Path, _guzip: bool) -> Result<()> {
        todo!()
    }

    /// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, finds bottleneck
    fn bottleneck(url: &[Url], out: &Path, gunzip: bool) -> Result<()> {
        let mut mrt_hm = mrt_data_from_gz_url(&url, out, gunzip, false)?;

        let as_bottleneck: HashMap<Address, u32> = mrt_parse::find_as_bottleneck(&mut mrt_hm)?;
        data_op::write_bottleneck(as_bottleneck)?;
        Ok(())
    }
}

/// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, and writes
/// to file
fn mrt_data_from_gz_url(
    url: &[Url],
    _out: &Path,
    _gunzip: bool,
    save_raw_data: bool,
) -> Result<HashMap<Address, HashSet<Vec<u32>>>, Error> {
    let mut mrt_hm: HashMap<Address, HashSet<Vec<u32>>> = HashMap::new();

    for u in url {
        mrt_parse::parse_mrt_from_gz_url(&u, &mut mrt_hm)?;
    }

    if save_raw_data {
        data_op::write_mrt_data(&mrt_hm)?;
    }

    Ok(mrt_hm)
}