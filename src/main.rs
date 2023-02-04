use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

mod assets;
mod launch;
mod louse;

const FIRE_LINE: f32 = -300.0;
const FLOOR_Y: f32 = -190.0;
const GRAVITY: f32 = -550.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 854.0,
            height: 480.0,
            // mode: WindowMode::Fullscreen,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(AudioPlugin)
    .add_audio_channel::<SoundEffectsAudioChannel>()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(launch::LaunchResource {
        velocity_multiplier: 7.0,
        ..Default::default()
    })
    .add_event::<louse::SpawnLouseEvent>()
    .add_startup_system(setup_camera)
    .add_startup_system(setup_physics)
    .add_startup_system(setup_level_system)
    .add_system(launch::fling_louse_system.label("fling_louse"))
    .add_system(louse::spawn_louse_system.after("fling_louse"));

    app.run();
}

#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

#[derive(Component)]
pub struct MainCamera;

// setup the main camera
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        .insert(MainCamera);
}

// setup the 2D Rapier physics
fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
    rapier_config.gravity = Vec2::new(0.0, GRAVITY);
}

// setup level of the game
pub fn setup_level_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn fire line indicator
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/dashedLine.png"),
        transform: Transform::from_translation(Vec3::new(FIRE_LINE, 0.0, -1.0)),
        ..Default::default()
    });

    // spawn the ground image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/scalp_ground.png"),
        transform: Transform {
            translation: Vec3::new(0.0, FLOOR_Y, -2.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn ground hitbox
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10000.0, 50.0))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, FLOOR_Y - 10.0, 0.0)),
        ))
        .insert(Restitution::new(0.35))
        .insert(Friction::new(0.9));
}
 