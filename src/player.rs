use std::os::unix::raw::time_t;
use crate::{Bundle, Commands, SpriteSheetBundle};
use bevy::prelude::*;
use crate::components::{Player, Velocity};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_system);
    }
}

fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform {
            translation: Vec3::new(0., 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Player)
        .insert(Velocity {x: 0.0, y: 0.0});
}

fn player_keyboard_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -20.
        } else if kb.pressed(KeyCode::Right) {
            20.
        } else {
            0.
        }
    }
}

fn player_movement_system(
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        velocity.y -= 2.5;

        translation.x += velocity.x * time.delta_seconds();
        translation.y += velocity.y * time.delta_seconds();
    }
}
