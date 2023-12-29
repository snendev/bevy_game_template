use bevy::prelude::*;

use leafwing_input_manager::prelude::ActionState;

use bevy_game_blueprints::*;

use crate::{CharacterControl, Controller, ControllerBundle};

pub(crate) fn sync_controllers(
    mut commands: Commands,
    player_query: Query<(Entity, &Controller), Changed<Controller>>,
) {
    for (entity, controller) in player_query.iter() {
        debug!(
            "Attaching controller {:?} to entity ({:?})",
            controller, entity,
        );
        let mut builder = commands.entity(entity);
        match controller {
            Controller::One => {
                builder
                    .remove::<ControllerBundle>()
                    .insert(ControllerBundle::key_controller_one());
            }
            Controller::Two => {
                builder
                    .remove::<ControllerBundle>()
                    .insert(ControllerBundle::key_controller_two());
            }
        };
    }
}

pub(crate) fn queue_inputs(
    mut commands: Commands,
    actions_query: Query<(Entity, &ActionState<CharacterControl>)>,
) {
    for (entity, action_state) in actions_query.iter() {
        let mut movement = Vec2::ZERO;
        for control in action_state.get_pressed() {
            movement += CharacterMovement::from(&control).0;
        }
        if movement != Vec2::ZERO {
            commands.entity(entity).insert(CharacterMovement(movement));
        }
    }
}

pub const FOLLOW_EPSILON: f32 = 5.;

// TODO: does leafwing have an API for this?
pub(crate) fn set_mobile_actions(
    touch_input: Res<Touches>,
    mut actions: Query<(&Transform, &mut CharacterMovement), With<Character>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    for (transform, mut movement) in actions.iter_mut() {
        if let Some(touch_position) = touch_input.first_pressed_position() {
            // TODO: does this need to be generalized for use with the editor?
            let (camera, camera_transform) = camera.single();
            if let Some(touch_position) =
                camera.viewport_to_world_2d(camera_transform, touch_position)
            {
                let diff = touch_position - transform.translation.xy();
                if diff.length() > FOLLOW_EPSILON {
                    movement.0 = diff;
                }
            }
        }
    }
}
