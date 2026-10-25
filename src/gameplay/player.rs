use crate::GameState;
use crate::gameplay::actions::VehicleControls;
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

const PLAYER_SPEED: f32 = 8.0;
const STEER_SPEED: f32 = 2.5; // radians/sec

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                FixedUpdate,
                move_player.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Mesh3d(asset_value(Cuboid::new(1., 0.7, 2.)))
        MeshMaterial3d<StandardMaterial>(
            asset_value(StandardMaterial {
                base_color: Color::srgb(0.9, 0.45, 0.2),
                perceptual_roughness: 0.4,
                ..default()
            })
        )
        Transform::from_translation(Vec3::new(0.0, 0.35, 0.0))
        RigidBody::Dynamic
        Collider::cuboid(1.0, 0.7, 2.0) // match mesh extents
        SweptCcd
        // Keep the vehicle upright: only allow it to yaw, not pitch/roll over.
        LockedAxes::new().lock_rotation_x().lock_rotation_z()
        LinearDamping(0.5)
        AngularDamping(5.0)

        LinearVelocity::default()
        Player
        DespawnOnExit<GameState>(GameState::Playing)
    });
}


// fn move_player(
//     time: Res<Time>,
//     actions: Res<VehicleControls>,
//     mut player_query: Query<&mut Transform, With<Player>>,
// ) {
//     let delta = time.delta_secs();

//     for mut transform in &mut player_query {
//         let throttle = if actions.accelerate {
//             1.0
//         } else if actions.brake {
//             -0.5
//         } else {
//             0.0
//         };

//         if throttle != 0.0 {
//             let forward = transform.rotation * Vec3::NEG_Z;
//             transform.translation += forward * throttle * PLAYER_SPEED * delta;
//         }

//         if actions.steering != 0.0 {
//             transform.rotation *= Quat::from_rotation_y(
//                 -actions.steering * delta * 2.0,
//             );
//         }
//     }
// }

fn move_player(
    actions: Res<VehicleControls>,
    mut player_query: Query<(&Transform, Forces), With<Player>>,
) {
    for (transform, mut forces) in &mut player_query {
        let throttle = if actions.accelerate { 1.0 } else if actions.brake { -0.5 } else { 0.0 };
        if throttle != 0.0 {
            let forward = transform.rotation * Vec3::NEG_Z;
            forces.apply_force(forward * throttle * PLAYER_SPEED * 7.0); // tune
        }

        // Drive yaw directly instead of applying torque: torque against a box
        // resting flush on the ground fights static friction at the contact
        // corners and jitters. LockedAxes keeps this confined to the Y axis.
        forces.angular_velocity_mut().y = -actions.steering * STEER_SPEED;
    }
}
