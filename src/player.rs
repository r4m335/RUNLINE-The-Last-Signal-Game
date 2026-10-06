use bevy::prelude::*;
use crate::types::*;
use crate::director::RunDirector;

#[derive(Component)]
pub struct ShieldVisual;

#[derive(Component)]
pub struct MagnetVisual;

#[derive(Component)]
pub struct OverdriveVisual;

#[derive(Component)]
pub struct SlideSparksVisual;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_run_reset_player)
            .add_systems(
                Update,
                (
                    player_input,
                    player_visual_smoothing,
                    player_powerup_visuals,
                    stumble_recovery,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                player_physics_fixed.run_if(in_state(AppState::InGame)),
            );
    }
}

fn handle_run_reset_player(
    mut events: EventReader<RunResetEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_q: Query<(&mut Player, &mut Transform)>,
    mut history: ResMut<PlayerMovementHistory>,
    stats: Res<GameRunStats>,
) {
    for _ in events.read() {
        *history = PlayerMovementHistory::default();

        if let Ok((mut player, mut transform)) = player_q.get_single_mut() {
            transform.translation = Vec3::new(0.0, 0.65, 0.0);
            transform.rotation = Quat::IDENTITY;
            transform.scale = Vec3::ONE;

            player.lane = Lane::Center;
            player.target_x = 0.0;
            player.y_velocity = 0.0;
            player.is_grounded = true;
            player.is_sliding = false;
            player.slide_timer = 0.0;
            player.character = stats.selected_character;
            player.has_double_jumped = false;
            player.invulnerable_timer = 0.0;
            player.dash_cooldown = 0.0;
        } else {
            spawn_player_entity(&mut commands, &mut meshes, &mut materials, stats.selected_character);
        }
    }
}

#[allow(dead_code)]
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stats: Res<GameRunStats>,
) {
    spawn_player_entity(&mut commands, &mut meshes, &mut materials, stats.selected_character);
}

