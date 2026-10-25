//! Character select.

use super::widget;
use super::{GoToMenu, Menu, SelectedCharacter};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::CharacterSelect), spawn_screen)
            .add_observer(set_character);
    }
}

struct RosterEntry {
    name: &'static str,
    color: Color,
}

const CHARACTERS: &[RosterEntry] = &[
    RosterEntry {
        name: "Vex",
        color: Color::srgb(0.85, 0.2, 0.25),
    },
    RosterEntry {
        name: "Nimbus",
        color: Color::srgb(0.3, 0.55, 0.95),
    },
    RosterEntry {
        name: "Juno",
        color: Color::srgb(0.35, 0.7, 0.35),
    },
    RosterEntry {
        name: "Ember",
        color: Color::srgb(0.95, 0.65, 0.15),
    },
];

#[derive(Component, Clone, Copy)]
struct SetCharacter(usize);

fn spawn_screen(mut commands: Commands, selected: Res<SelectedCharacter>) {
    let options: Vec<_> = CHARACTERS
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let bg = widget::selected_color(index == selected.0);
            widget::option_button(entry.name, entry.color, bg, SetCharacter(index))
        })
        .collect();

    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::CharacterSelect)
        Children [
            @widget::header("Choose Your Character")
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
                @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::PlayerCount))
                --
                @widget::button("Next".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::KartSelect))
            ]
        ]
    });
}

fn set_character(
    activate: On<Activate>,
    changed: Query<&SetCharacter>,
    mut options: Query<(&SetCharacter, &mut BackgroundColor, &mut widget::ButtonColors)>,
    mut selected: ResMut<SelectedCharacter>,
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
