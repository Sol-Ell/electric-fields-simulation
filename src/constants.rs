use std::f32::consts::PI;

pub const MESH_WIDTH: u32 = 32;
pub const MESH_HEIGHT: u32 = 32;

pub const EPSILON_O: f32 = 8.854187812e-12;
pub const K_CONSTANT: f32 = 1. / (4. * PI * EPSILON_O);

pub const PROTON_CHARGE: f32 = 1.60217663e-19;
pub const ELECTRON_CHARGE: f32 = -PROTON_CHARGE;