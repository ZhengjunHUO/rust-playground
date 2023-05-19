use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cmd {
    // 子命令
    #[command(subcommand)]
    subcmd: Option<SubCmd>,

    // 识别为带value的Option
    #[arg(short, long, value_name = "/PATH/TO/CONFIG")]
    config: Option<PathBuf>,

    // 识别为不带value的Option
    // 统计出现次数
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    // 识别为Argument
    target: Option<String>,
}

#[derive(Subcommand)]
enum SubCmd {
    Valid {
        // 识别为不带value的Option
        #[arg(short, long)]
        all: bool,
    },
}

// eg. ./target/debug/cli_clap -vvv -c /etc/hosts xxx valid -a
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
        Some(SubCmd::Valid { all }) => {
            if *all {
                println!("[Debug] Valid all !");
            } else {
                println!("[Debug] Valid by default !");
            }
        }
        None => {}
    }

    if let Some(target) = cmd.target.as_deref() {
        println!("[Debug] Target: {}", target);
    }
}
