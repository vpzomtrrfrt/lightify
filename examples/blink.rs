extern crate lightify;

use lightify::Gateway;

fn main() {
    let interval = std::time::Duration::new(1, 0);

    let mut conn = std::net::TcpStream::connect(std::env::args().nth(1).expect("Missing address")).unwrap();

    println!("{:?}", conn.identify().unwrap());

    loop {
        conn.set_all(false).unwrap();
        std::thread::sleep(interval);
        conn.set_all(true).unwrap();
        std::thread::sleep(interval);
    }
}
