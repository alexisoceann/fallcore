use crate::GameState;
use crate::gameplay::actions::VehicleControls;
use crate::gameplay::player::Player;
use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::DefaultSpatialRadius;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

/// Separate channel for the spatial sounds, so that pausing the main track
/// (for the flying sound) does not affect them.
#[derive(Resource)]
struct SpatialTrack;

/// Distance (world units) at which a spatial sound becomes silent.
const SPATIAL_RADIUS: f32 = 16.0;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, SpatialAudioPlugin))
            .add_audio_channel::<SpatialTrack>()
            .insert_resource(DefaultSpatialRadius {
                radius: SPATIAL_RADIUS,
            })
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_systems(
                Update,
                (setup_spatial_audio, control_flying_sound).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), stop_audio);
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn stop_audio(
    mut commands: Commands,
    audio: Res<FlyingAudio>,
    spatial_audio: Res<AudioChannel<SpatialTrack>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(mut instance) = audio_instances.get_mut(&audio.0) {
        instance.stop(AudioTween::default());
    }
    // Stop all spatial sounds (the finish areas).
    spatial_audio.stop();
    commands.remove_resource::<FlyingAudio>();
}

/// Sets up the spatial audio: the player is the receiver and every
/// spatial audio emitter plays the goal sound.
fn setup_spatial_audio(
    mut commands: Commands,
    players: Query<Entity, Added<Player>>,
    mut emitters: Query<&mut SpatialAudioEmitter, Added<SpatialAudioEmitter>>,
    spatial_audio: Res<AudioChannel<SpatialTrack>>,
    audio_assets: Res<AudioAssets>,
) {
    // The player is the listener of the spatial sounds.
    for player in &players {
        commands.entity(player).insert(SpatialAudioReceiver);
    }
    // Every emitter plays its own instance of the goal sound. The volume and
    // the panning are controlled by the spatial audio plugin, based on the
    // distance between the emitter and the receiver.
    for mut emitter in &mut emitters {
        let handle = spatial_audio
            .play(audio_assets.goal.clone())
            .looped()
            .handle();
        emitter.instances.push(handle);
    }
}

fn control_flying_sound(
    vehicles: Query<&VehicleControls>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let any_throttle = vehicles.iter().any(|v| v.accelerate || v.brake);
    if let Some(mut instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } if any_throttle => {
                instance.resume(AudioTween::default());
            }
            PlaybackState::Playing { .. } if !any_throttle => {
                instance.pause(AudioTween::default());
            }
            _ => {}
        }
    }
}
