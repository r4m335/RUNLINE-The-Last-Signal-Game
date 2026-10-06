use bevy::prelude::*;
use crate::types::*;
use crate::director::RunDirector;
use crate::pooling::{PoolAssets, EntityPool};
use crate::patterns::{select_validated_pattern, PatternChunk};

const SEGMENT_LENGTH: f32 = 40.0;
const SPAWN_AHEAD_DISTANCE: f32 = 160.0;
const DESPAWN_BEHIND_DISTANCE: f32 = 40.0;

#[derive(Resource)]
pub struct TrackManager {
    pub next_spawn_z: f32,
    pub last_chunk: Option<PatternChunk>,
}

impl Default for TrackManager {
    fn default() -> Self {
        Self {
            next_spawn_z: 20.0,
            last_chunk: None,
        }
    }
}

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TrackManager>()
            .add_systems(Update, handle_run_reset_track)
            .add_systems(
                Update,
                (
                    maintain_rolling_track,
                    despawn_distant_entities,
                    handle_zone_lighting_events,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn handle_run_reset_track(
    mut events: EventReader<RunResetEvent>,
    mut commands: Commands,
    mut track_mgr: ResMut<TrackManager>,
    env_assets: Res<crate::environment::EnvironmentAssets>,
    prop_assets: Option<Res<crate::environment_props::PropAssets>>,
    signage_assets: Option<Res<crate::environment_signage::SignageAssets>>,
    pool_assets: Res<PoolAssets>,
    mut pool: ResMut<EntityPool>,
    director: Res<RunDirector>,
    stats: Res<GameRunStats>,
    segments_q: Query<Entity, With<TrackSegmentMarker>>,
    mut pooled_q: Query<(
        Entity,
        &PooledItem,
        Option<&mut Transform>,
        Option<&mut Visibility>,
    ), With<Despawnable>>,
    unpooled_q: Query<Entity, (Or<(With<ActiveObstacle>, With<CollectibleItem>)>, Without<PooledItem>)>,
) {
    for _ in events.read() {
        // 1. Recycle all active pooled obstacles & items cleanly back to pool
        for (entity, pooled, mut trans_opt, mut vis_opt) in pooled_q.iter_mut() {
            if let Some(ref mut vis) = vis_opt {
                **vis = Visibility::Hidden;
            }
            if let Some(ref mut t) = trans_opt {
                t.translation = Vec3::new(0.0, -9999.0, 0.0);
            }
            commands.entity(entity)
                .remove::<Despawnable>()
                .remove::<ActiveObstacle>()
                .remove::<CollectibleItem>()
                .remove::<MovingObstacle>();
            pool.push(pooled.pool_type, entity);
        }

        // 1b. Despawn any unpooled items to prevent entity leaks
        for unpooled in unpooled_q.iter() {
            commands.entity(unpooled).despawn_recursive();
        }

        // 2. Despawn existing road segments
        for seg in segments_q.iter() {
            commands.entity(seg).despawn_recursive();
        }

        // 3. Reset manager state & spawn 5 clean rolling segments ahead
        track_mgr.next_spawn_z = 20.0;
        track_mgr.last_chunk = None;

        for _ in 0..5 {
            let z_center = track_mgr.next_spawn_z - SEGMENT_LENGTH * 0.5;
            let z_start = track_mgr.next_spawn_z;

            spawn_segment(
                &mut commands,
                &env_assets,
                prop_assets.as_deref(),
                signage_assets.as_deref(),
                z_center,
                SEGMENT_LENGTH,
                (-z_center).max(0.0),
            );

            if track_mgr.next_spawn_z < -40.0 {
                let last_chunk_ref = track_mgr.last_chunk.clone();
                let chunk = spawn_pattern_chunk(
                    &mut commands,
                    &pool_assets,
                    &mut pool,
                    z_start,
                    director.profile.pattern_complexity,
                    last_chunk_ref.as_ref(),
                    stats.speed,
                    0.0,
                );
                track_mgr.last_chunk = Some(chunk);
            }

            track_mgr.next_spawn_z -= SEGMENT_LENGTH;
        }
    }
}

// -------------------------------------------------------------
// ROLLING WORLD WINDOW & PROCEDURAL PATTERN GENERATION
// -------------------------------------------------------------
fn maintain_rolling_track(
    mut commands: Commands,
    mut track_mgr: ResMut<TrackManager>,
    env_assets: Res<crate::environment::EnvironmentAssets>,
    prop_assets: Option<Res<crate::environment_props::PropAssets>>,
    signage_assets: Option<Res<crate::environment_signage::SignageAssets>>,
    pool_assets: Res<PoolAssets>,
    mut pool: ResMut<EntityPool>,
    player_q: Query<&Transform, With<Player>>,
    director: Res<RunDirector>,
    stats: Res<GameRunStats>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    // Maintain fixed window ahead of player
    while track_mgr.next_spawn_z > p_trans.translation.z - SPAWN_AHEAD_DISTANCE {
        let z_center = track_mgr.next_spawn_z - SEGMENT_LENGTH * 0.5;
        let z_start = track_mgr.next_spawn_z;

        spawn_segment(
            &mut commands,
            &env_assets,
            prop_assets.as_deref(),
            signage_assets.as_deref(),
            z_center,
            SEGMENT_LENGTH,
            (-z_center).max(0.0),
        );

        let last_chunk_ref = track_mgr.last_chunk.clone();
        let chunk = spawn_pattern_chunk(
            &mut commands,
            &pool_assets,
            &mut pool,
            z_start,
            director.profile.pattern_complexity,
            last_chunk_ref.as_ref(),
            stats.speed,
            stats.distance,
        );
        track_mgr.last_chunk = Some(chunk);

        track_mgr.next_spawn_z -= SEGMENT_LENGTH;
    }
}

// -------------------------------------------------------------
// OBJECT POOLING SPAWNER WITH SOLVABILITY VALIDATION
// -------------------------------------------------------------
pub fn spawn_pattern_chunk(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    z_start: f32,
    complexity: u8,
    prev_chunk: Option<&PatternChunk>,
    player_speed: f32,
    distance: f32,
) -> PatternChunk {
    use rand::Rng;
    let chunk = select_validated_pattern(complexity, prev_chunk, player_speed, distance);
    let mut rng = rand::thread_rng();

    // 1. Coordinated obstacle placement with guaranteed physical clearance
    for obs_def in &chunk.obstacles {
        let world_z = z_start + obs_def.rel_z;
        let lane_x = obs_def.lane.x_pos();

        match obs_def.obstacle_type {
            ObstacleType::LowBarrier => {
                if let Some(entity) = pool.pop(PoolType::LowBarrier) {
                    commands.entity(entity).insert((
                        Transform::from_xyz(lane_x, 0.42, world_z),
                        Visibility::Inherited,
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::LowBarrier,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::LowBarrier, is_active: true },
                    ));
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: pool_assets.mesh_barrier_rail.clone(),
                            material: pool_assets.mat_hazard.clone(),
                            transform: Transform::from_xyz(lane_x, 0.42, world_z),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::LowBarrier,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::LowBarrier, is_active: true },
                    )).with_children(|barrier| {
                        // Left & right steel stanchions
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_stanchion.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(-0.95, -0.05, 0.0),
                            ..default()
                        });
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_stanchion.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(0.95, -0.05, 0.0),
                            ..default()
                        });
                        // Ground mounting footplates
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_foot.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(-0.95, -0.40, 0.0),
                            ..default()
                        });
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_foot.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(0.95, -0.40, 0.0),
                            ..default()
                        });
                        // High-visibility amber LED warning top strip
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_led_strip.clone(),
                            material: pool_assets.mat_barrier_led.clone(),
                            transform: Transform::from_xyz(0.0, 0.25, 0.0),
                            ..default()
                        });
                        // Stanchion warning strobe caps (Jump cues)
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_strobe.clone(),
                            material: pool_assets.mat_barrier_led.clone(),
                            transform: Transform::from_xyz(-0.95, 0.35, 0.0),
                            ..default()
                        });
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_strobe.clone(),
                            material: pool_assets.mat_barrier_led.clone(),
                            transform: Transform::from_xyz(0.95, 0.35, 0.0),
                            ..default()
                        });
                        // Center hydraulic lock housing
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_lock_box.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(0.0, 0.05, 0.12),
                            ..default()
                        });
                        // Lower kick skirt
                        barrier.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_skirt.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(0.0, -0.22, 0.0),
                            ..default()
                        });
                    });
                }
            }
            ObstacleType::HighHangingWire => {
                if let Some(entity) = pool.pop(PoolType::HighHangingWire) {
                    commands.entity(entity).insert((
                        Transform::from_xyz(lane_x, 1.7, world_z),
                        Visibility::Inherited,
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::HighHangingWire,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::HighHangingWire, is_active: true },
                    ));
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: pool_assets.mesh_wire_bundle.clone(),
                            material: pool_assets.mat_wire_insulation.clone(),
                            transform: Transform::from_xyz(lane_x, 1.7, world_z),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::HighHangingWire,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::HighHangingWire, is_active: true },
                    )).with_children(|wire| {
                        // Upper catenary structural conduit
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_catenary.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(0.0, 0.22, 0.0),
                            ..default()
                        });
                        // Center overhead red danger beacon
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_barrier_strobe.clone(),
                            material: pool_assets.mat_wire_beacon.clone(),
                            transform: Transform::from_xyz(0.0, 0.32, 0.0),
                            ..default()
                        });
                        // Left & right ceiling anchor mounts
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_anchor.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(-1.05, 0.20, 0.0),
                            ..default()
                        });
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_anchor.clone(),
                            material: pool_assets.mat_barrier_steel.clone(),
                            transform: Transform::from_xyz(1.05, 0.20, 0.0),
                            ..default()
                        });
                        // Dangling severed copper leads (live sparking)
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_dangle.clone(),
                            material: pool_assets.mat_wire_copper.clone(),
                            transform: Transform::from_xyz(-0.55, -0.16, 0.0),
                            ..default()
                        });
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_dangle.clone(),
                            material: pool_assets.mat_wire_copper.clone(),
                            transform: Transform::from_xyz(0.45, -0.18, 0.0),
                            ..default()
                        });
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_dangle.clone(),
                            material: pool_assets.mat_wire_copper.clone(),
                            transform: Transform::from_xyz(-0.15, -0.22, 0.0),
                            ..default()
                        });
                        // Live high-voltage spark cores (piercing electric arc yellow-white)
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_spark.clone(),
                            material: pool_assets.mat_wire_spark.clone(),
                            transform: Transform::from_xyz(-0.55, -0.26, 0.0),
                            ..default()
                        });
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_spark.clone(),
                            material: pool_assets.mat_wire_spark.clone(),
                            transform: Transform::from_xyz(0.45, -0.28, 0.0),
                            ..default()
                        });
                        // Downward clearance guide indicator - highlights the open void underneath for sliding!
                        wire.spawn(PbrBundle {
                            mesh: pool_assets.mesh_wire_chevron.clone(),
                            material: pool_assets.mat_wire_chevron.clone(),
                            transform: Transform::from_xyz(0.0, -0.34, 0.0),
                            ..default()
                        });
                    });
                }
            }
            ObstacleType::TallPillar => {
                if let Some(entity) = pool.pop(PoolType::TallPillar) {
                    commands.entity(entity).insert((
                        Transform::from_xyz(lane_x, 2.0, world_z),
                        Visibility::Inherited,
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::TallPillar,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::TallPillar, is_active: true },
                    ));
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: pool_assets.mesh_pillar_body.clone(),
                            material: pool_assets.mat_pillar_armor.clone(),
                            transform: Transform::from_xyz(lane_x, 2.0, world_z),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::TallPillar,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::TallPillar, is_active: true },
                    )).with_children(|pillar| {
                        // Flared concrete pedestal base
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_base.clone(),
                            material: pool_assets.mat_pillar_pedestal.clone(),
                            transform: Transform::from_xyz(0.0, -1.75, 0.0),
                            ..default()
                        });
                        // Left & right armored conduit trunks
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_trunk.clone(),
                            material: pool_assets.mat_pillar_armor.clone(),
                            transform: Transform::from_xyz(-0.82, 0.0, 0.0),
                            ..default()
                        });
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_trunk.clone(),
                            material: pool_assets.mat_pillar_armor.clone(),
                            transform: Transform::from_xyz(0.82, 0.0, 0.0),
                            ..default()
                        });
                        // Front warning placard plate
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_placard.clone(),
                            material: pool_assets.mat_hazard.clone(),
                            transform: Transform::from_xyz(0.0, 0.20, 0.54),
                            ..default()
                        });
                        // Left & right perimeter vertical red LED warning strips
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_beacon.clone(),
                            material: pool_assets.mat_pillar_beacon.clone(),
                            transform: Transform::from_xyz(-0.84, 0.0, 0.54),
                            ..default()
                        });
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_beacon.clone(),
                            material: pool_assets.mat_pillar_beacon.clone(),
                            transform: Transform::from_xyz(0.84, 0.0, 0.54),
                            ..default()
                        });
                        // Top structural collar
                        pillar.spawn(PbrBundle {
                            mesh: pool_assets.mesh_pillar_collar.clone(),
                            material: pool_assets.mat_pillar_armor.clone(),
                            transform: Transform::from_xyz(0.0, 1.85, 0.0),
                            ..default()
                        });
                    });
                }
            }
            ObstacleType::StaticTrain => {
                if let Some(entity) = pool.pop(PoolType::StaticTrain) {
                    commands.entity(entity).insert((
                        Transform::from_xyz(lane_x, 1.25, world_z),
                        Visibility::Inherited,
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::StaticTrain,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::StaticTrain, is_active: true },
                    ));
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: pool_assets.mesh_train_static.clone(),
                            material: pool_assets.mat_train_body.clone(),
                            transform: Transform::from_xyz(lane_x, 1.25, world_z),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::StaticTrain,
                            size: obs_def.size,
                        },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::StaticTrain, is_active: true },
                    )).with_children(|train| {
                        // Front bumper cowcatcher
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bumper.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -0.85, 6.15),
                            ..default()
                        });
                        // Driver cab windscreen
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windscreen.clone(),
                            material: pool_assets.mat_train_windscreen.clone(),
                            transform: Transform::from_xyz(0.0, 0.45, 6.05),
                            ..default()
                        });
                        // Twin headlights
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_headlamp.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(-0.72, 0.10, 6.10),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_headlamp.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(0.72, 0.10, 6.10),
                            ..default()
                        });
                        // Upper red marker lights
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_marker_red.clone(),
                            material: pool_assets.mat_train_marker_red.clone(),
                            transform: Transform::from_xyz(-0.70, 1.05, 6.05),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_marker_red.clone(),
                            material: pool_assets.mat_train_marker_red.clone(),
                            transform: Transform::from_xyz(0.70, 1.05, 6.05),
                            ..default()
                        });
                        // Rooftop HVAC pod
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_hvac.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, 1.30, 0.0),
                            ..default()
                        });
                        // Bogies
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bogie.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -1.10, 3.8),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bogie.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -1.10, -3.8),
                            ..default()
                        });
                        // Side illuminated passenger window bands
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windows_static.clone(),
                            material: pool_assets.mat_train_windows.clone(),
                            transform: Transform::from_xyz(-1.11, 0.35, 0.0),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windows_static.clone(),
                            material: pool_assets.mat_train_windows.clone(),
                            transform: Transform::from_xyz(1.11, 0.35, 0.0),
                            ..default()
                        });
                    });
                }
            }
            ObstacleType::MovingTrain { speed } => {
                if let Some(entity) = pool.pop(PoolType::MovingTrain) {
                    commands.entity(entity).insert((
                        Transform::from_xyz(lane_x, 1.25, world_z),
                        Visibility::Inherited,
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::MovingTrain { speed },
                            size: obs_def.size,
                        },
                        MovingObstacle { speed },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::MovingTrain, is_active: true },
                    ));
                } else {
                    commands.spawn((
                        PbrBundle {
                            mesh: pool_assets.mesh_train_moving.clone(),
                            material: pool_assets.mat_train_body.clone(),
                            transform: Transform::from_xyz(lane_x, 1.25, world_z),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: obs_def.lane,
                            obstacle_type: ObstacleType::MovingTrain { speed },
                            size: obs_def.size,
                        },
                        MovingObstacle { speed },
                        Despawnable { z_center: world_z },
                        PooledItem { pool_type: PoolType::MovingTrain, is_active: true },
                    )).with_children(|train| {
                        // Front bumper cowcatcher
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bumper.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -0.85, 7.15),
                            ..default()
                        });
                        // Driver cab windscreen
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windscreen.clone(),
                            material: pool_assets.mat_train_windscreen.clone(),
                            transform: Transform::from_xyz(0.0, 0.45, 7.05),
                            ..default()
                        });
                        // Twin headlights
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_headlamp.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(-0.72, 0.10, 7.10),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_headlamp.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(0.72, 0.10, 7.10),
                            ..default()
                        });
                        // Upper red marker lights
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_marker_red.clone(),
                            material: pool_assets.mat_train_marker_red.clone(),
                            transform: Transform::from_xyz(-0.70, 1.05, 7.05),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_marker_red.clone(),
                            material: pool_assets.mat_train_marker_red.clone(),
                            transform: Transform::from_xyz(0.70, 1.05, 7.05),
                            ..default()
                        });
                        // Rooftop HVAC pod
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_hvac.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, 1.30, 0.0),
                            ..default()
                        });
                        // Bogies
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bogie.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -1.10, 4.8),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_bogie.clone(),
                            material: pool_assets.mat_train_chassis.clone(),
                            transform: Transform::from_xyz(0.0, -1.10, -4.8),
                            ..default()
                        });
                        // Side illuminated passenger window bands
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windows_moving.clone(),
                            material: pool_assets.mat_train_windows.clone(),
                            transform: Transform::from_xyz(-1.11, 0.35, 0.0),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_windows_moving.clone(),
                            material: pool_assets.mat_train_windows.clone(),
                            transform: Transform::from_xyz(1.11, 0.35, 0.0),
                            ..default()
                        });
                    });
                }
            }
        }
    }

    // 2. Guiding crystal arc / resonance trail
    for frag_def in &chunk.fragments {
        let world_z = z_start + frag_def.rel_z;
        let lane_x = frag_def.lane.x_pos();

        if let Some(entity) = pool.pop(PoolType::EchoFragment) {
            commands.entity(entity).insert((
                Transform::from_xyz(lane_x, frag_def.y_pos, world_z),
                Visibility::Inherited,
                CollectibleItem {
                    item_type: CollectibleType::EchoFragment { value: 1 },
                    lane: frag_def.lane,
                    initial_y: frag_def.y_pos,
                    rot_speed: 3.5,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::EchoFragment, is_active: true },
            ));
        } else {
            commands.spawn((
                PbrBundle {
                    mesh: pool_assets.mesh_fragment.clone(),
                    material: pool_assets.mat_fragment.clone(),
                    transform: Transform::from_xyz(lane_x, frag_def.y_pos, world_z),
                    ..default()
                },
                CollectibleItem {
                    item_type: CollectibleType::EchoFragment { value: 1 },
                    lane: frag_def.lane,
                    initial_y: frag_def.y_pos,
                    rot_speed: 3.5,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::EchoFragment, is_active: true },
            ));
        }
    }

    // 3. Optional Rare Data Chip (15% chance)
    if rng.gen_bool(0.15) {
        let chip_lane = Lane::from_index(rng.gen_range(-1..=1));
        let world_z = z_start - 24.0;
        let index = rng.gen_range(0..crate::story::DATA_CHIP_LOGS.len());

        if let Some(entity) = pool.pop(PoolType::DataChip) {
            commands.entity(entity).insert((
                Transform::from_xyz(chip_lane.x_pos(), 1.1, world_z),
                Visibility::Inherited,
                CollectibleItem {
                    item_type: CollectibleType::DataChip { index },
                    lane: chip_lane,
                    initial_y: 1.1,
                    rot_speed: 2.2,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::DataChip, is_active: true },
            ));
        } else {
            commands.spawn((
                PbrBundle {
                    mesh: pool_assets.mesh_chip.clone(),
                    material: pool_assets.mat_chip.clone(),
                    transform: Transform::from_xyz(chip_lane.x_pos(), 1.1, world_z),
                    ..default()
                },
                CollectibleItem {
                    item_type: CollectibleType::DataChip { index },
                    lane: chip_lane,
                    initial_y: 1.1,
                    rot_speed: 2.2,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::DataChip, is_active: true },
            ));
        }
    }

    // 4. Optional Rare Power-Up (18% chance)
    if rng.gen_bool(0.18) {
        let p_lane = Lane::from_index(rng.gen_range(-1..=1));
        let world_z = z_start - 34.0;
        let roll = rng.gen_range(0..4);
        let p_type = match roll {
            0 => CollectibleType::EchoShield,
            1 => CollectibleType::Magnet,
            2 => CollectibleType::Overdrive,
            _ => CollectibleType::DoubleJump,
        };

        if let Some(entity) = pool.pop(PoolType::PowerUp) {
            commands.entity(entity).insert((
                Transform::from_xyz(p_lane.x_pos(), 1.2, world_z),
                Visibility::Inherited,
                CollectibleItem {
                    item_type: p_type,
                    lane: p_lane,
                    initial_y: 1.2,
                    rot_speed: 4.0,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::PowerUp, is_active: true },
            ));
        } else {
            commands.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(p_lane.x_pos(), 1.2, world_z),
                    ..default()
                },
                CollectibleItem {
                    item_type: p_type,
                    lane: p_lane,
                    initial_y: 1.2,
                    rot_speed: 4.0,
                },
                Despawnable { z_center: world_z },
                PooledItem { pool_type: PoolType::PowerUp, is_active: true },
            )).with_children(|parent| {
                match p_type {
                    CollectibleType::EchoShield => {
                        // Hexagonal core
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_hex.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            ..default()
                        });
                        // 3 orbiting deflector plates
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_plate.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.38),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_plate.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            transform: Transform::from_xyz(-0.33, 0.0, -0.19).with_rotation(Quat::from_rotation_y(2.094)),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_plate.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            transform: Transform::from_xyz(0.33, 0.0, -0.19).with_rotation(Quat::from_rotation_y(-2.094)),
                            ..default()
                        });
                        // Translucent energy ring
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_ring.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            ..default()
                        });
                    }
                    CollectibleType::Magnet => {
                        // Horseshoe top arch
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_arch.clone(),
                            material: pool_assets.mat_powerup_magnet.clone(),
                            transform: Transform::from_xyz(0.0, 0.22, 0.0),
                            ..default()
                        });
                        // Left & right downward prongs
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_prong.clone(),
                            material: pool_assets.mat_powerup_magnet.clone(),
                            transform: Transform::from_xyz(-0.20, -0.06, 0.0),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_prong.clone(),
                            material: pool_assets.mat_powerup_magnet.clone(),
                            transform: Transform::from_xyz(0.20, -0.06, 0.0),
                            ..default()
                        });
                        // Silver/chrome contact poles
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_pole.clone(),
                            material: pool_assets.mat_magnet_pole.clone(),
                            transform: Transform::from_xyz(-0.20, -0.32, 0.0),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_pole.clone(),
                            material: pool_assets.mat_magnet_pole.clone(),
                            transform: Transform::from_xyz(0.20, -0.32, 0.0),
                            ..default()
                        });
                        // Orbiting flux ring
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_ring.clone(),
                            material: pool_assets.mat_powerup_magnet.clone(),
                            transform: Transform::from_xyz(0.0, -0.08, 0.0),
                            ..default()
                        });
                    }
                    CollectibleType::Overdrive => {
                        // Elongated diamond turbine spike
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_overdrive_diamond.clone(),
                            material: pool_assets.mat_powerup_overdrive.clone(),
                            ..default()
                        });
                        // Dual tilted gyro rings
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_overdrive_ring.clone(),
                            material: pool_assets.mat_powerup_overdrive.clone(),
                            transform: Transform::from_rotation(Quat::from_rotation_x(0.785)),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_overdrive_ring.clone(),
                            material: pool_assets.mat_powerup_overdrive.clone(),
                            transform: Transform::from_rotation(Quat::from_rotation_z(0.785)),
                            ..default()
                        });
                    }
                    _ => {
                        // Double Jump: dual upward winged chevrons & double thrust rings
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_jump_chevron.clone(),
                            material: pool_assets.mat_powerup_jump.clone(),
                            transform: Transform::from_xyz(0.0, -0.08, 0.0),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_jump_chevron.clone(),
                            material: pool_assets.mat_powerup_jump.clone(),
                            transform: Transform::from_xyz(0.0, 0.12, 0.0),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_jump_ring.clone(),
                            material: pool_assets.mat_powerup_jump.clone(),
                            transform: Transform::from_xyz(0.0, -0.26, 0.0),
                            ..default()
                        });
                    }
                }
            });
        }
    }

    chunk
}

