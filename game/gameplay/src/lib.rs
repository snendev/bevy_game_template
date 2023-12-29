use bevy::prelude::*;

pub use bevy_game_blueprints as blueprints;
use blueprints::{Character, CharacterMovement};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct GameplaySet;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, consume_actions.in_set(GameplaySet));

        #[cfg(debug_assertions)]
        app.register_type::<Character>()
            .register_type::<CharacterMovement>();
    }
}

fn consume_actions(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Transform, &CharacterMovement)>,
) {
    const SPEED: f32 = 150.;
    for (entity, mut transform, movement) in player_query.iter_mut() {
        let movement = movement.0.normalize();
        let delta = Vec3::new(
            movement.x * SPEED * time.delta_seconds(),
            movement.y * SPEED * time.delta_seconds(),
            0.,
        );
        transform.translation += delta;
        commands.entity(entity).remove::<CharacterMovement>();
    }
}
