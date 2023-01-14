//! Shows how to render simple primitive shapes with a single color.

mod arrow;
mod charge;
mod components;
mod constants;
mod events;
mod resources;

use arrow::ArrowPlugin;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use charge::ChargePlugin;
use components::MainCamera;
use events::NewChargeEvent;
use resources::{GameTextures, WinSize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NewChargeEvent>()
        .add_plugin(ArrowPlugin)
        .add_plugin(ChargePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::CRIMSON),
            },
            ..default()
        })
        .insert(MainCamera);

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    commands.insert_resource(GameTextures {
        negative_charge: asset_server.load("textures/negative-charge.png"),
        positive_charge: asset_server.load("textures/positive-charge.png"),
        arrow: asset_server.load("textures/arrow.png"),
    });
}
