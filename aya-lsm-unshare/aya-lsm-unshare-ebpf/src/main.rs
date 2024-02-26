#![no_std]
#![no_main]

use aya_bpf::{
    macros::lsm,
    programs::LsmContext,
};
use aya_log_ebpf::info;

#[lsm(hook = "cred_prepare")]
pub fn cred_prepare(ctx: LsmContext) -> i32 {
    match try_cred_prepare(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_cred_prepare(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "lsm hook cred_prepare called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
