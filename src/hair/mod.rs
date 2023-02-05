use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    assets::{self, GameAssets},
    louse,
    states::{self, AppStates},
};

pub fn spawn_hair(commands: &mut Commands, game_assets: &assets::GameAssets, position: Vec2) {
    let root_entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.hair_root_image.clone(),
            transform: Transform::from_translation(position.extend(-1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(12.0, 25.0))
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

    let bottom_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 32.0))
        .local_anchor2(Vec2::new(0.0, -32.0))
        .motor_position(0.0, 800.0, 10.0);

    let bottom_entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.hair_bottom_image.clone(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y + 64.0, -1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(12.0, 25.0))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ImpulseJoint::new(root_entity, bottom_joint))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(HairComponent {
            max_health: 450.0,
            health: 450.0,
            broken_image: game_assets.hair_bottom_broken_image.clone(),
        })
        .id();

    let top_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 32.0))
        .local_anchor2(Vec2::new(0.0, -50.0))
        .motor_position(0.0, 130.0, 10.0);

    let top_entity = commands
        .spawn(SpriteBundle {
            texture: game_assets.hair_top_image.clone(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y + 82.0, -1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 40.0))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ImpulseJoint::new(bottom_entity, top_joint))
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
    mut hair_query: Query<(Entity, &mut HairComponent, &mut Handle<Image>)>,
    louse_query: Query<(Entity, &louse::LouseComponent, &Velocity)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (hair_entity, mut hair_component, mut image) in hair_query.iter_mut() {
        for (louse_entity, louse_component, louse_velocity) in louse_query.iter() {
            for event in collision_events_vec.iter() {
                match event {
                    CollisionEvent::Stopped(entity_1, entity_2, _) => {
                        if (louse_entity == *entity_1 && hair_entity == *entity_2)
                            || (louse_entity == *entity_2 && hair_entity == *entity_1)
                                && louse_velocity.linvel.length() > 5.0
                        {
                            hair_component.health -=
                                louse_component.damage * louse_velocity.linvel.length()
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
        }
    }
}

pub fn check_roots_system(
    root_query: Query<&RootComponent>,
    mut app_state: ResMut<State<AppStates>>,
) {
    if let Result::Err(_) = root_query.get_single() {
        app_state.set(AppStates::Victory).unwrap();
    }
}
