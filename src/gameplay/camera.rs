use crate::GameState;
use crate::gameplay::player::{Player, PlayerEntities, spawn_players};
use bevy::{
    prelude::*,
    camera::Viewport,
    window::WindowResized,
};

pub struct CameraPlugin;

/// A camera that follows one player. `slot`/`player_count` identify which
/// quadrant of the split screen it owns, so its viewport can be recomputed
/// on window resize.
#[derive(Component)]
pub struct GameCamera {
    target: Entity,
    slot: usize,
    player_count: usize,
}

/// Offset of the camera relative to the player it follows.
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 1.0, 5.0);
/// How quickly the camera catches up with the player.
const CAMERA_FOLLOW_SPEED: f32 = 6.0;

/// This plugin handles the game cameras, one per player, which each follow
/// their own player. With more than one player the window is split into
/// quadrants:
/// - 1 player: fullscreen.
/// - 2 players: top/bottom halves.
/// - 3 players: top-left, top-right, bottom-left (bottom-right unused).
/// - 4 players: all four quadrants.
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_game_cameras.after(spawn_players),
        )
            .add_systems(
                Update,
                (follow_player, resize_viewports).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_game_cameras(mut commands: Commands, players: Res<PlayerEntities>, window: Single<&Window>) {
    let player_count = players.0.len();
    let size = window.physical_size();
    for (slot, &target) in players.0.iter().enumerate() {
        commands.spawn((
            Camera3d::default(),
            Camera {
                viewport: viewport_for(slot, player_count, size),
                order: slot as isize,
                ..default()
            },
            // AtmosphereSettings::default(),
            Msaa::Off,
            Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
            GameCamera {
                target,
                slot,
                player_count,
            },
            DespawnOnExit(GameState::Playing),
        ));
    }
}

/// The viewport rectangle for one player's quadrant, or `None` for a single
/// player (fullscreen).
fn viewport_for(slot: usize, player_count: usize, size: UVec2) -> Option<Viewport> {
    match player_count {
        0 | 1 => None,
        2 => {
            let half_height = size.y / 2;
            let y = slot as u32 * half_height;
            Some(Viewport {
                physical_position: UVec2::new(0, y),
                physical_size: UVec2::new(size.x, half_height),
                ..default()
            })
        }
        _ => {
            let half_width = size.x / 2;
            let half_height = size.y / 2;
            let column = (slot % 2) as u32;
            let row = (slot / 2) as u32;
            Some(Viewport {
                physical_position: UVec2::new(column * half_width, row * half_height),
                physical_size: UVec2::new(half_width, half_height),
                ..default()
            })
        }
    }
}

/// Recomputes every camera's viewport when the window is resized, so the
/// split-screen layout keeps matching the window.
fn resize_viewports(
    mut resize_events: MessageReader<WindowResized>,
    mut cameras: Query<(&GameCamera, &mut Camera)>,
    window: Single<&Window>,
) {
    if resize_events.read().next().is_none() {
        return;
    }
    let size = window.physical_size();
    for (game_camera, mut camera) in &mut cameras {
        camera.viewport = viewport_for(game_camera.slot, game_camera.player_count, size);
    }
}

/// Moves each camera behind its own player, following that player's rotation.
fn follow_player(
    time: Res<Time>,
    // The queries are disjoint, because both of them access the transform.
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_query: Query<(&GameCamera, &mut Transform), Without<Player>>,
) {
    for (game_camera, mut camera_transform) in &mut camera_query {
        let Ok(player_transform) = player_query.get(game_camera.target) else {
            continue;
        };

        // Rotate the offset with the player so the camera stays behind it.
        let rotated_offset = player_transform.rotation * CAMERA_OFFSET;
        let target = player_transform.translation + rotated_offset;

        // Smoothly follow the target position.
        let factor = 1.0 - (-CAMERA_FOLLOW_SPEED * time.delta_secs()).exp();
        camera_transform.translation = camera_transform.translation.lerp(target, factor);

        // Look toward a point ahead of the player, rather than its center.
        let forward = player_transform.rotation * Vec3::NEG_Z;
        let look_target = player_transform.translation + forward * 3.0 + Vec3::Y * 0.5;

        camera_transform.look_at(look_target, Vec3::Y);
    }
}
