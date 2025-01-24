use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt::{Display, Formatter},
    path::{Component, Path, Prefix},
    sync::OnceLock,
};

mod input;

use aho_corasick::AhoCorasick;
use input::{basename, dirname, remove_extension};
//指定应写入缓冲区的内容每个“Token”包含文本或占位符变体，
//在收集了给定命令模板的所有令牌后，将用于生成命令。
/*
1.Placeholder：占位符，无具体含义。
2.Basename：路径的基本名称。
3.Parent：路径的父目录。
4.NoExt：去掉扩展名的路径。
5.BasenameNoExt：路径的基本名称（不含扩展名）。
6.Text(String)：存储任意文本内容。
*/
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Placeholder,
    Basename,
    Parent,
    NoExt,
    BasenameNoExt,
    Text(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Placeholder => f.write_str("{}")?,
            Token::Basename => f.write_str("{/}")?,
            Token::Parent => f.write_str("{//}")?,
            Token::NoExt => f.write_str("{.}")?,
            Token::BasenameNoExt => f.write_str("{/.}")?,
            Token::Text(ref string) => f.write_str(string)?,
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormatTemplate {
    Toekns(Vec<Token>),
    Text(String),
}

static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new();

impl FormatTemplate {
    pub fn has_tokens(&self) -> bool {
        matches!(self, FormatTemplate::Toekns(_))
    }

    pub fn parse(fmt: &str) -> Self {
        const BRACE_LEN: usize = '{'.len_utf8();
        let mut tokens = Vec::new();
        let mut remaining = fmt;
        let mut buf = String::new();
        let placeholders = PLACEHOLDERS.get_or_init(|| {
            AhoCorasick::new(["{{", "}}", "{}", "{/}", "{//}", "{.}", "{/.}"]).unwrap()
        });
        while let Some(m) = placeholders.find(remaining) {
            match m.pattern().as_u32() {
                0 | 1 => {
                    // 我们发现了转义的 {{ 或 }}，因此将
                    // 直到第一个字符的所有内容添加到缓冲区
                    // 然后跳过第二个。
                    buf += &remaining[..m.start() + BRACE_LEN];
                    remaining = &remaining[m.end()..];
                }
                id if !remaining[m.end()..].starts_with('}') => {
                    buf += &remaining[..m.start()];
                    if !buf.is_empty() {
                        tokens.push(Token::Text(std::mem::take(&mut buf)));
                    }
                    tokens.push(token_from_pattern_id(id));
                    remaining = &remaining[m.end()..];
                }
                _ => {
                    buf += &remaining[..m.end()];
                    remaining = &remaining[m.end() + BRACE_LEN..]
                }
            }
        }
        if !remaining.is_empty() {
            buf += remaining;
        }
        if tokens.is_empty() {
            return FormatTemplate::Text(buf);
        }
        if !buf.is_empty() {
            tokens.push(Token::Text(buf));
        }
        debug_assert!(!tokens.is_empty());
        FormatTemplate::Toekns(tokens)
    }

    ///从此模板生成结果字符串。如果 path_separator 为 Some，则它将替换
    /// 所有占位符标记中的路径分隔符。固定文本和标记不受
    /// 路径分隔符替换的影响。
    pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString {
        use Token::*;
        let path = path.as_ref();

        match *self {
            Self::Toekns(ref tokens) => {
                let mut s = OsString::new();
                for token in tokens {
                    match token {
                        Basename => s.push(Self::replace_separator(basename(path), path_separator)),
                        BasenameNoExt => s.push(Self::replace_separator(
                            &remove_extension(basename(path).as_ref()),
                            path_separator,
                        )),
                        NoExt => s.push(Self::replace_separator(
                            &remove_extension(path),
                            path_separator,
                        )),
                        Parent => s.push(Self::replace_separator(&dirname(path), path_separator)),
                        Placeholder => {
                            s.push(Self::replace_separator(path.as_ref(), path_separator));
                        }
                        Text(ref string) => s.push(string),
                    }
                }
                s
            }
            Self::Text(ref text) => OsString::from(text),
        }
    }

    ///将输入中的路径分隔符替换为自定义分隔符字符串。如果 path_separator
    /// 为 None，则只需返回从输入借用的 Cow<OsStr>。否则，输入将被
    /// 解释为路径，其组件将被迭代并重新连接到新的
    /// OsString。
    fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr> {
        if path_separator.is_none() {
            return Cow::Borrowed(path);
        }

        let path_separator = path_separator.unwrap();
        let mut out = OsString::with_capacity(path.len());
        let mut components = Path::new(path).components().peekable();

        while let Some(comp) = components.next() {
            match comp {
                Component::Prefix(prefix) => {
                    if let Prefix::UNC(server, share) = prefix.kind() {
                        out.push(path_separator);
                        out.push(path_separator);
                        out.push(server);
                        out.push(path_separator);
                        out.push(share);
                    } else {
                        out.push(comp.as_os_str());
                    }
                }

                Component::RootDir => out.push(path_separator),

                _ => {
                    out.push(comp.as_os_str());
                    if components.peek().is_some() {
                        out.push(path_separator);
                    }
                }
            }
        }
        Cow::Owned(out)
    }
}

fn token_from_pattern_id(id: u32) -> Token {
    use Token::*;
    match id {
        2 => Placeholder,
        3 => Basename,
        4 => Parent,
        5 => NoExt,
        6 => BasenameNoExt,
        _ => unreachable!(),
    }
}
