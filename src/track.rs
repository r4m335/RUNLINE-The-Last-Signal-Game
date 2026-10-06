use bevy::prelude::*;
use crate::types::*;
use crate::zones::get_zone_for_distance;
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    )>,
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
                &mut meshes,
                &mut materials,
                z_center,
                SEGMENT_LENGTH,
                0.0,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            &mut meshes,
            &mut materials,
            z_center,
            SEGMENT_LENGTH,
            stats.distance,
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
    _distance: f32,
) -> PatternChunk {
    use rand::Rng;
    let chunk = select_validated_pattern(complexity, prev_chunk, player_speed);
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
                            mesh: pool_assets.mesh_barrier.clone(),
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
                    ));
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
                            mesh: pool_assets.mesh_wire.clone(),
                            material: pool_assets.mat_wire.clone(),
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
                    ));
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
                            mesh: pool_assets.mesh_pillar.clone(),
                            material: pool_assets.mat_pillar.clone(),
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
                    ));
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
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_light.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(-0.7, 0.2, 6.05),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_light.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(0.7, 0.2, 6.05),
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
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_light.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(-0.7, 0.2, 7.05),
                            ..default()
                        });
                        train.spawn(PbrBundle {
                            mesh: pool_assets.mesh_train_light.clone(),
                            material: pool_assets.mat_train_light.clone(),
                            transform: Transform::from_xyz(0.7, 0.2, 7.05),
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
        let roll = rng.gen_range(0..3);
        let (p_type, p_mat) = match roll {
            0 => (CollectibleType::EchoShield, pool_assets.mat_powerup_shield.clone()),
            1 => (CollectibleType::Magnet, pool_assets.mat_powerup_magnet.clone()),
            _ => (CollectibleType::Overdrive, pool_assets.mat_powerup_overdrive.clone()),
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
                PbrBundle {
                    mesh: pool_assets.mesh_powerup.clone(),
                    material: p_mat,
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
            ));
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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    z_center: f32,
    length: f32,
    distance: f32,
) {
    let zone = get_zone_for_distance(distance);

    let floor_mat = materials.add(StandardMaterial {
        base_color: zone.track_base_color,
        perceptual_roughness: 0.6,
        metallic: 0.2,
        ..default()
    });

    let rail_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.75, 0.8),
        emissive: zone.rail_emissive,
        metallic: 0.95,
        perceptual_roughness: 0.1,
        ..default()
    });

    let arch_mat = materials.add(StandardMaterial {
        base_color: zone.arch_color,
        metallic: 0.7,
        perceptual_roughness: 0.4,
        ..default()
    });

    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: zone.accent_glow,
        ..default()
    });

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // Main Track Bed / Floor
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(8.6, 0.4, length)),
                material: floor_mat,
                transform: Transform::from_xyz(0.0, -0.2, 0.0),
                ..default()
            });

            // Side Balustrades / Low Walls
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.8, length)),
                material: arch_mat.clone(),
                transform: Transform::from_xyz(-4.3, 0.2, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.8, length)),
                material: arch_mat.clone(),
                transform: Transform::from_xyz(4.3, 0.2, 0.0),
                ..default()
            });

            // 6 Glowing Steel Rails (2 for each of 3 lanes)
            let rail_mesh = meshes.add(Cuboid::new(0.08, 0.08, length));
            for lane_idx in -1..=1 {
                let center_x = Lane::from_index(lane_idx).x_pos();
                seg.spawn(PbrBundle {
                    mesh: rail_mesh.clone(),
                    material: rail_mat.clone(),
                    transform: Transform::from_xyz(center_x - 0.48, 0.04, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: rail_mesh.clone(),
                    material: rail_mat.clone(),
                    transform: Transform::from_xyz(center_x + 0.48, 0.04, 0.0),
                    ..default()
                });
            }

            // Cross ties (sleepers)
            let tie_mesh = meshes.add(Cuboid::new(8.0, 0.05, 0.35));
            let num_ties = (length / 2.5) as i32;
            for i in -num_ties / 2..=num_ties / 2 {
                seg.spawn(PbrBundle {
                    mesh: tie_mesh.clone(),
                    material: arch_mat.clone(),
                    transform: Transform::from_xyz(0.0, 0.01, i as f32 * 2.5),
                    ..default()
                });
            }

            // Overhead Cyber Arch & Neon Beam
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(9.2, 0.5, 0.8)),
                material: arch_mat.clone(),
                transform: Transform::from_xyz(0.0, 4.2, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(8.4, 0.15, 0.2)),
                material: accent_mat,
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                ..default()
            });

            // Support Pillars
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.6, 4.4, 0.8)),
                material: arch_mat.clone(),
                transform: Transform::from_xyz(-4.5, 2.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.6, 4.4, 0.8)),
                material: arch_mat,
                transform: Transform::from_xyz(4.5, 2.0, 0.0),
                ..default()
            });
        });
}
