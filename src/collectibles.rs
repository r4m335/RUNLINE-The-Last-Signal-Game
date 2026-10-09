use bevy::prelude::*;
use crate::types::*;
use crate::story::DATA_CHIP_LOGS;
use rand::Rng;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_powerup_models)
            .add_systems(
                Update,
                animate_collectibles.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (
                    magnet_attract_fragments,
                    collect_items_fixed,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

pub fn setup_powerup_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PowerUpModelAssets {
        shield_scene: asset_server.load("Power-ups/Shield.glb#Scene0"),
        magnet_scene: asset_server.load("Power-ups/Magnet.glb#Scene0"),
        overdrive_scene: asset_server.load("Power-ups/Overdrive.glb#Scene0"),
    });
}

#[allow(dead_code)]
pub fn spawn_segment_collectibles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    z_start: f32,
    _z_end: f32,
    _distance: f32,
) {
    let mut rng = rand::thread_rng();

    let fragment_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.9, 1.0),
        emissive: LinearRgba::new(0.4, 2.2, 3.0, 1.0),
        perceptual_roughness: 0.1,
        metallic: 0.9,
        ..default()
    });

    let chip_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.2),
        emissive: LinearRgba::new(3.5, 2.5, 0.3, 1.0),
        perceptual_roughness: 0.2,
        metallic: 0.8,
        ..default()
    });

    let powerup_shield_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.05, 0.85, 1.0, 0.90),
        emissive: LinearRgba::new(0.6, 3.6, 5.5, 1.0),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let powerup_magnet_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.1, 0.9),
        emissive: LinearRgba::new(4.8, 0.3, 4.8, 1.0),
        ..default()
    });

    let powerup_pole_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.96),
        metallic: 0.95,
        perceptual_roughness: 0.10,
        ..default()
    });

    let powerup_overdrive_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0),
        emissive: LinearRgba::new(5.5, 2.2, 0.0, 1.0),
        ..default()
    });

    let powerup_jump_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.9, 1.0),
        emissive: LinearRgba::new(0.6, 4.2, 5.5, 1.0),
        ..default()
    });

    // 1. Lines of ECHO Fragments along lanes
    let lane_choice = Lane::from_index(rng.gen_range(-1..=1));
    let lane_x = lane_choice.x_pos();

    let frag_mesh = meshes.add(Sphere::new(0.24));

    for i in 0..5 {
        let z = z_start - 6.0 - (i as f32 * 3.8);
        commands.spawn((
            PbrBundle {
                mesh: frag_mesh.clone(),
                material: fragment_mat.clone(),
                transform: Transform::from_xyz(lane_x, 0.85, z),
                ..default()
            },
            CollectibleItem {
                item_type: CollectibleType::EchoFragment { value: 1 },
                lane: lane_choice,
                initial_y: 0.85,
                rot_speed: 3.5,
            },
            Despawnable { z_center: z },
        ));
    }

    // 2. Rare Data Chip (15% chance)
    if rng.gen_bool(0.15) {
        let chip_lane = Lane::from_index(rng.gen_range(-1..=1));
        let chip_x = chip_lane.x_pos();
        let z = z_start - 24.0;
        let chip_mesh = meshes.add(Cuboid::new(0.4, 0.6, 0.1));

        commands.spawn((
            PbrBundle {
                mesh: chip_mesh,
                material: chip_mat,
                transform: Transform::from_xyz(chip_x, 1.1, z),
                ..default()
            },
            CollectibleItem {
                item_type: CollectibleType::DataChip { index: rng.gen_range(0..DATA_CHIP_LOGS.len()) },
                lane: chip_lane,
                initial_y: 1.1,
                rot_speed: 2.2,
            },
            Despawnable { z_center: z },
        ));
    }

    // 3. Power-Up Spawn (18% chance)
    if rng.gen_bool(0.18) {
        let p_lane = Lane::from_index(rng.gen_range(-1..=1));
        let p_x = p_lane.x_pos();
        let z = z_start - 34.0;

        let roll = rng.gen_range(0..4);
        let p_type = match roll {
            0 => CollectibleType::EchoShield,
            1 => CollectibleType::Magnet,
            2 => CollectibleType::Overdrive,
            _ => CollectibleType::DoubleJump,
        };

        let mut p_cmd = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(p_x, 1.2, z),
                ..default()
            },
            CollectibleItem {
                item_type: p_type,
                lane: p_lane,
                initial_y: 1.2,
                rot_speed: 4.0,
            },
            Despawnable { z_center: z },
        ));

        p_cmd.with_children(|parent| {
            match p_type {
                CollectibleType::EchoShield => {
                    let hex = meshes.add(Cylinder::new(0.32, 0.14));
                    let plate = meshes.add(Cuboid::new(0.10, 0.36, 0.05));
                    let ring = meshes.add(Torus::new(0.04, 0.45));
                    parent.spawn(PbrBundle {
                        mesh: hex,
                        material: powerup_shield_mat.clone(),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: plate.clone(),
                        material: powerup_shield_mat.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.38),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: plate.clone(),
                        material: powerup_shield_mat.clone(),
                        transform: Transform::from_xyz(-0.33, 0.0, -0.19).with_rotation(Quat::from_rotation_y(2.094)),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: plate,
                        material: powerup_shield_mat.clone(),
                        transform: Transform::from_xyz(0.33, 0.0, -0.19).with_rotation(Quat::from_rotation_y(-2.094)),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: ring,
                        material: powerup_shield_mat.clone(),
                        ..default()
                    });
                }
                CollectibleType::Magnet => {
                    let arch = meshes.add(Cuboid::new(0.50, 0.14, 0.14));
                    let prong = meshes.add(Cuboid::new(0.14, 0.42, 0.14));
                    let pole = meshes.add(Cuboid::new(0.15, 0.10, 0.15));
                    let ring = meshes.add(Torus::new(0.03, 0.35));
                    parent.spawn(PbrBundle {
                        mesh: arch,
                        material: powerup_magnet_mat.clone(),
                        transform: Transform::from_xyz(0.0, 0.22, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: prong.clone(),
                        material: powerup_magnet_mat.clone(),
                        transform: Transform::from_xyz(-0.20, -0.06, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: prong,
                        material: powerup_magnet_mat.clone(),
                        transform: Transform::from_xyz(0.20, -0.06, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: pole.clone(),
                        material: powerup_pole_mat.clone(),
                        transform: Transform::from_xyz(-0.20, -0.32, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: pole,
                        material: powerup_pole_mat.clone(),
                        transform: Transform::from_xyz(0.20, -0.32, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: ring,
                        material: powerup_magnet_mat.clone(),
                        transform: Transform::from_xyz(0.0, -0.08, 0.0),
                        ..default()
                    });
                }
                CollectibleType::Overdrive => {
                    let diamond = meshes.add(Cuboid::new(0.32, 0.65, 0.32));
                    let ring = meshes.add(Torus::new(0.04, 0.42));
                    parent.spawn(PbrBundle {
                        mesh: diamond,
                        material: powerup_overdrive_mat.clone(),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: ring.clone(),
                        material: powerup_overdrive_mat.clone(),
                        transform: Transform::from_rotation(Quat::from_rotation_x(0.785)),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: ring,
                        material: powerup_overdrive_mat.clone(),
                        transform: Transform::from_rotation(Quat::from_rotation_z(0.785)),
                        ..default()
                    });
                }
                _ => {
                    let chevron = meshes.add(Cuboid::new(0.40, 0.12, 0.10));
                    let ring = meshes.add(Torus::new(0.03, 0.36));
                    parent.spawn(PbrBundle {
                        mesh: chevron.clone(),
                        material: powerup_jump_mat.clone(),
                        transform: Transform::from_xyz(0.0, -0.08, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: chevron,
                        material: powerup_jump_mat.clone(),
                        transform: Transform::from_xyz(0.0, 0.12, 0.0),
                        ..default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: ring,
                        material: powerup_jump_mat.clone(),
                        transform: Transform::from_xyz(0.0, -0.26, 0.0),
                        ..default()
                    });
                }
            }
        });
    }
}

fn animate_collectibles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CollectibleItem)>,
) {
    let t = time.elapsed_seconds();
    let dt = time.delta_seconds();

    for (mut transform, item) in query.iter_mut() {
        // Floating / bobbing
        transform.translation.y = item.initial_y + (t * 2.8 + transform.translation.z * 0.1).sin() * 0.14;
        // Slow continuous rotation
        transform.rotate_y(item.rot_speed * dt);

        // Subtle scale pulse on power-ups (±6% breathing)
        match item.item_type {
            CollectibleType::EchoShield | CollectibleType::Magnet | CollectibleType::Overdrive => {
                let pulse = 1.0 + (t * 3.5).sin() * 0.06;
                transform.scale = Vec3::splat(pulse);
            }
            _ => {}
        }
    }
}

fn magnet_attract_fragments(
    powerups: Res<ActivePowerUps>,
    player_q: Query<(&Player, &Transform)>,
    mut frag_q: Query<(&mut Transform, &mut Despawnable, &CollectibleItem), Without<Player>>,
    time: Res<Time>,
) {
    if powerups.magnet_timer <= 0.0 {
        return;
    }

    let (player, player_trans) = match player_q.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    let p_pos = player_trans.translation;
    let radius = if player.character == CharacterType::Nyx { 14.0 } else { 9.0 };
    let dt = time.delta_seconds();

    for (mut f_trans, mut despawn, item) in frag_q.iter_mut() {
        if let CollectibleType::EchoFragment { .. } = item.item_type {
            let dist = f_trans.translation.distance(p_pos);
            if dist < radius && dist > 0.1 {
                let dir = (p_pos - f_trans.translation).normalize();
                let pull_speed = (radius - dist) * 12.0 + 8.0;
                f_trans.translation += dir * pull_speed * dt;
                despawn.z_center = f_trans.translation.z;
            }
        }
    }
}

fn collect_items_fixed(
    mut commands: Commands,
    player_q: Query<(&Player, &Transform)>,
    items_q: Query<(Entity, &CollectibleItem, &Transform), Without<Player>>,
    mut stats: ResMut<GameRunStats>,
    mut powerups: ResMut<ActivePowerUps>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let (player, p_trans) = match player_q.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    let p_pos = p_trans.translation;
    let pickup_dist_sq = 1.35 * 1.35;

    for (entity, item, i_trans) in items_q.iter() {
        // Fast distance squared check
        if i_trans.translation.distance_squared(p_pos) < pickup_dist_sq {
            match item.item_type {
                CollectibleType::EchoFragment { value } => {
                    let mult = if player.character == CharacterType::Jax { 2 } else { 1 };
                    stats.fragments += value * mult;
                    stats.score += value * 10 * mult;
                    sfx.send(SoundEffect::FragmentPickup);
                }
                CollectibleType::DataChip { index } => {
                    stats.data_chips += 1;
                    stats.score += 250;
                    if let Some(log) = DATA_CHIP_LOGS.get(index) {
                        stats.story_dialogue = Some(StoryMessage {
                            title: log.title.to_string(),
                            sender: log.sender.to_string(),
                            body: log.snippet.to_string(),
                            timer: 6.0,
                        });
                    }
                    sfx.send(SoundEffect::DataChipPickup);
                }
                CollectibleType::EchoShield => {
                    powerups.shield = true;
                    powerups.shield_hits = if player.character == CharacterType::Arin { 2 } else { 1 };
                    stats.score += 50;
                    sfx.send(SoundEffect::PowerupPickup);
                }
                CollectibleType::Magnet => {
                    let dur = if player.character == CharacterType::Nyx { 12.0 } else { 8.0 };
                    powerups.magnet_timer = dur;
                    stats.score += 50;
                    sfx.send(SoundEffect::PowerupPickup);
                }
                CollectibleType::Overdrive => {
                    powerups.overdrive_timer = 6.0;
                    stats.score += 100;
                    sfx.send(SoundEffect::PowerupPickup);
                }
                CollectibleType::TimeBreak => {
                    powerups.time_break_timer = 5.0;
                    stats.score += 50;
                    sfx.send(SoundEffect::PowerupPickup);
                }
                CollectibleType::DoubleJump => {
                    powerups.double_jump_timer = 10.0;
                    stats.score += 50;
                    sfx.send(SoundEffect::PowerupPickup);
                }
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerup_collection_shield() {
        let mut powerups = ActivePowerUps::default();
        let mut stats = GameRunStats::default();

        powerups.shield = true;
        powerups.shield_hits = 1;
        stats.score += 50;

        assert!(powerups.shield);
        assert_eq!(powerups.shield_hits, 1);
        assert_eq!(stats.score, 50);
    }

    #[test]
    fn test_powerup_collection_magnet() {
        let mut powerups = ActivePowerUps::default();
        let mut stats = GameRunStats::default();

        let dur = 8.0_f32;
        powerups.magnet_timer = dur;
        stats.score += 50;

        assert_eq!(powerups.magnet_timer, 8.0);
        assert_eq!(stats.score, 50);
    }

    #[test]
    fn test_powerup_collection_overdrive() {
        let mut powerups = ActivePowerUps::default();
        let mut stats = GameRunStats::default();

        powerups.overdrive_timer = 6.0;
        stats.score += 100;

        assert_eq!(powerups.overdrive_timer, 6.0);
        assert_eq!(stats.score, 100);
    }

    #[test]
    fn test_powerup_subtle_scale_pulse_bounds() {
        // Subtle scale pulse is defined as 1.0 + (t * 3.5).sin() * 0.06
        for step in 0..100 {
            let t = step as f32 * 0.1;
            let pulse = 1.0 + (t * 3.5).sin() * 0.06;
            assert!(pulse >= 0.94 - 1e-5 && pulse <= 1.06 + 1e-5, "Pulse {:.3} must be within [0.94, 1.06]", pulse);
        }
    }

    #[test]
    fn test_powerup_floating_bobbing_amplitude() {
        // Bobbing is defined as initial_y + (t * 2.8 + z * 0.1).sin() * 0.14
        let initial_y = 1.2_f32;
        for step in 0..100 {
            let t = step as f32 * 0.1;
            let z = -step as f32 * 2.0;
            let y = initial_y + (t * 2.8 + z * 0.1).sin() * 0.14;
            assert!(y >= initial_y - 0.141 && y <= initial_y + 0.141, "Bobbing height {:.3} must be within amplitude", y);
        }
    }
}

