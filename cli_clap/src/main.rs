use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "TestCli")]
#[command(author = "ZhengjunHUO <firelouiszj@hotmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "explore clap", long_about = None)]
struct Cmd {
    // 子命令
    #[command(subcommand)]
    subcmd: Option<SubCmd>,

    // 识别为带value的Option
    // 只接受--config (long)
    #[arg(long, value_name = "/PATH/TO/CONFIG")]
    config: Option<PathBuf>,

    // 类别u8加上Count, 识别为不带value的Option
    // 统计出现次数
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    // 识别为Argument
    target: Option<String>,
}

#[derive(Subcommand)]
enum SubCmd {
    Valid {
        // 类别为布尔值，识别为不带value的Option
        #[arg(short, long)]
        all: bool,

        // 类别为字符串，识别为带value的Option
        #[arg(long)]
        inline: String,
    },
    Override {
        // 类别为Vec, 可以接受数个Arguments
        delta: Vec<PathBuf>,
    },
}

// eg. ./target/debug/cli_clap -vvv --config /etc/hosts huo valid -a --inline never
// eg. ./target/debug/cli_clap override d1.conf d2.conf d3.conf
fn main() {
    let cmd = Cmd::parse();

    if let Some(conf) = cmd.config.as_deref() {
        println!("[Debug] Config file: {}", conf.display());
    }

    match cmd.verbose {
        0 => println!("Verbose level 0 !"),
        1 => println!("Verbose level 1 !"),
        2 => println!("Verbose level 2 !"),
        _ => println!("Verbose level 3 !"),
    }

    match &cmd.subcmd {
        Some(SubCmd::Valid { all, inline }) => {
            if *all {
                println!("[Debug] Valid all, inline: {} !", inline);
            } else {
                println!("[Debug] Valid by default, inline: {}!", inline);
            }
        }
        Some(SubCmd::Override { delta }) => {
            println!("[Debug] Delta files: {:?}", delta);
        }
        None => {}
    }

    if let Some(target) = cmd.target.as_deref() {
        println!("[Debug] Target: {}", target);
    }
}
