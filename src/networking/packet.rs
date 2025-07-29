use std::io::ErrorKind;

use bevy::{ecs::event::Event, log::info};
use binary_util::{ByteReader, ByteWriter};

use crate::{
    active_strokes::active_stroke::RemoveStrokeEvent,
    load_file::read_stroke,
    networking::packets::{
        Packet, new_point::NewPointPacketData, stroke_complete::StrokeCompleteData,
        stroke_removed::StrokeRemovedPacket,
    },
    stroke::{Stroke, StrokeData},
};

use crate::networking::packets::stroke_complete;

#[repr(u8)]
pub enum PacketType {
    StrokeComplete = 1,
    NewPoint = 2,
    StrokeRemoved = 3,
}

impl TryFrom<u16> for PacketType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == PacketType::StrokeComplete as u16 => Ok(PacketType::StrokeComplete),
            x if x == PacketType::NewPoint as u16 => Ok(PacketType::NewPoint),
            x if x == PacketType::StrokeRemoved as u16 => Ok(PacketType::StrokeRemoved),
            _ => Err(()),
        }
    }
}

pub enum PacketData {
    StrokeComplete(StrokeCompleteData),
    NewPoint(NewPointPacketData),
    StrokeRemoved(StrokeRemovedPacket),
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
            info!(
                "Unknown Packet type: {} (Did you forget to implement this type in PacketType::try_from)",
                packet_type
            );

            return Err(std::io::Error::new(
                ErrorKind::Other,
                "Unknown packet format",
            ));
        }
    };

    let packet = match packet_type {
        PacketType::StrokeComplete => StrokeCompleteData::parse(&mut reader),
        PacketType::NewPoint => NewPointPacketData::parse(&mut reader),
        PacketType::StrokeRemoved => StrokeRemovedPacket::parse(&mut reader),
    };

    match packet {
        Ok(packet) => return Ok(packet),
        Err(err) => return Err(err),
    };
}

pub fn write_packet(writer: &mut ByteWriter, packet: &PacketData) -> Result<(), std::io::Error> {
    let packet_type = match packet {
        PacketData::StrokeComplete(_) => PacketType::StrokeComplete,
        PacketData::NewPoint(_) => PacketType::NewPoint,
        PacketData::StrokeRemoved(stroke_removed_packet) => PacketType::StrokeRemoved,
    };

    writer.write_u16(packet_type as u16)?;

    match packet {
        PacketData::StrokeComplete(stroke_complete_data) => stroke_complete_data.write(writer),
        PacketData::NewPoint(new_point_packet_data) => new_point_packet_data.write(writer),
        PacketData::StrokeRemoved(stroke_removed_packet) => stroke_removed_packet.write(writer),
    }
}
