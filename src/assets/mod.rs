use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "sprites/bugLouse.png")]
    pub basic_louse_image: Handle<Image>,

    #[asset(path = "sprites/skibug.png")]
    pub exploding_louse_image: Handle<Image>,

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

    #[asset(path = "sprites/hair_top_broken.png")]
    pub hair_top_broken_image: Handle<Image>,

    #[asset(path = "sprites/hair_bottom_broken.png")]
    pub hair_bottom_broken_image: Handle<Image>,

    #[asset(path = "sprites/hair_root_broken.png")]
    pub hair_root_broken_image: Handle<Image>,

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

    #[asset(
        paths("sounds/crunch1.mp3", "sounds/crunch2.mp3", "sounds/crunch3.mp3",),
        collection(typed)
    )]
    pub crunch_sounds: Vec<Handle<AudioSource>>,

    #[asset(paths("sprites/dandruff_big.png",), collection(typed))]
    pub dandruff_big_images: Vec<Handle<Image>>,

    #[asset(
        paths(
            "sprites/dandruff_small1.png",
            "sprites/dandruff_small2.png",
            "sprites/dandruff_small3.png",
            "sprites/dandruff_small4.png",
        ),
        collection(typed)
    )]
    pub dandruff_small_images: Vec<Handle<Image>>,

    #[asset(
        paths(
            "sprites/hair_flake1.png",
            "sprites/hair_flake2.png",
            "sprites/hair_flake3.png",
            "sprites/hair_flake4.png",
        ),
        collection(typed)
    )]
    pub hair_flakes: Vec<Handle<Image>>,

    #[asset(path = "sounds/ScalpInvaders.mp3")]
    pub game_music: Handle<AudioSource>,

    #[asset(path = "sounds/hair_die.mp3")]
    pub hair_die: Handle<AudioSource>,

    #[asset(path = "sounds/RRNT.mp3")]
    pub rrnt: Handle<AudioSource>,

    #[asset(path = "sounds/bug_squish.mp3")]
    pub bug_squish: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(path = "sounds/ScalpChillMusic.mp3")]
    pub menu_music: Handle<AudioSource>,

    #[asset(path = "sounds/celebration.mp3")]
    pub celebration: Handle<AudioSource>,
}
