use binary_util::ByteReader;

use crate::networking::{packet::PacketData, packets::Packet};

#[derive(Clone)]
pub struct StrokeRemovedPacket {
    pub timestamp: f64,
    pub id_random: u32,
}

impl Packet for StrokeRemovedPacket {
    fn parse(
        reader: &mut ByteReader,
    ) -> Result<crate::networking::packet::PacketData, std::io::Error> {
        let timestamp = reader.read_f64()?;
        let id_random = reader.read_u32()?;

        Ok(PacketData::StrokeRemoved(StrokeRemovedPacket {
            timestamp: timestamp,
            id_random: id_random,
        }))
    }

    fn write(&self, writer: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        writer.write_f64(self.timestamp)?;
        writer.write_u32(self.id_random)?;

        Ok(())
    }
}
