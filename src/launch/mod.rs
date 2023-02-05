use crate::{assets, level::LevelResource, louse};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct LaunchResource {
    pub initial_position: Option<Vec2>,
    pub velocity_multiplier: f32,
}

pub fn fling_louse_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mouse_button: Res<Input<MouseButton>>,
    mut spawn_louse_events: EventWriter<louse::SpawnLouseEvent>,
    mut fling_resource: ResMut<LaunchResource>,
    mut level_resource: ResMut<LevelResource>,
    audio_channel: Res<AudioChannel<crate::SoundEffectsAudioChannel>>,
    game_assets: Res<assets::GameAssets>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let curr_window = windows.get_primary().unwrap();
        let (camera, camera_gl_transform) = camera.single();

        let initial_position =
            get_cursor_physics_position(camera, camera_gl_transform, curr_window);

        if let Some(initial_pos_val) = initial_position {
            if initial_pos_val.x < crate::FIRE_LINE && !level_resource.louse_queue.is_empty() {
                fling_resource.initial_position = initial_position;
                audio_channel.play(game_assets.slingshot_pull_sound.clone());
            } else {
                audio_channel.play(game_assets.rrnt.clone());
            }
        }
    } else if mouse_button.just_released(MouseButton::Left) {
        if let Some(initial_position) = fling_resource.initial_position {
            let curr_window = windows.get_primary().unwrap();
            let (camera, camera_gl_transform) = camera.single();

            let final_position =
                get_cursor_physics_position(camera, camera_gl_transform, curr_window);

            if let Some(final_pos_val) = final_position {
                if final_pos_val.x < crate::FIRE_LINE {
                    let velocity =
                        (initial_position - final_pos_val) * fling_resource.velocity_multiplier;
                    audio_channel.stop();

                    audio_channel.play(game_assets.slingshot_release_sound.clone());

                    let louse_type = level_resource.louse_queue.pop();

                    if let Some(louse_type) = louse_type {
                        spawn_louse_events.send(louse::SpawnLouseEvent {
                            position: final_pos_val,
                            velocity,
                            louse_type,
                        });
                    } else {
                        // TODO: Lose the game
                    }
                } else {
                    audio_channel.stop();
                    audio_channel.play(game_assets.rrnt.clone());
                }
            } else {
                audio_channel.play(game_assets.rrnt.clone());
            }
        }
        fling_resource.initial_position = None;
    }
}

pub fn get_cursor_physics_position(
    camera: &Camera,
    camera_gl_transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world =
            camera_gl_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        Some(world_pos)
    } else {
        None
    }
}
