use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    assets::{self, GameAssets},
    chunks, louse,
    states::{self, AppStates},
};

// these are used with a sqrt function to decide the n of mid segments
const MID_SEG_LOW: i32 = 0;
const MID_SEG_HIGH: i32 = 100;

pub fn spawn_hair(commands: &mut Commands, game_assets: &assets::GameAssets, position: Vec2) {
    let root_height = 23.0;
    let seg_collider_height = 20.0;
    let seg_position_multiplier = 60.0;
    let joint_top = 30.0;
    let joint_bottom = -30.0;
    let top_extra_pos = 82.0;
    let base_stiffness = 500.0;
    let stiffness_decay = 1.4;
    let base_radius = 14.0;
    let radius_decay = 0.65;

    let num_mid_segments = 10 - (thread_rng().gen_range(MID_SEG_LOW..=MID_SEG_HIGH) as f32).sqrt().floor() as i32;
    // create root segment
    let root_entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.hair_root_image.clone(),
            transform: Transform::from_translation(position.extend(-1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::capsule_y(root_height, base_radius))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(HairComponent {
            max_health: 500.0,
            health: 500.0,
            broken_image: game_assets.hair_root_broken_image.clone(),
        })
        .insert(RootComponent)
        .id();

    let bottom_joint_bottom = if num_mid_segments == 0 { -55.0 } else { -32.0 };
    let bottom_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 32.0))
        .local_anchor2(Vec2::new(0.0, bottom_joint_bottom))
        .motor_position(0.0, base_stiffness, 10.0);

    // use these to chain to the next midjoint
    let mut prev_entity = root_entity;
    let mut prev_joint = bottom_joint;
    let mut i = 0;
    while i < num_mid_segments {
        let radius = base_radius * (f32::powf(radius_decay, i as f32 + 1.0));

        let mid_entity = commands
            .spawn(SpriteBundle {
                texture: game_assets.hair_bottom_image.clone(),
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y + 64.0 + (i as f32) * seg_position_multiplier,
                    -1.0,
                )),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule_y(seg_collider_height, radius))
            .insert(Restitution::new(0.3))
            .insert(states::AppStateComponent(states::AppStates::Game))
            .insert(ImpulseJoint::new(prev_entity, prev_joint))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(HairComponent {
                max_health: 450.0,
                health: 450.0,
                broken_image: game_assets.hair_bottom_broken_image.clone(),
            })
            .id();

        let mid_joint_bottom = if i == num_mid_segments - 1 {
            joint_bottom - 20.0
        } else {
            joint_bottom
        };
        let stiffness = base_stiffness * (f32::powf(stiffness_decay, i as f32 + 1.0));

        let mid_joint = RevoluteJointBuilder::new()
            .local_anchor1(Vec2::new(0.0, joint_top))
            .local_anchor2(Vec2::new(0.0, mid_joint_bottom))
            .motor_position(0.0, stiffness, 10.0);

        i = i + 1;
        prev_entity = mid_entity;
        prev_joint = mid_joint;
    }

    let radius = base_radius * (f32::powf(radius_decay, i as f32 + 1.0));
    let top_entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.hair_top_image.clone(),
            transform: Transform::from_translation(Vec3::new(
                position.x,
                position.y + top_extra_pos + (i as f32) * seg_position_multiplier,
                -1.0,
            )),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(43.0, radius))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ImpulseJoint::new(prev_entity, prev_joint))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(HairComponent {
            max_health: 400.0,
            health: 400.0,
            broken_image: game_assets.hair_top_broken_image.clone(),
        })
        .id();
}

#[derive(Component)]
pub struct HairComponent {
    pub max_health: f32,
    pub health: f32,
    pub broken_image: Handle<Image>,
}

#[derive(Component)]
pub struct RootComponent;

pub fn hair_system(
    mut commands: Commands,
    mut hair_query: Query<(Entity, &mut HairComponent, &mut Handle<Image>, &Transform)>,
    louse_query: Query<(Entity, &louse::LouseComponent, &Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
    game_assets: Res<assets::GameAssets>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (hair_entity, mut hair_component, mut image, transform) in hair_query.iter_mut() {
        for (louse_entity, louse_component, louse_velocity) in louse_query.iter() {
            for event in collision_events_vec.iter() {
                match event {
                    CollisionEvent::Stopped(entity_1, entity_2, _) => {
                        if (louse_entity == *entity_1 && hair_entity == *entity_2)
                            || (louse_entity == *entity_2 && hair_entity == *entity_1)
                                && louse_velocity.linvel.length() > 5.0
                        {
                            hair_component.health -=
                                louse_component.damage * louse_velocity.linvel.length();

                            audio_channel.play(
                                game_assets
                                    .crunch_sounds
                                    .choose(&mut rand::thread_rng())
                                    .unwrap()
                                    .clone(),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        if hair_component.health / hair_component.max_health <= 0.5 {
            *image = hair_component.broken_image.clone();
        }

        if hair_component.health <= 0.0 {
            commands.entity(hair_entity).despawn();

            chunks::spawn_chunk_explosion(
                &mut commands,
                game_assets.hair_flakes.clone(),
                Vec2::new(transform.translation.x, transform.translation.y),
                10,
                0.5,
            );

            audio_channel.play(game_assets.hair_die.clone());
        }
    }
}

pub fn check_roots_system(
    root_query: Query<&RootComponent>,
    mut app_state: ResMut<State<AppStates>>,
) {
    if root_query.is_empty() {
        app_state.set(AppStates::Victory).unwrap();
    }
}
