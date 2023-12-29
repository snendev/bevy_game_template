#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub use bevy_game_controls as controls;
pub use bevy_game_flavor as flavor;
pub use bevy_game_gameplay as gameplay;
pub use bevy_game_gameplay::blueprints;

use bevy_game_controls::{ControllerBundle, ControllerPlugin, ControllerSet};
use bevy_game_flavor::{
    audio::AudioSet, graphics::GraphicsSet, loading::AssetLoadState, FlavorPlugin,
};
use bevy_game_gameplay::{blueprints::Character, GameplayPlugin, GameplaySet};

mod menu;
use menu::MenuLoadState;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(States)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((
                menu::MenuPlugin,
                FlavorPlugin,
                GameplayPlugin,
                ControllerPlugin,
            ))
            .configure_sets(
                Update,
                (ControllerSet, AudioSet, GraphicsSet, GameplaySet).chain(),
            )
            // TODO: Spawn scenes dynamically, or make a new plugin
            .add_systems(OnEnter(GameState::Playing), spawn_character)
            .add_systems(
                Update,
                detect_menu_ready.run_if(
                    resource_exists_and_changed::<State<MenuLoadState>>()
                        .or_else(resource_exists_and_changed::<State<AssetLoadState>>()),
                ),
            );
    }
}

fn detect_menu_ready(
    menu_load_state: Res<State<MenuLoadState>>,
    asset_load_state: Res<State<AssetLoadState>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let (MenuLoadState::Active, AssetLoadState::Active) =
        (menu_load_state.get(), asset_load_state.get())
    {
        if *game_state.get() == GameState::Loading {
            next_game_state.set(GameState::Menu);
        }
    }
}

fn spawn_character(mut commands: Commands) {
    commands.spawn((Character::bundle(), ControllerBundle::key_controller_one()));
}
