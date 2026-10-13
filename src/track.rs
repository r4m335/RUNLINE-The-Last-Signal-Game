use crate::director::RunDirector;
use crate::obstacles::resolve_obstacle_pool_type;
use crate::patterns::{select_validated_pattern, ObstacleSpawnDef, PatternChunk};
use crate::pooling::{EntityPool, PoolAssets};
use crate::types::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const SEGMENT_LENGTH: f32 = 40.0;
const SPAWN_AHEAD_DISTANCE: f32 = 160.0;
const DESPAWN_BEHIND_DISTANCE: f32 = 40.0;

pub(crate) fn initial_track_spawn_z(distance: f32) -> f32 {
    -distance + SEGMENT_LENGTH * 0.5
}

#[derive(Resource)]
pub struct TrackManager {
    pub next_spawn_z: f32,
    pub last_chunk: Option<PatternChunk>,
}

#[derive(SystemParam)]
struct TrackEnvironmentAssets<'w> {
    env: Res<'w, crate::environment::EnvironmentAssets>,
    props: Option<Res<'w, crate::environment_props::PropAssets>>,
    signage: Option<Res<'w, crate::environment_signage::SignageAssets>>,
    neon: Option<Res<'w, crate::neon_district::NeonDistrictAssets>>,
    industrial: Option<Res<'w, crate::industrial_sector::IndustrialSectorAssets>>,
    flooded: Option<Res<'w, crate::flooded_metro::FloodedMetroAssets>>,
    sky_rail: Option<Res<'w, crate::sky_rail::SkyRailAssets>>,
    forbidden: Option<Res<'w, crate::forbidden_line::ForbiddenLineAssets>>,
    echo_core: Option<Res<'w, crate::echo_core::EchoCoreAssets>>,
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
    environment_assets: TrackEnvironmentAssets,
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
        track_mgr.next_spawn_z = initial_track_spawn_z(stats.distance);
        track_mgr.last_chunk = None;

        for _ in 0..5 {
            let z_center = track_mgr.next_spawn_z - SEGMENT_LENGTH * 0.5;
            let z_start = track_mgr.next_spawn_z;

            spawn_segment(
                &mut commands,
                &environment_assets.env,
                environment_assets.props.as_deref(),
                environment_assets.signage.as_deref(),
                environment_assets.neon.as_deref(),
                environment_assets.industrial.as_deref(),
                environment_assets.flooded.as_deref(),
                environment_assets.sky_rail.as_deref(),
                environment_assets.forbidden.as_deref(),
                environment_assets.echo_core.as_deref(),
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
    environment_assets: TrackEnvironmentAssets,
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
            &environment_assets.env,
            environment_assets.props.as_deref(),
            environment_assets.signage.as_deref(),
            environment_assets.neon.as_deref(),
            environment_assets.industrial.as_deref(),
            environment_assets.flooded.as_deref(),
            environment_assets.sky_rail.as_deref(),
            environment_assets.forbidden.as_deref(),
            environment_assets.echo_core.as_deref(),
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

        // Zone 7 absolute safety guarantee:
        // 1. Never spawn physical ground trains in Zone 7 (The ECHO Core: 12,000m - 14,000m).
        // 2. Never spawn procedural obstacles in Acts 4 & 5 (13,300m - 14,000m) to preserve boss and finale exclusivity.
        let is_zone_7 = (12000.0..=14000.0).contains(&obstacle_distance);
        if is_zone_7
            && (matches!(
                obs_def.obstacle_type,
                ObstacleType::MovingTrain { .. } | ObstacleType::StaticTrain
            ) || obstacle_distance >= 13300.0)
        {
            continue;
        }

        let visual_pool_type = resolve_obstacle_pool_type(obs_def.obstacle_type, obstacle_distance);
        let is_zone_2 = matches!(
            visual_pool_type,
            PoolType::NeonRoadBarrier
                | PoolType::NeonOverheadScanner
                | PoolType::NeonCheckpointPillar
                | PoolType::NeonAutoVan
                | PoolType::NeonCyberBus
        );
        let is_zone_3 = matches!(
            visual_pool_type,
            PoolType::IndustrialRoadBarrier
                | PoolType::IndustrialOverheadPipe
                | PoolType::IndustrialSupportPillar
                | PoolType::IndustrialFreightTransport
                | PoolType::IndustrialCargoTransport
        );
        let is_zone_4 = matches!(
            visual_pool_type,
            PoolType::FloodedBarrier
                | PoolType::FloodedPowerCable
                | PoolType::FloodedPumpColumn
                | PoolType::FloodedSubmergedTrain
                | PoolType::FloodedMovingTrain
        );
        let is_zone_5 = matches!(
            visual_pool_type,
            PoolType::SkyRailMaintenanceGate
                | PoolType::SkyRailClearanceBeam
                | PoolType::SkyRailSignalTower
                | PoolType::SkyRailDerelictMagCar
                | PoolType::SkyRailMagTrain
        );
        let is_zone_6 = matches!(
            visual_pool_type,
            PoolType::ForbiddenQuarantineBarrier
                | PoolType::ForbiddenConduitBundle
                | PoolType::ForbiddenContainmentPillar
                | PoolType::ForbiddenFacilityTransport
                | PoolType::ForbiddenHeavyMover
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
                } else if is_zone_3 {
                    spawn_industrial_barrier(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_4 {
                    spawn_flooded_barrier(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_5 {
                    spawn_sky_rail_maintenance_gate(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_6 {
                    spawn_forbidden_quarantine_barrier(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                    );
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
                } else if is_zone_3 {
                    spawn_industrial_overhead_pipe(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_4 {
                    spawn_flooded_power_cable(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_5 {
                    spawn_sky_rail_clearance_beam(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_6 {
                    spawn_forbidden_conduit_bundle(commands, pool_assets, pool, *obs_def, world_z);
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
                } else if is_zone_3 {
                    spawn_industrial_support_pillar(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_4 {
                    spawn_flooded_pump_column(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_5 {
                    spawn_sky_rail_signal_tower(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_6 {
                    spawn_forbidden_containment_pillar(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                    );
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
                } else if is_zone_3 {
                    spawn_industrial_freight_transport(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                    );
                } else if is_zone_4 {
                    spawn_flooded_submerged_train(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_5 {
                    spawn_sky_rail_derelict_mag_car(commands, pool_assets, pool, *obs_def, world_z);
                } else if is_zone_6 {
                    spawn_forbidden_facility_transport(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                    );
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
                } else if is_zone_3 {
                    spawn_industrial_cargo_transport(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                        speed,
                    );
                } else if is_zone_4 {
                    spawn_flooded_moving_train(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                        speed,
                    );
                } else if is_zone_5 {
                    spawn_sky_rail_mag_train(commands, pool_assets, pool, *obs_def, world_z, speed);
                } else if is_zone_6 {
                    spawn_forbidden_heavy_mover(
                        commands,
                        pool_assets,
                        pool,
                        *obs_def,
                        world_z,
                        speed,
                    );
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
    flooded_assets: Option<&crate::flooded_metro::FloodedMetroAssets>,
    sky_rail_assets: Option<&crate::sky_rail::SkyRailAssets>,
    forbidden_assets: Option<&crate::forbidden_line::ForbiddenLineAssets>,
    echo_core_assets: Option<&crate::echo_core::EchoCoreAssets>,
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
        flooded_assets,
        sky_rail_assets,
        forbidden_assets,
        echo_core_assets,
        z_center,
        length,
        distance,
    );
}

fn industrial_active(
    lane: Lane,
    obstacle_type: ObstacleType,
    size: Vec3,
    world_z: f32,
    pool_type: PoolType,
) -> (ActiveObstacle, Despawnable, PooledItem) {
    (
        ActiveObstacle {
            lane,
            obstacle_type,
            size,
        },
        Despawnable { z_center: world_z },
        PooledItem {
            pool_type,
            is_active: true,
        },
    )
}

fn spawn_industrial_barrier(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = industrial_active(
        obs_def.lane,
        ObstacleType::LowBarrier,
        obs_def.size,
        world_z,
        PoolType::IndustrialRoadBarrier,
    );
    if let Some(entity) = pool.pop(PoolType::IndustrialRoadBarrier) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_industrial_barrier_frame.clone(),
                material: pool_assets.mat_industrial_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|barrier| {
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_industrial_barrier_stripe.clone(),
                material: pool_assets.mat_industrial_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.02),
                ..default()
            });
            for x in [-0.94, 0.94] {
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_industrial_barrier_support.clone(),
                    material: pool_assets.mat_industrial_steel.clone(),
                    transform: Transform::from_xyz(x, -0.03, 0.0),
                    ..default()
                });
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_barrier_foot.clone(),
                    material: pool_assets.mat_industrial_rust.clone(),
                    transform: Transform::from_xyz(x, -0.40, 0.0),
                    ..default()
                });
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_barrier_strobe.clone(),
                    material: pool_assets.mat_industrial_warning.clone(),
                    transform: Transform::from_xyz(x, 0.27, 0.0),
                    ..default()
                });
            }
        });
}

fn spawn_industrial_overhead_pipe(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = industrial_active(
        obs_def.lane,
        ObstacleType::HighHangingWire,
        obs_def.size,
        world_z,
        PoolType::IndustrialOverheadPipe,
    );
    if let Some(entity) = pool.pop(PoolType::IndustrialOverheadPipe) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.7, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_industrial_pipe.clone(),
                material: pool_assets.mat_industrial_pipe.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.7, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|pipe| {
            pipe.spawn(PbrBundle {
                mesh: pool_assets.mesh_industrial_pipe_bundle.clone(),
                material: pool_assets.mat_industrial_rust.clone(),
                transform: Transform::from_xyz(0.0, 0.16, 0.0),
                ..default()
            });
            pipe.spawn(PbrBundle {
                mesh: pool_assets.mesh_industrial_clearance.clone(),
                material: pool_assets.mat_industrial_hazard.clone(),
                transform: Transform::from_xyz(0.0, -0.18, 0.02),
                ..default()
            });
            for x in [-1.04, 1.04] {
                pipe.spawn(PbrBundle {
                    mesh: pool_assets.mesh_wire_anchor.clone(),
                    material: pool_assets.mat_industrial_steel.clone(),
                    transform: Transform::from_xyz(x, 0.16, 0.0),
                    ..default()
                });
            }
        });
}

fn spawn_industrial_support_pillar(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = industrial_active(
        obs_def.lane,
        ObstacleType::TallPillar,
        obs_def.size,
        world_z,
        PoolType::IndustrialSupportPillar,
    );
    if let Some(entity) = pool.pop(PoolType::IndustrialSupportPillar) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_pillar_body.clone(),
                material: pool_assets.mat_industrial_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|pillar| {
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_pillar_base.clone(),
                material: pool_assets.mat_industrial_rust.clone(),
                transform: Transform::from_xyz(0.0, -1.78, 0.0),
                ..default()
            });
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_pillar_collar.clone(),
                material: pool_assets.mat_industrial_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.95, 0.0),
                ..default()
            });
            for x in [-0.55, 0.55] {
                pillar.spawn(PbrBundle {
                    mesh: pool_assets.mesh_pillar_trunk.clone(),
                    material: pool_assets.mat_industrial_rust.clone(),
                    transform: Transform::from_xyz(x, 0.0, 0.57),
                    ..default()
                });
            }
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_pillar_placard.clone(),
                material: pool_assets.mat_industrial_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.35, 0.58),
                ..default()
            });
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_pillar_beacon.clone(),
                material: pool_assets.mat_industrial_warning.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -0.58),
                ..default()
            });
        });
}

