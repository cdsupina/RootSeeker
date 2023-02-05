use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets, states};

pub fn spawn_hair(commands: &mut Commands, sprite_assets: &assets::GameAssets, position: Vec2) {
    let root_entity = commands
        .spawn(SpriteBundle {
            texture: sprite_assets.hair_root_image.clone(),
            transform: Transform::from_translation(position.extend(-1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(12.0, 25.0))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .id();

    let bottom_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 32.0))
        .local_anchor2(Vec2::new(0.0, -32.0))
        .motor_position(0.0, 800.0, 10.0);

    let bottom_entity = commands
        .spawn(SpriteBundle {
            texture: sprite_assets.hair_bottom_image.clone(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y + 64.0, -1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(12.0, 25.0))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ImpulseJoint::new(root_entity, bottom_joint))
        .id();

    let top_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 32.0))
        .local_anchor2(Vec2::new(0.0, -50.0))
        .motor_position(0.0, 130.0, 10.0);

    let top_entity = commands
        .spawn(SpriteBundle {
            texture: sprite_assets.hair_top_image.clone(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y + 82.0, -1.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 40.0))
        .insert(Restitution::new(0.3))
        .insert(states::AppStateComponent(states::AppStates::Game))
        .insert(ImpulseJoint::new(bottom_entity, top_joint))
        .id();
}
