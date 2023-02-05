use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "sprites/bugLouse.png")]
    pub basic_louse_image: Handle<Image>,

    #[asset(path = "sprites/dashedLine.png")]
    pub fire_line_image: Handle<Image>,

    #[asset(path = "sprites/scalp_ground.png")]
    pub ground_image: Handle<Image>,

    #[asset(path = "sounds/slingshot_release.mp3")]
    pub slingshot_release_sound: Handle<AudioSource>,

    #[asset(path = "sounds/slingshot_pull.mp3")]
    pub slingshot_pull_sound: Handle<AudioSource>,

    #[asset(
        paths(
            "sounds/wahoo1.mp3",
            "sounds/wahoo2.mp3",
            "sounds/wahoo3.mp3",
            "sounds/wahoo4.mp3",
            "sounds/wahoo5.mp3",
            "sounds/wahoo6.mp3"
        ),
        collection(typed)
    )]
    pub wahoo_sounds: Vec<Handle<AudioSource>>,

    #[asset(path = "sounds/ScalpInvaders.mp3")]
    pub game_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(path = "sounds/ScalpChillMusic.mp3")]
    pub menu_music: Handle<AudioSource>,

    #[asset(path = "sounds/celebration.mp3")]
    pub celebration: Handle<AudioSource>,
}
