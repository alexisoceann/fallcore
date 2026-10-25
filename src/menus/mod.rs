mod character;
mod kart;
mod main;
mod pregame;
mod race_loading;
mod settings;
mod track;
mod widget;

use crate::GameState;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Menu>()
            .init_resource::<PlayerCount>()
            .init_resource::<SelectedCharacter>()
            .init_resource::<SelectedKart>()
            .init_resource::<SelectedTrack>()
            .add_systems(OnEnter(GameState::Menu), enter_menu)
            .add_systems(OnExit(GameState::Menu), exit_menu)
            .add_observer(navigate)
            .add_plugins((
                widget::ButtonColorPlugin,
                main::MainMenuPlugin,
                pregame::PregamePlugin,
                character::CharacterSelectPlugin,
                kart::KartSelectPlugin,
                track::TrackSelectPlugin,
                race_loading::RaceLoadingPlugin,
                settings::SettingsPlugin,
            ));
    }
}

/// Which pre-game screen is active while [`GameState::Menu`].
#[derive(States, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Menu {
    #[default]
    None,
    Main,
    PlayerCount,
    CharacterSelect,
    KartSelect,
    TrackSelect,
    /// Shown after confirming the race, while the actual race setup happens.
    Loading,
    Settings,
}

/// How many local players will race. Set by the player-count screen.
#[derive(Resource, Clone, Copy, Debug)]
pub(crate) struct PlayerCount(pub(crate) u8);

impl Default for PlayerCount {
    fn default() -> Self {
        PlayerCount(1)
    }
}

/// Index into the character roster (see `menus::character`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct SelectedCharacter(pub(crate) usize);

/// Index into the kart roster (see `menus::kart`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct SelectedKart(pub(crate) usize);

/// Index into the track roster (see `menus::track`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct SelectedTrack(pub(crate) usize);

/// Attach to a button to make activating it switch to a different [`Menu`] screen.
/// Handled by one shared observer, so every screen can reuse it for "Back"/"Next"/etc.
#[derive(Component, Clone, Copy)]
struct GoToMenu(Menu);

fn enter_menu(mut commands: Commands, mut next_menu: ResMut<NextState<Menu>>) {
    commands.spawn((Camera2d, Msaa::Off, DespawnOnExit(GameState::Menu)));
    next_menu.set(Menu::Main);
}

fn exit_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn navigate(
    activate: On<Activate>,
    targets: Query<&GoToMenu>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    if let Ok(target) = targets.get(activate.entity) {
        next_menu.set(target.0);
    }
}
