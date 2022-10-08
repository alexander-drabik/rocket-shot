use bevy::ecs::entity::Entities;
use bevy::prelude::*;

use crate::{App, WindowSize};
use crate::components::Ground;

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, ground_spawn_system)
            .add_system(ground_movement_system);
    }
}

fn ground_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("ground.png"),
        transform: Transform {
            translation: Vec3::new(0., -120.0+64.0/2.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Ground);
}

fn ground_movement_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Ground>>,
    mut window_size: Res<WindowSize>
) {
    for (mut transform, entity) in query.iter_mut() {
        let x = window_size.h/240.0;
        let ratio_width = x * 320.0;
        let added_with = (window_size.w-ratio_width)/2.0;
        if transform.translation.x < 320.0/-2.0-added_with/x {
            commands.entity(entity).despawn();
        }
    }
}
