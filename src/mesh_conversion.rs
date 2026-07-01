use bevy::{color::ColorToComponents, log::info};

use crate::{BACKGROUND, line_builder::LineBuilder, stroke::{Stroke, StrokeType}};

pub fn timestamp_to_z_offset(timestamp: f64, stroke_type: &StrokeType) -> f32 {
    const SECONDS_PER_YEAR: f64 = 31556952.0;
    const START_TIME: f64 = 1740000000.0;
    const END_TIME: f64 = START_TIME + (SECONDS_PER_YEAR * 20.0);

    let z_offset = inverse_lerp(START_TIME, END_TIME, timestamp);

    let z_offset = match stroke_type {
        crate::stroke::StrokeType::LineArt(_) => lerp(0.5, 1.0, z_offset ),
        crate::stroke::StrokeType::LegacyEraser => lerp(0.0, 0.5, z_offset ),
        crate::stroke::StrokeType::Paint(_) => lerp(0.0, 0.5, z_offset ),
    };
    return z_offset as f32;
}

fn inverse_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}

fn lerp(a: f64, b: f64, v: f64) -> f64 {
   a * (1.0 - v) + b * v
}

pub fn stroke_to_mesh(stroke: &Stroke) -> (Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<u32>) {
    let pressures = match &stroke.data.pressures {
        Some(pressures) => pressures.clone(),
        None => Vec::new(),
    };

    let mut builder = LineBuilder::new_with(stroke.data.points.clone(), pressures);

    builder.width = stroke.data.width;
    builder.default_color = match stroke.data.stroke_type {
        crate::stroke::StrokeType::LineArt(color) => color,
        crate::stroke::StrokeType::LegacyEraser => BACKGROUND,
        crate::stroke::StrokeType::Paint(color) => color,
    };


    let z_offset = timestamp_to_z_offset(stroke.metadata.timestamp, &stroke.data.stroke_type);

    let mut colors = Vec::<[f32; 4]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();

    builder.build();

    vertices.reserve(builder.vertices.len());
    colors.reserve(builder.colors.len());

    for p in &builder.vertices {
        vertices.push([
            p.x + stroke.metadata.origin.x,
            p.y + stroke.metadata.origin.y,
            0.0,
        ]);

        let mut color = builder.default_color.to_linear().to_f32_array(); // LinearRgba::from_u8_array_no_alpha().to_f32_array();

        color[3] = z_offset as f32;
        colors.push(color);
    }

    return (vertices, colors, builder.indices);
}
