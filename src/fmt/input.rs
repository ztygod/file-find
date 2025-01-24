use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::filesystem::strip_current_dir;

//用于提取路径的基本名称（base name），即路径中最后一个组件（例如文件名或目录名）。
pub fn basename(path: &Path) -> &OsStr {
    path.file_name().unwrap_or(path.as_os_str())
}

//去掉文件后拓展名 如.txt
pub fn remove_extension(path: &Path) -> OsString {
    let dirname = dirname(path);
    let stem = path.file_stem().unwrap_or(path.as_os_str());

    let path = PathBuf::from(dirname).join(stem);

    strip_current_dir(&path).to_owned().into_os_string()
}

//从给定的路径中提取父目录的路径。如果路径没有父目录（例如根路径或当前目录），则返回默认值。
pub fn dirname(path: &Path) -> OsString {
    path.parent()
        .map(|p| {
            if p == OsStr::new("") {
                OsString::from(".")
            } else {
                p.as_os_str().to_owned()
            }
        })
        .unwrap_or_else(|| path.as_os_str().to_owned())
}