pub fn spawn_player_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    selected_character: CharacterType,
) {
    let jacket_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.12, 0.16),
        perceptual_roughness: 0.40,
        metallic: 0.55,
        ..default()
    });

    let armor_trim_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.18, 0.20, 0.24),
        metallic: 0.88,
        perceptual_roughness: 0.25,
        ..default()
    });

    let orange_accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.42, 0.05),
        emissive: LinearRgba::new(1.2, 0.4, 0.0, 1.0),
        perceptual_roughness: 0.35,
        ..default()
    });

    let cyan_echo_core_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.85, 1.0),
        emissive: LinearRgba::new(0.6, 3.8, 5.0, 1.0),
        ..default()
    });

    let visor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.35, 0.0),
        emissive: LinearRgba::new(4.5, 1.6, 0.1, 1.0),
        ..default()
    });

    let boots_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.13, 0.15),
        metallic: 0.90,
        perceptual_roughness: 0.28,
        ..default()
    });

    let shield_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.8, 1.0, 0.35),
        emissive: LinearRgba::new(0.2, 0.8, 1.2, 1.0),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let aura_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.2, 1.0, 0.3),
        emissive: LinearRgba::new(1.0, 0.2, 1.5, 1.0),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Spawn Root Courier (athletic jacket torso)
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.58, 0.52, 0.36)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.65, 0.0),
                ..default()
            },
            Player {
                lane: Lane::Center,
                target_x: 0.0,
                y_velocity: 0.0,
                is_grounded: true,
                is_sliding: false,
                slide_timer: 0.0,
                character: selected_character,
                has_double_jumped: false,
                invulnerable_timer: 0.0,
                dash_cooldown: 0.0,
            },
        ))
        .with_children(|parent| {
            // 1. Tactical Chest Rig & Center Zipper Seam
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.26, 0.44, 0.08)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.02, -0.16),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.05, 0.46, 0.04)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.02, -0.20),
                ..default()
            });

            // 2. Courier Belt & Utility Hip Pouches
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.52, 0.14, 0.34)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.0, -0.24, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.14, 0.22)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(-0.28, -0.24, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.14, 0.22)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.28, -0.24, 0.0),
                ..default()
            });

            // 3. Head, Faceted Courier Helmet & Cyber-Visor
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.34, 0.34, 0.36)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.44, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.32, 0.13, 0.14)),
                material: visor_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.46, -0.16),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.04, 0.14, 0.08)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(-0.18, 0.48, -0.04),
                ..default()
            });

            // 4. Padded Shoulders & Athletic Arms
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.18, 0.16, 0.24)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(-0.35, 0.18, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.18, 0.16, 0.24)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.35, 0.18, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.24, 0.14)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(-0.35, 0.0, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.24, 0.14)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.35, 0.0, 0.0),
                ..default()
            });
            // Cybernetic forearms
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.24, 0.14)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(-0.35, -0.18, -0.04),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.24, 0.14)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.35, -0.18, -0.04),
                ..default()
            });
            // Left gauntlet cyan holographic wrist display
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.08, 0.04, 0.10)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(-0.35, -0.18, -0.11),
                ..default()
            });

            // 5. Tactical Thighs & Articulated Kinetic Thruster Boots
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.17, 0.30, 0.20)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(-0.16, -0.36, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.17, 0.30, 0.20)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.16, -0.36, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.20, 0.28, 0.34)),
                material: boots_mat.clone(),
                transform: Transform::from_xyz(-0.16, -0.56, 0.03),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.20, 0.28, 0.34)),
                material: boots_mat,
                transform: Transform::from_xyz(0.16, -0.56, 0.03),
                ..default()
            });
            // Titanium toe caps
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.18, 0.10, 0.10)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(-0.16, -0.65, -0.12),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.18, 0.10, 0.10)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.16, -0.65, -0.12),
                ..default()
            });
            // Kinetic sole micro-thruster nozzles
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.10, 0.05, 0.12)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(-0.16, -0.68, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.10, 0.05, 0.12)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(0.16, -0.68, 0.0),
                ..default()
            });

            // 6. Courier Spine Rig & Pulsing Cyan ECHO Core
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.16, 0.48, 0.10)),
                material: armor_trim_mat,
                transform: Transform::from_xyz(0.0, 0.04, 0.19),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.20)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.12, 0.26),
                ..default()
            });
            // Courier hard-drive data pod container
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.24, 0.18, 0.12)),
                material: jacket_mat,
                transform: Transform::from_xyz(0.0, -0.14, 0.22),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.25, 0.04, 0.04)),
                material: orange_accent_mat,
                transform: Transform::from_xyz(0.0, -0.14, 0.28),
                ..default()
            });

            // 7. Kinetic Powerup & Movement Visual Attachments
            // Shield Bubble
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(1.3)),
                    material: shield_mat,
                    transform: Transform::from_scale(Vec3::ZERO),
                    ..default()
                },
                ShieldVisual,
            ));

            // Magnet Aura
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Torus::new(0.8, 1.4)),
                    material: aura_mat,
                    transform: Transform::from_scale(Vec3::ZERO),
                    ..default()
                },
                MagnetVisual,
            ));

            // Overdrive Visual Trail
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 0.1, 1.8)),
                    material: cyan_echo_core_mat,
                    transform: Transform::from_xyz(0.0, -0.7, 1.0).with_scale(Vec3::ZERO),
                    ..default()
                },
                OverdriveVisual,
            ));

            // Rail Slide Sparks (Kinetic amber sparks during slide)
            let sparks_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.65, 0.1),
                emissive: LinearRgba::new(4.5, 2.0, 0.2, 1.0),
                ..default()
            });
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.65, 0.08, 0.9)),
                    material: sparks_mat,
                    transform: Transform::from_xyz(0.0, -0.68, 0.35).with_scale(Vec3::ZERO),
                    ..default()
                },
                SlideSparksVisual,
            ));
        });
}

