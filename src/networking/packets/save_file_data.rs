use bevy::{ecs::event::Event, math::Vec2};
use binary_util::ByteReader;

use crate::{
    load_file::read_stroke,
    networking::{packet::PacketData, packets::Packet},
    player_sprite::get_player_state::PlayerStateData,
    stroke::Stroke,
};

pub struct SaveFileData {
    pub data: Vec<u8>,
}

impl Packet for SaveFileData {
    fn parse(
        reader: &mut ByteReader,
    ) -> Result<crate::networking::packet::PacketData, std::io::Error> {
        let len = reader.read_u32()?;

        let mut slice: Box<[u8]> = vec![0; len.try_into().unwrap()].into_boxed_slice();
        reader.read(&mut slice).unwrap();

        Ok(PacketData::SaveFileData(SaveFileData {
            data: slice.to_vec(),
        }))
    }

    fn write(&self, writer: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        writer.write_u32(u32::try_from(self.data.len()).unwrap());
        writer.write(&self.data);

        Ok(())
    }
}
