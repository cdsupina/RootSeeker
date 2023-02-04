use crate::louse;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Resource, Default, Debug)]
pub struct LaunchResource {
    pub initial_position: Option<Vec2>,
    pub velocity_multiplier: f32,
}

pub fn fling_louse_system(
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mouse_button: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut spawn_louse_events: EventWriter<louse::SpawnLouseEvent>,
    mut fling_resource: ResMut<LaunchResource>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let curr_window = windows.get_primary().unwrap();
        let (camera, camera_gl_transform) = camera.single();

        let initial_position =
            get_cursor_physics_position(camera, camera_gl_transform, curr_window);

        if initial_position.x < crate::FIRE_LINE {
            fling_resource.initial_position = Some(initial_position);
            audio_channel.play(asset_server.load("sounds/slingshot_pull.mp3"));
        }
    } else if mouse_button.just_released(MouseButton::Left) {
        if let Some(initial_position) = fling_resource.initial_position {
            let curr_window = windows.get_primary().unwrap();
            let (camera, camera_gl_transform) = camera.single();

            let final_position =
                get_cursor_physics_position(camera, camera_gl_transform, curr_window);

            println!("Initial Postion: {}", initial_position);
            println!("Final Position: {}", final_position);

            let velocity = (initial_position - final_position) * fling_resource.velocity_multiplier;
            audio_channel.stop();

            audio_channel.play(asset_server.load("sounds/slingshot_release.mp3"));
            spawn_louse_events.send(louse::SpawnLouseEvent {
                position: final_position,
                velocity,
            });

            fling_resource.initial_position = None;
        }
    }
}

pub fn get_cursor_physics_position(
    camera: &Camera,
    camera_gl_transform: &GlobalTransform,
    window: &Window,
) -> Vec2 {
    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world =
            camera_gl_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    } else {
        Vec2::ZERO
    }
}
