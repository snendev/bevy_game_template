use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
#[derive(Component, Reflect)]
pub struct Character;

impl Character {
    pub fn bundle() -> CharacterBundle {
        CharacterBundle::default()
    }
}

#[derive(Debug)]
#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    name: Name,
    movement: CharacterMovement,
    spatial: SpatialBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            character: Character,
            name: Name::new("Character"),
            movement: CharacterMovement::default(),
            spatial: SpatialBundle::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component, Reflect)]
pub struct CharacterMovement(pub Vec2);
