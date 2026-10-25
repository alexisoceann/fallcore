//! Player-count select: how many local players will race.
//!
//! Bevy has no single packaged "counter" widget (its own `feathers_counter`
//! example builds the same -/value/+ shape out of plain buttons + a resource),
//! so this follows that pattern with fallcore's own button styling.

use super::widget;
use super::{GoToMenu, Menu, PlayerCount};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct PregamePlugin;

impl Plugin for PregamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::PlayerCount), spawn_screen)
            .add_systems(
                Update,
                update_count_label
                    .run_if(in_state(Menu::PlayerCount).and_then(resource_changed::<PlayerCount>)),
            )
            .add_observer(decrement_player_count)
            .add_observer(increment_player_count);
    }
}

const MIN_PLAYERS: u8 = 1;
const MAX_PLAYERS: u8 = 4;

#[derive(Component, Clone, Copy, Default)]
struct Decrement;

#[derive(Component, Clone, Copy, Default)]
struct Increment;

#[derive(Component, Default, Clone)]
struct PlayerCountLabel;

fn spawn_screen(mut commands: Commands, count: Res<PlayerCount>) {
    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::PlayerCount)
        Children [
            @widget::header("How Many Players?")
            --
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(16.0),
            }
            Children [
                @widget::button_small("-".to_string(), Decrement)
                --
                Text({count.0.to_string()})
                TextFont { font_size: FontSize::Px(40.0) }
                TextColor(widget::TEXT_COLOR)
                PlayerCountLabel
                --
                @widget::button_small("+".to_string(), Increment)
            ]
            --
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
            }
            Children [
                @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::Main))
                --
                @widget::button("Next".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::CharacterSelect))
            ]
        ]
    });
}

fn decrement_player_count(
    activate: On<Activate>,
    buttons: Query<&Decrement>,
    mut count: ResMut<PlayerCount>,
) {
    if buttons.get(activate.entity).is_ok() {
        count.0 = count.0.saturating_sub(1).max(MIN_PLAYERS);
    }
}

fn increment_player_count(
    activate: On<Activate>,
    buttons: Query<&Increment>,
    mut count: ResMut<PlayerCount>,
) {
    if buttons.get(activate.entity).is_ok() {
        count.0 = (count.0 + 1).min(MAX_PLAYERS);
    }
}

fn update_count_label(count: Res<PlayerCount>, mut label: Single<&mut Text, With<PlayerCountLabel>>) {
    label.0 = count.0.to_string();
}
