#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    let home = IpAddr {
        kind: IPAddrKind::V4,
        address: String::from("192.168.0.1"),
    };
    println!("{:?} {:?}", home.address, home.kind);
    route(&four);
    route(&six)
}

fn route(ip_kind: &IPAddrKind) {
    println!("{:?}", ip_kind)
}
