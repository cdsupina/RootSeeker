use crate::{assets, states};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

const CHUNK_HITBOX: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const CHUNK_VELOCITY: Vec2 = Vec2 { x: 0.0, y: -10.0 };
const CHUNK_SPIN: f32 = 0.5;

// Event data describing spawning "chunks"
pub struct SpawnChunkEvent {
    pub position: Vec2,
    pub velocity: Vec2,
}

// Checks for chunk-spawning events and spawns the chunk
pub fn spawn_chunk_system(
    mut commands: Commands,
    sprite_assets: Res<assets::GameAssets>,
    mut spawn_chunk_events: EventReader<SpawnChunkEvent>,
) {
    for event in spawn_chunk_events.iter() {
        spawn_chunk(&mut commands, &sprite_assets, event.position);
    }
}

// Spawn a chunk in the game world
pub fn spawn_chunk(commands: &mut Commands, sprite_assets: &assets::GameAssets, position: Vec2) {
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.dandruffBig.clone(),
            transform: Transform::from_translation(position.extend(0.0)),
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