fn spawn_industrial_freight_transport(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = industrial_active(
        obs_def.lane,
        ObstacleType::StaticTrain,
        obs_def.size,
        world_z,
        PoolType::IndustrialFreightTransport,
    );
    if let Some(entity) = pool.pop(PoolType::IndustrialFreightTransport) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_industrial_freight_body.clone(),
                material: pool_assets.mat_industrial_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|freight| {
            freight.spawn(PbrBundle {
                mesh: pool_assets.mesh_industrial_freight_cab.clone(),
                material: pool_assets.mat_industrial_glass.clone(),
                transform: Transform::from_xyz(0.0, 0.55, 5.05),
                ..default()
            });
            for x in [-1.08, 1.08] {
                freight.spawn(PbrBundle {
                    mesh: pool_assets.mesh_industrial_freight_panel.clone(),
                    material: pool_assets.mat_industrial_rust.clone(),
                    transform: Transform::from_xyz(x, 1.25, 0.0),
                    ..default()
                });
                for z in [-4.2, 4.2] {
                    freight.spawn(PbrBundle {
                        mesh: pool_assets.mesh_industrial_freight_wheel.clone(),
                        material: pool_assets.mat_industrial_rust.clone(),
                        transform: Transform::from_xyz(x, -0.72, z)
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                        ..default()
                    });
                }
            }
            for x in [-0.70, 0.70] {
                freight.spawn(PbrBundle {
                    mesh: pool_assets.mesh_train_headlamp.clone(),
                    material: pool_assets.mat_industrial_warning.clone(),
                    transform: Transform::from_xyz(x, 0.10, 6.05),
                    ..default()
                });
            }
        });
}

