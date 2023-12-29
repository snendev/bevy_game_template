use bevy::prelude::*;

pub mod audio;
pub mod graphics;
pub mod loading;

pub struct FlavorPlugin;

impl Plugin for FlavorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loading::AssetLoadingPlugin,
            audio::GameAudioPlugin,
            graphics::GameGraphicsPlugin,
        ));
    }
}
