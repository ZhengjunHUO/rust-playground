use anyhow::Context;
use aya::programs::{Xdp, XdpFlags};
use aya::{include_bytes_aligned, BpfLoader, Btf};
use aya_log::BpfLogger;
use clap::Parser;
use log::warn;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    #[cfg(debug_assertions)]
    let mut bpf = BpfLoader::new()
        .btf(Btf::from_sys_fs().ok().as_ref())
        .map_pin_path("/sys/fs/bpf/aya-xdp")
        .load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/debug/aya-xdp"
        ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = BpfLoader::new()
        .btf(Btf::from_sys_fs().ok().as_ref())
        .map_pin_path("/sys/fs/bpf/aya-xdp")
        .load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/release/aya-xdp"
        ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut Xdp = bpf.program_mut("aya_xdp").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;
    program.pin("/sys/fs/bpf/aya-xdp-prog")?;

    Ok(())
}
