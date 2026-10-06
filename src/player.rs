use bevy::prelude::*;
use crate::types::*;
use crate::director::RunDirector;

#[derive(Component)]
pub struct ShieldVisual;

#[derive(Component)]
pub struct MagnetVisual;

#[derive(Component)]
pub struct OverdriveVisual;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
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

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stats: Res<GameRunStats>,
) {
    let jacket_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.14, 0.18),
        perceptual_roughness: 0.3,
        metallic: 0.8,
        ..default()
    });

    let cyan_echo_core_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.9, 1.0),
        emissive: LinearRgba::new(0.0, 2.0, 2.5, 1.0),
        ..default()
    });

    let visor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.3, 0.0),
        emissive: LinearRgba::new(2.5, 0.8, 0.0, 1.0),
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

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.7, 1.1, 0.5)),
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
                character: stats.selected_character,
                has_double_jumped: false,
                invulnerable_timer: 0.0,
                dash_cooldown: 0.0,
            },
        ))
        .with_children(|parent| {
            // ECHO Core on Kai's back
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.22)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.2, 0.3),
                ..default()
            });

            // Head & Visor
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.45, 0.45, 0.45)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.8, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.38, 0.15, 0.12)),
                material: visor_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.82, -0.22),
                ..default()
            });

            // Thruster Boots
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.22, 0.35, 0.35)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(-0.25, -0.65, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.22, 0.35, 0.35)),
                material: jacket_mat,
                transform: Transform::from_xyz(0.25, -0.65, 0.0),
                ..default()
            });

            // Thruster Glow
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.09)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(-0.25, -0.85, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.09)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(0.25, -0.85, 0.0),
                ..default()
            });

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

    // Jump
    if keyboard.just_pressed(KeyCode::KeyW)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::Space)
    {
        if player.is_grounded {
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

    // Slide / Dive
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        if !player.is_grounded {
            player.y_velocity = -22.0; // Rapid aerial dive
        }
        player.is_sliding = true;
        player.slide_timer = 0.75;
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
    stats.score += (advance * stats.multiplier * 2.0) as u32;

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
    history.current_lateral_velocity = dx * 18.0;
    transform.translation.x += dx * (18.0 * dt).min(1.0);

    // Dynamic banking roll angle during lane change
    let target_tilt = (-dx * 0.15).clamp(-0.25, 0.25);
    let cur_rot = transform.rotation;
    let target_rot = Quat::from_rotation_z(target_tilt);
    transform.rotation = cur_rot.slerp(target_rot, 15.0 * dt);
}

fn player_powerup_visuals(
    mut powerups: ResMut<ActivePowerUps>,
    time: Res<Time>,
    mut shield_q: Query<&mut Transform, (With<ShieldVisual>, Without<MagnetVisual>, Without<OverdriveVisual>)>,
    mut magnet_q: Query<&mut Transform, (With<MagnetVisual>, Without<ShieldVisual>, Without<OverdriveVisual>)>,
    mut boost_q: Query<&mut Transform, (With<OverdriveVisual>, Without<ShieldVisual>, Without<MagnetVisual>)>,
) {
    let dt = time.delta_seconds();

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
