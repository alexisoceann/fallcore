use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

/// Input context for one player's vehicle. Each joined player gets their own
/// entity with this component, bound to one of the fixed keyboard schemes
/// (see `actions::player_input_scene`).
#[derive(Component, Default, Clone)]
pub struct PlayerInput;

#[derive(InputAction)]
#[action_output(f32)]
pub struct SteerLeft;

#[derive(InputAction)]
#[action_output(f32)]
pub struct SteerRight;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Accelerate;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Brake;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ActivatePowerup;