// -------------------------------------------------------------
// INPUT SYSTEM (Runs in Update for immediate responsiveness)
// -------------------------------------------------------------
fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    mut powerups: ResMut<ActivePowerUps>,
    mut history: ResMut<PlayerMovementHistory>,
    time: Res<Time>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let mut player = match query.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    let now = time.elapsed_seconds();

    // Lane switches
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        let old = player.lane;
        player.lane = player.lane.move_left();
        if old != player.lane {
            player.target_x = player.lane.x_pos();
            history.recent_switches.push((now, player.lane));
            history.last_switch_time = now;
            history.recent_switches.retain(|(t, _)| (now - t) <= 2.0);
            history.switch_count_last_second = history
                .recent_switches
                .iter()
                .filter(|(t, _)| (now - t) <= 1.0)
                .count() as u32;
            sfx.send(SoundEffect::LaneSwitch);
        }
    } else if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        let old = player.lane;
        player.lane = player.lane.move_right();
        if old != player.lane {
            player.target_x = player.lane.x_pos();
            history.recent_switches.push((now, player.lane));
            history.last_switch_time = now;
            history.recent_switches.retain(|(t, _)| (now - t) <= 2.0);
            history.switch_count_last_second = history
                .recent_switches
                .iter()
                .filter(|(t, _)| (now - t) <= 1.0)
                .count() as u32;
            sfx.send(SoundEffect::LaneSwitch);
        }
    }

    // Jump (can cancel an active slide for fast fluid parkour transitions)
    if keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::Space)
    {
        if player.is_grounded || player.is_sliding {
            player.y_velocity = 13.5;
            player.is_grounded = false;
            player.is_sliding = false;
            player.slide_timer = 0.0;
            sfx.send(SoundEffect::Jump);
        } else if powerups.double_jump_timer > 0.0 && !player.has_double_jumped {
            player.y_velocity = 12.0;
            player.has_double_jumped = true;
            sfx.send(SoundEffect::Jump);
        }
    }

    // Slide / Dive (snappy 0.58s duration, dive downwards if airborne)
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        if !player.is_grounded {
            player.y_velocity = -24.0; // Rapid aerial dive
        }
        player.is_sliding = true;
        player.slide_timer = 0.58;
        sfx.send(SoundEffect::Slide);
    }

    // Special Courier Dash (Shift)
    if keyboard.just_pressed(KeyCode::ShiftLeft) && player.dash_cooldown <= 0.0 {
        match player.character {
            CharacterType::Mira => {
                player.dash_cooldown = 4.0;
                player.invulnerable_timer = 1.2;
                powerups.overdrive_timer = 1.5;
                sfx.send(SoundEffect::Dash);
            }
            CharacterType::Jax => {
                player.dash_cooldown = 5.0;
                powerups.overdrive_timer = 2.0;
                sfx.send(SoundEffect::Dash);
            }
            _ => {
                player.dash_cooldown = 8.0;
                powerups.overdrive_timer = 1.0;
                sfx.send(SoundEffect::Dash);
            }
        }
    }
}

// -------------------------------------------------------------
// FIXED TIMESTEP SIMULATION (Runs in FixedUpdate)
// -------------------------------------------------------------
fn player_physics_fixed(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut stats: ResMut<GameRunStats>,
    director: Res<RunDirector>,
    powerups: Res<ActivePowerUps>,
) {
    let (mut player, mut transform) = match query.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let dt = time.delta_seconds();

    // Cooldown timers
    if player.dash_cooldown > 0.0 {
        player.dash_cooldown = (player.dash_cooldown - dt).max(0.0);
    }
    if player.invulnerable_timer > 0.0 {
        player.invulnerable_timer = (player.invulnerable_timer - dt).max(0.0);
    }

    // Vertical physics simulation
    let ground_y = 0.65;
    if !player.is_grounded {
        player.y_velocity -= 34.0 * dt;
        transform.translation.y += player.y_velocity * dt;

        if transform.translation.y <= ground_y {
            transform.translation.y = ground_y;
            player.y_velocity = 0.0;
            player.is_grounded = true;
            player.has_double_jumped = false;
        }
    }

    // Slide timer
    if player.is_sliding {
        player.slide_timer -= dt;
        if player.slide_timer <= 0.0 {
            player.is_sliding = false;
            transform.scale = Vec3::ONE;
        } else {
            transform.scale = Vec3::new(1.1, 0.45, 1.2);
            if player.is_grounded {
                transform.translation.y = 0.35;
            }
        }
    } else {
        transform.scale = Vec3::ONE;
        if player.is_grounded {
            transform.translation.y = ground_y;
        }
    }

    // Speed calculation from Director
    let mut speed_mult = director.profile.speed_multiplier;
    if powerups.overdrive_timer > 0.0 {
        speed_mult *= 1.45;
    }
    if powerups.time_break_timer > 0.0 {
        speed_mult *= 0.65;
    }
    if player.character == CharacterType::Jax {
        speed_mult *= 1.15;
    }

    let current_speed = stats.base_speed * speed_mult;
    stats.speed = current_speed;

    // Advance forward in simulation
    let advance = current_speed * dt;
    transform.translation.z -= advance;
    stats.distance += advance;
    stats.score_accum += advance * stats.multiplier * 2.0;
    stats.score = stats.score_accum as u32;

    if stats.score > stats.high_score {
        stats.high_score = stats.score;
    }
    if stats.distance > stats.best_distance {
        stats.best_distance = stats.distance;
    }
}

