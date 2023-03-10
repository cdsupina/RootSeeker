use std::time::Duration;

use bevy::{app::AppExit, log::Level, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    assets::{self, GameOverAssets},
    hair,
};
//use bevy_hanabi::prelude::*;

use crate::{
    chunks,
    //effects::EffectTimer,
    level::{self, LevelResource},
};

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
    Credits,
    Instructions,
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
    mut level_resource: ResMut<level::LevelResource>,
) {
    // Create a color gradient for the particles

    /*
    commands
        .spawn(ParticleEffectBundle {
            // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
            effect: ParticleEffect::new(effect).with_z_layer_2d(Some(5.0)),
            ..default()
        })
        .insert(Name::new("effect:2d"));
    */

    // reset level resource
    *level_resource = LevelResource {
        louse_queue: crate::LOUSE_QUEUE.to_vec(),
        lose_timer: Timer::from_seconds(crate::LOSE_TIME, TimerMode::Once),
    };

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

    // spawn the prompt/instructions
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.prompt.clone(),
            transform: Transform {
                translation: Vec3::new(-370.0, 180.0, -2.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game));

    // spawn the background hair(s)
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.bg_hair.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, crate::FLOOR_Y + 90.0, -3.0),
                scale: Vec3::new(1.2, 1.2, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game));
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.bg_hair2.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, crate::FLOOR_Y + 190.0, -3.1),
                scale: Vec3::new(1.1, 1.1, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game));
    commands
        .spawn(SpriteBundle {
            texture: sprite_assets.bg_hair3.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, crate::FLOOR_Y + 190.0, -3.2),
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

    // spawn dandruff chunks
    let num_chunks: i32 = 15;
    let mut i = 0;
    while i < num_chunks {
        chunks::spawn_chunk(
            &mut commands,
            sprite_assets.dandruff_big_images.clone(),
            Vec2::new(
                thread_rng().gen_range(-300.0..=400.0),
                thread_rng().gen_range(-100.0..=300.0),
            ),
            Vec2::new(0.0, -10.0),
            10.0,
        );
        i = i + 1;
    }
}

pub fn start_instructions_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    let input = keyboard_input.just_released(KeyCode::Return)
        || keyboard_input.just_released(KeyCode::Space);

    if input {
        app_state.set(AppStates::Instructions).unwrap();
        keyboard_input.reset(KeyCode::Return);
        keyboard_input.reset(KeyCode::Space);
    }
}

pub fn start_credits_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    let input = keyboard_input.just_released(KeyCode::C);

    if input {
        app_state.set(AppStates::Credits).unwrap();
        keyboard_input.reset(KeyCode::C);
    }
}

pub fn start_main_menu_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    let input = keyboard_input.just_released(KeyCode::M);

    if input {
        app_state.set(AppStates::MainMenu).unwrap();
        keyboard_input.reset(KeyCode::M);
    }
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
}

pub fn clean_up_main_menu_system(audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>) {
    audio_channel.stop();
}

pub fn clean_up_game_system(audio_channel: Res<AudioChannel<crate::GameMusicAudioChannel>>) {
    audio_channel.stop();
}

// setup level of the game
pub fn setup_victory_menu_system(
    mut commands: Commands,
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

    commands
        .spawn(SpriteBundle {
            texture: menu_assets.win_screen.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Victory));
}

pub fn clean_up_victory_menu_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
) {
    audio_channel.stop();
}

// setup level of the game
pub fn setup_gameover_menu_system(
    mut commands: Commands,
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

    commands
        .spawn(SpriteBundle {
            texture: menu_assets.game_over.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::GameOver));
}

pub fn clean_up_gameover_menu_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
) {
    audio_channel.stop();
}

// setup level of the game
pub fn setup_instructions_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
    menu_assets: Res<assets::MenuAssets>,
    mut commands: Commands,
) {
    commands
        .spawn(SpriteBundle {
            texture: menu_assets.instructions_screen.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Instructions));
}

// setup level of the game
pub fn setup_credits_system(
    audio_channel: Res<AudioChannel<crate::MenuMusicAudioChannel>>,
    menu_assets: Res<assets::MenuAssets>,
    mut commands: Commands,
) {
    commands
        .spawn(SpriteBundle {
            texture: menu_assets.credits_screen.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Credits));
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_dance_lice(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn setup_dance_lice(
    mut commands: Commands,
    menu_assets: Res<assets::MenuAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = menu_assets.dance_sheet.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(151.0, 155.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(-225.0, -25.0, 0.0),
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(AppStateComponent(AppStates::Victory));
}
