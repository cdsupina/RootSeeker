use std::{time::Duration, /*thread::__FastLocalKeyInner*/};

use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::{self, GameOverAssets}, hair};

// states of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    LoadingMainMenu,
    MainMenu,
    PauseMenu,
    LoadingGame,
    Game,
    GameOver,
    Victory,
}

// used for tagging entities that are part of the game state
#[derive(Component)]
pub struct AppStateComponent(pub AppStates);

// remove entities tagged for the current app state
pub fn clear_state_system(
    mut commands: Commands,
    mut despawn_entities_query: Query<(Entity, &AppStateComponent)>,
    app_state: Res<State<AppStates>>,
) {
    for (entity, entity_app_state) in despawn_entities_query.iter_mut() {
        if *app_state.current() == entity_app_state.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Start the game by entering the game loading state
pub fn start_game_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    // check for keyboard or gamepad input
    let mut start_input = keyboard_input.just_released(KeyCode::Return)
        || keyboard_input.just_released(KeyCode::Space);

    // if input read enter the game loading state
    if start_input {
        // set the state to loading game state
        app_state.set(AppStates::LoadingGame).unwrap();

        // play sound effect
        //audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Return);
        keyboard_input.reset(KeyCode::Space);
    }
}

pub fn start_gameover_system(
    mut commands: Commands,
    game_over: Res<assets::GameAssets>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    // check for keyboard or gamepad input
    let input = keyboard_input.just_released(KeyCode::L);

    // if input read enter the game loading state
    if input {
        // set the state to loading game state
        app_state.set(AppStates::GameOver).unwrap();

        // play sound effect
        //audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        commands
        .spawn(SpriteBundle {
            texture: game_over.game_over.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::GameOver));

        // reset input
        keyboard_input.reset(KeyCode::L);
    }

}

pub fn start_victory_system(
    mut commands: Commands,
    sprite_assets: Res<assets::GameAssets>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    // check for keyboard or gamepad input
    let input = keyboard_input.just_released(KeyCode::W);

    // if input read enter the game loading state
    if input {
        // set the state to loading game state
        app_state.set(AppStates::Victory).unwrap();

        // play sound effect
        //audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        commands
        .spawn(SpriteBundle {
            texture: sprite_assets.win_screen.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu));

        // reset input
        keyboard_input.reset(KeyCode::W);
    }
}

/// Quit the game if quit input read
pub fn quit_game_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // check for input
    let quit_input = keyboard_input.just_released(KeyCode::Escape);

    // quit app if input read
    if quit_input {
        app_exit_events.send(AppExit);
    }
}

// setup level of the game
pub fn setup_game_system(
    mut commands: Commands,
    sprite_assets: Res<assets::GameAssets>,
    audio_channel: Res<AudioChannel<crate::GameMusicAudioChannel>>,
    game_assets: Res<assets::GameAssets>,
) {
    audio_channel
        .play(game_assets.game_music.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(2.0),
            AudioEasing::Linear,
        ))
        .looped();

    // spawn fire line indicator
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.fire_line_image.clone(),
            transform: Transform::from_translation(Vec3::new(crate::FIRE_LINE, 0.0, -1.0)),
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game));

    // spawn the background
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.backGround.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -5.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game));

            // spawn the ground image
    commands
    .spawn(SpriteBundle {
        texture: sprite_assets.ground_image.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, crate::FLOOR_Y, -2.0),
            scale: Vec3::new(1.5, 1.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(AppStateComponent(AppStates::Game));

    // spawn ground hitbox
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10000.0, 50.0))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, crate::FLOOR_Y, 0.0)),
        ))
        .insert(Restitution::new(0.35))
        .insert(Friction::new(0.9))
        .insert(AppStateComponent(AppStates::Game));

    // spawn hairs
    hair::spawn_hair(&mut commands, &sprite_assets, Vec2::new(0.0, -130.0));
    hair::spawn_hair(&mut commands, &sprite_assets, Vec2::new(350.0, -130.0));
    hair::spawn_hair(&mut commands, &sprite_assets, Vec2::new(170.0, -130.0));
    hair::spawn_hair(&mut commands, &sprite_assets, Vec2::new(-190.0, -130.0));
}

// setup level of the game dh: Main Men
pub fn setup_main_menu_system(
    mut commands: Commands,
    menu_music_audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
    menu_assets: Res<assets::MenuAssets>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    let mut test = -50.0;

    let mut key_test: bool = keyboard_input.just_released(KeyCode::D);

    if key_test {
        test = 0.0;
    } else {
        test = -50.0;
    }
  

    menu_music_audio_channel
        .play(menu_assets.menu_music.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(2.0),
            AudioEasing::Linear,
        ))
        .looped();

        commands
        .spawn(SpriteBundle {
            texture: menu_assets.menu_title.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu));

        commands
        .spawn(SpriteBundle {
            texture: menu_assets.menu_button_start.clone(),
            transform: Transform {
                translation: Vec3::new(-150.0, -50.0, 0.2),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu));

        commands
        .spawn(SpriteBundle {
            texture: menu_assets.menu_button_start_selected.clone(),
            transform: Transform {
                translation: Vec3::new(-150.0, test, 0.1),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu));

        commands
        .spawn(SpriteBundle {
            texture: menu_assets.menu_button_credits.clone(),
            transform: Transform {
                translation: Vec3::new(-150.0, -125.0, 0.1),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu));
}

pub fn clean_up_main_menu_system(audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>) {
    audio_channel.stop();
}

pub fn clean_up_game_system(audio_channel: Res<AudioChannel<crate::GameMusicAudioChannel>>) {
    audio_channel.stop();
}

// setup level of the game
pub fn setup_victory_menu_system(
    sound_effects_audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,

    menu_music_audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
    menu_assets: Res<assets::MenuAssets>,
) {
    sound_effects_audio_channel.play(menu_assets.celebration.clone());

    menu_music_audio_channel
        .play(menu_assets.menu_music.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(2.0),
            AudioEasing::Linear,
        ))
        .looped();
}

pub fn clean_up_victory_menu_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
) {
    audio_channel.stop();
}

// setup level of the game
pub fn setup_gameover_menu_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
    menu_assets: Res<assets::MenuAssets>,
) {
    audio_channel
        .play(menu_assets.menu_music.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(2.0),
            AudioEasing::Linear,
        ))
        .looped();
}

pub fn clean_up_gameover_menu_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
) {
    audio_channel.stop();
}
