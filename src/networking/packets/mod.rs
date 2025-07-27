use binary_util::{ByteReader, ByteWriter};

use crate::networking::packet::PacketData;

pub mod new_point;
pub mod stroke_complete;

pub trait Packet {
    fn parse(data: &mut ByteReader) -> Result<PacketData, std::io::Error>;

    fn write(&self, writer: &mut ByteWriter) -> Result<(), std::io::Error>;
}
