fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;
    println!("FOUR = {:?}\nSIX = {:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("HOME kind = {:?}, address = {:?}", home.kind, home.address);
    println!("LOOPBACK kind = {:?}, address = {:?}", loopback.kind, loopback.address);
}

fn route(ip_kind: IpAddrKind) { 
    println!("ROUTE = {:?}", ip_kind);
}


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


/*
struct Ipv4Addr {}
struct Ipv6Addr {}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

