use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(States)]
pub enum AssetLoadState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Active,
}

pub struct AssetLoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetLoadState>().add_loading_state(
            LoadingState::new(AssetLoadState::Loading)
                .continue_to_state(AssetLoadState::Active)
                .load_collection::<AudioAssets>()
                .load_collection::<SpriteAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "textures/bevy.png")]
    pub character: Handle<Image>,
}
