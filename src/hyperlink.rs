use std::{
    fmt::{self, Formatter, Write},
    path::{Path, PathBuf},
};

use crate::filesystem::absolute_path;

pub(crate) struct PathUrl(PathBuf);

impl PathUrl {
    pub(crate) fn new(path: &Path) -> Option<PathUrl> {
        Some(PathUrl(absolute_path(path).ok()?))
    }
}

impl fmt::Display for PathUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file://{}", host())?;
        let bytes = self.0.as_os_str().as_encoded_bytes();
        for &byte in bytes.iter() {
            encode(f, byte)?;
        }
        Ok(())
    }
}

fn encode(f: &mut Formatter, byte: u8) -> fmt::Result {
    match byte {
        b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'/' | b':' | b'-' | b'.' | b'_' | b'~' => {
            f.write_char(byte.into())
        }
        #[cfg(windows)]
        b'\\' => f.write_char('/'),
        _ => {
            write!(f, "%{byte:02X}")
        }
    }
}

#[cfg(unix)]
fn host() -> &'static str {
    use std::sync::OnceLock;

    static HOSTNAME: OnceLock<String> = OnceLock::new();

    HOSTNAME
        .get_or_init(|| {
            nix::unistd::gethostname()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_default()
        })
        .as_ref()
}

#[cfg(not(unix))]
const fn host() -> &'static str {
    ""
}