fn spawn_industrial_cargo_transport(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
    speed: f32,
) {
    let active = industrial_active(
        obs_def.lane,
        ObstacleType::MovingTrain { speed },
        obs_def.size,
        world_z,
        PoolType::IndustrialCargoTransport,
    );
    if let Some(entity) = pool.pop(PoolType::IndustrialCargoTransport) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_industrial_freight_body.clone(),
                material: pool_assets.mat_industrial_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ))
        .with_children(|cargo| {
            cargo.spawn(PbrBundle {
                mesh: pool_assets.mesh_industrial_freight_cab.clone(),
                material: pool_assets.mat_industrial_glass.clone(),
                transform: Transform::from_xyz(0.0, 0.55, 6.05),
                ..default()
            });
            for x in [-1.08, 1.08] {
                cargo.spawn(PbrBundle {
                    mesh: pool_assets.mesh_industrial_freight_panel.clone(),
                    material: pool_assets.mat_industrial_hazard.clone(),
                    transform: Transform::from_xyz(x, 1.25, -0.8),
                    ..default()
                });
                for z in [-5.0, 5.0] {
                    cargo.spawn(PbrBundle {
                        mesh: pool_assets.mesh_industrial_freight_wheel.clone(),
                        material: pool_assets.mat_industrial_rust.clone(),
                        transform: Transform::from_xyz(x, -0.72, z)
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                        ..default()
                    });
                }
            }
            for x in [-0.70, 0.70] {
                cargo.spawn(PbrBundle {
                    mesh: pool_assets.mesh_train_headlamp.clone(),
                    material: pool_assets.mat_industrial_warning.clone(),
                    transform: Transform::from_xyz(x, 0.10, 7.05),
                    ..default()
                });
            }
        });
}

fn flooded_active(
    lane: Lane,
    obstacle_type: ObstacleType,
    size: Vec3,
    world_z: f32,
    pool_type: PoolType,
) -> (ActiveObstacle, Despawnable, PooledItem) {
    (
        ActiveObstacle {
            lane,
            obstacle_type,
            size,
        },
        Despawnable { z_center: world_z },
        PooledItem {
            pool_type,
            is_active: true,
        },
    )
}

fn spawn_flooded_barrier(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = flooded_active(
        obs_def.lane,
        ObstacleType::LowBarrier,
        obs_def.size,
        world_z,
        PoolType::FloodedBarrier,
    );
    if let Some(entity) = pool.pop(PoolType::FloodedBarrier) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            // Main barricade structural frame spanning the lane
            PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_frame.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|barrier| {
            // Full-height high-contrast hazard board (facing oncoming runner)
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_plate.clone(),
                material: pool_assets.mat_flooded_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.08),
                ..default()
            });

            // Prominent horizontal top hurdle rail - marks the jump clearance crest clearly!
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_top_rail.clone(),
                material: pool_assets.mat_flooded_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.38, 0.0),
                ..default()
            });

            // Bold diagonal hazard contrast strip across the board face
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_stripe.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, 0.04, 0.15),
                ..default()
            });

            // Bottom splash guard seal resting on the transit causeway
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_skirt.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, -0.34, 0.0),
                ..default()
            });

            // Dual heavy rusted triangular A-frame stanchions on the ends
            for x in [-1.02, 1.02] {
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_barrier_support.clone(),
                    material: pool_assets.mat_flooded_rust.clone(),
                    transform: Transform::from_xyz(x, 0.0, 0.0),
                    ..default()
                });
            }

            // Triple high-intensity amber warning strobes across the top crest
            for x in [-1.02, 0.0, 1.02] {
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_barrier_strobe.clone(),
                    material: pool_assets.mat_flooded_amber.clone(),
                    transform: Transform::from_xyz(x, 0.44, 0.08),
                    ..default()
                });
            }
        });
}

fn spawn_flooded_power_cable(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = flooded_active(
        obs_def.lane,
        ObstacleType::HighHangingWire,
        obs_def.size,
        world_z,
        PoolType::FloodedPowerCable,
    );
    if let Some(entity) = pool.pop(PoolType::FloodedPowerCable) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.70, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            // Main horizontal overhead gantry beam spanning the lane
            PbrBundle {
                mesh: pool_assets.mesh_flooded_cable_conduit.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.70, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|gate| {
            // Two substantial vertical support columns straddling the lane
            for x in [-1.15, 1.15] {
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_cable_bundle.clone(),
                    material: pool_assets.mat_flooded_rust.clone(),
                    transform: Transform::from_xyz(x, -0.75, 0.0),
                    ..default()
                });
                // Rusted column footing collars anchored into the transit slab
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_gate_foot.clone(),
                    material: pool_assets.mat_flooded_steel.clone(),
                    transform: Transform::from_xyz(x, -1.60, 0.0),
                    ..default()
                });
                // Twin amber warning strobes atop the column junctions
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_barrier_strobe.clone(),
                    material: pool_assets.mat_flooded_amber.clone(),
                    transform: Transform::from_xyz(x, 0.26, 0.16),
                    ..default()
                });
            }
            // Large high-contrast hazard panel across the front of the beam
            gate.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_cable_tag.clone(),
                material: pool_assets.mat_flooded_hazard.clone(),
                transform: Transform::from_xyz(0.0, 0.02, 0.16),
                ..default()
            });
            // Diagonal hazard contrast band along lower face of the panel
            gate.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_gate_strip.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, -0.08, 0.20),
                ..default()
            });
            // Downward pointing clearance indicator teeth marking the ducking threshold
            for x in [-0.65, 0.0, 0.65] {
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_gate_tooth.clone(),
                    material: pool_assets.mat_flooded_hazard.clone(),
                    transform: Transform::from_xyz(x, -0.22, 0.16),
                    ..default()
                });
            }
        });
}

fn spawn_flooded_pump_column(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = flooded_active(
        obs_def.lane,
        ObstacleType::TallPillar,
        obs_def.size,
        world_z,
        PoolType::FloodedPumpColumn,
    );
    if let Some(entity) = pool.pop(PoolType::FloodedPumpColumn) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_flooded_pipe_column.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|pillar| {
            // Heavy base mounting collar anchored in water
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_column_collar.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, -1.6, 0.0),
                ..default()
            });
            // High-pressure emergency shut-off valve wheel
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_column_valve.clone(),
                material: pool_assets.mat_flooded_amber.clone(),
                transform: Transform::from_xyz(0.0, 0.15, 0.62)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            // Hazard caution plate on the column face
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_cable_tag.clone(),
                material: pool_assets.mat_flooded_hazard.clone(),
                transform: Transform::from_xyz(0.0, -0.65, 0.62),
                ..default()
            });
        });
}

