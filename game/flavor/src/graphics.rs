use bevy::prelude::*;

use bevy_game_blueprints::Character;

use crate::loading::SpriteAssets;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct GraphicsSet;

pub struct GameGraphicsPlugin;

impl Plugin for GameGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_player
                .in_set(GraphicsSet)
                .run_if(resource_exists::<SpriteAssets>()),
        );
    }
}

fn render_player(
    mut commands: Commands,
    character_query: Query<(Entity, &Transform), Added<Character>>,
    textures: Res<SpriteAssets>,
) {
    for (character, transform) in character_query.iter() {
        commands.entity(character).insert(SpriteBundle {
            texture: textures.character.clone(),
            transform: *transform,
            ..Default::default()
        });
    }
}
