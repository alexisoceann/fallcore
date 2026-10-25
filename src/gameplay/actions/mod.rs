use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use std::collections::HashMap;

use crate::GameState;
use crate::menus::InputSource;
use game_control::*;

mod game_control;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<PlayerInput>()
            .add_systems(
                Update,
                read_vehicle_actions.run_if(in_state(GameState::Playing)),
            );
    }
}

/// One player's current control inputs, written each frame from their own
/// keyboard scheme's bound actions (see [`player_input_scene`]).
#[derive(Component, Default, Clone)]
pub struct VehicleControls {
    pub steering: f32,
    pub accelerate: bool,
    pub brake: bool,
    pub activate_powerup: bool,
}

/// The input context and bindings for one player's keyboard scheme (see
/// [`InputSource`]). Compose this into a player's entity (see
/// `gameplay::player::spawn_players`) alongside their vehicle components
/// and a [`VehicleControls`].
pub fn player_input_scene(source: InputSource) -> impl Scene {
    let (left, right, accelerate, brake, powerup) = bindings_for(source);

    bsn! {
        PlayerInput
        Actions<PlayerInput> [
            Action::<SteerLeft>::new()
            Bindings [
                left
            ]
            --
            Action::<SteerRight>::new()
            Bindings [
                right
            ]
            --
            Action::<Accelerate>::new()
            ~Press::new(1.0)
            Bindings [
                accelerate
            ]
            --
            Action::<Brake>::new()
            ~Press::new(1.0)
            Bindings [
                brake
            ]
            --
            Action::<ActivatePowerup>::new()
            ~Press::new(1.0)
            Bindings [
                powerup
            ]
        ]
    }
}

/// `(left, right, accelerate, brake, powerup)` bindings for one input source.
fn bindings_for(source: InputSource) -> (Binding, Binding, Binding, Binding, Binding) {
    let InputSource::Keyboard(scheme) = source;
    let (left, right, accelerate, brake, powerup) = scheme.keys();
    (
        Binding::from(left),
        Binding::from(right),
        Binding::from(accelerate),
        Binding::from(brake),
        Binding::from(powerup),
    )
}

/// Reads every player's bound actions and writes the result into their own
/// [`VehicleControls`]. Each player has their own `PlayerInput` context
/// entity (which is their vehicle entity itself), so action entities are
/// matched back to their player via the [`ActionOf`] relationship.
fn read_vehicle_actions(
    steer_left: Query<(&Action<SteerLeft>, &ActionOf<PlayerInput>)>,
    steer_right: Query<(&Action<SteerRight>, &ActionOf<PlayerInput>)>,
    accelerate: Query<(&Action<Accelerate>, &ActionOf<PlayerInput>)>,
    brake: Query<(&Action<Brake>, &ActionOf<PlayerInput>)>,
    powerup: Query<(&Action<ActivatePowerup>, &ActionOf<PlayerInput>)>,
    mut vehicles: Query<&mut VehicleControls>,
) {
    let mut left_by_player = HashMap::new();
    for (value, context) in &steer_left {
        left_by_player.insert(**context, **value);
    }

    for (value, context) in &steer_right {
        let player = **context;
        let left = left_by_player.get(&player).copied().unwrap_or(0.0);
        if let Ok(mut controls) = vehicles.get_mut(player) {
            controls.steering = (**value - left).clamp(-1.0, 1.0);
        }
    }

    for (value, context) in &accelerate {
        if let Ok(mut controls) = vehicles.get_mut(**context) {
            controls.accelerate = **value;
        }
    }
    for (value, context) in &brake {
        if let Ok(mut controls) = vehicles.get_mut(**context) {
            controls.brake = **value;
        }
    }
    for (value, context) in &powerup {
        if let Ok(mut controls) = vehicles.get_mut(**context) {
            controls.activate_powerup = **value;
        }
    }
}
