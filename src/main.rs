use clap::Parser; // 引入派生宏

pub mod cli;
pub mod dir_entry;
pub mod filesystem;
pub mod filter;
use crate::cli::Opts;

fn main() {
    let opts: Opts = Opts::parse(); // 自动解析命令行参数

    // 打印解析结果
    println!("{:?}", opts);
}
