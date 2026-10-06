use bevy::prelude::*;
use crate::types::*;
use crate::zones::get_zone_for_distance;
use rand::Rng;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_moving_trains,
                lane_aware_collision_check,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[allow(dead_code)]
pub fn spawn_segment_obstacles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    z_start: f32,
    z_end: f32,
    distance: f32,
    density_override: Option<f32>,
) {
    let mut rng = rand::thread_rng();
    let zone = get_zone_for_distance(distance);
    let density = density_override.unwrap_or(zone.obstacle_density);

    let hazard_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.4, 0.1),
        emissive: LinearRgba::new(1.2, 0.4, 0.0, 1.0),
        perceptual_roughness: 0.4,
        ..default()
    });

    let pillar_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.22, 0.26),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    let train_body_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.18, 0.25),
        metallic: 0.9,
        perceptual_roughness: 0.3,
        ..default()
    });

    let train_light_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.95, 0.8),
        emissive: LinearRgba::new(4.0, 3.8, 2.5, 1.0),
        ..default()
    });

    let wire_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.1),
        emissive: LinearRgba::new(1.5, 1.2, 0.0, 1.0),
        ..default()
    });

    // Subdivide segment into 2 obstacle slots
    let slots = [z_start - 12.0, z_start - 28.0];

    for &z_pos in &slots {
        if z_pos < z_end {
            continue;
        }

        if rng.gen_bool(density as f64) {
            let pattern = rng.gen_range(0..5);

            match pattern {
                0 => {
                    // Pattern 0: Low Barrier on one lane, Pillar on another
                    let blocked_idx = rng.gen_range(-1..=1);
                    let jump_idx = (blocked_idx + 1 + rng.gen_range(0..2)) % 3 - 1;

                    let blocked_lane = Lane::from_index(blocked_idx);
                    let jump_lane = Lane::from_index(jump_idx);

                    // Tall Pillar
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.8, 4.0, 1.2)),
                            material: pillar_mat.clone(),
                            transform: Transform::from_xyz(blocked_lane.x_pos(), 2.0, z_pos),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: blocked_lane,
                            obstacle_type: ObstacleType::TallPillar,
                            size: Vec3::new(1.8, 4.0, 1.2),
                        },
                        Despawnable { z_center: z_pos },
                    ));

                    // Low Barrier (jumpable)
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(2.1, 0.85, 0.4)),
                            material: hazard_mat.clone(),
                            transform: Transform::from_xyz(jump_lane.x_pos(), 0.42, z_pos),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: jump_lane,
                            obstacle_type: ObstacleType::LowBarrier,
                            size: Vec3::new(2.1, 0.85, 0.4),
                        },
                        Despawnable { z_center: z_pos },
                    ));
                }
                1 => {
                    // Pattern 1: High Hanging Cable across one lane (slide under)
                    let slide_lane = Lane::from_index(rng.gen_range(-1..=1));
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(2.2, 0.5, 0.3)),
                            material: wire_mat.clone(),
                            transform: Transform::from_xyz(slide_lane.x_pos(), 1.7, z_pos),
                            ..default()
                        },
                        ActiveObstacle {
                            lane: slide_lane,
                            obstacle_type: ObstacleType::HighHangingWire,
                            size: Vec3::new(2.2, 0.5, 0.3),
                        },
                        Despawnable { z_center: z_pos },
                    ));
                }
                2 => {
                    // Pattern 2: Stationary Metro Train on one lane
                    let train_lane = Lane::from_index(rng.gen_range(-1..=1));
                    let lane_x = train_lane.x_pos();

                    commands
                        .spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(2.2, 2.5, 12.0)),
                                material: train_body_mat.clone(),
                                transform: Transform::from_xyz(lane_x, 1.25, z_pos),
                                ..default()
                            },
                            ActiveObstacle {
                                lane: train_lane,
                                obstacle_type: ObstacleType::StaticTrain,
                                size: Vec3::new(2.2, 2.5, 12.0),
                            },
                            Despawnable { z_center: z_pos },
                        ))
                        .with_children(|train| {
                            train.spawn(PbrBundle {
                                mesh: meshes.add(Sphere::new(0.2)),
                                material: train_light_mat.clone(),
                                transform: Transform::from_xyz(-0.7, 0.2, 6.05),
                                ..default()
                            });
                            train.spawn(PbrBundle {
                                mesh: meshes.add(Sphere::new(0.2)),
                                material: train_light_mat.clone(),
                                transform: Transform::from_xyz(0.7, 0.2, 6.05),
                                ..default()
                            });
                        });
                }
                3 => {
                    // Pattern 3: Moving Metro Train approaching if distance > 600m
                    if distance > 600.0 {
                        let train_lane = Lane::from_index(rng.gen_range(-1..=1));
                        let lane_x = train_lane.x_pos();

                        commands
                            .spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(2.2, 2.5, 14.0)),
                                    material: train_body_mat.clone(),
                                    transform: Transform::from_xyz(lane_x, 1.25, z_pos - 15.0),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: train_lane,
                                    obstacle_type: ObstacleType::MovingTrain { speed: 11.0 },
                                    size: Vec3::new(2.2, 2.5, 14.0),
                                },
                                MovingObstacle { speed: 11.0 },
                                Despawnable { z_center: z_pos },
                            ))
                            .with_children(|train| {
                                train.spawn(PbrBundle {
                                    mesh: meshes.add(Sphere::new(0.25)),
                                    material: train_light_mat.clone(),
                                    transform: Transform::from_xyz(-0.7, 0.2, 7.05),
                                    ..default()
                                });
                                train.spawn(PbrBundle {
                                    mesh: meshes.add(Sphere::new(0.25)),
                                    material: train_light_mat.clone(),
                                    transform: Transform::from_xyz(0.7, 0.2, 7.05),
                                    ..default()
                                });
                            });
                    }
                }
                _ => {
                    // Double barrier with one safe passage
                    let safe_idx = rng.gen_range(-1..=1);
                    for l in -1..=1 {
                        if l != safe_idx {
                            let barrier_lane = Lane::from_index(l);
                            commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(2.1, 0.85, 0.4)),
                                    material: hazard_mat.clone(),
                                    transform: Transform::from_xyz(barrier_lane.x_pos(), 0.42, z_pos),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane: barrier_lane,
                                    obstacle_type: ObstacleType::LowBarrier,
                                    size: Vec3::new(2.1, 0.85, 0.4),
                                },
                                Despawnable { z_center: z_pos },
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn update_moving_trains(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Despawnable, &MovingObstacle)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut despawn, moving) in query.iter_mut() {
        transform.translation.z += moving.speed * dt;
        despawn.z_center = transform.translation.z;
    }
}

