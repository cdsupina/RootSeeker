use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        settings::{WgpuFeatures, WgpuSettings},
    },
};
use bevy_asset_loader::prelude::*;
//use bevy_hanabi::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

mod assets;
mod chunks;
//mod effects;
mod hair;
mod launch;
mod level;
mod louse;
mod states;
mod ui;

const FIRE_LINE: f32 = -300.0;
const FLOOR_Y: f32 = -200.0;
const GRAVITY: f32 = -550.0;
const LOUSE_QUEUE: [louse::LouseType; 18] = [
    louse::LouseType::Basic,
    louse::LouseType::Basic,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
    louse::LouseType::Basic,
    louse::LouseType::Exploding,
    louse::LouseType::Basic,
];
const LOSE_TIME: f32 = 2.0;

fn main() {
    /*
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    */
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 854.0,
            height: 480.0,
            // mode: WindowMode::Fullscreen,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    //.add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(AudioPlugin)
    //.add_plugin(HanabiPlugin)
    .add_audio_channel::<SoundEffectsAudioChannel>()
    .add_audio_channel::<GameMusicAudioChannel>()
    .add_audio_channel::<MenuMusicAudioChannel>()
    //.insert_resource(options)
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(launch::LaunchResource {
        velocity_multiplier: 7.0,
        ..Default::default()
    })
    .insert_resource(level::LevelResource {
        louse_queue: LOUSE_QUEUE.to_vec(),
        lose_timer: Timer::from_seconds(LOSE_TIME, TimerMode::Once),
    })
    .add_event::<louse::SpawnLouseEvent>()
    .add_startup_system(setup_camera);

    app.add_state(states::AppStates::LoadingMainMenu); // start game in the main menu state
    app.add_loading_state(
        LoadingState::new(states::AppStates::LoadingGame)
            .continue_to_state(states::AppStates::Game)
            .with_collection::<assets::GameAssets>(),
    );

    app.add_loading_state(
        LoadingState::new(states::AppStates::LoadingMainMenu)
            .continue_to_state(states::AppStates::MainMenu)
            .with_collection::<assets::MenuAssets>(),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Game)
            .with_system(setup_physics.label("init"))
            .with_system(states::setup_game_system.after("init")),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Game)
            .with_system(launch::fling_louse_system.label("fling_louse"))
            .with_system(louse::spawn_louse_system.after("fling_louse"))
            .with_system(states::start_gameover_system)
            .with_system(states::start_victory_system)
            .with_system(hair::hair_system)
            .with_system(hair::check_roots_system)
            .with_system(level::lose_system)
            .with_system(louse::louse_behavior_system)
            .with_system(chunks::chunk_system)
            .with_system(ui::game_ui_system), //.with_system(effects::effect_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Game)
            .with_system(states::clean_up_game_system)
            .with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::MainMenu)
            .with_system(states::setup_main_menu_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::MainMenu)
            .with_system(states::start_instructions_system)
            .with_system(states::start_credits_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::MainMenu)
            .with_system(states::clear_state_system)
            .with_system(states::clean_up_main_menu_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::GameOver)
            .with_system(states::setup_gameover_menu_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::GameOver)
            .with_system(states::start_game_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::GameOver)
            .with_system(states::clear_state_system)
            .with_system(states::clean_up_gameover_menu_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Victory)
            .with_system(states::setup_victory_menu_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Victory)
            .with_system(states::start_game_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Victory)
            .with_system(states::clear_state_system)
            .with_system(states::clean_up_victory_menu_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Instructions)
            .with_system(states::setup_instructions_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Instructions)
            .with_system(states::start_game_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Instructions).with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Credits).with_system(states::setup_credits_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Credits)
            .with_system(states::start_main_menu_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Credits).with_system(states::clear_state_system),
    );

    app.run();
}

#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

#[derive(Resource)]
pub struct GameMusicAudioChannel;

#[derive(Resource)]
pub struct MenuMusicAudioChannel;

#[derive(Component)]
pub struct MainCamera;

// setup the main camera
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 200.0),
            projection: OrthographicProjection {
                //scale: 1.0,
                //scaling_mode: ScalingMode::FixedVertical(1.),
                ..Default::default()
            },
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
