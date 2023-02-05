use crate::{states};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

const CHUNK_SPIN: f32 = 0.5;
const CHUNK_SCALE_MIN: f32 = 0.7;
const CHUNK_SCALE_MAX: f32 = 1.2;

// Spawn a bunch of chunks from one position, exploding outwards
pub fn spawn_chunk_explosion(
    commands: &mut Commands,
    sprite_choices: Vec<Handle<Image>>,
    position: Vec2,
    num_chunks: i32,
    hitbox_size: f32,
) {
    let mut i = 0;
    while i < num_chunks {
        spawn_chunk(
            commands,
            sprite_choices.clone(),
            position,
            Vec2::new(
                thread_rng().gen_range(-220.0..=220.0),
                thread_rng().gen_range(-40.0..=500.0),
            ),
            hitbox_size,
        );
        i = i + 1;
    }
}
// Spawn a chunk in the game world
pub fn spawn_chunk(
    commands: &mut Commands,
    sprite_choices: Vec<Handle<Image>>,
    position: Vec2,
    velocity: Vec2,
    hitbox_size: f32,
) {
    let scale: f32 = thread_rng().gen_range(CHUNK_SCALE_MIN..=CHUNK_SCALE_MAX);
    commands
        .spawn(SpriteBundle {
            texture: sprite_choices
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
            transform: Transform {
                translation: position.extend(0.0),
                scale: Vec3::new(scale, scale, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(hitbox_size, hitbox_size))
        .insert(Velocity {
            linvel: velocity,
            angvel: thread_rng().gen_range(-CHUNK_SPIN..=CHUNK_SPIN), // random spin
        })
        .insert(Restitution::new(0.0))
        .insert(states::AppStateComponent(states::AppStates::Game));
}
