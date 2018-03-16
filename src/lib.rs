#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate quick_error;
extern crate rand;

use rand::Rng;

macro_rules! take(
    ($r:expr, $c:expr) => {{
        let mut buf = [0; $c];
        let len = $r.read(&mut buf)?;
        if len < $c {
            Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof,
                                    "Stream ended unexpectedly"))
        }
        else {
            Ok(buf)
        }
    }});

mod parse;
mod error;

pub use parse::SystemInfo;
pub use error::Error;

pub trait Gateway {
    fn identify(&mut self) -> Result<parse::SystemInfo, Error>;
    fn set_all(&mut self, bool) -> Result<(), Error>;
    fn set_rgbw(&mut self, address: &[u8; 8], color: &[u8; 4]) -> Result<(), Error>;
}

const HELLO_PACKET: [u8; 13] = [
    0x0B, 0x00, 0x00, 0x13,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x00];

const BROADCAST_PACKET_OFF: [u8; 17] = [
    0x0f, 0x00, 0x00, 0x32,
    0x01, 0x00, 0x00, 0x00,
    0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x00
];

const BROADCAST_PACKET_ON: [u8; 17] = [
    0x0f, 0x00, 0x00, 0x32,
    0x01, 0x00, 0x00, 0x00,
    0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x01
];

impl<T> Gateway for T where T: std::io::Read + std::io::Write {
    fn identify(&mut self) -> Result<parse::SystemInfo, Error> {
        self.write_all(&HELLO_PACKET)?;
        parse::read_discovery_response(self)
    }
    fn set_all(&mut self, state: bool) -> Result<(), Error> {
        self.write_all(&if state {BROADCAST_PACKET_ON} else {BROADCAST_PACKET_OFF})?;
        take!(self, 20)?;
        Ok(())
    }
    fn set_rgbw(&mut self, address: &[u8; 8], color: &[u8; 4]) -> Result<(), Error> {
        let packet = [&[0x14, 0x00, 0x00, 0x36][..],
        &rand::thread_rng().gen::<[u8; 4]>()[..],
        &address[..],
        &color[..],
        &[0x00, 0x00][..]].concat();
        self.write_all(&packet)?;
        take!(self, 20)?;
        Ok(())
    }
}
