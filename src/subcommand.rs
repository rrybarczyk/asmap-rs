use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum Subcommand {
    Download {
        #[structopt(name = "OUT", long = "out", short = "o", default_value = "dump")]
        out: PathBuf,

        #[structopt(name = "NUMBER", long = "number", short = "n", use_delimiter(true))]
        number: Vec<u32>,
    },
    Bottleneck {
        /// Directory to read mrt formatted gz files from
        #[structopt(name = "DUMP", long = "dump", short = "d")]
        dump: Vec<PathBuf>,

        /// Save to file if directory path is provided, otherwise print to stdout.
        #[structopt(name = "OUT", long = "out", short = "o")]
        out: Option<PathBuf>,
    },
}

impl Subcommand {
    pub(crate) fn run(self) -> Result<(), Error> {
        match self {
            Self::Download { out, number } => Self::download(&out, &number),
            Self::Bottleneck { dump, out } => Self::bottleneck(&dump, out.as_deref()),
        }
    }

    /// Downloads the gz file from data.ris.ripe.net and save to `dump` directory.
    fn download(out: &Path, number: &[u32]) -> Result<()> {
        // Create target directory
        fs::create_dir_all(out).map_err(|io_error| Error::IoError {
            io_error,
            path: out.into(),
        })?;

        if number.is_empty() {
            for i in 0..=24 {
                Self::download_file(out, i)?;
            }
        } else {
            for i in number {
                Self::download_file(out, *i)?;
            }
        }

        Ok(())
    }

    fn download_file(out: &Path, number: u32) -> Result<()> {
        let url = format!("http://data.ris.ripe.net/rrc{:02}/latest-bview.gz", number);
        let mut res = reqwest::blocking::get(&url).map_err(|reqwest_error| Error::Reqwest {
            url: url.to_string(),
            reqwest_error,
        })?;

        let dst = out.join(format!("rrc{:02}-latest-bview.gz", number));
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

    /// Reads gz mrt data from urls defined by range, decompresses them, parses mrt output, finds bottleneck.
    fn bottleneck(dump: &[PathBuf], out: Option<&Path>) -> Result<()> {
        let bottleneck = Bottleneck::locate(dump)?;
        bottleneck.write(out)?;

        Ok(())
    }
}
