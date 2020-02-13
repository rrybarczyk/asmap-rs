use crate::common::*;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    name = "asmap",
    about = "Parse mrt formatted files and find asn bottleneck"
)]
pub(crate) struct Opt {
    #[structopt(subcommand)]
    pub(crate) cmd: Subcommand,
}

impl Opt {
    pub(crate) fn run(self) -> Result<(), Error> {
        self.cmd.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_download_basic() -> Result<(), structopt::clap::Error> {
        let url = "http://data.ris.ripe.net/rrc02/latest-bview.gz";
        let have = Opt::from_iter_safe(vec!["asmap", "download", "--url", url])?;

        let want = Opt {
            cmd: Subcommand::Download {
                url: vec!["http://data.ris.ripe.net/rrc02/latest-bview.gz"
                    .parse()
                    .unwrap()],
                out: "gz-dumps".into(),
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
            cmd: Subcommand::Download {
                url: vec!["http://data.ris.ripe.net/rrc02/latest-bview.gz"
                    .parse()
                    .unwrap()],
                out: "out-dir".into(),
                gunzip: true,
            },
        };

        assert_eq!(have, want);
        Ok(())
    }
}
