use bevy::{input::InputSystem as BevyInputSystem, prelude::*};

use leafwing_input_manager::plugin::InputManagerPlugin;

mod controllers;
pub use controllers::*;
mod systems;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct ControllerSet;

/// This plugin adds controllers using leafwing-input-manager that are used to attach
/// components for further processing in game ticks.
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CharacterControl>::default())
            .configure_sets(PreUpdate, ControllerSet.after(BevyInputSystem))
            .add_systems(
                PreUpdate,
                (
                    systems::sync_controllers,
                    systems::queue_inputs,
                    systems::set_mobile_actions,
                )
                    .chain()
                    .in_set(ControllerSet),
            );

        #[cfg(debug_assertions)]
        app.register_type::<Controller>()
            .register_type::<CharacterControl>();
    }
}
