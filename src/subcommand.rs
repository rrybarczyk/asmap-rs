use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum Subcommand {
    /// Downloads and saves the MRT formatted gz files
    Download {
        /// Range of specific RIS files to download [default: [00, 24]]
        #[structopt(
            name = "RIPE_COLLECTOR_NUMBER",
            long = "ripe_collector_number",
            short = "n",
            use_delimiter(true)
        )]
        ripe_collector_number: Vec<u32>,

        /// Directory to write MRT formatted gz files
        #[structopt(name = "OUT", long = "out", short = "o", default_value = "dump")]
        out: PathBuf,
    },
    /// Reads and decompresses the MRT gz files, parses the AS Paths, determines the AS bottleneck, saves result
    FindBottleneck {
        /// Directory path of the MRT formatted gz files to find bottleneck of
        #[structopt(name = "DIRECTORY", long = "dir", short = "d")]
        dir: PathBuf,

        /// Directory to write result [default: print to the timestamped file in current location]
        #[structopt(name = "OUT", long = "out", short = "o")]
        out: Option<PathBuf>,
    },
}

impl Subcommand {
    pub(crate) fn run(self) -> Result<(), Error> {
        match self {
            Self::Download {
                out,
                ripe_collector_number,
            } => Self::download(&out, &ripe_collector_number),
            Self::FindBottleneck { dir, out } => Self::find_bottleneck(&dir, out.as_deref()),
        }
    }

    /// Downloads the gz file from data.ris.ripe.net and save to the corresponding directory.
    fn download(out: &Path, ripe_collector_number: &[u32]) -> Result<()> {
        // Create target directory
        fs::create_dir_all(out).map_err(|io_error| Error::IoError {
            io_error,
            path: out.into(),
        })?;

        if ripe_collector_number.is_empty() {
            for i in 0..=24 {
                Self::download_file(out, i)?;
            }
        } else {
            for i in ripe_collector_number {
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
    fn find_bottleneck(dump: &PathBuf, out: Option<&Path>) -> Result<()> {
        // Until the work is done, store results in this file so that
        // a disrupted process doesn't look finalized.
        let temp_result_file = tempfile();
        let mut temp_result_file = match temp_result_file {
            Ok(temp_result_file) => temp_result_file,
            Err(error) => panic!("ERROR while creating temp file: {:?}", error),
        };

        FindBottleneck::locate(dump, &mut temp_result_file)?;

        // Once the work is done, move result to the non-temporary file.
        temp_result_file.seek(SeekFrom::Start(0)).unwrap();
        FindBottleneck::write(&temp_result_file, out)?;

        Ok(())
    }
}
