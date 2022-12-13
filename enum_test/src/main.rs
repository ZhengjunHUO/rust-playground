// attach data to each variant of the enum directly
#[derive(Debug)]
enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

#[derive(Debug)]
#[allow(unused)]
struct IpAddrV4 {
    addr: [u8; 4],
    cidr: u8,
}

#[derive(Debug)]
#[allow(unused)]
struct IpAddrV6 {
    addr: [u16; 8],
    cidr: u8,
}

fn main() {
    let ip4 = IpAddr::V4(IpAddrV4 {
        addr: [127, 0, 0, 1],
        cidr: 8,
    });

    let ip6 = IpAddr::V6(IpAddrV6 {
        addr: [0, 0, 0, 0, 0, 0, 0, 1],
        cidr: 128,
    });

    trace_route(&ip4);
    trace_route(&ip6);
}

fn trace_route(ip: &IpAddr) {
    match ip {
        IpAddr::V4(ipv4) => {
            let mut addr = String::from(" ");
            let dot = String::from(".");
            let slash = String::from("/");
            let len = ipv4.addr.len();
            for (i, &v) in ipv4.addr.iter().enumerate() {
                addr.push_str(&v.to_string());
                if i < len - 1 {
                    addr.push_str(&dot);
                } else {
                    addr.push_str(&slash);
                }
            }
            addr.push_str(&ipv4.cidr.to_string());
            println!("Tracing the route to an IPv4 addr: {:?} ...", addr);
        }
        IpAddr::V6(ipv6) => println!(
            "Tracing the route to an IPv6 addr: {:?}/{:?} ...",
            ipv6.addr, ipv6.cidr
        ),
        // _ => (),
    }
}
