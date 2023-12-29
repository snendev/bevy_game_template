use bevy::prelude::*;

use leafwing_input_manager::prelude::{ActionState, Actionlike, InputManagerBundle, InputMap};

use bevy_game_blueprints::CharacterMovement;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component, Reflect)]
pub enum Controller {
    #[default]
    One,
    Two,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(Component, Reflect)]
#[derive(Actionlike)]
pub enum CharacterControl {
    Up,
    Down,
    Left,
    Right,
}

impl From<&CharacterControl> for CharacterMovement {
    fn from(control: &CharacterControl) -> Self {
        match control {
            CharacterControl::Up => CharacterMovement(Vec2::new(0.0, 1.0)),
            CharacterControl::Down => CharacterMovement(Vec2::new(0.0, -1.0)),
            CharacterControl::Left => CharacterMovement(Vec2::new(-1.0, 0.0)),
            CharacterControl::Right => CharacterMovement(Vec2::new(1.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub struct ControllerBundle {
    input_manager: InputManagerBundle<CharacterControl>,
}

impl ControllerBundle {
    fn new(bindings: [(KeyCode, CharacterControl); 4]) -> Self {
        let input_map = InputMap::new(bindings).build();
        ControllerBundle {
            input_manager: InputManagerBundle::<CharacterControl> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }

    pub fn key_controller_one() -> Self {
        Self::new([
            (KeyCode::W, CharacterControl::Up),
            (KeyCode::A, CharacterControl::Left),
            (KeyCode::S, CharacterControl::Down),
            (KeyCode::D, CharacterControl::Right),
        ])
    }

    pub fn key_controller_two() -> Self {
        Self::new([
            (KeyCode::Up, CharacterControl::Up),
            (KeyCode::Left, CharacterControl::Left),
            (KeyCode::Down, CharacterControl::Down),
            (KeyCode::Right, CharacterControl::Right),
        ])
    }
}
