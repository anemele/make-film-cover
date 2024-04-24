use clap::Parser;

/// make film folder cover for Windows explorer
#[derive(Parser, Debug)]
#[clap(version)]
pub(crate) struct Args {
    /// film folder path, with image file named `cover.*` in it
    #[clap(required=true)]
    pub(crate) path: Vec<String>,
}
