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

/// A full-screen column that centers its children. The base node for every menu screen.
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
        action
        Children [
            Text(text)
            TextFont { font_size: FontSize::Px(28.0) }
            TextColor(TEXT_COLOR)
        ]
    }
}

/// A small square button, e.g. the "-"/"+" of a stepper.
pub(super) fn button_small<A: Component + Clone + Unpin>(text: String, action: A) -> impl Scene {
    bsn! {
        Button
        Hovered
        Node {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(NORMAL_BUTTON)
        ButtonColors::default()
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

pub(super) fn selected_color(is_selected: bool) -> Color {
    if is_selected { SELECTED_BUTTON } else { NORMAL_BUTTON }
}
