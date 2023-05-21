use clap::{Args, CommandFactory, Parser, Subcommand};
//use clap::ValueEnum;
use std::ops::RangeInclusive;
use std::path::PathBuf;

const PORT_RANGE: RangeInclusive<usize> = 1001..=65535;

#[derive(Parser)]
#[command(name = "TestCli")]
#[command(author = "ZhengjunHUO <firelouiszj@hotmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "explore clap", long_about = None)]
// 子命令也会继承根命令的版本(-V)
#[command(propagate_version = true)]
struct Cmd {
    // 子命令
    #[command(subcommand)]
    subcmd: Option<SubCmd>,

    // 识别为带value的Option
    // 只接受--config (long)
    // 依赖于target
    /// Specify the path to configuration file
    #[arg(long, requires = "cible", value_name = "/PATH/TO/CONFIG")]
    config: Option<PathBuf>,

    // 类别u8加上Count, 识别为不带value的Option
    // 统计出现次数
    /// Adjust the verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    // 识别为Argument
    #[arg(group = "cible")]
    target: Option<String>,
}

#[derive(Subcommand)]
enum SubCmd {
    /// Validate the given configuration file
    Valid {
        // 类别为布尔值，识别为不带value的Option
        #[arg(short, long)]
        all: bool,

        // 类别为字符串，识别为带value的Option
        #[arg(long, default_value = "always")]
        inline: String,
    },
    /// Provide extra config files to override the default one
    Override(OverrideArgs),
    /// Specify which environment should be deployed to
    Deploy {
        //#[arg(short, long, value_enum)]
        //infra: Infra,
        #[command(flatten)]
        infra: Infra,

        /// Tcp port the app will listen on
        #[arg(short, long, default_value_t = 8080, value_parser = clap::value_parser!(u16).range(1001..))]
        tcp_port: u16,

        /// Udp port the app will listen on
        #[arg(short, long, default_value_t = 8088, value_parser = validate_port)]
        udp_port: u16,
    },
}

//#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
//enum Infra {
//    K8S,
//    Baremetal,
//}

// 必须声明任意一种option
//#[group(required = true, multiple = false)]
#[derive(Args, Debug)]
struct Infra {
    #[arg(long)]
    k8s: bool,

    #[arg(long)]
    docker: bool,

    #[arg(long)]
    baremetal: bool,
}

#[derive(Args)]
struct OverrideArgs {
    // 类别为Vec, 可以接受数个Arguments
    delta: Vec<PathBuf>,
}

// eg. ./target/debug/cli_clap -vvv --config /etc/hosts huo valid -a --inline never
// eg. ./target/debug/cli_clap override d1.conf d2.conf d3.conf
// eg. ./target/debug/cli_clap deploy --k8s
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
                println!("[Debug] Valid all, inline: {:?} !", inline);
            } else {
                println!("[Debug] Valid by default, inline: {:?}!", inline);
            }
        }
        Some(SubCmd::Override(oa)) => {
            println!("[Debug] Delta files: {:?}", oa.delta);
        }
        Some(SubCmd::Deploy {
            infra,
            tcp_port,
            udp_port,
        }) => {
            match (infra.k8s, infra.docker, infra.baremetal) {
                (true, false, false) => println!("[Debug] Will deployed on kubernetes"),
                (false, true, false) => println!("[Debug] Will deployed on docker"),
                (false, false, true) => println!("[Debug] Will deployed on baremetal"),
                //_ => (),
                // 移除struct Infra的#[group(required = true, multiple = false)]
                // 自定义validate的规则和错误信息
                _ => {
                    let mut cmmd = Cmd::command();
                    cmmd.error(
                        clap::error::ErrorKind::ArgumentConflict,
                        "One (and only one) option <--k8s|--docker|--baremetal> is required.",
                    )
                    .exit();
                }
            };
            println!("[Debug] Will listen on tcp port {:?}", tcp_port);
            println!("[Debug] Will listen on udp port {:?}", udp_port);
        }
        None => {}
    }

    if let Some(target) = cmd.target.as_deref() {
        println!("[Debug] Target: {}", target);
    }
}

fn validate_port(s: &str) -> Result<u16, String> {
    let p: usize = s.parse().map_err(|_| format!("{} is not a number", s))?;
    if PORT_RANGE.contains(&p) {
        Ok(p as u16)
    } else {
        Err(format!(
            "Port should between [{}-{}]",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
