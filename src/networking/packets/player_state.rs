use bevy::{ecs::event::Event, math::Vec2};
use binary_util::ByteReader;

use crate::{
    load_file::read_stroke,
    networking::{packet::PacketData, packets::Packet},
    player_sprite::get_player_state::PlayerStateData,
    stroke::Stroke,
};

impl Packet for PlayerStateData {
    fn parse(
        reader: &mut ByteReader,
    ) -> Result<crate::networking::packet::PacketData, std::io::Error> {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let zoom = reader.read_f32()?;
        let rotation = reader.read_f32()?;

        Ok(PacketData::PlayerState(PlayerStateData {
            position: Vec2 { x: x, y: y },
            zoom: zoom,
            rotation: rotation,
        }))
    }

    fn write(&self, writer: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        writer.write_f32(self.position.x);
        writer.write_f32(self.position.y);
        writer.write_f32(self.zoom);
        writer.write_f32(self.rotation);

        Ok(())
    }
}