// -------------------------------------------------------------
// VISUAL SMOOTHING (Runs in Update for high-framerate rendering)
// -------------------------------------------------------------
fn player_visual_smoothing(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
    mut history: ResMut<PlayerMovementHistory>,
) {
    let (player, mut transform) = match query.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let dt = time.delta_seconds();

    // Smooth horizontal lane transition
    let dx = player.target_x - transform.translation.x;
    history.current_lateral_velocity = dx * 20.0;
    transform.translation.x += dx * (20.0 * dt).min(1.0);

    // Dynamic banking roll angle during lane change
    let target_tilt = (-dx * 0.20).clamp(-0.28, 0.28);
    let cur_rot = transform.rotation;
    let target_rot = Quat::from_rotation_z(target_tilt);
    transform.rotation = cur_rot.slerp(target_rot, (24.0 * dt).min(1.0));
}

fn player_powerup_visuals(
    mut powerups: ResMut<ActivePowerUps>,
    time: Res<Time>,
    player_q: Query<&Player, Without<SlideSparksVisual>>,
    mut shield_q: Query<&mut Transform, (With<ShieldVisual>, Without<MagnetVisual>, Without<OverdriveVisual>, Without<SlideSparksVisual>)>,
    mut magnet_q: Query<&mut Transform, (With<MagnetVisual>, Without<ShieldVisual>, Without<OverdriveVisual>, Without<SlideSparksVisual>)>,
    mut boost_q: Query<&mut Transform, (With<OverdriveVisual>, Without<ShieldVisual>, Without<MagnetVisual>, Without<SlideSparksVisual>)>,
    mut sparks_q: Query<&mut Transform, (With<SlideSparksVisual>, Without<ShieldVisual>, Without<MagnetVisual>, Without<OverdriveVisual>)>,
) {
    let dt = time.delta_seconds();

    // Rail Slide Sparks
    if let Ok(player) = player_q.get_single() {
        if let Ok(mut st) = sparks_q.get_single_mut() {
            let target_scale = if player.is_sliding {
                let jitter = (time.elapsed_seconds() * 45.0).sin() * 0.25 + 1.0;
                Vec3::new(jitter, 1.0, 1.3)
            } else {
                Vec3::ZERO
            };
            st.scale = st.scale.lerp(target_scale, (25.0 * dt).min(1.0));
        }
    }

    // Shield
    if let Ok(mut st) = shield_q.get_single_mut() {
        let target_scale = if powerups.shield { Vec3::splat(1.1) } else { Vec3::ZERO };
        st.scale = st.scale.lerp(target_scale, 10.0 * dt);
        st.rotate_y(1.5 * dt);
    }

    // Magnet
    if powerups.magnet_timer > 0.0 {
        powerups.magnet_timer = (powerups.magnet_timer - dt).max(0.0);
    }
    if let Ok(mut mt) = magnet_q.get_single_mut() {
        let target_scale = if powerups.magnet_timer > 0.0 { Vec3::splat(1.0) } else { Vec3::ZERO };
        mt.scale = mt.scale.lerp(target_scale, 8.0 * dt);
        mt.rotate_z(3.0 * dt);
    }

    // Overdrive
    if powerups.overdrive_timer > 0.0 {
        powerups.overdrive_timer = (powerups.overdrive_timer - dt).max(0.0);
    }
    if let Ok(mut ot) = boost_q.get_single_mut() {
        let target_scale = if powerups.overdrive_timer > 0.0 { Vec3::new(1.2, 0.2, 2.5) } else { Vec3::ZERO };
        ot.scale = ot.scale.lerp(target_scale, 12.0 * dt);
    }

    // Time Break & Double Jump timers
    if powerups.time_break_timer > 0.0 {
        powerups.time_break_timer = (powerups.time_break_timer - dt).max(0.0);
    }
    if powerups.double_jump_timer > 0.0 {
        powerups.double_jump_timer = (powerups.double_jump_timer - dt).max(0.0);
    }
}

fn stumble_recovery(time: Res<Time>, mut stats: ResMut<GameRunStats>) {
    if stats.stumble_intensity > 0.0 {
        stats.stumble_intensity = (stats.stumble_intensity - time.delta_seconds() * 2.5).max(0.0);
    }
}
