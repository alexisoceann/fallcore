use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Component, Default, Clone)]
pub struct PlayerInput;

#[derive(InputAction)]
#[action_output(f32)]
pub struct MoveUp;

#[derive(InputAction)]
#[action_output(f32)]
pub struct MoveDown;

#[derive(InputAction)]
#[action_output(f32)]
pub struct MoveLeft;

#[derive(InputAction)]
#[action_output(f32)]
pub struct MoveRight;

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
