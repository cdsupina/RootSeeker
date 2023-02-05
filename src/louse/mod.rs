use crate::{assets, states};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

const LOUSE_HITBOX: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const LOUSE_SPIN: f32 = 2.0;

// Event data describing spawning lice
pub struct SpawnLouseEvent {
    pub position: Vec2,
    pub velocity: Vec2,
}

// Checks for lice spawning events and spawns the lice
pub fn spawn_louse_system(
    mut commands: Commands,
    sprite_assets: Res<assets::GameAssets>,
    mut spawn_louse_events: EventReader<SpawnLouseEvent>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
    game_assets: Res<assets::GameAssets>,
) {
    for event in spawn_louse_events.iter() {
        spawn_louse(
            &mut commands,
            &sprite_assets,
            event.position,
            event.velocity,
        );
        audio_channel.play(
            game_assets
                .wahoo_sounds
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
        );
    }
}

// Spawn a louse in the game world
pub fn spawn_louse(
    commands: &mut Commands,
    sprite_assets: &assets::GameAssets,
    position: Vec2,
    velocity: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.basic_louse_image.clone(),
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(LOUSE_HITBOX.x, LOUSE_HITBOX.y))
        .insert(Velocity {
            linvel: velocity,
            angvel: thread_rng().gen_range(-LOUSE_SPIN..=LOUSE_SPIN), // random spin
        })
        .insert(Restitution::new(0.0))
        .insert(states::AppStateComponent(states::AppStates::Game));
}
