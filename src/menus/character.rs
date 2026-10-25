//! Character select & player join.
//!
//! This replaces the old separate "how many players" screen: there is no
//! player-count picker anymore, the player count is simply however many of
//! the 3 fixed keyboard schemes (WASD, IJKL, Arrow Keys) have joined here.
//! A scheme only shows a cursor once it's actually touched (pressed
//! left/right/accelerate) — it isn't shown or movable just because it
//! exists, since all 3 are always "available" even when nobody's using 2 of
//! them. Once a cursor appears, moving it with Left/Right looks around, and
//! pressing the Accelerate key joins (locking in whatever character it's
//! previewing). A joined player can keep changing their pick the same way,
//! or leave with their Brake key.

use super::widget;
use super::{
    GoToMenu, InputSource, JoinedPlayer, KeyboardScheme, MAX_PLAYERS, Menu, Players, SLOT_COLORS,
    poll_join_leave, poll_left_right,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursors>()
            .add_systems(OnEnter(Menu::CharacterSelect), enter_screen)
            .add_systems(
                Update,
                handle_input.run_if(in_state(Menu::CharacterSelect)),
            );
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

/// Marks the screen's root, so it can be despawned and rebuilt whenever a
/// scheme joins, leaves, or moves its preview cursor.
#[derive(Component, Default, Clone)]
struct CharacterSelectRoot;

/// Which character each not-yet-joined keyboard scheme is currently
/// previewing. A scheme only gets an entry here once it's actually pressed
/// something — see the module docs.
#[derive(Resource, Default, Clone)]
struct Cursors(HashMap<InputSource, usize>);

fn enter_screen(mut commands: Commands, mut cursors: ResMut<Cursors>, players: Res<Players>) {
    cursors.0.clear();
    spawn_screen(&mut commands, &players, &cursors);
}

/// Polls every keyboard scheme for join/leave/cursor-move input, then
/// rebuilds the screen from scratch if anything actually changed. Rebuilding
/// the whole tiny screen is simpler and plenty cheap for a menu, and avoids
/// patching per-tile state incrementally.
fn handle_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    existing: Query<Entity, With<CharacterSelectRoot>>,
    mut players: ResMut<Players>,
    mut cursors: ResMut<Cursors>,
) {
    let mut changed = false;
    let roster_len = CHARACTERS.len();

    for scheme in KeyboardScheme::ALL {
        let source = InputSource::Keyboard(scheme);
        let (left, right) = poll_left_right(source, &keyboard);
        let (join, leave) = poll_join_leave(source, &keyboard);
        let joined_index = players.0.iter().position(|p| p.source == source);

        if let Some(player_index) = joined_index {
            if leave {
                players.0.remove(player_index);
                cursors.0.insert(source, 0);
                changed = true;
                continue;
            }

            let delta = if left {
                Some(-1)
            } else if right {
                Some(1)
            } else {
                None
            };
            if let Some(delta) = delta {
                let player = &mut players.0[player_index];
                player.character_index = cycle_index(player.character_index, delta, roster_len);
                player.color = CHARACTERS[player.character_index].color;
                changed = true;
            }
        } else {
            // Don't create a cursor just because this scheme exists (all 3
            // always "exist") — only once it's touched.
            let existing_cursor = cursors.0.get(&source).copied();
            if existing_cursor.is_none() && !(left || right || join) {
                continue;
            }

            let cursor = existing_cursor.unwrap_or(0);
            let mut preview = cursor;
            if left {
                preview = cycle_index(cursor, -1, roster_len);
            } else if right {
                preview = cycle_index(cursor, 1, roster_len);
            }

            if join && players.0.len() < MAX_PLAYERS {
                players.0.push(JoinedPlayer {
                    source,
                    character_index: preview,
                    color: CHARACTERS[preview].color,
                    kart_index: 0,
                });
                cursors.0.remove(&source);
                changed = true;
            } else if existing_cursor != Some(preview) {
                cursors.0.insert(source, preview);
                changed = true;
            }
        }
    }

    if changed {
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        spawn_screen(&mut commands, &players, &cursors);
    }
}

fn cycle_index(index: usize, delta: i32, len: usize) -> usize {
    (index as i32 + delta).rem_euclid(len as i32) as usize
}

fn spawn_screen(commands: &mut Commands, players: &Players, cursors: &Cursors) {
    let tiles: Vec<_> = CHARACTERS
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            // One labeled cursor pip per player/preview currently pointing
            // at this tile: "P1".."P3" in a joined player's slot color, or
            // an unlabeled neutral pip for an unjoined scheme still looking
            // around. Labels (not just color) so this reads fine for
            // colorblind players too.
            let mut cursor_pips: Vec<(Color, String)> = players
                .0
                .iter()
                .enumerate()
                .filter(|(_, p)| p.character_index == index)
                .map(|(slot, _)| (SLOT_COLORS[slot], format!("P{}", slot + 1)))
                .collect();
            let claimed = !cursor_pips.is_empty();
            let previewed = cursors.0.values().any(|&cursor| cursor == index);
            cursor_pips.extend(
                cursors
                    .0
                    .values()
                    .filter(|&&cursor| cursor == index)
                    .map(|_| (widget::TEXT_COLOR, String::new())),
            );

            let bg = if claimed {
                widget::SELECTED_BUTTON
            } else if previewed {
                widget::HOVERED_BUTTON
            } else {
                widget::NORMAL_BUTTON
            };

            widget::display_tile(entry.name.to_string(), entry.color, bg, cursor_pips)
        })
        .collect();

    let joined = players.0.len();
    let status = if joined == 0 {
        format!(
            "{}, {}, or {} \u{2014} Left/Right to look around, Accelerate to join",
            KeyboardScheme::Wasd.label(),
            KeyboardScheme::Ijkl.label(),
            KeyboardScheme::Arrows.label(),
        )
    } else {
        format!("{joined}/{MAX_PLAYERS} joined \u{2014} Accelerate to add another, Brake to leave")
    };

    commands.spawn_scene(bsn! {
        @widget::root()
        CharacterSelectRoot
        DespawnOnExit<Menu>(Menu::CharacterSelect)
        Children [
            @widget::header("Choose Your Character")
            --
            Text(status)
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
                @widget::button("Back".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::Main))
                --
                @widget::button("Next".to_string(), widget::NORMAL_BUTTON, GoToMenu(Menu::KartSelect))
            ]
        ]
    });
}
