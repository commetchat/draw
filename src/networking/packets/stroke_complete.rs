use binary_util::ByteReader;

use crate::{
    load_file::read_stroke,
    networking::{packet::PacketData, packets::Packet},
    stroke::Stroke,
};

#[derive(Clone)]
pub struct StrokeCompleteData {
    pub data: Stroke,
}

impl Packet for StrokeCompleteData {
    fn parse(
        reader: &mut ByteReader,
    ) -> Result<crate::networking::packet::PacketData, std::io::Error> {
        let result = read_stroke(reader, &None);
        match result {
            Ok(stroke) => Ok(PacketData::StrokeComplete(StrokeCompleteData {
                data: stroke,
            })),
            Err(err) => Err(err),
        }
    }

    fn write(&self, writer: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        writer.write_u32(self.data.metadata.id_random)?;
        writer.write_f64(self.data.metadata.timestamp)?;

        writer.write_f32(self.data.metadata.origin.x)?;
        writer.write_f32(self.data.metadata.origin.y)?;

        let bytes = self.data.data.write_data();
        writer.write_u32(u32::try_from(bytes.len()).unwrap())?;
        writer.write(&bytes)?;

        Ok(())
    }
}
