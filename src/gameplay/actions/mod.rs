use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::GameState;
use crate::gameplay::actions::game_control::{GameControl, get_movement};
use crate::gameplay::player::Player;

mod game_control;

/// Minimum distance (world units) between the player and the touch position
/// to trigger a movement.
pub const FOLLOW_EPSILON: f32 = 0.4;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions.
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Result {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position()
        && let Ok((camera, camera_transform)) = camera.single()
        && let Ok(ray) = camera.viewport_to_world(camera_transform, touch_position)
        && let Some(target) =
            ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    {
        let diff = target.xz()
            - player
                .single()
                .map(|transform| transform.translation.xz())
                .unwrap_or(target.xz());
        if diff.length() > FOLLOW_EPSILON {
            player_movement = diff.normalize();
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    Ok(())
}
