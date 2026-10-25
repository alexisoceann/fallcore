//! Kart select.

use super::widget;
use super::{GoToMenu, Menu, SelectedKart};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct KartSelectPlugin;

impl Plugin for KartSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::KartSelect), spawn_screen)
            .add_observer(set_kart);
    }
}

struct RosterEntry {
    name: &'static str,
    color: Color,
}

const KARTS: &[RosterEntry] = &[
    RosterEntry {
        name: "Wasp",
        color: Color::srgb(0.95, 0.85, 0.15),
    },
    RosterEntry {
        name: "Comet",
        color: Color::srgb(0.6, 0.3, 0.85),
    },
    RosterEntry {
        name: "Boulder",
        color: Color::srgb(0.55, 0.55, 0.55),
    },
    RosterEntry {
        name: "Dart",
        color: Color::srgb(0.15, 0.75, 0.7),
    },
];

#[derive(Component, Clone, Copy)]
struct SetKart(usize);

fn spawn_screen(mut commands: Commands, selected: Res<SelectedKart>) {
    let options: Vec<_> = KARTS
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let bg = widget::selected_color(index == selected.0);
            widget::option_button(entry.name, entry.color, bg, SetKart(index))
        })
        .collect();

    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::KartSelect)
        Children [
            @widget::header("Choose Your Kart")
            --
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
            }
            Children [
                { options }
            ]
            --
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
            }
            Children [
                @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::CharacterSelect))
                --
                @widget::button("Next".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::TrackSelect))
            ]
        ]
    });
}

fn set_kart(
    activate: On<Activate>,
    changed: Query<&SetKart>,
    mut options: Query<(&SetKart, &mut BackgroundColor, &mut widget::ButtonColors)>,
    mut selected: ResMut<SelectedKart>,
) {
    let Ok(choice) = changed.get(activate.entity) else {
        return;
    };
    selected.0 = choice.0;
    for (option, mut color, mut colors) in &mut options {
        let bg = widget::selected_color(option.0 == choice.0);
        colors.normal = bg;
        *color = bg.into();
    }
}
