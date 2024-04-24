use std::path::{Path, PathBuf};

use crate::consts::DEFAULT_IMAGE_FILE_NAME;

/// read the dir, find the file whose name starts with `cover`
pub(crate) fn find_image_file(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref();
    if !path.is_dir() {
        return None;
    }
    let Ok(dirs) = path.read_dir() else {
        return None;
    };
    for r in dirs {
        let Ok(entry) = r else {
            continue;
        };
        let it = entry.path();
        if !it.is_file() {
            continue;
        }
        let Some(stem) = it.file_stem() else {
            continue;
        };
        if stem.eq(DEFAULT_IMAGE_FILE_NAME) {
            return Some(it);
        }
    }

    None
}
