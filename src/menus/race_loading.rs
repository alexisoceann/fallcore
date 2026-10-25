//! A short transitional screen between confirming the race on the track-select
//! screen and gameplay actually starting.
//!
//! fallcore currently loads every asset up front, before the main menu ever
//! appears, so there is nothing left to load by the time the player reaches
//! this screen. It holds for a fixed minimum duration purely so the cut from
//! menu to gameplay doesn't feel like a glitch. If per-race asset loading
//! (e.g. a track loaded lazily) is added later, this is where the real
//! "is it ready yet?" check would replace the timer.

use super::Menu;
use super::widget;
use crate::GameState;
use bevy::prelude::*;

pub struct RaceLoadingPlugin;

impl Plugin for RaceLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::Loading), spawn_screen)
            .add_systems(Update, tick.run_if(in_state(Menu::Loading)));
    }
}

const MIN_DISPLAY_SECS: f32 = 0.5;

#[derive(Resource, Deref, DerefMut)]
struct LoadingTimer(Timer);

fn spawn_screen(mut commands: Commands) {
    commands.insert_resource(LoadingTimer(Timer::from_seconds(
        MIN_DISPLAY_SECS,
        TimerMode::Once,
    )));
    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::Loading)
        Children [
            @widget::header("Loading...")
        ]
    });
}

fn tick(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).is_finished() {
        next_state.set(GameState::Playing);
    }
}
