use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

pub(super) const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub(super) const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub(super) const SELECTED_BUTTON: Color = Color::srgb(0.25, 0.55, 0.3);
pub(super) const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Recolors a button's background on hover, based on its [`ButtonColors`].
/// Shared by every menu screen.
pub struct ButtonColorPlugin;

impl Plugin for ButtonColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button_colors);
    }
}

#[derive(Component, Clone)]
pub(super) struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: NORMAL_BUTTON,
            hovered: HOVERED_BUTTON,
        }
    }
}

fn update_button_colors(
    mut buttons: Query<(&Hovered, &mut BackgroundColor, &ButtonColors), Changed<Hovered>>,
) {
    for (hovered, mut color, button_colors) in &mut buttons {
        *color = if hovered.get() {
            button_colors.hovered
        } else {
            button_colors.normal
        }
        .into();
    }
}

/// A full-screen column that centers its children. The base node for every
/// menu screen. `TabGroup` makes its buttons keyboard-tabbable (Tab/Shift+Tab,
/// Enter/Space to activate) as a self-contained set.
pub(super) fn root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
        }
        TabGroup::default()
    }
}

/// A large title label.
pub(super) fn header(text: &'static str) -> impl Scene {
    bsn! {
        Text(text)
        TextFont { font_size: FontSize::Px(48.0) }
        TextColor(TEXT_COLOR)
    }
}

/// A text button. `background` lets the caller pre-select a button (e.g. [`SELECTED_BUTTON`])
/// as its resting color; `action` is a marker component read by the screen's own observer(s).
pub(super) fn button<A: Component + Clone + Unpin>(
    text: String,
    background: Color,
    action: A,
) -> impl Scene {
    bsn! {
        Button
        Hovered
        Node {
            width: Val::Px(220.0),
            height: Val::Px(64.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(background)
        ButtonColors { normal: background }
        TabIndex(0)
        action
        Children [
            Text(text)
            TextFont { font_size: FontSize::Px(28.0) }
            TextColor(TEXT_COLOR)
        ]
    }
}

/// A roster-entry button: a color swatch over a name, used by the character/kart/track
/// select screens.
pub(super) fn option_button<A: Component + Clone + Unpin>(
    name: &'static str,
    swatch: Color,
    background: Color,
    action: A,
) -> impl Scene {
    bsn! {
        Button
        Hovered
        Node {
            width: Val::Px(160.0),
            height: Val::Px(140.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
        }
        BackgroundColor(background)
        ButtonColors { normal: background }
        TabIndex(0)
        action
        Children [
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
            }
            BackgroundColor(swatch)
            --
            Text(name)
            TextFont { font_size: FontSize::Px(22.0) }
            TextColor(TEXT_COLOR)
        ]
    }
}

/// A non-interactive roster tile: a color swatch over a name, with a row of
/// small "selection cursor" pips underneath — one per player/preview
/// currently pointing at it. Used by the character-select and kart-select
/// screens, whose tiles are driven by keyboard polling rather than mouse
/// clicks. Each pip is labeled ("P1", "P2", ...) rather than relying
/// on color alone, since color is the only thing distinguishing players.
pub(super) fn display_tile(
    name: String,
    swatch: Color,
    background: Color,
    cursor_pips: Vec<(Color, String)>,
) -> impl Scene {
    let cursors: Vec<_> = cursor_pips
        .into_iter()
        .map(|(color, label)| pip(color, label))
        .collect();

    bsn! {
        Node {
            width: Val::Px(160.0),
            height: Val::Px(140.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
        }
        BackgroundColor(background)
        Children [
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
            }
            BackgroundColor(swatch)
            --
            Text(name)
            TextFont { font_size: FontSize::Px(22.0) }
            TextColor(TEXT_COLOR)
            TextLayout::justify(Justify::Center)
            --
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                height: Val::Px(18.0),
            }
            Children [
                { cursors }
            ]
        ]
    }
}

/// One labeled "selection cursor" pip (see [`display_tile`]). `label` is
/// typically "P1".."P4" for a joined player, or empty for an unjoined
/// source that's merely previewing.
fn pip(color: Color, label: String) -> impl Scene {
    bsn! {
        Node {
            width: Val::Px(28.0),
            height: Val::Px(18.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(color)
        Children [
            Text(label)
            TextFont { font_size: FontSize::Px(11.0) }
            TextColor(Color::BLACK)
        ]
    }
}

pub(super) fn selected_color(is_selected: bool) -> Color {
    if is_selected { SELECTED_BUTTON } else { NORMAL_BUTTON }
}
