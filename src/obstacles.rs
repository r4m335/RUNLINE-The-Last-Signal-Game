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
        base_color: Color::srgb(1.0, 0.50, 0.05),
        emissive: LinearRgba::new(3.5, 1.2, 0.0, 1.0),
        perceptual_roughness: 0.25,
        ..default()
    });

    let pillar_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.20, 0.24),
        metallic: 0.90,
        perceptual_roughness: 0.25,
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
        emissive: LinearRgba::new(4.5, 4.0, 2.5, 1.0),
        ..default()
    });

    let wire_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.88, 0.10),
        emissive: LinearRgba::new(4.5, 3.8, 0.2, 1.0),
        perceptual_roughness: 0.25,
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

        // Vertical collision check based on explicit player capsule bounding volume (feet bottom and head top)
        let collided = check_obstacle_vertical_collision(
            p_y,
            p_trans.scale.y,
            player.is_sliding,
            obs.obstacle_type,
            o_pos.y,
            obs.size.y,
        );

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

// -------------------------------------------------------------
// EXPLICIT PLAYER CAPSULE BOUNDING VOLUME VERTICAL COLLISION
// -------------------------------------------------------------
/// Evaluates vertical collision between the player's physical volume and an obstacle.
/// - Standing Kai: root at Y=0.65m, feet touch ground at Y=0.0m (bottom = p_y - 0.65m), head top at Y=1.60m (top = p_y + 0.95m).
/// - Sliding Kai: low parkour slide (scale.y = 0.45, root at 0.35m -> bottom ≈ 0.06m, top ≈ 0.78m).
pub fn check_obstacle_vertical_collision(
    p_y: f32,
    p_scale_y: f32,
    is_sliding: bool,
    obstacle_type: ObstacleType,
    obs_y: f32,
    obs_size_y: f32,
) -> bool {
    let p_bottom = p_y - (0.65 * p_scale_y);
    let p_top = p_y + (0.95 * p_scale_y);

    let o_half_y = obs_size_y * 0.5;
    let o_bottom = obs_y - o_half_y;
    let o_top = obs_y + o_half_y;

    match obstacle_type {
        ObstacleType::LowBarrier => {
            // Cleared if jumping: feet must clear barrier top (with 0.10m fair foot clearance tolerance)
            // If standing (p_bottom ~ 0.0) or sliding (p_bottom ~ 0.06), collides with hurdle top (~0.845m)
            p_bottom < (o_top - 0.10)
        }
        ObstacleType::HighHangingWire => {
            // Cleared if sliding/ducking: player's head must stay beneath wire bottom (with 0.12m ducking clearance tolerance)
            // If standing (p_top ~ 1.60m) or jumping, collides with severed wire bottom (~1.45m)
            !is_sliding || (p_top > (o_bottom + 0.12))
        }
        ObstacleType::TallPillar => {
            // Full 4.0m vertical lane block: must lane-switch
            true
        }
        ObstacleType::StaticTrain | ObstacleType::MovingTrain { .. } => {
            // Cleared if jumping onto / clearing carriage roof
            p_bottom < (o_top - 0.10)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_low_barrier_standing_hits() {
        // Standing at normal ground Y=0.65, scale=1.0, not sliding
        // LowBarrier center Y=0.42, height=0.85 -> top=0.845
        let collided = check_obstacle_vertical_collision(
            0.65, 1.0, false, ObstacleType::LowBarrier, 0.42, 0.85,
        );
        assert!(collided, "Standing player must collide with low barrier");
    }

    #[test]
    fn test_low_barrier_jump_clears() {
        // Jumping player at realistic apex (Y=1.84m with v0=9.0m/s, ~1.19m vertical rise)
        let collided = check_obstacle_vertical_collision(
            1.84, 1.0, false, ObstacleType::LowBarrier, 0.42, 0.85,
        );
        assert!(!collided, "Jumping player at Y=1.84m must clear low barrier");
    }

    #[test]
    fn test_low_barrier_sliding_hits() {
        // Sliding into low barrier (scale.y=0.45, Y=0.35, is_sliding=true)
        let collided = check_obstacle_vertical_collision(
            0.35, 0.45, true, ObstacleType::LowBarrier, 0.42, 0.85,
        );
        assert!(collided, "Sliding player must NOT clear low barrier (must jump)");
    }

    #[test]
    fn test_hanging_wire_standing_hits() {
        // Standing at Y=0.65, scale=1.0 -> head top = 1.60m
        // HighHangingWire center Y=1.7, height=0.5 -> bottom=1.45m
        let collided = check_obstacle_vertical_collision(
            0.65, 1.0, false, ObstacleType::HighHangingWire, 1.7, 0.5,
        );
        assert!(collided, "Standing player must hit high hanging wire");
    }

    #[test]
    fn test_hanging_wire_sliding_clears() {
        // Sliding at Y=0.35, scale=0.45, is_sliding=true -> head top ≈ 0.78m < 1.45m
        let collided = check_obstacle_vertical_collision(
            0.35, 0.45, true, ObstacleType::HighHangingWire, 1.7, 0.5,
        );
        assert!(!collided, "Sliding player must cleanly duck under hanging wire");
    }

    #[test]
    fn test_hanging_wire_jumping_hits() {
        // Jumping into hanging wire (Y=2.0m, scale=1.0, not sliding)
        let collided = check_obstacle_vertical_collision(
            2.0, 1.0, false, ObstacleType::HighHangingWire, 1.7, 0.5,
        );
        assert!(collided, "Jumping into hanging wire must collide");
    }

    #[test]
    fn test_pillar_always_collides_vertically() {
        // Pillar is full lane block
        let collided = check_obstacle_vertical_collision(
            0.65, 1.0, false, ObstacleType::TallPillar, 2.0, 4.0,
        );
        assert!(collided, "Pillar must always collide in vertical check");
    }
}