fn spawn_flooded_submerged_train(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = flooded_active(
        obs_def.lane,
        ObstacleType::StaticTrain,
        obs_def.size,
        world_z,
        PoolType::FloodedSubmergedTrain,
    );
    if let Some(entity) = pool.pop(PoolType::FloodedSubmergedTrain) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|train| {
            // Recessed empty gondola floor (bottom cargo bed, hollow interior with NO goods)
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_empty_gondola_floor.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(0.0, -0.92, 0.0),
                ..default()
            });
            // Left & Right high gondola side walls
            for x in [-1.04, 1.04] {
                train.spawn(PbrBundle {
                    mesh: pool_assets.mesh_empty_gondola_wall_side.clone(),
                    material: pool_assets.mat_flooded_steel.clone(),
                    transform: Transform::from_xyz(x, 0.05, 0.0),
                    ..default()
                });
                // Top cap rim rail along the top edge of the side walls
                train.spawn(PbrBundle {
                    mesh: pool_assets.mesh_empty_gondola_rim.clone(),
                    material: pool_assets.mat_flooded_rust.clone(),
                    transform: Transform::from_xyz(x, 1.10, 0.0),
                    ..default()
                });
                // External vertical reinforcement stakes
                for z_stake in [-4.6, -2.3, 0.0, 2.3, 4.6] {
                    let stake_x = if x < 0.0 { -1.11 } else { 1.11 };
                    train.spawn(PbrBundle {
                        mesh: pool_assets.mesh_empty_gondola_stake.clone(),
                        material: pool_assets.mat_flooded_rust.clone(),
                        transform: Transform::from_xyz(stake_x, 0.05, z_stake),
                        ..default()
                    });
                }
            }
            // Front & Rear bulkheads
            for (z_end, is_front) in [(5.85, true), (-5.85, false)] {
                train.spawn(PbrBundle {
                    mesh: pool_assets.mesh_empty_gondola_wall_end.clone(),
                    material: pool_assets.mat_flooded_steel.clone(),
                    transform: Transform::from_xyz(0.0, 0.05, z_end),
                    ..default()
                });
                if is_front {
                    // Front heavy crash bumper
                    train.spawn(PbrBundle {
                        mesh: pool_assets.mesh_train_bumper.clone(),
                        material: pool_assets.mat_flooded_rust.clone(),
                        transform: Transform::from_xyz(0.0, -0.65, 6.05),
                        ..default()
                    });
                    // Front bold diagonal hazard warning plate
                    train.spawn(PbrBundle {
                        mesh: pool_assets.mesh_flooded_barrier_plate.clone(),
                        material: pool_assets.mat_flooded_hazard.clone(),
                        transform: Transform::from_xyz(0.0, -0.15, 6.08),
                        ..default()
                    });
                }
            }
        });
}

fn spawn_flooded_moving_train(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
    speed: f32,
) {
    let active = flooded_active(
        obs_def.lane,
        ObstacleType::MovingTrain { speed },
        obs_def.size,
        world_z,
        PoolType::FloodedMovingTrain,
    );
    if let Some(entity) = pool.pop(PoolType::FloodedMovingTrain) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ))
        .with_children(|service| {
            // Heavy freight flatbed chassis platform (lower frame spanning the 8.5m carriage)
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_service_body.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(0.0, -0.85, 0.0),
                ..default()
            });

            // --- FORWARD SWITCHER CABIN & MOTOR HOUSING (Z in [1.0, 4.2]) ---
            // Armored operator cabin body
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_service_cab.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.30, 2.60),
                ..default()
            });
            // Raised industrial switcher cab roof
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_train_roof.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, 1.20, 2.60)
                    .with_scale(Vec3::new(0.92, 0.80, 0.28)),
                ..default()
            });
            // Armored cab front visor / windscreen
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_train_windscreen.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, 0.70, 4.22)
                    .with_scale(Vec3::new(0.85, 0.60, 1.0)),
                ..default()
            });
            // Roof exhaust vent / dynamic brake blister
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_train_hvac.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, 1.25, 1.60)
                    .with_scale(Vec3::new(0.80, 0.70, 0.60)),
                ..default()
            });
            // High-intensity amber transit headlamps piercing forward
            for x in [-0.75, 0.75] {
                service.spawn(PbrBundle {
                    mesh: pool_assets.mesh_flooded_service_light.clone(),
                    material: pool_assets.mat_flooded_amber.clone(),
                    transform: Transform::from_xyz(x, 0.25, 4.28),
                    ..default()
                });
            }
            // Front heavy cowcatcher / crash plow bumper
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_train_bumper.clone(),
                material: pool_assets.mat_flooded_rust.clone(),
                transform: Transform::from_xyz(0.0, -0.65, 4.28),
                ..default()
            });
            // Bold full-width front hazard caution plate
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_flooded_barrier_plate.clone(),
                material: pool_assets.mat_flooded_hazard.clone(),
                transform: Transform::from_xyz(0.0, -0.15, 4.30),
                ..default()
            });

            // --- REAR CARGO DECK HAULING ABUNDANT GOODS (Z in [-4.25, 1.0]) ---
            // 1. Intermodal freight shipping container (heavy industrial container)
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_cargo_container_short.clone(),
                material: pool_assets.mat_cargo_container.clone(),
                transform: Transform::from_xyz(0.0, 0.225, -2.50),
                ..default()
            });
            // 2. Stack of heavy wooden equipment cargo crates
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_cargo_crate_stack.clone(),
                material: pool_assets.mat_cargo_wood.clone(),
                transform: Transform::from_xyz(-0.52, -0.025, 0.05),
                ..default()
            });
            // 3. High-voltage industrial cable reel / wire spool
            service.spawn(PbrBundle {
                mesh: pool_assets.mesh_cargo_cable_reel.clone(),
                material: pool_assets.mat_flooded_steel.clone(),
                transform: Transform::from_xyz(0.50, 0.10, 0.05)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            // 4. Strapped fuel / chemical hazard barrels
            for x in [-0.50, 0.50] {
                service.spawn(PbrBundle {
                    mesh: pool_assets.mesh_cargo_drum.clone(),
                    material: pool_assets.mat_flooded_hazard.clone(),
                    transform: Transform::from_xyz(x, -0.125, 0.85),
                    ..default()
                });
            }
        });
}

// -------------------------------------------------------------
// ZONE 5 — SKY RAIL DEDICATED OBSTACLE VISUALS
// -------------------------------------------------------------

fn sky_rail_active(
    lane: Lane,
    obstacle_type: ObstacleType,
    size: Vec3,
    world_z: f32,
    pool_type: PoolType,
) -> (ActiveObstacle, Despawnable, PooledItem) {
    (
        ActiveObstacle {
            lane,
            obstacle_type,
            size,
        },
        Despawnable { z_center: world_z },
        PooledItem {
            pool_type,
            is_active: true,
        },
    )
}

