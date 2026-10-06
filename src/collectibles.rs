use bevy::prelude::*;
use crate::types::*;
use crate::story::DATA_CHIP_LOGS;
use rand::Rng;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
        base_color: Color::srgb(0.1, 0.6, 1.0),
        emissive: LinearRgba::new(0.5, 1.8, 3.5, 1.0),
        ..default()
    });

    let powerup_magnet_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.9),
        emissive: LinearRgba::new(2.8, 0.2, 2.8, 1.0),
        ..default()
    });

    let powerup_overdrive_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0),
        emissive: LinearRgba::new(3.5, 1.2, 0.0, 1.0),
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
        let (p_type, p_mat) = match roll {
            0 => (CollectibleType::EchoShield, powerup_shield_mat),
            1 => (CollectibleType::Magnet, powerup_magnet_mat),
            2 => (CollectibleType::Overdrive, powerup_overdrive_mat),
            _ => (CollectibleType::DoubleJump, powerup_shield_mat),
        };

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Torus::new(0.25, 0.45)),
                material: p_mat,
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
    }
}

fn animate_collectibles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CollectibleItem)>,
) {
    let t = time.elapsed_seconds();
    let dt = time.delta_seconds();

    for (mut transform, item) in query.iter_mut() {
        transform.translation.y = item.initial_y + (t * 3.0 + transform.translation.z * 0.1).sin() * 0.15;
        transform.rotate_y(item.rot_speed * dt);
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
