use std::sync::OnceLock;

use anyhow::anyhow;
use regex::Regex;

//SI 前缀,前缀基于 10 的幂，用于标准化地表示不同数量级的单位。
const KILO: u64 = 1000;
const MEGA: u64 = KILO * 1000;
const GIGA: u64 = MEGA * 1000;
const TERA: u64 = GIGA * 1000;

//二进制前缀（基于 2 的幂）二进制前缀基于 2 的幂，它们在计算机科学中非常重要，因为计算机的内存和处理都是基于二进制的
//在这里我们用来表示文件的大小
const KIBI: u64 = 1024;
const MEBI: u64 = KIBI * 1024;
const GIBI: u64 = MEBI * 1024;
const TEBI: u64 = GIBI * 1024;

static SIZE_CAPTURES: OnceLock<Regex> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeFilter {
    MAX(u64),
    Min(u64),
    Equals(u64),
}

impl SizeFilter {
    //该方法将一个字符串转换为 SizeFilter 枚举的值。
    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        SizeFilter::parse_opt(s)
            .ok_or_else(|| anyhow!("'{}'不是有效的大小限制。输入file-find -h获取帮助", s))
    }

    fn parse_opt(s: &str) -> Option<Self> {
        let pattern =
            SIZE_CAPTURES.get_or_init(|| Regex::new(r"(?i)^([+-]?)(\d+)(b|[kmgt]i?b?)$").unwrap());
        if !pattern.is_match(s) {
            return None;
        }

        let captures = pattern.captures(s)?;
        let limit_kind = captures.get(1).map_or("+", |m| m.as_str());
        let quantity = captures
            .get(2)
            .and_then(|v| v.as_str().parse::<u64>().ok())?;

        let multiplier = match &captures.get(3).map_or("b", |m| m.as_str()).to_lowercase()[..] {
            v if v.starts_with("ki") => KIBI,
            v if v.starts_with('k') => KILO,
            v if v.starts_with("mi") => MEBI,
            v if v.starts_with('m') => MEGA,
            v if v.starts_with("gi") => GIBI,
            v if v.starts_with('g') => GIGA,
            v if v.starts_with("ti") => TEBI,
            v if v.starts_with('t') => TERA,
            "b" => 1,
            _ => return None,
        };

        let size = quantity * multiplier;
        match limit_kind {
            "+" => Some(SizeFilter::Min((size))),
            "-" => Some(SizeFilter::MAX((size))),
            "" => Some(SizeFilter::Equals(size)),
            _ => None,
        }
    }

    pub fn is_within(&self, size: u64) -> bool {
        match *self {
            SizeFilter::MAX(limit) => size <= limit,
            SizeFilter::Min(limit) => size >= limit,
            SizeFilter::Equals(limit) => size == limit,
        }
    }
}