fn spawn_sky_rail_maintenance_gate(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = sky_rail_active(
        obs_def.lane,
        ObstacleType::LowBarrier,
        obs_def.size,
        world_z,
        PoolType::SkyRailMaintenanceGate,
    );
    if let Some(entity) = pool.pop(PoolType::SkyRailMaintenanceGate) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            // Main gate structural frame
            PbrBundle {
                mesh: pool_assets.mesh_sky_gate_frame.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|gate| {
            // High-contrast amber hazard board
            gate.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_gate_hazard_panel.clone(),
                material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.08),
                ..default()
            });
            // Top magnetic lockout rail
            gate.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_gate_top_rail.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.36, 0.0),
                ..default()
            });
            // Side A-frame stanchions
            for x in [-0.96, 0.96] {
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_gate_stanchion.clone(),
                    material: pool_assets.mat_sky_obs_steel.clone(),
                    transform: Transform::from_xyz(x, -0.05, 0.0),
                    ..default()
                });
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_gate_foot.clone(),
                    material: pool_assets.mat_sky_obs_dark.clone(),
                    transform: Transform::from_xyz(x, -0.38, 0.0),
                    ..default()
                });
                // Amber warning beacon atop stanchion
                gate.spawn(PbrBundle {
                    mesh: pool_assets.mesh_barrier_strobe.clone(),
                    material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                    transform: Transform::from_xyz(x, 0.38, 0.04),
                    ..default()
                });
            }
            // Central cyan magnetic flux sensor
            gate.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_gate_sensor.clone(),
                material: pool_assets.mat_sky_obs_cyan_glow.clone(),
                transform: Transform::from_xyz(0.0, -0.10, 0.12),
                ..default()
            });
        });
}

fn spawn_sky_rail_clearance_beam(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = sky_rail_active(
        obs_def.lane,
        ObstacleType::HighHangingWire,
        obs_def.size,
        world_z,
        PoolType::SkyRailClearanceBeam,
    );
    if let Some(entity) = pool.pop(PoolType::SkyRailClearanceBeam) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.70, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            // Main horizontal aerodynamic gantry beam spanning the lane
            PbrBundle {
                mesh: pool_assets.mesh_sky_beam_body.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.70, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|beam| {
            // Vertical side aerodynamic struts
            for x in [-1.08, 1.08] {
                beam.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_beam_strut.clone(),
                    material: pool_assets.mat_sky_obs_dark.clone(),
                    transform: Transform::from_xyz(x, 0.20, 0.0),
                    ..default()
                });
                // Amber strobe at outer margins
                beam.spawn(PbrBundle {
                    mesh: pool_assets.mesh_barrier_strobe.clone(),
                    material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                    transform: Transform::from_xyz(x, 0.24, 0.12),
                    ..default()
                });
            }
            // Prominent clearance warning face plate
            beam.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_beam_plate.clone(),
                material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                transform: Transform::from_xyz(0.0, 0.02, 0.14),
                ..default()
            });
            // Cyan magnetic conduit along top
            beam.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_beam_conduit.clone(),
                material: pool_assets.mat_sky_obs_cyan_glow.clone(),
                transform: Transform::from_xyz(0.0, 0.16, 0.10),
                ..default()
            });
            // Downward clearance teeth marking ducking threshold
            for x in [-0.60, 0.0, 0.60] {
                beam.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_beam_tooth.clone(),
                    material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                    transform: Transform::from_xyz(x, -0.20, 0.10),
                    ..default()
                });
            }
        });
}

fn spawn_sky_rail_signal_tower(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = sky_rail_active(
        obs_def.lane,
        ObstacleType::TallPillar,
        obs_def.size,
        world_z,
        PoolType::SkyRailSignalTower,
    );
    if let Some(entity) = pool.pop(PoolType::SkyRailSignalTower) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            // Main tower core
            PbrBundle {
                mesh: pool_assets.mesh_sky_tower_core.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|tower| {
            // Reinforced track pedestal base
            tower.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_tower_pedestal.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, -1.75, 0.0),
                ..default()
            });
            // Cyan magnetic coil collar in center
            tower.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_tower_collar.clone(),
                material: pool_assets.mat_sky_obs_cyan_glow.clone(),
                transform: Transform::from_xyz(0.0, 0.05, 0.0),
                ..default()
            });
            // Side structural bracing trusses
            for x in [-0.78, 0.78] {
                tower.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_tower_truss.clone(),
                    material: pool_assets.mat_sky_obs_dark.clone(),
                    transform: Transform::from_xyz(x, 0.0, 0.0),
                    ..default()
                });
            }
            // Front-facing amber warning placard
            tower.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_tower_placard.clone(),
                material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                transform: Transform::from_xyz(0.0, -0.65, 0.52),
                ..default()
            });
            // Pinnacle red aviation warning beacon on top
            tower.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_tower_beacon.clone(),
                material: pool_assets.mat_sky_obs_red_beacon.clone(),
                transform: Transform::from_xyz(0.0, 1.95, 0.0),
                ..default()
            });
        });
}

fn spawn_sky_rail_derelict_mag_car(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = sky_rail_active(
        obs_def.lane,
        ObstacleType::StaticTrain,
        obs_def.size,
        world_z,
        PoolType::SkyRailDerelictMagCar,
    );
    if let Some(entity) = pool.pop(PoolType::SkyRailDerelictMagCar) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|car| {
            // Main aerodynamic dark body
            car.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_car_body.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            // Front aerodynamic sloped nose wedge (facing Kai at +Z)
            car.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_car_nose.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, -0.22, 5.25),
                ..default()
            });
            // Front sloped dark windshield
            car.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_car_windscreen.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, 0.45, 5.15)
                    .with_rotation(Quat::from_rotation_x(-0.25)),
                ..default()
            });
            // Left & right cyan magnetic levitation guide runners
            for x in [-1.05, 1.05] {
                car.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_mag_car_skirt.clone(),
                    material: pool_assets.mat_sky_obs_cyan_glow.clone(),
                    transform: Transform::from_xyz(x, -0.85, 0.0),
                    ..default()
                });
                car.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_mag_car_window_strip.clone(),
                    material: pool_assets.mat_sky_obs_dark.clone(),
                    transform: Transform::from_xyz(x * 1.02, 0.35, 0.0),
                    ..default()
                });
            }
            // Front emergency distress cluster: twin amber markers + upper red marker
            for x in [-0.75, 0.75] {
                car.spawn(PbrBundle {
                    mesh: pool_assets.mesh_barrier_strobe.clone(),
                    material: pool_assets.mat_sky_obs_hazard_amber.clone(),
                    transform: Transform::from_xyz(x, -0.05, 5.75),
                    ..default()
                });
            }
            car.spawn(PbrBundle {
                mesh: pool_assets.mesh_barrier_strobe.clone(),
                material: pool_assets.mat_sky_obs_red_beacon.clone(),
                transform: Transform::from_xyz(0.0, 0.95, 5.40),
                ..default()
            });
        });
}

