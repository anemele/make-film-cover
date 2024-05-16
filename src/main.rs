mod cli;
mod consts;
mod fs;
mod img;
mod utils;

use clap::Parser;

use cli::Args;
use fs::{set_attr_readonly, write_desktop_ini};
use glob::glob;
use img::make_icon;
use utils::find_image_file;

fn main() {
    let args = Args::parse();
    // dbg!(&args);

    let paths = args
        .path
        .iter()
        .filter_map(|p| glob(&p).ok())
        .flatten()
        .filter_map(|p| p.ok())
        .filter(|p| p.is_dir());
    // dbg!(&paths);

    for path in paths {
        let Some(img_path) = find_image_file(&path) else {
            eprintln!("{}: not found `cover.*`", path.display());
            continue;
        };
        if let Err(e) = make_icon(&img_path) {
            eprintln!("{}: {}", path.display(), e);
            continue;
        }
        if !write_desktop_ini(&path).is_ok_and(|x| x) {
            eprintln!("{}: failed to write `desktop.ini`", path.display());
            continue;
        }
        if !set_attr_readonly(&path).is_ok_and(|x| x) {
            eprintln!("{}: failed to set attribute `readonly`", path.display());
            continue;
        }
    }
}
