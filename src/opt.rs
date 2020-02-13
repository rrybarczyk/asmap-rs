use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    name = "asmap",
    about = "Parse mrt formatted files and find asn bottleneck"
)]
pub(crate) struct Opt {
    #[structopt(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum Command {
    Download {
        #[structopt(name = "URL", long = "url", short = "u")]
        url: Vec<String>,

        #[structopt(name = "OUT", long = "out", short = "o", default_value = "gz-dumps")]
        out: String,

        #[structopt(name = "GUNZIP", long = "gunzip")]
        gunzip: bool,
    },
    Bottleneck {
        #[structopt(name = "URL", long = "url", short = "u")]
        url: Vec<String>,

        #[structopt(name = "OUT", long = "out", short = "o", default_value = "gz-dumps")]
        out: String,

        #[structopt(name = "GUNZIP", long = "gunzip")]
        gunzip: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_download_basic() -> Result<(), structopt::clap::Error> {
        let url = "http://data.ris.ripe.net/rrc02/latest-bview.gz";
        let have = Opt::from_iter_safe(vec!["asmap", "download", "--url", url])?;

        let want = Opt {
            cmd: Command::Download {
                url: vec!["http://data.ris.ripe.net/rrc02/latest-bview.gz".to_string()],
                out: "gz-dumps".to_string(),
                gunzip: false,
            },
        };

        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn cli_download_with_options() -> Result<(), structopt::clap::Error> {
        let url = "http://data.ris.ripe.net/rrc02/latest-bview.gz";
        let out = "out-dir";
        let have = Opt::from_iter_safe(vec![
            "asmap", "download", "--url", url, "--out", out, "--gunzip",
        ])?;

        let want = Opt {
            cmd: Command::Download {
                url: vec!["http://data.ris.ripe.net/rrc02/latest-bview.gz".to_string()],
                out: "out-dir".to_string(),
                gunzip: true,
            },
        };

        assert_eq!(have, want);
        Ok(())
    }
}
