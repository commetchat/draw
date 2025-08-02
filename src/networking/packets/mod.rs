use binary_util::{ByteReader, ByteWriter};

use crate::networking::packet::PacketData;

pub mod new_point;
pub mod player_state;
pub mod save_file_data;
pub mod stroke_complete;
pub mod stroke_removed;

pub trait Packet {
    fn parse(data: &mut ByteReader) -> Result<PacketData, std::io::Error>;

    fn write(&self, writer: &mut ByteWriter) -> Result<(), std::io::Error>;
}
