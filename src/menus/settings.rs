//! A placeholder settings screen, reachable from the main menu.

use super::widget;
use super::{GoToMenu, Menu};
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::Settings), spawn_screen);
    }
}

fn spawn_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::Settings)
        Children [
            @widget::header("Settings")
            --
            @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::Main))
        ]
    });
}
