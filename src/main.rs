use clap::Parser; // 引入派生宏

pub mod cli;
pub mod config;
pub mod dir_entry;
pub mod error;
pub mod error_codes;
pub mod filesystem;
pub mod filetypes;
pub mod filter;
pub mod fmt;
pub mod hyperlink;
pub mod output;
pub mod regex_helper;
use crate::cli::Opts;

fn main() {
    let opts: Opts = Opts::parse(); // 自动解析命令行参数

    // 打印解析结果
    println!("{:?}", opts);
}
