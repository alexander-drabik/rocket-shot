use bevy::prelude::*;

use crate::App;
use crate::components::Ground;

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ground_spawn_system);
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


