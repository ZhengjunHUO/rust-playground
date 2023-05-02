mod firewall {
    include!(concat!(env!("OUT_DIR"), "/cgroup_fw.skel.rs"));
}

use anyhow::{bail, Result};
use firewall::*;
use libc;

fn increase_rlimit() -> Result<()> {
    let rl = libc::rlimit {
        rlim_cur: 1 << 20,
        rlim_max: 1 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rl) } != 0 {
        bail!("Error increasing rlimit");
    }

    Ok(())
}

fn main() -> Result<()> {
    let builder = CgroupFwSkelBuilder::default();
    increase_rlimit()?;
    let open = builder.open()?;
    let mut obj = open.load()?;
    obj.attach()?;
    println!("Ready!");

    // Add logic here

    Ok(())
}
