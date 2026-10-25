//! The title screen's main menu: play, settings, or quit.

use super::widget;
use super::{GoToMenu, Menu};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::Main), spawn_screen)
            .add_observer(quit);
    }
}

#[derive(Component, Clone, Copy, Default)]
struct Quit;

fn spawn_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::Main)
        Children [
            @widget::header("Fallcore")
            --
            @widget::button("Play".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::PlayerCount))
            --
            @widget::button("Settings".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::Settings))
            --
            @widget::button("Quit".to_string(), widget::NORMAL_BUTTON, Quit)
        ]
    });
}

fn quit(activate: On<Activate>, quit_buttons: Query<&Quit>, mut app_exit: MessageWriter<AppExit>) {
    if quit_buttons.get(activate.entity).is_ok() {
        app_exit.write(AppExit::Success);
    }
}
