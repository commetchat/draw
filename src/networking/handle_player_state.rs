use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Query,
    },
    math::{Quat, Vec2Swizzles},
};

use crate::{
    active_strokes::active_stroke::ActiveStrokeEvent,
    lerp_transform::TargetTransform,
    networking::{network_owned::NetworkOwned, packet::ReceivedPacket},
    player_sprite::get_player_state::PlayerStateData,
};

pub fn handle_player_state_system(
    mut events: EventReader<ReceivedPacket>,
    mut sprites: Query<(&NetworkOwned, &mut TargetTransform)>,
) {
    for event in events.read() {
        match &event.data {
            super::packet::PacketData::PlayerState(player_state_data) => {
                for mut sprite in sprites.iter_mut() {
                    if sprite.0.owner != event.from {
                        continue;
                    }

                    let rotation = Quat::from_rotation_z(player_state_data.rotation);

                    sprite.1.transform.translation = player_state_data.position.xyx().with_z(0.0);
                    sprite.1.transform.rotation = rotation;
                }
            }
            _ => (),
        }
    }
}
