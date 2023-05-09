mod firewall {
    include!(concat!(env!("OUT_DIR"), "/cgroup_fw.skel.rs"));
}

use anyhow::{bail, Result};
use firewall::*;
use libbpf_rs::MapFlags;
use libc;
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;
use std::str::FromStr;

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
    let ctn_id = "252491db736e3ece6bccbb019c2953d4fa4907f3ba3e3742b00913674fc3e45a";
    let ip = "8.8.4.4";

    let builder = CgroupFwSkelBuilder::default();
    increase_rlimit()?;
    // Get an opened, pre-load bpf object
    let open = builder.open()?;
    // Get a loaded bpf object
    let mut obj = open.load()?;

    // Get target cgroup id
    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(format!(
            "/sys/fs/cgroup/system.slice/docker-{}.scope",
            ctn_id
        ))?;
    let cgroup_fd = f.as_raw_fd();

    // Get loaded program and attach to the cgroup
    let mut eg_link = obj.progs_mut().egress_filter().attach_cgroup(cgroup_fd)?;
    // The prog_type and attach_type are inferred from the c program
    // should be CgroupInetEgress here
    //println!("[DEBUG]: Attach type is {:?}", obj.progs().egress_filter().attach_type());
    eg_link.pin("/sys/fs/bpf/cgroup_egs_link")?;

    // Get loaded map
    let mut maps = obj.maps_mut();
    let eg_fw_map = maps.egress_blacklist();

    // Persist the map on bpf vfs
    eg_fw_map.pin("/sys/fs/bpf/cgroup_egs_map")?;

    // Apply a rule
    let ip_parsed = Ipv4Addr::from_str(&ip)?;
    let key = u32::from(ip_parsed).to_be_bytes();
    let value = u8::from(true).to_ne_bytes();
    eg_fw_map.update(&key, &value, MapFlags::ANY)?;

    println!("Done!");
    Ok(())
}
