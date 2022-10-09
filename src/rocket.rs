use std::f32::MAX;
use std::io::Cursor;
use bevy::ecs::entity::Entities;
use bevy::input::mouse::MouseMotion;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{App, DEGREE, Shooting, WindowSize};
use crate::components::{Ground, Player, Rocket, Velocity};

pub struct RocketPlugin;
impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(Shooting::Shooting).with_system(rocket_spawn_system))
            .add_system(rocket_movement_system);
    }
}

fn rocket_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<Shooting>>,
    query: Query<&Transform, With<Player>>,
    windows: Res<Windows>
) {
    let mut y = 0.0;
    if let Ok(player) = query.get_single() {
        y = player.translation.y;
    } else {
        y = 0.0;
    };


    let window = windows.get_primary().unwrap();
    let mut vy = 0.0;
    let mut vx = 0.0;
    if let Some(position) = window.cursor_position() {
        let x = window.width()/2.0;
        let y = window.height()/2.0;
        let mut rot = (position.y - y).atan2(position.x - x);
        println!("{} {} {} {}", x, y, position.x, position.y);
        vx = rot.cos()*10.0;
        vy = rot.sin()*10.0;
    }

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("rocket.png"),
        transform: Transform {
            translation: Vec3::new(0., y-5.0, 1.0),
            rotation: Quat::from_rotation_z(135.0*DEGREE),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Rocket)
        .insert(Velocity {x: vx, y: vy});
    state.set(Shooting::NotShooting).unwrap();
}

fn rocket_movement_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity, &Velocity), With<Rocket>>,
    mut ground_query: Query<(&mut Transform), (With<Ground>, Without<Rocket>)>,
    mut window_size: Res<WindowSize>
) {
    'outer: for (mut transform, entity, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation = Quat::from_rotation_z(
            velocity.y.atan2(velocity.x) + 135.0*DEGREE
        );


        for transform2 in ground_query.iter() {
            match collide(
                transform2.translation.xyz(), Vec2 {x: 32.0, y: 128.0},
                transform.translation.xyz(), Vec2 {x: 5.0, y: 5.0}
            ) {
                Some(collision) => {
                    commands.entity(entity).despawn();
                    break 'outer;
                }
                None => {}
            }
        }

        let x = window_size.h/240.0;
        let ratio_width = x * 320.0;
        let added_with = (window_size.w-ratio_width)/2.0;
        if transform.translation.x < 320.0/-2.0-added_with/x {
            commands.entity(entity).despawn();
        }
    }

}
