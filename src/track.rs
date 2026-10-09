use crate::director::RunDirector;
use crate::obstacles::resolve_obstacle_pool_type;
use crate::patterns::{select_validated_pattern, PatternChunk};
use crate::pooling::{EntityPool, PoolAssets};
use crate::types::*;
use bevy::prelude::*;

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
                    handle_zone_transition_events,
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
    neon_assets: Option<Res<crate::neon_district::NeonDistrictAssets>>,
    industrial_assets: Option<Res<crate::industrial_sector::IndustrialSectorAssets>>,
    pool_assets: Res<PoolAssets>,
    powerup_assets: Option<Res<PowerUpModelAssets>>,
    mut pool: ResMut<EntityPool>,
    director: Res<RunDirector>,
    stats: Res<GameRunStats>,
    segments_q: Query<Entity, With<TrackSegmentMarker>>,
    mut pooled_q: Query<
        (
            Entity,
            &PooledItem,
            Option<&mut Transform>,
            Option<&mut Visibility>,
        ),
        With<Despawnable>,
    >,
    unpooled_q: Query<
        Entity,
        (
            Or<(With<ActiveObstacle>, With<CollectibleItem>)>,
            Without<PooledItem>,
        ),
    >,
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
            commands
                .entity(entity)
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
                neon_assets.as_deref(),
                industrial_assets.as_deref(),
                z_center,
                SEGMENT_LENGTH,
                (-z_center).max(0.0),
            );

            if track_mgr.next_spawn_z < -40.0 {
                let last_chunk_ref = track_mgr.last_chunk.clone();
                let chunk = spawn_pattern_chunk(
                    &mut commands,
                    &pool_assets,
                    powerup_assets.as_deref(),
                    &mut pool,
                    z_start,
                    director.profile.pattern_complexity,
                    last_chunk_ref.as_ref(),
                    stats.speed,
                    (-z_start).max(0.0),
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
    neon_assets: Option<Res<crate::neon_district::NeonDistrictAssets>>,
    industrial_assets: Option<Res<crate::industrial_sector::IndustrialSectorAssets>>,
    pool_assets: Res<PoolAssets>,
    powerup_assets: Option<Res<PowerUpModelAssets>>,
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
            neon_assets.as_deref(),
            industrial_assets.as_deref(),
            z_center,
            SEGMENT_LENGTH,
            (-z_center).max(0.0),
        );

        let last_chunk_ref = track_mgr.last_chunk.clone();
        let chunk = spawn_pattern_chunk(
            &mut commands,
            &pool_assets,
            powerup_assets.as_deref(),
            &mut pool,
            z_start,
            director.profile.pattern_complexity,
            last_chunk_ref.as_ref(),
            stats.speed,
            (-z_start).max(0.0),
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
    powerup_assets: Option<&PowerUpModelAssets>,
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
        let obstacle_distance = (-world_z).max(0.0);
        let visual_pool_type = resolve_obstacle_pool_type(obs_def.obstacle_type, obstacle_distance);
        let is_zone_2 = matches!(
            visual_pool_type,
            PoolType::NeonRoadBarrier
                | PoolType::NeonOverheadScanner
                | PoolType::NeonCheckpointPillar
                | PoolType::NeonAutoVan
                | PoolType::NeonCyberBus
        );

        match obs_def.obstacle_type {
            ObstacleType::LowBarrier => {
                if is_zone_2 {
                    if let Some(entity) = pool.pop(PoolType::NeonRoadBarrier) {
                        commands.entity(entity).insert((
                            Transform::from_xyz(lane_x, 0.42, world_z),
                            Visibility::Inherited,
                            ActiveObstacle {
                                lane: obs_def.lane,
                                obstacle_type: ObstacleType::LowBarrier,
                                size: obs_def.size,
                            },
                            Despawnable { z_center: world_z },
                            PooledItem {
                                pool_type: PoolType::NeonRoadBarrier,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_frame.clone(),
                                    material: pool_assets.mat_neon_barricade_frame.clone(),
                                    transform: Transform::from_xyz(lane_x, 0.42, world_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: obs_def.lane,
                                    obstacle_type: ObstacleType::LowBarrier,
                                    size: obs_def.size,
                                },
                                Despawnable { z_center: world_z },
                                PooledItem {
                                    pool_type: PoolType::NeonRoadBarrier,
                                    is_active: true,
                                },
                            ))
                            .with_children(|barricade| {
                                barricade.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_stripe.clone(),
                                    material: pool_assets.mat_neon_barricade_hazard.clone(),
                                    transform: Transform::from_xyz(0.0, 0.0, 0.02),
                                    ..default()
                                });
                                barricade.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_foot.clone(),
                                    material: pool_assets.mat_neon_barricade_frame.clone(),
                                    transform: Transform::from_xyz(-0.95, -0.40, 0.0),
                                    ..default()
                                });
                                barricade.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_foot.clone(),
                                    material: pool_assets.mat_neon_barricade_frame.clone(),
                                    transform: Transform::from_xyz(0.95, -0.40, 0.0),
                                    ..default()
                                });
                                barricade.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_strobe.clone(),
                                    material: pool_assets.mat_neon_barricade_strobe.clone(),
                                    transform: Transform::from_xyz(-0.95, 0.28, 0.0),
                                    ..default()
                                });
                                barricade.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_barricade_strobe.clone(),
                                    material: pool_assets.mat_neon_barricade_strobe.clone(),
                                    transform: Transform::from_xyz(0.95, 0.28, 0.0),
                                    ..default()
                                });
                            });
                    }
                } else {
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
                            PooledItem {
                                pool_type: PoolType::LowBarrier,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
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
                                PooledItem {
                                    pool_type: PoolType::LowBarrier,
                                    is_active: true,
                                },
                            ))
                            .with_children(|barrier| {
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
            }
            ObstacleType::HighHangingWire => {
                if is_zone_2 {
                    if let Some(entity) = pool.pop(PoolType::NeonOverheadScanner) {
                        commands.entity(entity).insert((
                            Transform::from_xyz(lane_x, 1.7, world_z),
                            Visibility::Inherited,
                            ActiveObstacle {
                                lane: obs_def.lane,
                                obstacle_type: ObstacleType::HighHangingWire,
                                size: obs_def.size,
                            },
                            Despawnable { z_center: world_z },
                            PooledItem {
                                pool_type: PoolType::NeonOverheadScanner,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: pool_assets.mesh_neon_scanner_gantry.clone(),
                                    material: pool_assets.mat_neon_scanner_metal.clone(),
                                    transform: Transform::from_xyz(lane_x, 1.7, world_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: obs_def.lane,
                                    obstacle_type: ObstacleType::HighHangingWire,
                                    size: obs_def.size,
                                },
                                Despawnable { z_center: world_z },
                                PooledItem {
                                    pool_type: PoolType::NeonOverheadScanner,
                                    is_active: true,
                                },
                            ))
                            .with_children(|scanner| {
                                // Downward scanner sensor housing
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_scanner_housing.clone(),
                                    material: pool_assets.mat_neon_scanner_metal.clone(),
                                    transform: Transform::from_xyz(0.0, -0.06, 0.0),
                                    ..default()
                                });
                                // Downward laser scanning grid emitter
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_scanner_laser.clone(),
                                    material: pool_assets.mat_neon_scanner_laser.clone(),
                                    transform: Transform::from_xyz(0.0, -0.20, 0.0),
                                    ..default()
                                });
                                // Optical slide warning indicator arrows (left & right)
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_scanner_arrow.clone(),
                                    material: pool_assets.mat_neon_scanner_warning.clone(),
                                    transform: Transform::from_xyz(-0.55, -0.16, 0.18),
                                    ..default()
                                });
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_scanner_arrow.clone(),
                                    material: pool_assets.mat_neon_scanner_warning.clone(),
                                    transform: Transform::from_xyz(0.55, -0.16, 0.18),
                                    ..default()
                                });
                                // Ceiling mounting anchor pylons
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_wire_anchor.clone(),
                                    material: pool_assets.mat_neon_scanner_metal.clone(),
                                    transform: Transform::from_xyz(-1.05, 0.18, 0.0),
                                    ..default()
                                });
                                scanner.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_wire_anchor.clone(),
                                    material: pool_assets.mat_neon_scanner_metal.clone(),
                                    transform: Transform::from_xyz(1.05, 0.18, 0.0),
                                    ..default()
                                });
                            });
                    }
                } else {
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
                            PooledItem {
                                pool_type: PoolType::HighHangingWire,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
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
                                PooledItem {
                                    pool_type: PoolType::HighHangingWire,
                                    is_active: true,
                                },
                            ))
                            .with_children(|wire| {
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
            }
            ObstacleType::TallPillar => {
                if is_zone_2 {
                    if let Some(entity) = pool.pop(PoolType::NeonCheckpointPillar) {
                        commands.entity(entity).insert((
                            Transform::from_xyz(lane_x, 2.0, world_z),
                            Visibility::Inherited,
                            ActiveObstacle {
                                lane: obs_def.lane,
                                obstacle_type: ObstacleType::TallPillar,
                                size: obs_def.size,
                            },
                            Despawnable { z_center: world_z },
                            PooledItem {
                                pool_type: PoolType::NeonCheckpointPillar,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: pool_assets.mesh_neon_checkpoint_column.clone(),
                                    material: pool_assets.mat_neon_checkpoint_armor.clone(),
                                    transform: Transform::from_xyz(lane_x, 2.0, world_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: obs_def.lane,
                                    obstacle_type: ObstacleType::TallPillar,
                                    size: obs_def.size,
                                },
                                Despawnable { z_center: world_z },
                                PooledItem {
                                    pool_type: PoolType::NeonCheckpointPillar,
                                    is_active: true,
                                },
                            ))
                            .with_children(|pillar| {
                                // Top surveillance/security collar
                                pillar.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_checkpoint_collar.clone(),
                                    material: pool_assets.mat_neon_checkpoint_armor.clone(),
                                    transform: Transform::from_xyz(0.0, 1.85, 0.0),
                                    ..default()
                                });
                                // Bottom reinforced foundation
                                pillar.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_pillar_base.clone(),
                                    material: pool_assets.mat_neon_checkpoint_armor.clone(),
                                    transform: Transform::from_xyz(0.0, -1.75, 0.0),
                                    ..default()
                                });
                                // Left & right perimeter status beacons (vertical red warning strips)
                                pillar.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_checkpoint_beacon.clone(),
                                    material: pool_assets.mat_neon_checkpoint_red.clone(),
                                    transform: Transform::from_xyz(-0.80, 0.0, 0.54),
                                    ..default()
                                });
                                pillar.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_checkpoint_beacon.clone(),
                                    material: pool_assets.mat_neon_checkpoint_red.clone(),
                                    transform: Transform::from_xyz(0.80, 0.0, 0.54),
                                    ..default()
                                });
                                // Holographic warning sign / lane-blocked HUD screen
                                pillar.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_checkpoint_sign.clone(),
                                    material: pool_assets.mat_neon_checkpoint_display.clone(),
                                    transform: Transform::from_xyz(0.0, 0.30, 0.56),
                                    ..default()
                                });
                            });
                    }
                } else {
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
                            PooledItem {
                                pool_type: PoolType::TallPillar,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
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
                                PooledItem {
                                    pool_type: PoolType::TallPillar,
                                    is_active: true,
                                },
                            ))
                            .with_children(|pillar| {
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
            }
            ObstacleType::StaticTrain => {
                if is_zone_2 {
                    if let Some(entity) = pool.pop(PoolType::NeonAutoVan) {
                        commands.entity(entity).insert((
                            Transform::from_xyz(lane_x, 1.25, world_z),
                            Visibility::Inherited,
                            ActiveObstacle {
                                lane: obs_def.lane,
                                obstacle_type: ObstacleType::StaticTrain,
                                size: obs_def.size,
                            },
                            Despawnable { z_center: world_z },
                            PooledItem {
                                pool_type: PoolType::NeonAutoVan,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_body.clone(),
                                    material: pool_assets.mat_neon_van_body.clone(),
                                    transform: Transform::from_xyz(lane_x, 1.25, world_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: obs_def.lane,
                                    obstacle_type: ObstacleType::StaticTrain,
                                    size: obs_def.size,
                                },
                                Despawnable { z_center: world_z },
                                PooledItem {
                                    pool_type: PoolType::NeonAutoVan,
                                    is_active: true,
                                },
                            ))
                            .with_children(|van| {
                                // Shaped lower nose and raised autonomous cabin keep the
                                // silhouette readable without changing the collision root.
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_nose.clone(),
                                    material: pool_assets.mat_neon_van_body.clone(),
                                    transform: Transform::from_xyz(0.0, -0.42, 4.48),
                                    ..default()
                                });
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_hood.clone(),
                                    material: pool_assets.mat_neon_van_trim.clone(),
                                    transform: Transform::from_xyz(0.0, 0.20, 3.48),
                                    ..default()
                                });
                                // Upper delivery cargo pod & cabin
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_cabin.clone(),
                                    material: pool_assets.mat_neon_van_cabin.clone(),
                                    transform: Transform::from_xyz(0.0, 0.75, 0.8),
                                    ..default()
                                });
                                // Sloped sensor windshield (facing Kai at +Z)
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_train_windscreen.clone(),
                                    material: pool_assets.mat_neon_van_glass.clone(),
                                    transform: Transform::from_xyz(0.0, 0.66, 3.62)
                                        .with_rotation(Quat::from_rotation_x(-0.18)),
                                    ..default()
                                });
                                // Separate front headlights, rather than one emissive bar.
                                for &x in &[-0.66, 0.66] {
                                    van.spawn(PbrBundle {
                                        mesh: pool_assets.mesh_neon_van_headlamp_unit.clone(),
                                        material: pool_assets.mat_neon_van_headlight.clone(),
                                        transform: Transform::from_xyz(x, -0.08, 4.98),
                                        ..default()
                                    });
                                }
                                // Front full-width light blade adds a recognizable vehicle face.
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_headlights.clone(),
                                    material: pool_assets.mat_neon_van_headlight.clone(),
                                    transform: Transform::from_xyz(0.0, -0.28, 5.00),
                                    ..default()
                                });
                                // Rear red laser taillights (at -Z)
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_taillights.clone(),
                                    material: pool_assets.mat_neon_van_taillight.clone(),
                                    transform: Transform::from_xyz(0.0, -0.10, -4.91),
                                    ..default()
                                });
                                // Maglev skid runners (left & right)
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_skid.clone(),
                                    material: pool_assets.mat_neon_van_rubber.clone(),
                                    transform: Transform::from_xyz(-0.92, -0.75, 0.0),
                                    ..default()
                                });
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_skid.clone(),
                                    material: pool_assets.mat_neon_van_rubber.clone(),
                                    transform: Transform::from_xyz(0.92, -0.75, 0.0),
                                    ..default()
                                });
                                // Four wheel assemblies and reused metallic rims.
                                for &x in &[-1.04, 1.04] {
                                    for &z in &[-3.05, 3.05] {
                                        van.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_van_wheel.clone(),
                                            material: pool_assets.mat_neon_van_rubber.clone(),
                                            transform: Transform::from_xyz(x, -0.66, z)
                                                .with_rotation(Quat::from_rotation_z(
                                                    std::f32::consts::FRAC_PI_2,
                                                )),
                                            ..default()
                                        });
                                        van.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_van_rim.clone(),
                                            material: pool_assets.mat_neon_van_rim.clone(),
                                            transform: Transform::from_xyz(
                                                x + if x < 0.0 { -0.11 } else { 0.11 },
                                                -0.66,
                                                z,
                                            )
                                            .with_rotation(Quat::from_rotation_z(
                                                std::f32::consts::FRAC_PI_2,
                                            )),
                                            ..default()
                                        });
                                    }
                                }
                                // Door seams, fender strips, side mirrors and a small delivery mark.
                                for &x in &[-1.071, 1.071] {
                                    for &z in &[-1.25, 1.05] {
                                        van.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_van_panel.clone(),
                                            material: pool_assets.mat_neon_van_trim.clone(),
                                            transform: Transform::from_xyz(x, 0.58, z),
                                            ..default()
                                        });
                                    }
                                    van.spawn(PbrBundle {
                                        mesh: pool_assets.mesh_neon_van_sensor.clone(),
                                        material: pool_assets.mat_neon_van_marking.clone(),
                                        transform: Transform::from_xyz(x, 1.16, 3.18),
                                        ..default()
                                    });
                                }
                                // Side digital cargo route displays (left & right)
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_display.clone(),
                                    material: pool_assets.mat_neon_van_display.clone(),
                                    transform: Transform::from_xyz(-1.06, 0.25, -0.40),
                                    ..default()
                                });
                                van.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_van_display.clone(),
                                    material: pool_assets.mat_neon_van_display.clone(),
                                    transform: Transform::from_xyz(1.06, 0.25, -0.40),
                                    ..default()
                                });
                            });
                    }
                } else {
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
                            PooledItem {
                                pool_type: PoolType::StaticTrain,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
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
                                PooledItem {
                                    pool_type: PoolType::StaticTrain,
                                    is_active: true,
                                },
                            ))
                            .with_children(|train| {
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
            }
            ObstacleType::MovingTrain { speed } => {
                if is_zone_2 {
                    if let Some(entity) = pool.pop(PoolType::NeonCyberBus) {
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
                            PooledItem {
                                pool_type: PoolType::NeonCyberBus,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_body.clone(),
                                    material: pool_assets.mat_neon_bus_body.clone(),
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
                                PooledItem {
                                    pool_type: PoolType::NeonCyberBus,
                                    is_active: true,
                                },
                            ))
                            .with_children(|bus| {
                                // Angled lower nose makes the bus read as a transit vehicle
                                // at speed while keeping the collision root unchanged.
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_nose.clone(),
                                    material: pool_assets.mat_neon_bus_body.clone(),
                                    transform: Transform::from_xyz(0.0, -0.78, 6.42),
                                    ..default()
                                });
                                // Front panoramic windshield (facing Kai at +Z)
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_windscreen.clone(),
                                    material: pool_assets.mat_neon_bus_glass.clone(),
                                    transform: Transform::from_xyz(0.0, 0.38, 6.80)
                                        .with_rotation(Quat::from_rotation_x(-0.12)),
                                    ..default()
                                });
                                // Front illuminated digital destination sign ("EXPRESS // DOWNTOWN")
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_destination.clone(),
                                    material: pool_assets.mat_neon_bus_destination.clone(),
                                    transform: Transform::from_xyz(0.0, 0.95, 6.82),
                                    ..default()
                                });
                                // Twin front high-beam LED headlights (left & right)
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_headlights.clone(),
                                    material: pool_assets.mat_neon_van_headlight.clone(),
                                    transform: Transform::from_xyz(-0.75, -0.45, 6.82),
                                    ..default()
                                });
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_headlights.clone(),
                                    material: pool_assets.mat_neon_van_headlight.clone(),
                                    transform: Transform::from_xyz(0.75, -0.45, 6.82),
                                    ..default()
                                });
                                // Rear vertical laser taillights (left & right, at -Z)
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_taillights.clone(),
                                    material: pool_assets.mat_neon_van_taillight.clone(),
                                    transform: Transform::from_xyz(-0.85, 0.0, -6.81),
                                    ..default()
                                });
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_taillights.clone(),
                                    material: pool_assets.mat_neon_van_taillight.clone(),
                                    transform: Transform::from_xyz(0.85, 0.0, -6.81),
                                    ..default()
                                });
                                // Five separated side windows create a readable articulated
                                // passenger cabin instead of a single rectangular strip.
                                for &x in &[-1.10, 1.10] {
                                    for &z in &[-4.55, -2.30, 0.0, 2.30, 4.55] {
                                        bus.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_bus_window_panel.clone(),
                                            material: pool_assets.mat_neon_bus_glass.clone(),
                                            transform: Transform::from_xyz(x, 0.38, z),
                                            ..default()
                                        });
                                    }
                                    bus.spawn(PbrBundle {
                                        mesh: pool_assets.mesh_neon_bus_trim.clone(),
                                        material: pool_assets.mat_neon_bus_trim.clone(),
                                        transform: Transform::from_xyz(x, -0.32, 0.0),
                                        ..default()
                                    });
                                }
                                // Four wheel assemblies and bright inset rims.
                                for &x in &[-1.06, 1.06] {
                                    for &z in &[-4.25, 4.25] {
                                        bus.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_bus_wheel.clone(),
                                            material: pool_assets.mat_neon_bus_rubber.clone(),
                                            transform: Transform::from_xyz(x, -0.72, z)
                                                .with_rotation(Quat::from_rotation_z(
                                                    std::f32::consts::FRAC_PI_2,
                                                )),
                                            ..default()
                                        });
                                        bus.spawn(PbrBundle {
                                            mesh: pool_assets.mesh_neon_bus_rim.clone(),
                                            material: pool_assets.mat_neon_bus_rim.clone(),
                                            transform: Transform::from_xyz(
                                                x + if x < 0.0 { -0.12 } else { 0.12 },
                                                -0.72,
                                                z,
                                            )
                                            .with_rotation(Quat::from_rotation_z(
                                                std::f32::consts::FRAC_PI_2,
                                            )),
                                            ..default()
                                        });
                                    }
                                }
                                // Roof sensor pod and side-mounted illuminated trim.
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_sensor.clone(),
                                    material: pool_assets.mat_neon_bus_trim.clone(),
                                    transform: Transform::from_xyz(0.0, 1.20, 5.65),
                                    ..default()
                                });
                                // Aerodynamic lower skirt / maglev runners
                                bus.spawn(PbrBundle {
                                    mesh: pool_assets.mesh_neon_bus_skirt.clone(),
                                    material: pool_assets.mat_neon_bus_rubber.clone(),
                                    transform: Transform::from_xyz(0.0, -1.05, 0.0),
                                    ..default()
                                });
                            });
                    }
                } else {
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
                            PooledItem {
                                pool_type: PoolType::MovingTrain,
                                is_active: true,
                            },
                        ));
                    } else {
                        commands
                            .spawn((
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
                                PooledItem {
                                    pool_type: PoolType::MovingTrain,
                                    is_active: true,
                                },
                            ))
                            .with_children(|train| {
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
                PooledItem {
                    pool_type: PoolType::EchoFragment,
                    is_active: true,
                },
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
                PooledItem {
                    pool_type: PoolType::EchoFragment,
                    is_active: true,
                },
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
                PooledItem {
                    pool_type: PoolType::DataChip,
                    is_active: true,
                },
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
                PooledItem {
                    pool_type: PoolType::DataChip,
                    is_active: true,
                },
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

        let mut p_cmd = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(p_lane.x_pos(), 1.2, world_z),
                ..default()
            },
            CollectibleItem {
                item_type: p_type,
                lane: p_lane,
                initial_y: 1.2,
                rot_speed: 3.5,
            },
            Despawnable { z_center: world_z },
        ));

        p_cmd.with_children(|parent| {
            match p_type {
                CollectibleType::EchoShield => {
                    // Vertical holographic beacon beam ascending above pickup (does not obscure model)
                    parent.spawn(PbrBundle {
                        mesh: pool_assets.mesh_powerup_beacon_beam.clone(),
                        material: pool_assets.mat_beacon_shield.clone(),
                        transform: Transform::from_xyz(0.0, 0.78, 0.0),
                        ..default()
                    });

                    if let Some(assets) = powerup_assets {
                        // Shield GLB model (Original scale preserved)
                        parent.spawn(SceneBundle {
                            scene: assets.shield_scene.clone(),
                            transform: Transform::from_scale(Vec3::splat(0.48)),
                            ..default()
                        });
                    } else {
                        // Fallback procedural geometry
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_hex.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_plate.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.38),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_shield_ring.clone(),
                            material: pool_assets.mat_powerup_shield.clone(),
                            ..default()
                        });
                    }
                }
                CollectibleType::Magnet => {
                    // Vertical holographic beacon beam ascending above pickup (does not obscure model)
                    parent.spawn(PbrBundle {
                        mesh: pool_assets.mesh_powerup_beacon_beam.clone(),
                        material: pool_assets.mat_beacon_magnet.clone(),
                        transform: Transform::from_xyz(0.0, 0.78, 0.0),
                        ..default()
                    });

                    if let Some(assets) = powerup_assets {
                        // Magnet GLB model (Original scale preserved)
                        parent.spawn(SceneBundle {
                            scene: assets.magnet_scene.clone(),
                            transform: Transform::from_scale(Vec3::splat(0.48)),
                            ..default()
                        });
                    } else {
                        // Fallback procedural geometry
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_magnet_arch.clone(),
                            material: pool_assets.mat_powerup_magnet.clone(),
                            transform: Transform::from_xyz(0.0, 0.22, 0.0),
                            ..default()
                        });
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
                    }
                }
                CollectibleType::Overdrive => {
                    // Vertical holographic beacon beam ascending above pickup (does not obscure model)
                    parent.spawn(PbrBundle {
                        mesh: pool_assets.mesh_powerup_beacon_beam.clone(),
                        material: pool_assets.mat_beacon_overdrive.clone(),
                        transform: Transform::from_xyz(0.0, 0.78, 0.0),
                        ..default()
                    });

                    if let Some(assets) = powerup_assets {
                        // Overdrive GLB model (Original scale preserved)
                        parent.spawn(SceneBundle {
                            scene: assets.overdrive_scene.clone(),
                            transform: Transform::from_scale(Vec3::splat(0.48)),
                            ..default()
                        });
                    } else {
                        // Fallback procedural geometry
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_overdrive_diamond.clone(),
                            material: pool_assets.mat_powerup_overdrive.clone(),
                            ..default()
                        });
                        parent.spawn(PbrBundle {
                            mesh: pool_assets.mesh_overdrive_ring.clone(),
                            material: pool_assets.mat_powerup_overdrive.clone(),
                            transform: Transform::from_rotation(Quat::from_rotation_x(0.785)),
                            ..default()
                        });
                    }
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

    chunk
}

