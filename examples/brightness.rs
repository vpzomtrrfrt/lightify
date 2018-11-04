extern crate lightify;

use lightify::Gateway;


fn main() {
    let mut conn = std::net::TcpStream::connect(std::env::args().nth(1).expect("Missing address")).unwrap();
    let brightness: u8 = std::env::args().nth(2).expect("Missing brightness").parse().expect("Expected u8");
    let info = conn.identify().unwrap();
    conn.set_brightness(&info.lights[0].address, brightness).unwrap();
}
