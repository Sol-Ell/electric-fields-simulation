use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Arrow {
    pub direction_vec: Vec3,
}

impl Arrow {
    pub fn new() -> Self {
        Self {
            direction_vec: Vec3 { x: 0., y: 0., z: 0. },
        }
    }

    pub fn _from(direction_vec: Vec3) -> Self {
        Self { direction_vec }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Charge(pub f32);