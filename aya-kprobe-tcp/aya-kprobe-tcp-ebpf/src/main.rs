#![no_std]
#![no_main]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod vmlinux;

use aya_bpf::helpers::bpf_probe_read_kernel;
use aya_bpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;
use vmlinux::{sock, sock_common};

const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;

#[kprobe]
pub fn aya_kprobe_tcp(ctx: ProbeContext) -> u32 {
    match try_aya_kprobe_tcp(ctx) {
        Ok(ret) => ret,
        Err(ret) => match ret.try_into() {
            Ok(ret_val) => ret_val,
            Err(_) => 1,
        },
    }
}

fn try_aya_kprobe_tcp(ctx: ProbeContext) -> Result<u32, i64> {
    let socket: *mut sock = ctx.arg(0).ok_or(1i64)?;
    let sock_comm = unsafe {
        bpf_probe_read_kernel(&(*socket).__sk_common as *const sock_common).map_err(|e| e)?
    };

    match sock_comm.skc_family {
        AF_INET => {
            let saddr =
                u32::from_be(unsafe { sock_comm.__bindgen_anon_1.__bindgen_anon_1.skc_rcv_saddr });
            let daddr =
                u32::from_be(unsafe { sock_comm.__bindgen_anon_1.__bindgen_anon_1.skc_daddr });
            let sport: u16 = unsafe { sock_comm.__bindgen_anon_3.__bindgen_anon_1.skc_num };
            let dport =
                u16::from_be(unsafe { sock_comm.__bindgen_anon_3.__bindgen_anon_1.skc_dport });
            info!(&ctx, "[{:i}:{}] => [{:i}:{}]", saddr, sport, daddr, dport,);
            Ok(0)
        }
        AF_INET6 => {
            let saddr = sock_comm.skc_v6_rcv_saddr;
            let daddr = sock_comm.skc_v6_daddr;
            info!(
                &ctx,
                "[{:i}] => [{:i}]",
                unsafe { saddr.in6_u.u6_addr8 },
                unsafe { daddr.in6_u.u6_addr8 }
            );
            Ok(0)
        }
        _ => Ok(0),
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
