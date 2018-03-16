extern crate lightify;
extern crate rand;

use lightify::Gateway;
use rand::Rng;

fn main() {
    let mut conn = std::net::TcpStream::connect(std::env::args().nth(1).expect("Missing address")).unwrap();

    let info = conn.identify().unwrap();

    let mut rng = rand::thread_rng();

    let r: u8 = rng.gen();
    let g = rng.gen();
    let b = rng.gen();
    let w = r.min(g).min(b);

    conn.set_rgbw(&info.lights[0].address, &[r, g, b, w]).unwrap();
}