fn spawn_sky_rail_mag_train(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
    speed: f32,
) {
    let active = sky_rail_active(
        obs_def.lane,
        ObstacleType::MovingTrain { speed },
        obs_def.size,
        world_z,
        PoolType::SkyRailMagTrain,
    );
    if let Some(entity) = pool.pop(PoolType::SkyRailMagTrain) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ))
        .with_children(|train| {
            // Main high-speed train body
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_train_body.clone(),
                material: pool_assets.mat_sky_obs_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            // Bullet aerodynamic nose cone (facing Kai at +Z)
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_train_nose.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, -0.25, 6.45),
                ..default()
            });
            // Panoramic sloped cockpit windscreen
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_train_windscreen.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, 0.48, 6.30)
                    .with_rotation(Quat::from_rotation_x(-0.30)),
                ..default()
            });
            // Piercing high-intensity headlamps cutting through the storm
            for x in [-0.72, 0.72] {
                train.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_mag_train_headlamp.clone(),
                    material: pool_assets.mat_sky_obs_headlight.clone(),
                    transform: Transform::from_xyz(x, -0.05, 7.02),
                    ..default()
                });
            }
            // Glowing cyan magnetic levitation flux runners along bottom skirts
            for x in [-1.06, 1.06] {
                train.spawn(PbrBundle {
                    mesh: pool_assets.mesh_sky_mag_train_skirt.clone(),
                    material: pool_assets.mat_sky_obs_cyan_glow.clone(),
                    transform: Transform::from_xyz(x, -0.88, 0.0),
                    ..default()
                });
            }
            // Rooftop aerodynamic stabilizer fin & red emergency beacon
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_sky_mag_train_fin.clone(),
                material: pool_assets.mat_sky_obs_dark.clone(),
                transform: Transform::from_xyz(0.0, 1.25, 3.5),
                ..default()
            });
            train.spawn(PbrBundle {
                mesh: pool_assets.mesh_barrier_strobe.clone(),
                material: pool_assets.mat_sky_obs_red_beacon.clone(),
                transform: Transform::from_xyz(0.0, 1.15, 6.25),
                ..default()
            });
        });
}

fn forbidden_active(
    lane: Lane,
    obstacle_type: ObstacleType,
    size: Vec3,
    world_z: f32,
    pool_type: PoolType,
) -> (ActiveObstacle, Despawnable, PooledItem) {
    (
        ActiveObstacle {
            lane,
            obstacle_type,
            size,
        },
        Despawnable { z_center: world_z },
        PooledItem {
            pool_type,
            is_active: true,
        },
    )
}

fn spawn_forbidden_quarantine_barrier(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = forbidden_active(
        obs_def.lane,
        ObstacleType::LowBarrier,
        obs_def.size,
        world_z,
        PoolType::ForbiddenQuarantineBarrier,
    );
    if let Some(entity) = pool.pop(PoolType::ForbiddenQuarantineBarrier) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_forbidden_gate_frame.clone(),
                material: pool_assets.mat_forbidden_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 0.42, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|barrier| {
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_gate_plate.clone(),
                material: pool_assets.mat_forbidden_obs_concrete.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.04),
                ..default()
            });
            barrier.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_gate_hazard_stripe.clone(),
                material: pool_assets.mat_forbidden_obs_amber.clone(),
                transform: Transform::from_xyz(0.0, -0.22, 0.12),
                ..default()
            });
            for x in [-0.92, 0.92] {
                barrier.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_gate_strobe.clone(),
                    material: pool_assets.mat_forbidden_obs_crimson.clone(),
                    transform: Transform::from_xyz(x, 0.38, 0.05),
                    ..default()
                });
            }
        });
}

fn spawn_forbidden_conduit_bundle(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = forbidden_active(
        obs_def.lane,
        ObstacleType::HighHangingWire,
        obs_def.size,
        world_z,
        PoolType::ForbiddenConduitBundle,
    );
    if let Some(entity) = pool.pop(PoolType::ForbiddenConduitBundle) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.7, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_forbidden_conduit_bracket.clone(),
                material: pool_assets.mat_forbidden_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.7, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|wire| {
            wire.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_conduit_core.clone(),
                material: pool_assets.mat_forbidden_obs_concrete.clone(),
                transform: Transform::from_xyz(0.0, -0.06, 0.0),
                ..default()
            });
            for x in [-0.65, 0.0, 0.65] {
                wire.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_conduit_spark.clone(),
                    material: pool_assets.mat_forbidden_obs_cyan.clone(),
                    transform: Transform::from_xyz(x, -0.15, 0.08),
                    ..default()
                });
            }
            wire.spawn(PbrBundle {
                mesh: pool_assets.mesh_wire_chevron.clone(),
                material: pool_assets.mat_wire_chevron.clone(),
                transform: Transform::from_xyz(0.0, -0.34, 0.0),
                ..default()
            });
        });
}

fn spawn_forbidden_containment_pillar(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = forbidden_active(
        obs_def.lane,
        ObstacleType::TallPillar,
        obs_def.size,
        world_z,
        PoolType::ForbiddenContainmentPillar,
    );
    if let Some(entity) = pool.pop(PoolType::ForbiddenContainmentPillar) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_forbidden_pillar_shaft.clone(),
                material: pool_assets.mat_forbidden_obs_concrete.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 2.0, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|pillar| {
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_pillar_rib.clone(),
                material: pool_assets.mat_forbidden_obs_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            for y in [-1.2, 1.2] {
                pillar.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_pillar_collar.clone(),
                    material: pool_assets.mat_forbidden_obs_steel.clone(),
                    transform: Transform::from_xyz(0.0, y, 0.0),
                    ..default()
                });
            }
            pillar.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_pillar_beacon.clone(),
                material: pool_assets.mat_forbidden_obs_crimson.clone(),
                transform: Transform::from_xyz(0.0, 1.95, 0.45),
                ..default()
            });
        });
}

fn spawn_forbidden_facility_transport(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
) {
    let active = forbidden_active(
        obs_def.lane,
        ObstacleType::StaticTrain,
        obs_def.size,
        world_z,
        PoolType::ForbiddenFacilityTransport,
    );
    if let Some(entity) = pool.pop(PoolType::ForbiddenFacilityTransport) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            PbrBundle {
                mesh: pool_assets.mesh_forbidden_transport_chassis.clone(),
                material: pool_assets.mat_forbidden_obs_steel.clone(),
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            active.1,
            active.2,
        ))
        .with_children(|transport| {
            transport.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_transport_nose.clone(),
                material: pool_assets.mat_forbidden_obs_concrete.clone(),
                transform: Transform::from_xyz(0.0, -0.25, 5.2),
                ..default()
            });
            for x in [-1.05, 1.05] {
                transport.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_transport_skirt.clone(),
                    material: pool_assets.mat_forbidden_obs_steel.clone(),
                    transform: Transform::from_xyz(x, -0.85, 0.0),
                    ..default()
                });
            }
            for z in [-5.8, 5.8] {
                transport.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_transport_marker.clone(),
                    material: pool_assets.mat_forbidden_obs_crimson.clone(),
                    transform: Transform::from_xyz(0.0, 1.05, z),
                    ..default()
                });
            }
        });
}

