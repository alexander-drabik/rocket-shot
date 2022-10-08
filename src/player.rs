use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::utils::tracing::instrument::WithDispatch;
use crate::components::{Ground, Player, Velocity};

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
    mut ground_query: Query<(&mut Transform), (With<Ground>, Without<Player>)>,
    time: Res<Time>
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        velocity.y -= 2.5;

        for transform2 in ground_query.iter() {
            match collide(
                transform2.translation.xyz(), Vec2 {x: 32.0, y: 64.0},
                translation.xyz(), Vec2 {x: 32.0, y: 32.0} ) {
                Some(collision) => {
                    match collision {
                        Collision::Bottom => {
                            velocity.y = 50.0;
                        }
                        Collision::Right | Collision::Left => {
                            velocity.x = 0.0;
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }

        translation.y += velocity.y * time.delta_seconds();
        for mut transform2 in ground_query.iter_mut() {
            transform2.translation.x += -velocity.x * time.delta_seconds();
        }
    }
}
