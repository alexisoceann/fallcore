use crate::GameState;
use crate::gameplay::actions::Actions;
use avian3d::collision::collider::Collider;
use avian3d::dynamics::rigid_body::RigidBody;
use bevy::mesh::SphereKind;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// Radius of the player sphere in world units.
const PLAYER_RADIUS: f32 = 0.5;
/// Movement speed in world units per second.
const PLAYER_SPEED: f32 = 8.0;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(
            meshes.add(Sphere::new(PLAYER_RADIUS).mesh().kind(SphereKind::Uv {
                sectors: 16,
                stacks: 8,
            })),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.45, 0.2),
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, PLAYER_RADIUS, 0.0)),
        RigidBody::Kinematic,
        Collider::sphere(PLAYER_RADIUS),
        Player,
        DespawnOnExit(GameState::Playing),
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Some(movement) = actions.player_movement else {
        return;
    };
    // The player moves on the ground plane (x/z), the y position stays untouched.
    let movement = Vec3::new(
        movement.x * PLAYER_SPEED * time.delta_secs(),
        0.0,
        movement.y * PLAYER_SPEED * time.delta_secs(),
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
