mod actions;
mod audio;
mod camera;
mod finish_areas;
mod level;
mod player;
mod physics;

use bevy::prelude::*;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use camera::CameraPlugin;
// use finish_areas::FinishAreaPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            InternalAudioPlugin,
            CameraPlugin,
            // FinishAreaPlugin,
            LevelPlugin,
            PlayerPlugin,
            PhysicsPlugin,
        ));
    }
}
