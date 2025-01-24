use lscolors::LsColors;

use crate::fmt::FormatTemplate;

pub struct Config {
    //搜索是否注意大小写
    pub case_sensitive: bool,

    //是否在完整文件路径
    //或仅在基本名称（文件名或目录名称）内搜索。
    pub search_full_path: bool,

    //是否忽略隐藏文件或目录
    pub ignore_hidden: bool,

    //是否注意“.fdignore”文件。
    pub read_fdignore: bool,

    //是否跟随符号链接。
    pub follow_links: bool,

    //是否剥离' ./ '在搜索结果中
    pub strip_cwd_prefix: bool,

    /// 是否在路径上使用超链接
    pub hyperlink: bool,

    ///
    pub format: Option<FormatTemplate>,

    pub path_separator: Option<String>,

    pub actual_path_separator: String,

    pub ls_colors: Option<LsColors>,

    pub null_separator: bool,
}
