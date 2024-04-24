use crate::consts::ICON_FILE_NAME;
use crate::consts::INI_FILE;
use std::{fs, io, path::Path, process::Command};

pub(crate) fn write_desktop_ini(path: impl AsRef<Path>) -> io::Result<bool> {
    if !path.as_ref().is_dir() {
        return Ok(false);
    }
    let ini_path = path.as_ref().join(INI_FILE);
    if ini_path.exists() && fs::remove_file(&ini_path).is_err() {
        return Ok(false);
    }
    fs::write(
        &ini_path,
        format!("[.ShellClassInfo]\nIconResource={},0\n", ICON_FILE_NAME),
    )?;

    // let attr = fs::metadata(ini_path)?.file_attributes();
    // todo!("how to set the attribute of file and directory?");

    let ok = Command::new("attrib.exe")
        .arg("+h")
        .arg("+s")
        .arg(ini_path)
        .status()?
        .success();

    Ok(ok)
}

pub(crate) fn set_attr_readonly(path: impl AsRef<Path>) -> io::Result<bool> {
    let ok = Command::new("attrib.exe")
        .arg("+r")
        .arg(path.as_ref())
        .status()?
        .success();

    Ok(ok)
}
