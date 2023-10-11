#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::{map, xdp}, maps::HashMap, programs::XdpContext};
use aya_log_ebpf::info;
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

#[xdp]
pub fn aya_xdp(ctx: XdpContext) -> u32 {
    match try_aya_xdp(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[map(name = "INGRESS_FILTER")]
static mut INGRESS_FILTER: HashMap<u32, u32> = HashMap::<u32, u32>::pinned(128, 0);

fn try_aya_xdp(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = ptr_at(&ctx, 0)?;
    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = ptr_at(&ctx, EthHdr::LEN)?;
    let source_addr = u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let dest_addr = u32::from_be(unsafe { (*ipv4hdr).dst_addr });
    let dest_port;
    let proto;

    let source_port = match unsafe { (*ipv4hdr).proto } {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr =
                ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            dest_port = u16::from_be(unsafe { (*tcphdr).dest });
            proto = "TCP";
            u16::from_be(unsafe { (*tcphdr).source })
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr =
                ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
            dest_port = u16::from_be(unsafe { (*udphdr).dest });
            proto = "UDP";
            u16::from_be(unsafe { (*udphdr).source })
        }
        _ => return Ok(xdp_action::XDP_PASS),
    };

    let (action, action_literal) = if blacklisted(source_addr) {
        (xdp_action::XDP_DROP, "drop")
    } else {
        (xdp_action::XDP_PASS, "pass")
    };

    info!(
        &ctx,
        "{} in {:i}:{} > {:i}:{} [{}]", proto, source_addr, source_port, dest_addr, dest_port, action_literal
    );

    Ok(action)
}

fn blacklisted(addr: u32) -> bool {
    unsafe { INGRESS_FILTER.get(&addr).is_some() }
}

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
