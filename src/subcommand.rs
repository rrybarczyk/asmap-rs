use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum Subcommand {
    Download {
        #[structopt(name = "OUT", long = "out", short = "o", default_value = "dump")]
        out: PathBuf,
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
            Self::Download { out } => Self::download(&out),
            Self::Bottleneck { url, out, gunzip } => Self::bottleneck(&url, &out, gunzip),
        }
    }

    /// Downloads the gz file from data.ris.ripe.net and save to `dump` directory.
    fn download(out: &Path) -> Result<()> {
        // Create target directory
        fs::create_dir_all(out).map_err(|io_error| Error::IoError {
            io_error,
            path: out.into(),
        })?;
        let url = "http://data.ris.ripe.net/rrc01/latest-bview.gz";
        let mut res = reqwest::blocking::get(url).map_err(|reqwest_error| Error::Reqwest {
            url: url.to_string(),
            reqwest_error,
        })?;

        let dst = out.join("rrc01-latest-bview.gz");
        let file = File::create(&dst).map_err(|io_error| Error::IoError {
            io_error,
            path: dst.to_path_buf(),
        })?;

        let mut buf_write = BufWriter::new(file);
        io::copy(&mut res, &mut buf_write).map_err(|io_error| Error::IoError {
            io_error,
            path: out.to_path_buf(),
        })?;

        Ok(())
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