// -------------------------------------------------------------
// HIGH PERFORMANCE LANE-AWARE COLLISION DETECTION
// -------------------------------------------------------------
fn lane_aware_collision_check(
    mut commands: Commands,
    mut player_q: Query<(&mut Player, &Transform)>,
    obs_q: Query<(Entity, &ActiveObstacle, &Transform)>,
    mut powerups: ResMut<ActivePowerUps>,
    mut stats: ResMut<GameRunStats>,
    mut next_state: ResMut<NextState<AppState>>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let (mut player, p_trans) = match player_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    if player.invulnerable_timer > 0.0 || powerups.overdrive_timer > 0.0 {
        return;
    }

    let p_pos = p_trans.translation;
    let p_lane = player.lane;
    let p_y = p_pos.y;
    let p_z = p_pos.z;

    for (obs_entity, obs, obs_trans) in obs_q.iter() {
        // Fast Lane rejection: only obstacles on player's lane or transitional boundary
        if obs.lane != p_lane {
            // Generous lateral forgiveness (0.85m margin vs 1.2m half-lane spacing)
            let x_dist = (p_pos.x - obs_trans.translation.x).abs();
            if x_dist > 0.85 {
                continue;
            }
        }

        let o_pos = obs_trans.translation;
        let z_dist = (p_z - o_pos.z).abs();
        // 15% longitudinal forgiveness margin prevents frustrating near-miss phantom collisions
        let z_threshold = 0.35 + (obs.size.z * 0.42);

        // Fast Z rejection
        if z_dist > z_threshold {
            continue;
        }

        // Vertical collision check based on obstacle type with fair clearance forgiveness
        let o_half_y = obs.size.y * 0.5;
        let o_bottom = o_pos.y - o_half_y;
        let o_top = o_pos.y + o_half_y;

        let collided = match obs.obstacle_type {
            ObstacleType::LowBarrier => {
                // Cleared if jumping (feet clear hurdle with 0.28m vertical forgiveness)
                p_y < (o_top - 0.28)
            }
            ObstacleType::HighHangingWire => {
                // Cleared if sliding (staying low with 0.22m ducking forgiveness)
                !player.is_sliding || (p_y + 0.5) > (o_bottom + 0.22)
            }
            ObstacleType::TallPillar => {
                true // Full lane block
            }
            ObstacleType::StaticTrain | ObstacleType::MovingTrain { .. } => {
                // Cleared if on train roof with 0.25m forgiveness
                p_y < (o_top - 0.25)
            }
        };

        if collided {
            // Shield absorbs hit
            if powerups.shield {
                if powerups.shield_hits > 1 {
                    powerups.shield_hits -= 1;
                } else {
                    powerups.shield = false;
                    powerups.shield_hits = 0;
                }
                player.invulnerable_timer = 1.5;
                stats.stumble_intensity = 0.8;
                sfx.send(SoundEffect::ShieldBreak);
                commands.entity(obs_entity).despawn_recursive();
                return;
            }

            // Ex-security Arin character gets 1 second-chance stumble recovery
            if player.character == CharacterType::Arin && stats.stumble_intensity == 0.0 {
                stats.stumble_intensity = 1.0;
                player.invulnerable_timer = 1.8;
                sfx.send(SoundEffect::Stumble);
                commands.entity(obs_entity).despawn_recursive();
                return;
            }

            // Fatal crash
            sfx.send(SoundEffect::Crash);
            next_state.set(AppState::GameOver);
            return;
        }
    }
}
