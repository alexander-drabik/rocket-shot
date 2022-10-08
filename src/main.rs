mod player;
mod components;
mod ground;

use bevy::prelude::*;
use bevy_retro_camera::RetroCameraPlugin;
use bevy_retro_camera::RetroCameraBundle;
use crate::components::WindowSize;
use crate::ground::GroundPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "rocket shot".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.65)))
        .insert_resource(WindowSize{w: 0.0, h: 0.0})
        .add_plugin(RetroCameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GroundPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .run()
}

fn setup(mut commands: Commands) {
    let height: f32 = 240.0;  // Viewport size
    let scale: f32 = 1.0;  // Viewport scaling factor
    commands.spawn_bundle(RetroCameraBundle::fixed_height(height, scale));
}

fn window_resize_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WindowSize {
        w: window.width(),
        h: window.height()
    });
}
