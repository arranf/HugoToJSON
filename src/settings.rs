use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Hugo to JSON",
    about = "A tool to turn Hugo sites into a JSON representation."
)]
/// Represents the settings that can be provided either progamatically or as command line options.  
pub struct Settings {
    /// The path to [Hugo](https://gohugo.io/)'s [contents](https://gohugo.io/content-management/organization/) directory. Defaults to `./content/`
    #[structopt(parse(from_os_str))]
    pub scan_path: PathBuf,
    /// The path that index will be output to. If not provided, writes to stdout.
    #[structopt(short = "o", parse(from_os_str))]
    pub output: Option<PathBuf>,
    /// Include drafts in the output. By default, drafts are not included.
    #[structopt(long)]
    pub drafts: bool,
}
