mod character;
mod kart;
mod main;
mod race_loading;
mod settings;
mod track;
mod widget;

use crate::GameState;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Menu>()
            .init_resource::<Players>()
            .init_resource::<SelectedTrack>()
            .add_plugins(TabNavigationPlugin)
            .add_systems(OnEnter(GameState::Menu), enter_menu)
            .add_systems(OnExit(GameState::Menu), exit_menu)
            .add_observer(navigate)
            .add_plugins((
                widget::ButtonColorPlugin,
                main::MainMenuPlugin,
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
    CharacterSelect,
    KartSelect,
    TrackSelect,
    /// Shown after confirming the race, while the actual race setup happens.
    Loading,
    Settings,
}

/// Maximum number of local players that can join a race: one per fixed
/// keyboard scheme (see [`KeyboardScheme`]).
pub(crate) const MAX_PLAYERS: usize = 3;

/// A per-join-slot color, used to tell joined players' cursors apart on the
/// character-select grid (independent of their chosen character's color).
pub(crate) const SLOT_COLORS: [Color; MAX_PLAYERS] = [
    Color::srgb(0.9, 0.2, 0.2),
    Color::srgb(0.2, 0.5, 0.95),
    Color::srgb(0.3, 0.8, 0.3),
];

/// Where one player's input comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum InputSource {
    Keyboard(KeyboardScheme),
}

/// A fixed keyboard key-set a player can use. Every scheme has the same 5
/// roles: steer left/right, accelerate (also "join" on the
/// character-select screen), brake (also "leave"), and a powerup button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum KeyboardScheme {
    Wasd,
    Ijkl,
    Arrows,
}

impl KeyboardScheme {
    /// Every scheme, in the fixed order they're offered to join in.
    pub(crate) const ALL: [KeyboardScheme; 3] = [Self::Wasd, Self::Ijkl, Self::Arrows];

    /// A short label for the status line and instructions.
    pub(crate) fn label(self) -> &'static str {
        match self {
            KeyboardScheme::Wasd => "WASD",
            KeyboardScheme::Ijkl => "IJKL",
            KeyboardScheme::Arrows => "Arrow Keys",
        }
    }

    /// `(left, right, accelerate/join, brake/leave, powerup)`.
    pub(crate) fn keys(self) -> (KeyCode, KeyCode, KeyCode, KeyCode, KeyCode) {
        match self {
            KeyboardScheme::Wasd => (
                KeyCode::KeyA,
                KeyCode::KeyD,
                KeyCode::KeyW,
                KeyCode::KeyS,
                KeyCode::Space,
            ),
            KeyboardScheme::Ijkl => (
                KeyCode::KeyJ,
                KeyCode::KeyL,
                KeyCode::KeyI,
                KeyCode::KeyK,
                KeyCode::KeyO,
            ),
            KeyboardScheme::Arrows => (
                KeyCode::ArrowLeft,
                KeyCode::ArrowRight,
                KeyCode::ArrowUp,
                KeyCode::ArrowDown,
                KeyCode::Enter,
            ),
        }
    }
}

/// One local player who has joined on the character-select screen.
#[derive(Clone, Copy, Debug)]
pub(crate) struct JoinedPlayer {
    pub(crate) source: InputSource,
    /// Index into the character roster (see `menus::character`).
    pub(crate) character_index: usize,
    /// The chosen character's color; also used for the player's kart.
    pub(crate) color: Color,
    /// Index into the kart roster (see `menus::kart`). Each joined player
    /// picks their own, unlike the track, which is one shared choice.
    pub(crate) kart_index: usize,
}

/// Players who have joined, in join order (join order == split-screen slot
/// order). At most [`MAX_PLAYERS`] entries.
#[derive(Resource, Clone, Default)]
pub(crate) struct Players(pub(crate) Vec<JoinedPlayer>);

/// Index into the track roster (see `menus::track`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub(crate) struct SelectedTrack(pub(crate) usize);

/// `(left, right)` for one input source this frame. Shared by the
/// character-select and kart-select cursors.
pub(crate) fn poll_left_right(source: InputSource, keyboard: &ButtonInput<KeyCode>) -> (bool, bool) {
    let InputSource::Keyboard(scheme) = source;
    let (left, right, ..) = scheme.keys();
    (keyboard.just_pressed(left), keyboard.just_pressed(right))
}

/// `(join/accelerate, leave/brake)` for one input source this frame. Used
/// for joining/leaving on the character-select screen.
pub(crate) fn poll_join_leave(source: InputSource, keyboard: &ButtonInput<KeyCode>) -> (bool, bool) {
    let InputSource::Keyboard(scheme) = source;
    let (_, _, accelerate, brake, _) = scheme.keys();
    (keyboard.just_pressed(accelerate), keyboard.just_pressed(brake))
}

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

