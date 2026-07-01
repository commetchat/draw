use bevy::{
    color::{Color, ColorToPacked, Srgba},
    math::Vec2,
};
use binary_util::ByteReader;

use crate::{
    active_strokes::active_stroke::NewPointData, networking::{packet::PacketData, packets::Packet}, stroke::{StrokeSource, StrokeType},
};

#[derive(Clone)]
pub struct NewPointPacketData {
    pub data: NewPointData,
}

impl Packet for NewPointPacketData {
    fn parse(
        reader: &mut ByteReader,
    ) -> Result<crate::networking::packet::PacketData, std::io::Error> {
        let id_random = reader.read_u32()?;
        let timestamp = reader.read_f64()?;

        let origin_x = reader.read_f32()?;
        let origin_y = reader.read_f32()?;

        let stroke_type = reader.read_u8()?;

        let stroke_type = match stroke_type {
            0 => {
                let r = reader.read_u8()?;
                let g = reader.read_u8()?;
                let b = reader.read_u8()?;
                let col = Color::Srgba(Srgba::from_u8_array_no_alpha([r, g, b]));
                StrokeType::LineArt(col)
            }
            1 => {
                StrokeType::LegacyEraser
            }
            2 => {
                let r = reader.read_u8()?;
                let g = reader.read_u8()?;
                let b = reader.read_u8()?;
                let col = Color::Srgba(Srgba::from_u8_array_no_alpha([r, g, b]));
                StrokeType::Paint(col)
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid stroke type",
                ));
            }
        };

        let point_x = reader.read_f32()?;
        let point_y = reader.read_f32()?;

        let width = reader.read_f32()?;
        let pressure = reader.read_f32()?;

        Ok(PacketData::NewPoint(NewPointPacketData {
            data: NewPointData {
                timestamp: timestamp,
                id_random: id_random,
                stroke_origin: Vec2 {
                    x: origin_x,
                    y: origin_y,
                },
                stroke_type: stroke_type,
                point: Vec2 {
                    x: point_x,
                    y: point_y,
                },
                width: width,
                pressure: pressure,
                owner: None,
                source: StrokeSource::Remote,
            },
        }))
    }

    fn write(&self, writer: &mut binary_util::ByteWriter) -> Result<(), std::io::Error> {
        writer.write_u32(self.data.id_random)?;
        writer.write_f64(self.data.timestamp)?;

        writer.write_f32(self.data.stroke_origin.x)?;
        writer.write_f32(self.data.stroke_origin.y)?;

        match self.data.stroke_type {
            crate::stroke::StrokeType::LineArt(color) => {
                writer.write_u8(0);

                let color = color.to_srgba().to_u8_array_no_alpha();
                writer.write(&color)?;
            }
            crate::stroke::StrokeType::LegacyEraser => {
                writer.write_u8(1);
            }
            crate::stroke::StrokeType::Paint(color) => {
                writer.write_u8(2);

                let color = color.to_srgba().to_u8_array_no_alpha();
                writer.write(&color)?;
            }
        }

        writer.write_f32(self.data.point.x)?;
        writer.write_f32(self.data.point.y)?;

        writer.write_f32(self.data.width)?;
        writer.write_f32(self.data.pressure)?;

        Ok(())
    }
}
