use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTextures {
    pub negative_charge: Handle<Image>,
    pub positive_charge: Handle<Image>,
    pub arrow: Handle<Image>,
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
