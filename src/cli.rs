use clap::Parser; // 引入派生宏

use crate::filter::SizeFilter;
/// 一个简单的文件搜索工具
#[derive(Parser, Debug)]
#[command(name = "fd_search", version = "1.0", about = "A fast file search tool")]
pub struct Opts {
    /// 搜索的模式（正则表达式）
    #[arg(short, long)]
    pattern: String,

    /// 搜索的路径（默认为当前目录）
    #[arg(short = 'P', long, default_value = ".")]
    path: String,

    /// 是否包括隐藏文件
    #[arg(short = 'H', long)]
    hidden: bool,

    /// 限制搜索结果的数量
    #[arg(short, long)]
    limit: Option<u64>,
}
