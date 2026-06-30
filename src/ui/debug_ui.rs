use bevy::ecs::system::SystemParam;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use iyes_perf_ui::entry::PerfUiEntry;
use iyes_perf_ui::prelude::*;

use std::time::Duration;

use crate::active_strokes::active_stroke::ActiveStroke;

#[derive(Resource, Default)]
pub struct PendingStrokes {
    num_strokes: usize,
}

#[derive(Component, Default)]
pub struct PerfUiPendingStrokes;

impl PerfUiEntry for PerfUiPendingStrokes {
    type Value = u64;
    type SystemParam = (SRes<Time>, SRes<PendingStrokes>);

    fn label(&self) -> &str {
        "Pending Strokes"
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        if *value > 5u64 {
            return Some(Color::linear_rgb(1.0, 0.6, 0.0));
        }

        if *value > 10u64 {
            return Some(Color::linear_rgb(1.0, 0.0, 0.0));
        }

        return Some(Color::linear_rgb(0.0, 1.0, 0.0));
    }

    fn sort_key(&self) -> i32 {
        10000000
    }

    fn update_value(
        &self,
        (time, data): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        Some(u64::try_from(data.num_strokes).unwrap())
    }
}

pub fn update_pending_stroke_count(mut data: ResMut<PendingStrokes>, query: Query<&ActiveStroke>) {
    data.num_strokes = query.iter().count();
}
