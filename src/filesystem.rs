use std::{
    borrow::Cow,
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use normpath::PathExt;
/*
PathBuf 是 Rust 标准库中用于处理文件路径的一个类型，定义在 std::path 模块中。
它是一个可变的、拥有所有权的路径表示类型，主要用于操作和构建文件路径。
*/

/*
该函数的作用是将一个路径转换为绝对路径：
1.如果传入的路径已经是绝对路径，直接返回。
2.如果路径是相对路径：
    2.1移除路径前缀 "."（当前目录）。
    2.2将当前工作目录与该路径拼接，得到绝对路径。
*/
pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let path = path.strip_prefix(".").unwrap_or(path);
    env::current_dir().map(|path_buf| path_buf.join(path))
}

/*
针对一部分代码逐步分析：
这部分代码仅在 Windows 系统上执行（由 #[cfg(windows)] 属性控制）。我们逐步分析：

1.#[cfg(windows)] 是一个条件编译属性，仅在编译为 Windows 系统时有效。当程序在非 Windows 系统上编译时，这部分代码会被排除，不会被执行。

2.path_buf.as_path()：将 PathBuf 转换为 Path，因为 PathBuf 是 Path 的可变版本，as_path() 提供了不可变引用的接口。

3..to_string_lossy()：将 Path 转换为字符串。to_string_lossy() 会将路径转换为 UTF-8 编码的字符串。如果路径中包含无效的 UTF-8 字符，它会使用 �（U+FFFD）替代。

4..trim_start_matches(r"\\?\")：在 Windows 上，某些绝对路径可能以 \\?\ 开头，表示路径扩展模式（例如支持超过 260 字符的路径）。该方法用于去除路径中的前缀 \\?\，只保留实际路径部分。

例如，路径 \\?\C:\Program Files 会被转换为 C:\Program Files。
trim_start_matches(r"\\?\") 会去除这个前缀，只保留纯路径。
Path::new(...)：创建一个新的 Path，将处理后的路径字符串传入。

5..to_path_buf()：将 Path 转换回 PathBuf，这是一个可变的路径类型，之后可以继续进行修改。
*/
pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    let path_buf = path_absolute_form(path)?;

    #[cfg(windows)]
    let path_buf = Path::new(
        path_buf
            .as_path()
            .to_string_lossy()
            .trim_start_matches(r"\\?\"),
    )
    .to_path_buf();

    Ok(path_buf)
}

/*
该函数的作用是判断给定路径是否是有效的目录，条件为：

1.path.is_dir()：路径必须是一个目录。
2.需要满足以下两个条件之一：
    path.file_name().is_some()：路径必须包含文件名或目录名（即路径不是根路径）。
    path.normalize().is_ok()：路径在规范化过程中必须没有错误。
*/
pub fn is_existing_directory(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.normalize().is_ok())
}

//pub fn is_empty(entry:&)

/*
这些函数用于在 Windows 系统下处理或模拟文件类型检查。
它们都返回 false，因为 Windows 系统没有类 Unix 系统中常见的块设备、字符设备、套接字和管道文件类型。
通常，这些函数会在跨平台应用中与 Linux 或 macOS 上的类似函数结合使用，确保在不同操作系统下的文件类型处理。
*/
#[cfg(windows)]
pub fn is_block_device(_: fs::FileType) -> bool {
    false
}

#[cfg(windows)]
pub fn is_char_device(_: fs::FileType) -> bool {
    false
}

#[cfg(windows)]
pub fn is_socket(_: fs::FileType) -> bool {
    false
}

#[cfg(windows)]
pub fn is_pipe(_: fs::FileType) -> bool {
    false
}

/*
该函数的目的是将 OsStr 转换为字节数组，同时尽量避免多余的拷贝：
1.如果 OsStr 是合法的 Unicode，直接返回借用的字节引用（高效）。
2.如果 OsStr 包含非法 Unicode，返回替换后的字节数组（安全处理）。
*/
#[cfg(windows)]
pub fn osstr_to_bytes(input: &OsStr) -> Cow<[u8]> {
    let string = input.to_string_lossy();

    match string {
        Cow::Owned(string) => Cow::Owned(string.into_bytes()),
        Cow::Borrowed(string) => Cow::Borrowed(string.as_bytes()),
    }
}

pub fn strip_current_dir(path: &Path) -> &Path {
    path.strip_prefix(".").unwrap_or(path)
}

/*
该函数的作用是在 Windows 系统中根据 MSYSTEM 环境变量的值来决定是否返回路径分隔符 /。
这种情况通常发生在像 MSYS2 这样的环境中，它模拟类 Unix 系统的行为，使用正斜杠作为路径分隔符，即使是在 Windows 系统中。

如果在 Windows 系统中找到并且 MSYSTEM 环境变量非空，返回 Some("/")，表示使用 / 作为路径分隔符。
否则，返回 None，表示没有特定的路径分隔符。
*/
pub fn defaault_path_separator() -> Option<String> {
    if cfg!(windows) {
        let msystem = env::var("MSYSTEM").ok()?;
        if !msystem.is_empty() {
            return Some("/".to_owned());
        }
    }
    None
}
