use std::{cell::OnceCell, fs::Metadata, path::PathBuf};

use lscolors::Style;

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
