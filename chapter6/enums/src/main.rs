enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    if matches!(ip_kind, IpAddrKind::V4) {
        println!("V4");
    } else {
        println!("V6");
    }
}
