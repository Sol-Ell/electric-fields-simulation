use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::{
    components::{Charge, MainCamera},
    constants::{ELECTRON_CHARGE, PROTON_CHARGE},
    resources::GameTextures,
    NewChargeEvent,
};

pub struct ChargePlugin;

impl Plugin for ChargePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_events);
    }
}

fn keyboard_events(
    mut mouse_ev: EventReader<MouseButtonInput>,
    mut charge_event: EventWriter<NewChargeEvent>,
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    game_textures: Res<GameTextures>,
    windows: Res<Windows>,
) {
    use bevy::input::ButtonState;

    let window = windows.get_primary().expect("Failed to get primary window");

    let mut cursor_pos: Vec3;

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());

        let ndc = (screen_pos / window_size) * 2. - Vec2::ONE;

        let (camera, camera_transform) = q_camera.get_single().unwrap();

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        cursor_pos = ndc_to_world.project_point3(ndc.extend(-1.));
        cursor_pos.z = 0.;

        for ev in mouse_ev.iter() {
            if ev.state == ButtonState::Pressed {
                let mut texture: Handle<Image> = game_textures.negative_charge.clone();
                let mut coulons = ELECTRON_CHARGE;
                match ev.button {
                    MouseButton::Right => {
                        texture = game_textures.positive_charge.clone();
                        coulons = PROTON_CHARGE;
                    }
                    MouseButton::Left => (),
                    _ => break,
                }

                commands
                    .spawn(SpriteBundle {
                        texture: texture,
                        transform: Transform {
                            translation: cursor_pos,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Charge(coulons));

                charge_event.send(NewChargeEvent);
            }
        }
    }
}
