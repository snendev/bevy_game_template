use bevy::prelude::*;
use bevy_game_blueprints::CharacterMovement;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin, AudioTween, PlaybackState};

use crate::loading::{AssetLoadState, AudioAssets};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct AudioSet;

pub struct GameAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(AssetLoadState::Active), start_audio)
            .add_systems(
                Update,
                control_flying_sound.in_set(AudioSet).run_if(
                    resource_exists::<AudioAssets>().and_then(resource_exists::<MovementAudio>()),
                ),
            );

        #[cfg(debug_assertions)]
        app.register_type::<MovementAudio>();
    }
}

#[derive(Resource, Reflect)]
struct MovementAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(MovementAudio(handle));
}

fn control_flying_sound(
    query: Query<&CharacterMovement>,
    audio: Res<MovementAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if query.iter().any(|movement| movement.0 != Vec2::ZERO) {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if query.iter().all(|movement| movement.0 == Vec2::ZERO) {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
