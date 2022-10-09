use bevy::prelude::Component;
use crate::{Handle, Image};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}


#[derive(Default)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32
}

#[derive(Default)]
pub struct PlayerTextures {
    pub normal: Handle<Image>,
    pub shooting: Handle<Image>
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;