// -------------------------------------------------------------
// RECYCLING & DESPAWNING
// -------------------------------------------------------------
fn despawn_distant_entities(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut despawn_q: Query<(
        Entity,
        &Despawnable,
        Option<&PooledItem>,
        Option<&mut Transform>,
        Option<&mut Visibility>,
    ), Without<Player>>,
    mut pool: ResMut<EntityPool>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let despawn_z = p_trans.translation.z + DESPAWN_BEHIND_DISTANCE;

    for (entity, despawn, pooled_opt, mut trans_opt, mut vis_opt) in despawn_q.iter_mut() {
        if despawn.z_center > despawn_z {
            if let Some(pooled) = pooled_opt {
                // Recycle into pool! Zero entity allocations!
                if let Some(ref mut vis) = vis_opt {
                    **vis = Visibility::Hidden;
                }
                if let Some(ref mut t) = trans_opt {
                    t.translation = Vec3::new(0.0, -9999.0, 0.0);
                }
                commands.entity(entity)
                    .remove::<Despawnable>()
                    .remove::<ActiveObstacle>()
                    .remove::<CollectibleItem>()
                    .remove::<MovingObstacle>();
                pool.push(pooled.pool_type, entity);
            } else {
                // Track segment road elements despawn
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// -------------------------------------------------------------
// EVENT-DRIVEN ZONE LIGHTING
// -------------------------------------------------------------
fn handle_zone_lighting_events(
    mut zone_events: EventReader<ZoneChangedEvent>,
    mut clear_color: ResMut<ClearColor>,
    mut dir_light_q: Query<&mut DirectionalLight>,
    mut sfx: EventWriter<SoundEffect>,
) {
    for event in zone_events.read() {
        clear_color.0 = event.config.ambient_color;
        if let Ok(mut dl) = dir_light_q.get_single_mut() {
            dl.color = event.config.directional_color;
        }
        sfx.send(SoundEffect::ZoneTransition);
    }
}

fn spawn_segment(
    commands: &mut Commands,
    env_assets: &crate::environment::EnvironmentAssets,
    prop_assets: Option<&crate::environment_props::PropAssets>,
    signage_assets: Option<&crate::environment_signage::SignageAssets>,
    z_center: f32,
    length: f32,
    distance: f32,
) {
    crate::environment::spawn_modular_environment_slice(
        commands,
        env_assets,
        prop_assets,
        signage_assets,
        z_center,
        length,
        distance,
    );
}
