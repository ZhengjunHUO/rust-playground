// attach data to each variant of the enum directly
#[derive(Debug)]
enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

#[derive(Debug)]
struct IpAddrV4 {
    addr: (u8, u8, u8, u8),
    cidr: u8,
}

#[derive(Debug)]
struct IpAddrV6 {
    addr: (u16, u16, u16, u16, u16, u16, u16, u16),
    cidr: u8,
}

fn main() {
    let ip4 = IpAddr::V4(IpAddrV4{
        addr: (127, 0, 0, 1),
        cidr: 8,
    });

    let ip6 = IpAddr::V6(IpAddrV6{
        addr: (0, 0, 0, 0, 0, 0, 0, 1),
        cidr: 128,
    });

    trace_route(&ip4);
    trace_route(&ip6);
}

fn trace_route(ip: &IpAddr) {
    println!("Tracing the route to {:?} ...", ip);
}
