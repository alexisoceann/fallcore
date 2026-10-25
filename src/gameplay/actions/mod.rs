use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::GameState;
use game_control::*;

mod game_control;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<PlayerInput>()
            .init_resource::<VehicleControls>()
            .add_systems(Startup, set_movement_actions)
            .add_systems(
                Update,
                read_vehicle_actions.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct VehicleControls {
    pub player_movement: Option<Vec2>,
    pub steering: f32,
    pub accelerate: bool,
    pub brake: bool,
    pub activate_powerup: bool,
}

pub fn set_movement_actions(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        PlayerInput

        Actions<PlayerInput> [
            // Menu navigation
            Action::<MoveUp>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::ArrowUp)
            ]
            --
            Action::<MoveDown>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::ArrowDown)
            ]
            --
            Action::<MoveLeft>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::ArrowLeft)
            ]
            --
            Action::<MoveRight>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::ArrowRight)
            ]
            --

            // Driving
            Action::<SteerLeft>::new()
            Bindings [
                Binding(KeyCode::KeyA)
                --
                Binding(KeyCode::ArrowLeft)
            ]
            --
            Action::<SteerRight>::new()
            Bindings [
                Binding(KeyCode::KeyD)
                --
                Binding(KeyCode::ArrowRight)
            ]
            --
            Action::<Accelerate>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::KeyW)
                --
                Binding(KeyCode::ArrowUp)
            ]
            --
            Action::<Brake>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::KeyS)
                --
                Binding(KeyCode::ArrowDown)
            ]
            --
            Action::<ActivatePowerup>::new()
            ~Press::new(1.0)
            Bindings [
                Binding(KeyCode::Space)
            ]
        ]
    });
}

fn read_vehicle_actions(
    steer_left: Single<&Action<SteerLeft>>,
    steer_right: Single<&Action<SteerRight>>,
    accelerate: Single<&Action<Accelerate>>,
    brake: Single<&Action<Brake>>,
    powerup: Single<&Action<ActivatePowerup>>,
    mut controls: ResMut<VehicleControls>,
) {
    controls.steering = (***steer_right - ***steer_left).clamp(-1.0, 1.0);
    controls.accelerate = ***accelerate;
    controls.brake = ***brake;
    controls.activate_powerup = ***powerup;

    // info!(
    //     "steering={}, accelerate={}, brake={}",
    //     controls.steering,
    //     controls.accelerate,
    //     controls.brake,
    // );
}