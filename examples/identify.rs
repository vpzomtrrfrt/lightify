extern crate lightify;

use lightify::Gateway;

fn main() {
    let mut conn = std::net::TcpStream::connect(std::env::args().nth(1).expect("Missing address")).unwrap();

    println!("{:?}", conn.identify());
}
