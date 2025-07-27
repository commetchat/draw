use std::io::ErrorKind;

use bevy::ecs::event::Event;
use binary_util::ByteReader;

use crate::{
    load_file::read_stroke,
    stroke::{Stroke, StrokeData},
};

#[repr(u8)]
pub enum PacketType {
    ReceivedStrokeComplete = 1,
}

impl TryFrom<u16> for PacketType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == PacketType::ReceivedStrokeComplete as u16 => {
                Ok(PacketType::ReceivedStrokeComplete)
            }
            _ => Err(()),
        }
    }
}

pub enum PacketData {
    StrokeComplete(Stroke),
}

#[derive(Event)]
pub struct ReceivedPacket {
    pub from: String,
    pub data: PacketData,
}

pub fn parse_packet(data: Vec<u8>) -> Result<PacketData, std::io::Error> {
    let mut reader = ByteReader::from(data);

    let packet_type = reader.read_u16()?;

    let packet_type = match PacketType::try_from(packet_type) {
        Ok(packet_type) => packet_type,
        Err(_) => {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "Unknown packet format",
            ));
        }
    };

    let packet = match packet_type {
        PacketType::ReceivedStrokeComplete => match read_stroke(&mut reader, &None) {
            Ok(mut stroke) => PacketData::StrokeComplete(stroke),
            Err(err) => return Err(err),
        },
    };

    return Ok(packet);
}
