mod consts;
mod fs;
mod img;

use std::path::Path;

use clap::Parser;

fn run(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let img_path = fs::find_image_file(path)
        .ok_or_else(|| anyhow::anyhow!("{}: not found `cover.*`", path.display()))?;
    img::make_icon(img_path)?;
    fs::write_desktop_ini(path)
        .map(|_| anyhow::anyhow!("{}: failed to write `desktop.ini`", path.display()))?;
    fs::set_attr_readonly(path)
        .map(|_| anyhow::anyhow!("{}: failed to set attribute `readonly`", path.display()))?;
    Ok(())
}

/// make film folder cover for Windows explorer
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// film folder path, with image file named `cover.*` in it
    #[clap(required = true)]
    pub path: Vec<String>,
}

fn main() {
    let args = Args::parse();
    // dbg!(&args);

    let paths = args
        .path
        .iter()
        .filter_map(|p| glob::glob(p).ok())
        .flatten()
        .filter_map(|p| p.ok())
        .filter(|p| p.is_dir());
    // dbg!(&paths);

    for path in paths {
        if let Err(e) = run(&path) {
            eprintln!("{}", e);
        }
    }
}
