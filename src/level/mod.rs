use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{
    louse::{LouseComponent, LouseType},
    states::AppStates,
};

#[derive(Resource, Debug)]
pub struct LevelResource {
    pub louse_queue: Vec<LouseType>,
    pub lose_timer: Timer,
}

pub fn lose_system(
    mut level_resource: ResMut<LevelResource>,
    mut app_state: ResMut<State<AppStates>>,
    louse_query: Query<&LouseComponent>,
    time: Res<Time>,
) {
    if level_resource.louse_queue.is_empty() && louse_query.is_empty() {
        level_resource.lose_timer.tick(time.delta());
        if level_resource.lose_timer.just_finished() {
            app_state.set(AppStates::GameOver).unwrap();
        }
    }
}
