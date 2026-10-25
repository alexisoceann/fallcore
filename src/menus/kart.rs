//! Kart select: each already-joined player picks their own kart.
//!
//! Unlike character select, there's no joining/leaving here — the player
//! list is already fixed by the time you reach this screen. Each player
//! moves their own cursor over the grid with their own scheme's Left/Right
//! keys; there's nothing to separately confirm, whichever kart a player is
//! on when they move on to track select is their pick.

use super::widget;
use super::{GoToMenu, Menu, Players, SLOT_COLORS, poll_left_right};
use bevy::prelude::*;

pub struct KartSelectPlugin;

impl Plugin for KartSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::KartSelect), enter_screen)
            .add_systems(Update, handle_input.run_if(in_state(Menu::KartSelect)));
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

/// Marks the screen's root, so it can be despawned and rebuilt whenever a
/// player moves their kart cursor.
#[derive(Component, Default, Clone)]
struct KartSelectRoot;

fn enter_screen(mut commands: Commands, players: Res<Players>) {
    spawn_screen(&mut commands, &players);
}

fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    existing: Query<Entity, With<KartSelectRoot>>,
    mut players: ResMut<Players>,
) {
    let roster_len = KARTS.len();
    let mut changed = false;

    for player in &mut players.0 {
        let (left, right) = poll_left_right(player.source, &keyboard);
        let delta = if left {
            Some(-1)
        } else if right {
            Some(1)
        } else {
            None
        };
        if let Some(delta) = delta {
            player.kart_index = cycle_index(player.kart_index, delta, roster_len);
            changed = true;
        }
    }

    if changed {
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        spawn_screen(&mut commands, &players);
    }
}

fn cycle_index(index: usize, delta: i32, len: usize) -> usize {
    (index as i32 + delta).rem_euclid(len as i32) as usize
}

fn spawn_screen(commands: &mut Commands, players: &Players) {
    let tiles: Vec<_> = KARTS
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            // One labeled cursor pip ("P1".."P4") per player currently on
            // this kart — not just color, so this reads fine for
            // colorblind players too.
            let cursor_pips: Vec<(Color, String)> = players
                .0
                .iter()
                .enumerate()
                .filter(|(_, p)| p.kart_index == index)
                .map(|(slot, _)| (SLOT_COLORS[slot], format!("P{}", slot + 1)))
                .collect();
            let bg = widget::selected_color(!cursor_pips.is_empty());
            widget::display_tile(entry.name.to_string(), entry.color, bg, cursor_pips)
        })
        .collect();

    commands.spawn_scene(bsn! {
        @widget::root()
        KartSelectRoot
        DespawnOnExit<Menu>(Menu::KartSelect)
        Children [
            @widget::header("Choose Your Kart")
            --
            Text({"Left/Right to change your kart".to_string()})
            TextFont { font_size: FontSize::Px(20.0) }
            TextColor(widget::TEXT_COLOR)
            --
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
            }
            Children [
                { tiles }
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
