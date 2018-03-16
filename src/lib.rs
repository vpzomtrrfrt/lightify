#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate quick_error;

mod parse;
mod error;

pub use parse::SystemInfo;
pub use error::Error;

pub trait Gateway {
    fn identify(&mut self) -> Result<parse::SystemInfo, Error>;
}

const HELLO_PACKET: [u8; 13] = [
    0x0B, 0x00, 0x00, 0x13,
    0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x00];

impl<T> Gateway for T where T: std::io::Read + std::io::Write {
    fn identify(&mut self) -> Result<parse::SystemInfo, Error> {
        self.write_all(&HELLO_PACKET)?;
        parse::read_discovery_response(self)
    }
}
