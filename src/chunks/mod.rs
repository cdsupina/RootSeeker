use crate::{assets, states};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

const CHUNK_HITBOX: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const CHUNK_VELOCITY: Vec2 = Vec2 { x: 0.0, y: -10.0 };
const CHUNK_SPIN: f32 = 0.5;
const CHUNK_SCALE_MIN: f32 = 0.7;
const CHUNK_SCALE_MAX: f32 = 1.2;

// Spawn a chunk in the game world
pub fn spawn_chunk(commands: &mut Commands, sprite_assets: &assets::GameAssets, position: Vec2) {
    let scale: f32 = thread_rng().gen_range(CHUNK_SCALE_MIN..=CHUNK_SCALE_MAX);
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.dandruffBig.clone(),
            transform: Transform {
                translation: position.extend(0.0),
                scale: Vec3::new(scale, scale, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(CHUNK_HITBOX.x, CHUNK_HITBOX.y))
        .insert(Velocity {
            linvel: CHUNK_VELOCITY,
            angvel: thread_rng().gen_range(-CHUNK_SPIN..=CHUNK_SPIN), // random spin
        })
        .insert(Restitution::new(0.0))
        .insert(states::AppStateComponent(states::AppStates::Game));
}
