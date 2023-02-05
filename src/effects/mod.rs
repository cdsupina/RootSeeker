use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct EffectTimer {
    pub timer: Timer,
}

pub fn effect_system(
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut EffectTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in effect_query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_effect(commands: &mut Commands, effects: &mut Assets<EffectAsset>, pos: Vec2) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 0.0, 0.0));

    // Create a new effect asset spawning 30 particles per second from a circle
    // and slowly fading from blue-ish to transparent over their lifetime.
    // By default the asset spawns the particles at Z=0.
    let spawner = Spawner::rate(60.0.into());
    let effect = effects.add(
        EffectAsset {
            name: "Effect".into(),
            capacity: 4096,
            spawner,
            ..Default::default()
        }
        .init(PositionCircleModifier {
            radius: 5.0,
            speed: 50.0.into(),
            dimension: ShapeDimension::Surface,
            ..Default::default()
        })
        .init(ParticleLifetimeModifier { lifetime: 0.5 })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(5.0)),
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn(ParticleEffectBundle::new(effect).with_spawner(spawner))
        .insert(EffectTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .insert(Transform::from_xyz(pos.x, pos.y, 5.0))
        .insert(Name::new("effect"));

    effects.clear();
}