// -------------------------------------------------------------
// RECYCLING & DESPAWNING
// -------------------------------------------------------------
fn despawn_distant_entities(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut despawn_q: Query<
        (
            Entity,
            &Despawnable,
            Option<&PooledItem>,
            Option<&mut Transform>,
            Option<&mut Visibility>,
        ),
        Without<Player>,
    >,
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
                commands
                    .entity(entity)
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
// EVENT-DRIVEN ZONE TRANSITION SFX (UNIFIED GLOBAL LIGHTING)
// -------------------------------------------------------------
fn handle_zone_transition_events(
    mut zone_events: EventReader<ZoneChangedEvent>,
    mut sfx: EventWriter<SoundEffect>,
) {
    for _event in zone_events.read() {
        sfx.send(SoundEffect::ZoneTransition);
    }
}

fn spawn_segment(
    commands: &mut Commands,
    env_assets: &crate::environment::EnvironmentAssets,
    prop_assets: Option<&crate::environment_props::PropAssets>,
    signage_assets: Option<&crate::environment_signage::SignageAssets>,
    neon_assets: Option<&crate::neon_district::NeonDistrictAssets>,
    industrial_assets: Option<&crate::industrial_sector::IndustrialSectorAssets>,
    z_center: f32,
    length: f32,
    distance: f32,
) {
    crate::environment::spawn_modular_environment_slice(
        commands,
        env_assets,
        prop_assets,
        signage_assets,
        neon_assets,
        industrial_assets,
        z_center,
        length,
        distance,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obstacles::check_obstacle_vertical_collision;
    use crate::zones::ZONES;

    #[test]
    fn test_zone_transition_preserves_unified_global_lighting() {
        let mut app = App::new();
        let neutral_dir_color = Color::srgb(0.90, 0.90, 0.92);
        let neutral_clear_color = Color::srgb(0.04, 0.04, 0.05);

        app.insert_resource(ClearColor(neutral_clear_color))
            .add_event::<ZoneChangedEvent>()
            .add_event::<SoundEffect>()
            .add_systems(Update, handle_zone_transition_events);

        let dl_entity = app
            .world_mut()
            .spawn(DirectionalLight {
                color: neutral_dir_color,
                illuminance: 3200.0,
                ..default()
            })
            .id();

        let mut total_sfx_count = 0;
        let mut reader = app.world().resource::<Events<SoundEffect>>().get_reader();

        // Transition through all 7 zones
        for i in 0..ZONES.len() - 1 {
            app.world_mut().send_event(ZoneChangedEvent {
                from_zone: ZONES[i].id,
                to_zone: ZONES[i + 1].id,
                config: ZONES[i + 1],
            });
            app.update();

            let sfx_events = app.world().resource::<Events<SoundEffect>>();
            total_sfx_count += reader
                .read(sfx_events)
                .filter(|sfx| matches!(sfx, SoundEffect::ZoneTransition))
                .count();

            // Verify DirectionalLight color remains strictly neutral white across all zones
            let dl = app.world().get::<DirectionalLight>(dl_entity).unwrap();
            assert_eq!(
                dl.color,
                neutral_dir_color,
                "Zone transition from {} to {} must not recolor DirectionalLight",
                ZONES[i].id,
                ZONES[i + 1].id
            );

            // Verify ClearColor remains strictly neutral dark across all zones
            let clear = app.world().resource::<ClearColor>();
            assert_eq!(
                clear.0,
                neutral_clear_color,
                "Zone transition from {} to {} must not recolor ClearColor",
                ZONES[i].id,
                ZONES[i + 1].id
            );
        }

        assert_eq!(
            total_sfx_count,
            ZONES.len() - 1,
            "Every zone transition must emit a transition SFX without altering global lighting"
        );
    }

    #[test]
    fn test_obstacle_visual_selection_zone_1_metro_unchanged() {
        let test_distances = [0.0, 500.0, 1500.0, ZONES[0].end_distance - 0.001];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::LowBarrier,
                "Zone 1 must retain Metro LowBarrier"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::HighHangingWire,
                "Zone 1 must retain Metro HighHangingWire"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::TallPillar,
                "Zone 1 must retain Metro TallPillar"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::StaticTrain,
                "Zone 1 must retain Metro StaticTrain"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::MovingTrain,
                "Zone 1 must retain Metro MovingTrain"
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zone_2_neon_district() {
        let zone_2 = ZONES[1];
        let test_distances = [
            zone_2.start_distance,
            2500.0,
            3300.0,
            zone_2.end_distance - 0.001,
        ];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::NeonRoadBarrier,
                "Zone 2 must select Neon Road Barrier at {dist}m"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::NeonOverheadScanner,
                "Zone 2 must select Neon Overhead Scanner"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::NeonCheckpointPillar,
                "Zone 2 must select Neon Checkpoint Pillar"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::NeonAutoVan,
                "Zone 2 must select Neon Autonomous Delivery Van"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 12.0 }, dist),
                PoolType::NeonCyberBus,
                "Zone 2 must select Neon Futuristic Cyber Bus"
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zones_3_to_7_protected() {
        let test_distances = [
            ZONES[1].end_distance,
            ZONES[2].start_distance,
            ZONES[3].start_distance,
            ZONES[4].start_distance,
            ZONES[5].start_distance,
        ];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::LowBarrier,
                "Post-Zone-2 distance {}m must not use Zone 2 road visuals",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::HighHangingWire,
                "Post-Zone-2 distance {}m must not use Zone 2 road visuals",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::TallPillar,
                "Post-Zone-2 distance {}m must not use Zone 2 road visuals",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::StaticTrain,
                "Post-Zone-2 distance {}m must not use Zone 2 road visuals",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::MovingTrain,
                "Post-Zone-2 distance {}m must not use Zone 2 road visuals",
                dist
            );
        }
    }

    #[test]
    fn test_entity_pool_zero_cross_contamination() {
        let mut pool = EntityPool::default();
        let e_metro_barrier = Entity::from_raw(1);
        let e_neon_barrier = Entity::from_raw(2);
        let e_metro_wire = Entity::from_raw(3);
        let e_neon_scanner = Entity::from_raw(4);
        let e_metro_pillar = Entity::from_raw(5);
        let e_neon_pillar = Entity::from_raw(6);
        let e_metro_train = Entity::from_raw(7);
        let e_neon_van = Entity::from_raw(8);
        let e_metro_moving = Entity::from_raw(9);
        let e_neon_bus = Entity::from_raw(10);

        // Push all into respective pools
        pool.push(PoolType::LowBarrier, e_metro_barrier);
        pool.push(PoolType::NeonRoadBarrier, e_neon_barrier);
        pool.push(PoolType::HighHangingWire, e_metro_wire);
        pool.push(PoolType::NeonOverheadScanner, e_neon_scanner);
        pool.push(PoolType::TallPillar, e_metro_pillar);
        pool.push(PoolType::NeonCheckpointPillar, e_neon_pillar);
        pool.push(PoolType::StaticTrain, e_metro_train);
        pool.push(PoolType::NeonAutoVan, e_neon_van);
        pool.push(PoolType::MovingTrain, e_metro_moving);
        pool.push(PoolType::NeonCyberBus, e_neon_bus);

        // Verify vectors are isolated
        assert_eq!(pool.barriers.len(), 1);
        assert_eq!(pool.neon_barriers.len(), 1);
        assert_eq!(pool.wires.len(), 1);
        assert_eq!(pool.neon_scanners.len(), 1);
        assert_eq!(pool.pillars.len(), 1);
        assert_eq!(pool.neon_pillars.len(), 1);
        assert_eq!(pool.static_trains.len(), 1);
        assert_eq!(pool.neon_vans.len(), 1);
        assert_eq!(pool.moving_trains.len(), 1);
        assert_eq!(pool.neon_buses.len(), 1);

        // Pop from Zone 2 pools
        assert_eq!(pool.pop(PoolType::NeonRoadBarrier), Some(e_neon_barrier));
        assert_eq!(
            pool.pop(PoolType::NeonOverheadScanner),
            Some(e_neon_scanner)
        );
        assert_eq!(
            pool.pop(PoolType::NeonCheckpointPillar),
            Some(e_neon_pillar)
        );
        assert_eq!(pool.pop(PoolType::NeonAutoVan), Some(e_neon_van));
        assert_eq!(pool.pop(PoolType::NeonCyberBus), Some(e_neon_bus));

        // Zone 1 pools remain fully intact
        assert_eq!(pool.pop(PoolType::LowBarrier), Some(e_metro_barrier));
        assert_eq!(pool.pop(PoolType::HighHangingWire), Some(e_metro_wire));
        assert_eq!(pool.pop(PoolType::TallPillar), Some(e_metro_pillar));
        assert_eq!(pool.pop(PoolType::StaticTrain), Some(e_metro_train));
        assert_eq!(pool.pop(PoolType::MovingTrain), Some(e_metro_moving));
    }

    #[test]
    fn test_moving_obstacle_kinematics_and_despawn_center() {
        let mut transform = Transform::from_xyz(0.0, 1.25, -1200.0);
        let mut despawn = Despawnable { z_center: -1200.0 };
        let moving = MovingObstacle { speed: 15.0 };
        let dt = 0.5;

        // Kinematic simulation
        transform.translation.z += moving.speed * dt;
        despawn.z_center = transform.translation.z;

        assert_eq!(transform.translation.z, -1192.5);
        assert_eq!(despawn.z_center, -1192.5);
    }

    #[test]
    fn test_collision_rules_invariance_for_zone_2_obstacles() {
        // 1. Low Barrier / Neon Road Barricade: standing collides, jump apex clears
        assert!(check_obstacle_vertical_collision(
            0.65,
            1.0,
            false,
            ObstacleType::LowBarrier,
            0.42,
            0.85
        ));
        assert!(!check_obstacle_vertical_collision(
            1.84,
            1.0,
            false,
            ObstacleType::LowBarrier,
            0.42,
            0.85
        ));

        // 2. Overhead Scanner: standing collides, slide ducks under
        assert!(check_obstacle_vertical_collision(
            0.65,
            1.0,
            false,
            ObstacleType::HighHangingWire,
            1.7,
            0.5
        ));
        assert!(!check_obstacle_vertical_collision(
            0.35,
            0.45,
            true,
            ObstacleType::HighHangingWire,
            1.7,
            0.5
        ));

        // 3. Checkpoint Pillar: always collides vertically
        assert!(check_obstacle_vertical_collision(
            0.65,
            1.0,
            false,
            ObstacleType::TallPillar,
            2.0,
            4.0
        ));
    }
}
