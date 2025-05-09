use std::{ffi::OsStr, fs, os::windows::ffi::OsStrExt, path::Path, path::PathBuf};

use windows::Win32::Storage::FileSystem::{SetFileAttributesW, FILE_FLAGS_AND_ATTRIBUTES};
use windows::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
};

use crate::consts::DEFAULT_IMAGE_FILE_NAME;
use crate::consts::ICON_FILE_NAME;
use crate::consts::INI_FILE;

/// 设置 Windows 文件属性
fn set_file_attributes(
    path: impl AsRef<Path>,
    attributes: FILE_FLAGS_AND_ATTRIBUTES,
) -> windows::core::Result<()> {
    // 将路径转换为 Windows 风格的 UTF-16 字符串
    let wide_path: Vec<u16> = OsStr::new(path.as_ref())
        .encode_wide()
        .chain(Some(0))
        .collect();
    // 将路径转换为 Windows API 兼容的 UTF-16 格式
    let path_pcwstr = windows::core::PCWSTR::from_raw(wide_path.as_ptr());

    // 调用 Windows API
    let result = unsafe { SetFileAttributesW(path_pcwstr, attributes) };

    // 检查 API 调用结果
    if result.as_bool() {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}

pub fn set_attr_readonly(path: impl AsRef<Path>) -> anyhow::Result<()> {
    set_file_attributes(path, FILE_ATTRIBUTE_READONLY)?;
    Ok(())
}

pub fn set_attr_hidden(path: impl AsRef<Path>) -> anyhow::Result<()> {
    set_file_attributes(path, FILE_ATTRIBUTE_HIDDEN)?;
    Ok(())
}

pub fn write_desktop_ini(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let ini_path = path.as_ref().join(INI_FILE);
    if ini_path.exists() {
        fs::remove_file(&ini_path)?;
    }
    fs::write(
        &ini_path,
        format!("[.ShellClassInfo]\nIconResource={},0\n", ICON_FILE_NAME),
    )?;

    let attr = FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM;
    set_file_attributes(ini_path, attr)?;

    Ok(())
}

/// 获取文件名为 cover 的文件（不包含后缀名）
pub fn find_image_file(path: impl AsRef<Path>) -> Option<PathBuf> {
    let ret = path
        .as_ref()
        .read_dir()
        .ok()?
        .filter_map(|r| r.ok())
        .map(|r| r.path())
        .filter(|r| r.is_file())
        .find(|r| r.file_stem().is_some_and(|r| r.eq(DEFAULT_IMAGE_FILE_NAME)));

    ret
}
