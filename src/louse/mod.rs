use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

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
    asset_server: Res<AssetServer>,
    mut spawn_louse_events: EventReader<SpawnLouseEvent>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
) {
    for event in spawn_louse_events.iter() {
        spawn_louse(&mut commands, &asset_server, event.position, event.velocity);
        audio_channel
            .play(asset_server.load(format!("sounds/wahoo{}.mp3", thread_rng().gen_range(1..=6))));
    }
}

// Spawn a louse in the game world
pub fn spawn_louse(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    velocity: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/bugLouse.png"),
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(LOUSE_HITBOX.x, LOUSE_HITBOX.y))
        .insert(Velocity {
            linvel: velocity,
            angvel: thread_rng().gen_range(-LOUSE_SPIN..=LOUSE_SPIN), // random spin
        })
        .insert(Restitution::new(0.0));
}
