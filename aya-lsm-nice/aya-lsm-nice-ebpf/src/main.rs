#![no_std]
#![no_main]

use aya_bpf::{
    macros::lsm,
    programs::LsmContext,
};
use aya_log_ebpf::info;

#[lsm(hook = "task_setnice")]
pub fn task_setnice(ctx: LsmContext) -> i32 {
    match try_task_setnice(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_task_setnice(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "lsm hook task_setnice called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
