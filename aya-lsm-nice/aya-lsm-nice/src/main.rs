use aya::{include_bytes_aligned, BpfLoader};
use aya::{programs::Lsm, Btf};
use aya_log::BpfLogger;
use log::{debug, info, warn};
use std::process;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    let pid = process::id() as i32;
    info!("Running with pid {}", pid);

    #[cfg(debug_assertions)]
    let mut bpf =
        BpfLoader::new()
            .set_global("TARGET_PID", &pid, true)
            .load(include_bytes_aligned!(
                "../../target/bpfel-unknown-none/debug/aya-lsm-nice"
            ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf =
        BpfLoader::new()
            .set_global("TARGET_PID", &pid, true)
            .load(include_bytes_aligned!(
                "../../target/bpfel-unknown-none/release/aya-lsm-nice"
            ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let btf = Btf::from_sys_fs()?;
    let program: &mut Lsm = bpf.program_mut("task_setnice").unwrap().try_into()?;
    program.load("task_setnice", &btf)?;
    program.attach()?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
