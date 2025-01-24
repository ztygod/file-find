use std::{
    cell::OnceCell,
    fs::{FileType, Metadata},
    path::{Path, PathBuf},
};

use crate::config::Config;
use crate::filesystem::strip_current_dir;
use lscolors::{Colorable, LsColors, Style};
/*
DirEntryInner 枚举：
1.Normal(ignore::DirEntry)：这个变体存储了一个来自 ignore 库的 DirEntry。
    ignore::DirEntry 是 ignore 库用于表示目录条目的结构体，通常包含文件或目录的路径、元数据等信息。
    这个变体用于正常的文件或目录条目。
2.BrokenSymlink(PathBuf)：这个变体表示一个损坏的符号链接，存储了链接的路径。
    它用于处理那些指向不存在位置的符号链接。
*/
#[derive(Debug)]
enum DirEntryInner {
    Normal(ignore::DirEntry),
    BrokenSymlink(PathBuf),
}

#[derive(Debug)]
pub struct DirEntry {
    inner: DirEntryInner,
    metedata: OnceCell<Option<Metadata>>,
    style: OnceCell<Option<Style>>,
}

impl DirEntry {
    #[inline]
    pub fn normol(e: ignore::DirEntry) -> Self {
        Self {
            inner: DirEntryInner::Normal(e),
            metedata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }

    pub fn borken_symlink(path: PathBuf) -> Self {
        Self {
            inner: DirEntryInner::BrokenSymlink(path),
            metedata: OnceCell::new(),
            style: OnceCell::new(),
        }
    }

    pub fn path(&self) -> &Path {
        match &self.inner {
            DirEntryInner::Normal(e) => e.path(),
            DirEntryInner::BrokenSymlink(pathbuf) => pathbuf.as_path(),
        }
    }

    pub fn into_path(self) -> PathBuf {
        match self.inner {
            DirEntryInner::Normal(e) => e.into_path(),
            DirEntryInner::BrokenSymlink(pathbuf) => pathbuf,
        }
    }

    pub fn stripped_path(&self, config: &Config) -> &Path {
        if config.strip_cwd_prefix {
            strip_current_dir(self.path())
        } else {
            self.path()
        }
    }

    pub fn into_stripped_path(self, config: &Config) -> PathBuf {
        if config.strip_cwd_prefix {
            self.stripped_path(config).to_path_buf()
        } else {
            self.into_path()
        }
    }

    pub fn file_type(&self) -> Option<FileType> {
        match &self.inner {
            DirEntryInner::Normal(e) => e.file_type(),
            DirEntryInner::BrokenSymlink(_) => self.metedata().map(|m| m.file_type()),
        }
    }

    pub fn metedata(&self) -> Option<&Metadata> {
        self.metedata
            .get_or_init(|| match &self.inner {
                DirEntryInner::Normal(e) => e.metadata().ok(),
                DirEntryInner::BrokenSymlink(path) => path.symlink_metadata().ok(),
            })
            .as_ref()
    }

    pub fn depth(&self) -> Option<usize> {
        match &self.inner {
            DirEntryInner::Normal(e) => Some(e.depth()),
            DirEntryInner::BrokenSymlink(_) => None,
        }
    }

    pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
        self.style
            .get_or_init(|| ls_colors.style_for(self).cloned())
            .as_ref()
    }
}

impl PartialEq for DirEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path() == other.path()
    }
}

impl Eq for DirEntry {}

impl PartialOrd for DirEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path().cmp(other.path())
    }
}

impl Colorable for DirEntry {
    fn path(&self) -> PathBuf {
        self.path().to_owned()
    }

    fn file_name(&self) -> std::ffi::OsString {
        let name = match &self.inner {
            DirEntryInner::Normal(e) => e.file_name(),
            DirEntryInner::BrokenSymlink(path) => path
                .components()
                .last()
                .map(|c| c.as_os_str())
                .unwrap_or_else(|| path.as_os_str()),
        };
        name.to_owned()
    }

    fn file_type(&self) -> Option<FileType> {
        self.file_type()
    }

    fn metadata(&self) -> Option<Metadata> {
        self.metadata().clone()
    }
}
