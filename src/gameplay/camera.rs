use crate::GameState;
use crate::gameplay::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

/// The camera which follows the player.
#[derive(Component)]
pub struct GameCamera;

/// Offset of the camera relative to the player.
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 1.0, 5.0);
/// How quickly the camera catches up with the player.
const CAMERA_FOLLOW_SPEED: f32 = 6.0;

/// This plugin handles the game camera, which follows the player.
/// Camera logic is only active during the State `GameState::Playing`
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_game_camera)
            .add_systems(Update, follow_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Off,
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
        DespawnOnExit(GameState::Playing),
    ));
}

/// Moves the camera behind the player.
fn follow_player(
    time: Res<Time>,
    // The queries are disjoint, because both of them access the transform.
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    let target = player_transform.translation + CAMERA_OFFSET;
    let factor = 1.0 - (-CAMERA_FOLLOW_SPEED * time.delta_secs()).exp();
    camera_transform.translation = camera_transform.translation.lerp(target, factor);
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}
