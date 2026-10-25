// use crate::gameplay::player::Player;
// use crate::{GameResult, GameState};
// use bevy::prelude::*;
// use bevy_kira_audio::prelude::*;

// pub struct FinishAreaPlugin;

// #[derive(Component, Debug, PartialEq, Eq)]
// pub enum FinishArea {
//     Win,
//     Lose,
// }

// /// Distance of the finish areas from the player start position (world units).
// const AREA_DISTANCE: f32 = 8.0;
// /// Radius of a finish area (world units).
// const AREA_RADIUS: f32 = 2.0;

// /// This plugin handles finish_areas - when the player touches a finish area the game is over.
// impl Plugin for FinishAreaPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_systems(OnEnter(GameState::Playing), spawn_finish_areas)
//             .add_systems(
//                 Update,
//                 end_game_on_player_touch.run_if(in_state(GameState::Playing)),
//             );
//     }
// }

// fn spawn_finish_areas(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     let area = meshes.add(Cuboid::new(AREA_RADIUS * 2.0, 0.25, AREA_RADIUS * 2.0));

//     commands.spawn((
//         Mesh3d(area.clone()),
//         MeshMaterial3d(materials.add(StandardMaterial {
//             base_color: Color::hsl(120., 0.95, 0.7),
//             ..default()
//         })),
//         Transform::from_translation(Vec3::new(AREA_DISTANCE, 0.125, 0.0)),
//         FinishArea::Win,
//         SphereCollider {
//             radius: AREA_RADIUS,
//         },
//         // The finish areas emit a sound which is attenuated and panned
//         // depending on the distance to the player.
//         SpatialAudioEmitter::default(),
//         DespawnOnExit(GameState::Playing),
//     ));
//     commands.spawn((
//         Mesh3d(area),
//         MeshMaterial3d(materials.add(StandardMaterial {
//             base_color: Color::hsl(0., 0.95, 0.7),
//             ..default()
//         })),
//         Transform::from_translation(Vec3::new(-AREA_DISTANCE, 0.125, 0.0)),
//         FinishArea::Lose,
//         SphereCollider {
//             radius: AREA_RADIUS,
//         },
//         SpatialAudioEmitter::default(),
//         DespawnOnExit(GameState::Playing),
//     ));
// }

// fn end_game_on_player_touch(
//     mut commands: Commands,
//     mut next_state: ResMut<NextState<GameState>>,
//     player_query: Query<(&Transform, &SphereCollider), With<Player>>,
//     finish_query: Query<(&Transform, &SphereCollider, &FinishArea)>,
// ) {
//     // Normally there is a single player, but the loop works for any count.
//     for (player_tf, player_sphere) in player_query.iter() {
//         for (finish_tf, finish_sphere, finish_area) in finish_query.iter() {
//             // Vector between the two centres.
//             let delta = finish_tf.translation - player_tf.translation;
//             // Squared distance (cheaper than sqrt).
//             let dist_sq = delta.length_squared();
//             // Sum of radii.
//             let radius_sum = player_sphere.radius + finish_sphere.radius;

//             // Collision test.
//             if dist_sq <= radius_sum * radius_sum {
//                 match finish_area {
//                     FinishArea::Win => {
//                         info!("Player won that game!");
//                         commands.insert_resource(GameResult::Won);
//                     }
//                     FinishArea::Lose => {
//                         info!("Player lost that game!");
//                         commands.insert_resource(GameResult::Lost);
//                     }
//                 }
//                 next_state.set(GameState::Menu);
//                 return;
//             }
//         }
//     }
// }
