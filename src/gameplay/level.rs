use crate::GameState;
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_scene);
    }
}

/// Spawns the ground and the light of the 3d scene.
fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(200.0, 0.5, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.4, 0.35),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, -0.25, 0.0)),
        RigidBody::Static,
        Collider::cuboid(200.0, 0.5, 200.0),
        DespawnOnExit(GameState::Playing),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.4, 0.35),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 6., 8.0)),
        RigidBody::Dynamic,
        Collider::cuboid(2., 2., 2.),
        DespawnOnExit(GameState::Playing),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.4, 0.35),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 6., -8.0)),
        RigidBody::Dynamic,
        Collider::cuboid(2., 2., 2.),
        DespawnOnExit(GameState::Playing),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(GameState::Playing),
    ));
}
