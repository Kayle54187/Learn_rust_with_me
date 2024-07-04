#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddrKind {
    fn call(&self) {
        println!("IpAddrKind: {:?}", self);
    }
}

impl IpAddr {
    fn call(&self) {
        println!("IpAddr: {:?}", self);
    }

    fn new(kind: IpAddrKind, address: String) -> IpAddr {
        IpAddr { kind, address }
    }
}

fn main() {
    let localhost = IpAddrKind::V4(192, 168, 188, 88);
    let home = IpAddr::new(localhost, String::from("192.168.187"));

    home.call();
}
