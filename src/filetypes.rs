use faccess::PathExt;

use crate::dir_entry;
use crate::filesystem;

#[derive(Default)]
pub struct FileType {
    pub files: bool,
    pub directories: bool,
    pub symlibks: bool,
    pub block_devices: bool,
    pub chat_devices: bool,
    pub sockets: bool,
    pub pipes: bool,
    pub executables_only: bool,
    pub empty_only: bool,
}

impl FileType {
    pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool {
        if let Some(ref entry_type) = entry.file_type() {
            (!self.files && entry_type.is_file())
                || (!self.directories && entry_type.is_dir())
                || (!self.symlibks && entry_type.is_symlink())
                || (!self.block_devices && filesystem::is_block_device(*entry_type))
                || (!self.chat_devices && filesystem::is_char_device(*entry_type))
                || (!self.sockets && filesystem::is_socket(*entry_type))
                || (!self.pipes && filesystem::is_pipe(*entry_type))
                || (self.executables_only && !entry.path().executable())
                || (self.empty_only && !filesystem::is_empty(entry))
                || !(entry_type.is_file()
                    || entry_type.is_dir()
                    || entry_type.is_symlink()
                    || filesystem::is_block_device(*entry_type)
                    || filesystem::is_char_device(*entry_type)
                    || filesystem::is_socket(*entry_type)
                    || filesystem::is_pipe(*entry_type))
        } else {
            true
        }
    }
}
