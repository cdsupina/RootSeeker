use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    level::LevelResource,
    states::{AppStateComponent, AppStates},
};

#[derive(Component)]
pub struct LouseUI;

pub fn game_ui_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    level_resource: Res<LevelResource>,
    louse_ui_query: Query<Entity, With<LouseUI>>,
) {
    for entity in louse_ui_query.iter() {
        commands.entity(entity).despawn();
    }

    for (i, louse_type) in level_resource.louse_queue.iter().enumerate() {
        commands
            .spawn(ImageBundle {
                image: match louse_type {
                    crate::louse::LouseType::Basic => game_assets.basic_louse_image.clone().into(),
                    crate::louse::LouseType::Exploding => {
                        game_assets.exploding_louse_image.clone().into()
                    }
                },
                style: Style {
                    size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                    position: UiRect {
                        left: Val::Px(30.0 + i as f32 * 30.0),
                        bottom: Val::Percent(5.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.8).into(),
                ..Default::default()
            })
            .insert(AppStateComponent(AppStates::Game))
            .insert(LouseUI);
    }
}
