use std;

use error::Error;

struct DiscoveryHeader {
    count: u8
}

fn read_discovery_header<R: std::io::Read>(r: &mut R) -> Result<DiscoveryHeader, std::io::Error> {
    let mut buf = [0; 11];
    let count = r.read(&mut buf)?;
    if count < 11 {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof,
                                       "Couldn't read full header"));
    }
    Ok(DiscoveryHeader {
        count: buf[9]
    })
}

fn parse_u16(most: u8, least: u8) -> u16 {
    ((most as u16) << 8) | (least as u16)
}

#[derive(Debug)]
pub struct Light {
    id: u16,
    address: [u8; 8],
    bulb_type: u8,
    firmware_version: [u8; 4],
    online: bool,
    group_id: u8,
    status: bool,
    brightness: u8,
    temperature: u16,
    r: u8,
    g: u8,
    b: u8,
    w: u8,
    name: String
}

#[derive(Debug)]
pub struct SystemInfo {
    lights: Vec<Light>
}

pub fn read_discovery_response<R: std::io::Read>(r: &mut R) -> Result<SystemInfo, Error> {
    let header = read_discovery_header(r)?;
    let mut lights = Vec::new();
    let mut buf = [0; 50];
    for _i in 0..header.count {
        let len = r.read(&mut buf)?;
        if len < 50 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof,
                                           "Stream ended while reading light info").into());
        }
        lights.push(Light {
            id: parse_u16(buf[0], buf[1]),
            address: array_ref!(buf, 2, 8).clone(),
            bulb_type: buf[11],
            firmware_version: array_ref!(buf, 12, 4).clone(),
            online: buf[16] > 0,
            group_id: buf[17],
            status: buf[18] > 0,
            brightness: buf[19],
            temperature: parse_u16(buf[20], buf[21]),
            r: buf[22],
            g: buf[23],
            b: buf[24],
            w: buf[25],
            name: String::from_utf8(buf[26..49].iter().map(|x|*x).filter(|e|*e != 0).collect())?
        });
    }
    Ok(SystemInfo {
        lights
    })
}
