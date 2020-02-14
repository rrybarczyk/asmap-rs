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
        let have = Opt::from_iter_safe(vec!["asmap", "download"])?;

        let want = Opt {
            cmd: Subcommand::Download { out: "dump".into() },
        };

        assert_eq!(have, want);
        Ok(())
    }
}
