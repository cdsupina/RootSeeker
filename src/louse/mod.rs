use crate::{
    assets::{self, GameAssets},
    //effects::spawn_effect,
    states,
};
use bevy::prelude::*;
//use bevy_hanabi::EffectAsset;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

const LOUSE_HITBOX: Vec2 = Vec2 { x: 10.0, y: 10.0 };
const LOUSE_SPIN: f32 = 2.0;
const LOUSE_DESPAWN_TIME: f32 = 3.0;
const LOUSE_MIN_VEL: f32 = 5.0;

#[derive(Clone, Debug)]
pub enum LouseType {
    Basic,
    Exploding,
}

// Event data describing spawning lice
pub struct SpawnLouseEvent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub louse_type: LouseType,
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
            event.louse_type.clone(),
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
    louse_type: LouseType,
) {
    match louse_type {
        LouseType::Basic => {
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
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(LouseComponent {
                    damage: 0.5,
                    despawn_timer: Timer::from_seconds(LOUSE_DESPAWN_TIME, TimerMode::Once),
                })
                .insert(states::AppStateComponent(states::AppStates::Game));
        }
        LouseType::Exploding => {
            commands
                .spawn(SpriteBundle {
                    texture: sprite_assets.exploding_louse_image.clone(),
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
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(LouseComponent {
                    damage: 0.5,
                    despawn_timer: Timer::from_seconds(LOUSE_DESPAWN_TIME, TimerMode::Once),
                })
                .insert(states::AppStateComponent(states::AppStates::Game));
        }
    };
}

#[derive(Component)]
pub struct LouseComponent {
    pub damage: f32,
    pub despawn_timer: Timer,
}

pub fn louse_behavior_system(
    mut commands: Commands,
    mut louse_query: Query<(Entity, &mut LouseComponent, &Velocity, &Transform)>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
    //mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (louse_entity, mut louse_component, louse_vel, louse_trans) in louse_query.iter_mut() {
        if louse_vel.linvel.length() < LOUSE_MIN_VEL {
            louse_component.despawn_timer.tick(time.delta());
        } else {
            louse_component.despawn_timer.reset();
        }

        if louse_component.despawn_timer.just_finished() {
            commands.entity(louse_entity).despawn();
            audio_channel.play(game_assets.bug_squish.clone());
            /*
            spawn_effect(
                &mut commands,
                &mut effects,
                Vec2::new(louse_trans.translation.x, louse_trans.translation.y),
            );
            */
        }
    }
}
