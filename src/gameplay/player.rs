use crate::GameState;
use crate::gameplay::actions::{VehicleControls, player_input_scene};
use crate::menus::Players;
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

/// Spawned players, in join order. Join order is also split-screen slot
/// order (see `gameplay::camera`).
#[derive(Resource, Default, Clone)]
pub struct PlayerEntities(pub Vec<Entity>);

const PLAYER_SPEED: f32 = 8.0;
const STEER_SPEED: f32 = 2.5; // radians/sec

/// Spawn points for up to four players, spaced out enough that their
/// 1x2-unit karts don't start overlapping.
const SPAWN_POINTS: [Vec3; 4] = [
    Vec3::new(-2.0, 0.35, 1.0),
    Vec3::new(2.0, 0.35, 1.0),
    Vec3::new(-2.0, 0.35, -2.0),
    Vec3::new(2.0, 0.35, -2.0),
];

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerEntities>()
            .add_systems(OnEnter(GameState::Playing), spawn_players)
            .add_systems(
                FixedUpdate,
                move_player.run_if(in_state(GameState::Playing)),
            );
    }
}

/// Spawns every joined player's vehicle. `gameplay::camera`'s
/// `spawn_game_cameras` runs `.after()` this, so it always sees the
/// [`PlayerEntities`] this inserts (not the one, now-stale, from a previous
/// race, which `init_resource` alone wouldn't clear).
pub(super) fn spawn_players(mut commands: Commands, players: Res<Players>) {
    let mut entities = Vec::new();
    for (index, joined) in players.0.iter().enumerate().take(SPAWN_POINTS.len()) {
        let spawn = SPAWN_POINTS[index];
        let color = joined.color;
        let source = joined.source;
        let entity = commands
            .spawn_scene(bsn! {
                Mesh3d(asset_value(Cuboid::new(1., 0.7, 2.)))
                MeshMaterial3d<StandardMaterial>(
                    asset_value(StandardMaterial {
                        base_color: color,
                        perceptual_roughness: 0.4,
                        ..default()
                    })
                )
                Transform::from_translation(spawn)
                RigidBody::Dynamic
                Collider::cuboid(1.0, 0.7, 2.0) // match mesh extents
                SweptCcd
                // Keep the vehicle upright: only allow it to yaw, not pitch/roll over.
                LockedAxes::new().lock_rotation_x().lock_rotation_z()
                LinearDamping(0.5)
                AngularDamping(5.0)

                LinearVelocity::default()
                Player
                VehicleControls::default()
                @player_input_scene(source)
                DespawnOnExit<GameState>(GameState::Playing)
            })
            .id();
        entities.push(entity);
    }
    commands.insert_resource(PlayerEntities(entities));
}

fn move_player(mut player_query: Query<(&Transform, Forces, &VehicleControls), With<Player>>) {
    for (transform, mut forces, controls) in &mut player_query {
        let throttle = if controls.accelerate {
            1.0
        } else if controls.brake {
            -0.5
        } else {
            0.0
        };
        if throttle != 0.0 {
            let forward = transform.rotation * Vec3::NEG_Z;
            forces.apply_force(forward * throttle * PLAYER_SPEED * 7.0); // tune
        }

        // Drive yaw directly instead of applying torque: torque against a box
        // resting flush on the ground fights static friction at the contact
        // corners and jitters. LockedAxes keeps this confined to the Y axis.
        forces.angular_velocity_mut().y = -controls.steering * STEER_SPEED;
    }
}