fn spawn_forbidden_heavy_mover(
    commands: &mut Commands,
    pool_assets: &PoolAssets,
    pool: &mut EntityPool,
    obs_def: ObstacleSpawnDef,
    world_z: f32,
    speed: f32,
) {
    let active = forbidden_active(
        obs_def.lane,
        ObstacleType::MovingTrain { speed },
        obs_def.size,
        world_z,
        PoolType::ForbiddenHeavyMover,
    );
    if let Some(entity) = pool.pop(PoolType::ForbiddenHeavyMover) {
        commands.entity(entity).insert((
            Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
            Visibility::Inherited,
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ));
        return;
    }
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(obs_def.lane.x_pos(), 1.25, world_z),
                ..default()
            },
            active.0,
            MovingObstacle { speed },
            active.1,
            active.2,
        ))
        .with_children(|mover| {
            mover.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_mover_chassis.clone(),
                material: pool_assets.mat_forbidden_obs_steel.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            mover.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_mover_wedge.clone(),
                material: pool_assets.mat_forbidden_obs_concrete.clone(),
                transform: Transform::from_xyz(0.0, -0.15, 6.2),
                ..default()
            });
            for x in [-0.65, 0.65] {
                mover.spawn(PbrBundle {
                    mesh: pool_assets.mesh_forbidden_mover_headlamp.clone(),
                    material: pool_assets.mat_forbidden_obs_amber.clone(),
                    transform: Transform::from_xyz(x, -0.15, 7.1),
                    ..default()
                });
            }
            mover.spawn(PbrBundle {
                mesh: pool_assets.mesh_forbidden_mover_strobe.clone(),
                material: pool_assets.mat_forbidden_obs_crimson.clone(),
                transform: Transform::from_xyz(0.0, 1.30, 5.5),
                ..default()
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obstacles::check_obstacle_vertical_collision;
    use crate::zones::ZONES;

    #[test]
    fn debug_start_track_window_is_offset_without_replaying_prior_distance() {
        assert_eq!(initial_track_spawn_z(0.0), 20.0);
        assert_eq!(initial_track_spawn_z(5100.0), -5080.0);

        let first_center = initial_track_spawn_z(5100.0) - SEGMENT_LENGTH * 0.5;
        let last_center = first_center - SEGMENT_LENGTH * 4.0;
        assert_eq!(first_center, -5100.0);
        assert_eq!(last_center, -5260.0);
        assert!(last_center.abs() < 5100.0 + 200.0);
    }

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
    fn test_obstacle_visual_selection_zone_3_industrial_sector() {
        let test_distances = [
            ZONES[2].start_distance,
            4500.0,
            5200.0,
            ZONES[2].end_distance - 0.001,
        ];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::IndustrialRoadBarrier,
                "Zone 3 must select an industrial road barrier at {dist}m"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::IndustrialOverheadPipe,
                "Zone 3 must select an industrial overhead pipe"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::IndustrialSupportPillar,
                "Zone 3 must select an industrial support pillar"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::IndustrialFreightTransport,
                "Zone 3 must select an industrial freight transporter"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::IndustrialCargoTransport,
                "Zone 3 must select an industrial cargo transporter"
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zone_4_flooded_metro() {
        let test_distances = [6000.0, 6300.0, 6750.0, 7200.0, 7650.0, 7999.0];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::FloodedBarrier,
                "Zone 4 must select a flooded maintenance barrier"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::FloodedPowerCable,
                "Zone 4 must select a flooded power cable"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::FloodedPumpColumn,
                "Zone 4 must select a flooded pump column"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::FloodedSubmergedTrain,
                "Zone 4 must select a flooded submerged train"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::FloodedMovingTrain,
                "Zone 4 must select a flooded moving train"
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zone_5_sky_rail() {
        let test_distances = [8000.0, 8300.0, 8700.0, 9200.0, 9650.0, 9999.0];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::SkyRailMaintenanceGate,
                "Zone 5 must select a Sky Rail maintenance gate"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::SkyRailClearanceBeam,
                "Zone 5 must select a Sky Rail clearance beam"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::SkyRailSignalTower,
                "Zone 5 must select a Sky Rail signal tower"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::SkyRailDerelictMagCar,
                "Zone 5 must select a Sky Rail derelict mag-car"
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::SkyRailMagTrain,
                "Zone 5 must select a Sky Rail mag-train"
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zone_6_forbidden_line() {
        let test_distances = [10000.0, 10500.0, 11000.0, 11500.0, 11999.0];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::ForbiddenQuarantineBarrier,
                "Zone 6 distance {}m must select Forbidden Line quarantine barrier",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::ForbiddenConduitBundle,
                "Zone 6 distance {}m must select Forbidden Line conduit bundle",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::ForbiddenContainmentPillar,
                "Zone 6 distance {}m must select Forbidden Line containment pillar",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::ForbiddenFacilityTransport,
                "Zone 6 distance {}m must select Forbidden Line facility transport",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::ForbiddenHeavyMover,
                "Zone 6 distance {}m must select Forbidden Line heavy mover",
                dist
            );
        }
    }

    #[test]
    fn test_obstacle_visual_selection_zone_7_protected() {
        let test_distances = [ZONES[6].start_distance, 13000.0];
        for &dist in &test_distances {
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::LowBarrier, dist),
                PoolType::LowBarrier,
                "Post-Zone-6 distance {}m must fall back to default LowBarrier",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::HighHangingWire, dist),
                PoolType::HighHangingWire,
                "Post-Zone-6 distance {}m must fall back to default HighHangingWire",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::TallPillar, dist),
                PoolType::TallPillar,
                "Post-Zone-6 distance {}m must fall back to default TallPillar",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::StaticTrain, dist),
                PoolType::StaticTrain,
                "Post-Zone-6 distance {}m must fall back to default StaticTrain",
                dist
            );
            assert_eq!(
                resolve_obstacle_pool_type(ObstacleType::MovingTrain { speed: 10.0 }, dist),
                PoolType::MovingTrain,
                "Post-Zone-6 distance {}m must fall back to default MovingTrain",
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
        let e_industrial_barrier = Entity::from_raw(11);
        let e_industrial_pipe = Entity::from_raw(12);
        let e_industrial_pillar = Entity::from_raw(13);
        let e_industrial_freight = Entity::from_raw(14);
        let e_industrial_cargo = Entity::from_raw(15);
        let e_flooded_barrier = Entity::from_raw(16);
        let e_flooded_cable = Entity::from_raw(17);
        let e_flooded_pillar = Entity::from_raw(18);
        let e_flooded_freight = Entity::from_raw(19);
        let e_flooded_cargo = Entity::from_raw(20);
        let e_sky_rail_gate = Entity::from_raw(21);
        let e_sky_rail_beam = Entity::from_raw(22);
        let e_sky_rail_tower = Entity::from_raw(23);
        let e_sky_rail_car = Entity::from_raw(24);
        let e_sky_rail_train = Entity::from_raw(25);
        let e_forbidden_barrier = Entity::from_raw(26);
        let e_forbidden_conduit = Entity::from_raw(27);
        let e_forbidden_pillar = Entity::from_raw(28);
        let e_forbidden_transport = Entity::from_raw(29);
        let e_forbidden_mover = Entity::from_raw(30);

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
        pool.push(PoolType::IndustrialRoadBarrier, e_industrial_barrier);
        pool.push(PoolType::IndustrialOverheadPipe, e_industrial_pipe);
        pool.push(PoolType::IndustrialSupportPillar, e_industrial_pillar);
        pool.push(PoolType::IndustrialFreightTransport, e_industrial_freight);
        pool.push(PoolType::IndustrialCargoTransport, e_industrial_cargo);
        pool.push(PoolType::FloodedBarrier, e_flooded_barrier);
        pool.push(PoolType::FloodedPowerCable, e_flooded_cable);
        pool.push(PoolType::FloodedPumpColumn, e_flooded_pillar);
        pool.push(PoolType::FloodedSubmergedTrain, e_flooded_freight);
        pool.push(PoolType::FloodedMovingTrain, e_flooded_cargo);
        pool.push(PoolType::SkyRailMaintenanceGate, e_sky_rail_gate);
        pool.push(PoolType::SkyRailClearanceBeam, e_sky_rail_beam);
        pool.push(PoolType::SkyRailSignalTower, e_sky_rail_tower);
        pool.push(PoolType::SkyRailDerelictMagCar, e_sky_rail_car);
        pool.push(PoolType::SkyRailMagTrain, e_sky_rail_train);
        pool.push(PoolType::ForbiddenQuarantineBarrier, e_forbidden_barrier);
        pool.push(PoolType::ForbiddenConduitBundle, e_forbidden_conduit);
        pool.push(PoolType::ForbiddenContainmentPillar, e_forbidden_pillar);
        pool.push(PoolType::ForbiddenFacilityTransport, e_forbidden_transport);
        pool.push(PoolType::ForbiddenHeavyMover, e_forbidden_mover);

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
        assert_eq!(pool.industrial_barriers.len(), 1);
        assert_eq!(pool.industrial_pipes.len(), 1);
        assert_eq!(pool.industrial_pillars.len(), 1);
        assert_eq!(pool.industrial_freight.len(), 1);
        assert_eq!(pool.industrial_cargo.len(), 1);
        assert_eq!(pool.flooded_barriers.len(), 1);
        assert_eq!(pool.flooded_cables.len(), 1);
        assert_eq!(pool.flooded_pillars.len(), 1);
        assert_eq!(pool.flooded_freight.len(), 1);
        assert_eq!(pool.flooded_cargo.len(), 1);
        assert_eq!(pool.sky_rail_gates.len(), 1);
        assert_eq!(pool.sky_rail_beams.len(), 1);
        assert_eq!(pool.sky_rail_towers.len(), 1);
        assert_eq!(pool.sky_rail_cars.len(), 1);
        assert_eq!(pool.sky_rail_trains.len(), 1);
        assert_eq!(pool.forbidden_barriers.len(), 1);
        assert_eq!(pool.forbidden_conduits.len(), 1);
        assert_eq!(pool.forbidden_pillars.len(), 1);
        assert_eq!(pool.forbidden_transports.len(), 1);
        assert_eq!(pool.forbidden_movers.len(), 1);

        // Pop from Zone 6 pools
        assert_eq!(
            pool.pop(PoolType::ForbiddenQuarantineBarrier),
            Some(e_forbidden_barrier)
        );
        assert_eq!(
            pool.pop(PoolType::ForbiddenConduitBundle),
            Some(e_forbidden_conduit)
        );
        assert_eq!(
            pool.pop(PoolType::ForbiddenContainmentPillar),
            Some(e_forbidden_pillar)
        );
        assert_eq!(
            pool.pop(PoolType::ForbiddenFacilityTransport),
            Some(e_forbidden_transport)
        );
        assert_eq!(
            pool.pop(PoolType::ForbiddenHeavyMover),
            Some(e_forbidden_mover)
        );

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

        // Zone 3 pools remain isolated from Metro, Neon, and Flooded pools.
        assert_eq!(
            pool.pop(PoolType::IndustrialRoadBarrier),
            Some(e_industrial_barrier)
        );
        assert_eq!(
            pool.pop(PoolType::IndustrialOverheadPipe),
            Some(e_industrial_pipe)
        );
        assert_eq!(
            pool.pop(PoolType::IndustrialSupportPillar),
            Some(e_industrial_pillar)
        );
        assert_eq!(
            pool.pop(PoolType::IndustrialFreightTransport),
            Some(e_industrial_freight)
        );
        assert_eq!(
            pool.pop(PoolType::IndustrialCargoTransport),
            Some(e_industrial_cargo)
        );

        // Zone 4 pools remain isolated
        assert_eq!(pool.pop(PoolType::FloodedBarrier), Some(e_flooded_barrier));
        assert_eq!(pool.pop(PoolType::FloodedPowerCable), Some(e_flooded_cable));
        assert_eq!(
            pool.pop(PoolType::FloodedPumpColumn),
            Some(e_flooded_pillar)
        );
        assert_eq!(
            pool.pop(PoolType::FloodedSubmergedTrain),
            Some(e_flooded_freight)
        );
        assert_eq!(
            pool.pop(PoolType::FloodedMovingTrain),
            Some(e_flooded_cargo)
        );

        // Zone 5 pools remain isolated
        assert_eq!(
            pool.pop(PoolType::SkyRailMaintenanceGate),
            Some(e_sky_rail_gate)
        );
        assert_eq!(
            pool.pop(PoolType::SkyRailClearanceBeam),
            Some(e_sky_rail_beam)
        );
        assert_eq!(
            pool.pop(PoolType::SkyRailSignalTower),
            Some(e_sky_rail_tower)
        );
        assert_eq!(
            pool.pop(PoolType::SkyRailDerelictMagCar),
            Some(e_sky_rail_car)
        );
        assert_eq!(pool.pop(PoolType::SkyRailMagTrain), Some(e_sky_rail_train));

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
