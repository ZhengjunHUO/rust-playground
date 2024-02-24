#![no_std]
#![no_main]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod vmlinux;

use aya_bpf::{
    cty::c_int,
    macros::lsm,
    programs::LsmContext,
};
use aya_log_ebpf::info;
use vmlinux::task_struct;

#[no_mangle]
static TARGET_PID: i32 = 0;

/* From include/linux/lsm_hooks.h
 * @task_setnice:
 *	Check permission before setting the nice value of @p to @nice.
 *	@p contains the task_struct of process.
 *	@nice contains the new nice value.
 *	Return 0 if permission is granted.
*/

#[lsm(hook = "task_setnice")]
pub fn task_setnice(ctx: LsmContext) -> i32 {
    match unsafe { try_task_setnice(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_task_setnice(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "lsm hook task_setnice called");

    let ret: c_int = ctx.arg(2);
    if ret != 0 {
        return Err(ret);
    }

    let p: *const task_struct = ctx.arg(0);
    let pid: c_int = (*p).pid;
    let nice: c_int = ctx.arg(1);
    let target_pid: c_int = core::ptr::read_volatile(&TARGET_PID);

    info!(&ctx, "Mutate nice value to {} on proc {} (target proc: {})", nice, pid, target_pid);
    if pid == target_pid && nice < 0 {
        return Err(-1);
    }

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
