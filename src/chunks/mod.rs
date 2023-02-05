use crate::{assets, louse, states};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
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
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(hitbox_size, hitbox_size))
        .insert(Velocity {
            linvel: velocity,
            angvel: thread_rng().gen_range(-CHUNK_SPIN..=CHUNK_SPIN), // random spin
        })
        .insert(Restitution::new(0.0))
        .insert(ChunkComponent)
        .insert(states::AppStateComponent(states::AppStates::Game));
}

#[derive(Component)]
pub struct ChunkComponent;

pub fn chunk_system(
    mut commands: Commands,
    mut chunk_query: Query<Entity, With<ChunkComponent>>,
    mut louse_query: Query<(Entity, &mut louse::BasicLouseComponent, &mut Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
    game_assets: Res<assets::GameAssets>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for chunk_entity in chunk_query.iter_mut() {
        for (louse_entity, louse_component, mut louse_velocity) in louse_query.iter_mut() {
            for event in collision_events_vec.iter() {
                match event {
                    CollisionEvent::Started(entity_1, entity_2, _) => {
                        if (louse_entity == *entity_1 && chunk_entity == *entity_2)
                            || (louse_entity == *entity_2 && chunk_entity == *entity_1)
                        //&& louse_velocity.linvel.length() < 15.0
                        {
                            audio_channel.play(game_assets.munch.clone());

                            commands.entity(chunk_entity).despawn();
                            louse_velocity.linvel.x = thread_rng().gen_range(
                                louse_component.jump_range_x.0..=louse_component.jump_range_x.1,
                            );
                            louse_velocity.linvel.y = thread_rng().gen_range(
                                louse_component.jump_range_y.0..=louse_component.jump_range_y.1,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
