use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct NetworkOwned {
    pub owner: String,
}
