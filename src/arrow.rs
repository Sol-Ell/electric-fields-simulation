use bevy::prelude::*;

use crate::{
    components::{Arrow, Charge},
    constants::{K_CONSTANT, MESH_HEIGHT, MESH_WIDTH},
    resources::{GameTextures, WinSize},
    NewChargeEvent,
};

pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, arrow_startup_system)
            .add_system(on_new_charge_event);
    }
}

fn arrow_startup_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    game_textures: Res<GameTextures>,
) {
    let x_scale = win_size.w / (MESH_WIDTH + 2) as f32;
    let y_scale = win_size.h / (MESH_HEIGHT + 2) as f32;

    for y in (MESH_HEIGHT as i32 / -2)..(MESH_HEIGHT as i32 / 2) {
        for x in (MESH_WIDTH as i32 / -2)..(MESH_WIDTH as i32 / 2) {
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.arrow.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * x_scale + x_scale / 2.,
                            y as f32 * y_scale + y_scale / 2.,
                            0.,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Arrow::new());
        }
    }
}

fn on_new_charge_event(
    ev_new_charge: EventReader<NewChargeEvent>,
    mut arrows: Query<(&mut Arrow, &mut Transform)>,
    charges: Query<(&Charge, &Transform), (With<Charge>, Without<Arrow>)>,
) {
    if ev_new_charge.is_empty() {
        return;
    }
    let mut direction: Vec3 = Vec3::ZERO;
    for (mut arrow, mut arrow_trs) in arrows.iter_mut() {
        arrow.direction_vec *= 0.0;
        for (charge, charge_trs) in charges.into_iter() {
            let distance = arrow_trs.translation.distance(charge_trs.translation);
            let electric_field = charge.0 / distance.powf(2.) * K_CONSTANT;
            arrow.direction_vec += (arrow_trs.translation - charge_trs.translation)
                .normalize_or_zero()
                * electric_field;
        }
        arrow_trs.rotation =
            Quat::from_rotation_z(arrow.direction_vec.y.atan2(arrow.direction_vec.x));

        direction *= 0.0;
    }
}
