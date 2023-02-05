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


    #[asset(path = "sprites/hair_root.png")]
    pub hair_root_image: Handle<Image>,

    #[asset(path = "sprites/hair_bottom.png")]
    pub hair_bottom_image: Handle<Image>,

    #[asset(path = "sprites/hair_top.png")]
    pub hair_top_image: Handle<Image>,

    #[asset(path = "sprites/backGround.png")]
    pub backGround: Handle<Image>,

    #[asset(path = "sprites/hairBottom.png")]
    pub hairBottom: Handle<Image>,
    
    #[asset(path = "sprites/hairMiddle.png")]
    pub hairMiddle: Handle<Image>,

    #[asset(path = "sprites/hairTop.png")]
    pub hairTop: Handle<Image>,

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

    //image assetts
    #[asset(path = "sprites/menu_title.png")]
    pub menu_title: Handle<Image>,

    #[asset(path = "sprites/button_go_default.png")]
    pub button_go_default: Handle <Image>,

    #[asset(path = "sprites/button_go_selected.png")]
    pub button_go_selected: Handle <Image>,

    #[asset(path = "sprites/menu_button_start.png")]
    pub menu_button_start: Handle <Image>,

    #[asset(path = "sprites/menu_button_start_selected.png")]
    pub menu_button_star_selected: Handle <Image>,

    #[asset(path = "sprites/menu_button_credits.png")]
    pub menu_button_credits: Handle <Image>,

    #[asset(path = "sprites/menu_button_credits_selected.png")]
    pub menu_button_credits_selected: Handle <Image>,
}
