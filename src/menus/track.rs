//! Track select. The last screen before the race starts.

use super::widget;
use super::{GoToMenu, Menu, SelectedTrack};
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct TrackSelectPlugin;

impl Plugin for TrackSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::TrackSelect), spawn_screen)
            .add_observer(set_track);
    }
}

struct RosterEntry {
    name: &'static str,
    color: Color,
}

const TRACKS: &[RosterEntry] = &[
    RosterEntry {
        name: "Sunbaked Mesa",
        color: Color::srgb(0.8, 0.5, 0.3),
    },
    RosterEntry {
        name: "Frostbite Pass",
        color: Color::srgb(0.6, 0.8, 0.95),
    },
    RosterEntry {
        name: "Neon Sprawl",
        color: Color::srgb(0.9, 0.2, 0.75),
    },
];

#[derive(Component, Clone, Copy)]
struct SetTrack(usize);

fn spawn_screen(mut commands: Commands, selected: Res<SelectedTrack>) {
    let options: Vec<_> = TRACKS
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let bg = widget::selected_color(index == selected.0);
            widget::option_button(entry.name, entry.color, bg, SetTrack(index))
        })
        .collect();

    commands.spawn_scene(bsn! {
        @widget::root()
        DespawnOnExit<Menu>(Menu::TrackSelect)
        Children [
            @widget::header("Choose Your Track")
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
                @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::KartSelect))
                --
                @widget::button("Start Race".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::Loading))
            ]
        ]
    });
}

fn set_track(
    activate: On<Activate>,
    changed: Query<&SetTrack>,
    mut options: Query<(&SetTrack, &mut BackgroundColor, &mut widget::ButtonColors)>,
    mut selected: ResMut<SelectedTrack>,
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
