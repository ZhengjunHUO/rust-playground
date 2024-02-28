#![no_std]
#![no_main]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod vmlinux;

use aya_bpf::{cty::c_int, helpers::bpf_get_current_pid_tgid, macros::lsm, programs::LsmContext};
use aya_bpf_bindings::helpers::{bpf_get_current_task_btf, bpf_task_pt_regs};
//use aya_bpf_bindings::bindings::pt_regs;
use aya_log_ebpf::info;
use vmlinux::pt_regs;

#[lsm(hook = "cred_prepare")]
pub fn cred_prepare(ctx: LsmContext) -> i32 {
    match unsafe { try_cred_prepare(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_cred_prepare(ctx: LsmContext) -> Result<i32, i32> {
    let pid = bpf_get_current_pid_tgid() as u32;
    info!(&ctx, "lsm hook cred_prepare called by {}", pid);

    // return early if already KO
    let ret: c_int = ctx.arg(3);
    if ret != 0 {
        info!(&ctx, "[PID: {}] Already got ret code {}, quit!", pid, ret);
        return Err(ret);
    }

    let p = bpf_get_current_task_btf();
    let regs = bpf_task_pt_regs(p) as *const pt_regs;
    let syscall = (*regs).orig_ax;

    if syscall != 272 {
        return Ok(0);
    }
    info!(&ctx, "[PID: {}] Spot an unshare syscall !", pid);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
