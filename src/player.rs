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

#[derive(Component)]
pub struct PlayerLeftArm;

#[derive(Component)]
pub struct PlayerRightArm;

#[derive(Component)]
pub struct PlayerLeftLeg;

#[derive(Component)]
pub struct PlayerRightLeg;

#[derive(Component)]
pub struct PlayerBackpackStrap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_run_reset_player)
            .add_systems(
                Update,
                (
                    player_input,
                    player_visual_smoothing,
                    player_locomotion_animation,
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
        base_color: Color::srgb(0.08, 0.08, 0.09),
        perceptual_roughness: 0.70,
        metallic: 0.15,
        ..default()
    });

    let armor_trim_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.30, 0.34),
        metallic: 0.88,
        perceptual_roughness: 0.25,
        ..default()
    });

    let orange_accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.45, 0.05),
        emissive: LinearRgba::new(5.0, 2.0, 0.1, 1.0),
        perceptual_roughness: 0.25,
        ..default()
    });

    let cyan_echo_core_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.90, 1.0),
        emissive: LinearRgba::new(0.6, 4.5, 6.0, 1.0),
        ..default()
    });

    let visor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.95, 1.0),
        emissive: LinearRgba::new(0.4, 4.8, 6.5, 1.0),
        ..default()
    });

    let hair_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.05, 0.07),
        perceptual_roughness: 0.90,
        metallic: 0.10,
        ..default()
    });

    let shirt_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.88),
        perceptual_roughness: 0.80,
        metallic: 0.05,
        ..default()
    });

    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.82, 0.68, 0.58),
        perceptual_roughness: 0.65,
        metallic: 0.0,
        ..default()
    });

    let boots_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.11, 0.13),
        metallic: 0.85,
        perceptual_roughness: 0.30,
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

    // Spawn Root Courier (athletic V-shaped jacket torso)
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.56, 0.48, 0.36)),
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
            // 1. Undershirt Hem (peeking below jacket waistband)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.48, 0.10, 0.32)),
                material: shirt_mat.clone(),
                transform: Transform::from_xyz(0.0, -0.26, 0.0),
                ..default()
            });

            // 2. High Jacket Collar with Signal Orange Trim
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.34, 0.12, 0.30)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.26, 0.0),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.32, 0.04, 0.04)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.30, -0.14),
                ..default()
            });

            // 3. Tactical Chest Rig & Center Zipper Seam
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.24, 0.40, 0.08)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.02, -0.16),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.05, 0.42, 0.04)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.02, -0.19),
                ..default()
            });

            // 4. Head, Spiked Cyberpunk Hair & Wraparound Cyan Visor
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.28, 0.28, 0.28)),
                material: skin_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.44, 0.0),
                ..default()
            });
            // Hair crown & back spikes
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.34, 0.18, 0.36)),
                material: hair_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.52, 0.02),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.32, 0.22, 0.16)),
                material: hair_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.46, 0.14),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.30, 0.12, 0.10)),
                material: hair_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.50, -0.13),
                ..default()
            });
            // Neon Cyan Wraparound Visor (Visible from front, side, and 3/4 rear!)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.30, 0.08, 0.06)),
                material: visor_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.44, -0.16),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.04, 0.07, 0.18)),
                material: visor_mat.clone(),
                transform: Transform::from_xyz(-0.15, 0.44, -0.07),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.04, 0.07, 0.18)),
                material: visor_mat.clone(),
                transform: Transform::from_xyz(0.15, 0.44, -0.07),
                ..default()
            });

            // 5. Backpack Module (CRITICAL 3RD-PERSON CAMERA FOCUS)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.36, 0.44, 0.16)),
                material: jacket_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.06, 0.22),
                ..default()
            });
            // Top roll-bar handle
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.24, 0.06, 0.06)),
                material: armor_trim_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.29, 0.22),
                ..default()
            });
            // Inverted Signal Orange Triangle Core (`▽`) facing camera
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.18, 0.05, 0.03)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.14, 0.305),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.05, 0.03)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.09, 0.305),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.06, 0.05, 0.03)),
                material: orange_accent_mat.clone(),
                transform: Transform::from_xyz(0.0, 0.04, 0.305),
                ..default()
            });
            // Dual Vertical Neon Cyan Battery Bars
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.04, 0.28, 0.04)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(-0.17, 0.06, 0.28),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.04, 0.28, 0.04)),
                material: cyan_echo_core_mat.clone(),
                transform: Transform::from_xyz(0.17, 0.06, 0.28),
                ..default()
            });
            // Trailing Kinetic Straps / Ribbons
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.03, 0.24, 0.02)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(-0.10, -0.18, 0.26),
                    ..default()
                },
                PlayerBackpackStrap,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.03, 0.24, 0.02)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.10, -0.18, 0.26),
                    ..default()
                },
                PlayerBackpackStrap,
            ));

            // 6. Articulated Left Arm (Shoulder Pivot with Air Gap)
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(-0.36, 0.18, 0.0),
                    ..default()
                },
                PlayerLeftArm,
            )).with_children(|arm| {
                // Shoulder pauldron
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.14, 0.20)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
                // Upper arm (rolled-up sleeve)
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.13, 0.20, 0.14)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.13, 0.0),
                    ..default()
                });
                // Forearm (bare skin)
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.10, 0.18, 0.11)),
                    material: skin_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.28, 0.0),
                    ..default()
                });
                // Combat glove with cyan knuckle energy pad
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.11, 0.14, 0.12)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.40, 0.0),
                    ..default()
                });
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.08, 0.04, 0.06)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.42, -0.05),
                    ..default()
                });
            });

            // 7. Articulated Right Arm (Shoulder Pivot with Air Gap)
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.36, 0.18, 0.0),
                    ..default()
                },
                PlayerRightArm,
            )).with_children(|arm| {
                // Shoulder pauldron
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.14, 0.20)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
                // Upper arm (rolled-up sleeve)
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.13, 0.20, 0.14)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.13, 0.0),
                    ..default()
                });
                // Forearm (bare skin)
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.10, 0.18, 0.11)),
                    material: skin_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.28, 0.0),
                    ..default()
                });
                // Combat glove with cyan knuckle energy pad
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.11, 0.14, 0.12)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.40, 0.0),
                    ..default()
                });
                arm.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.08, 0.04, 0.06)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.42, -0.05),
                    ..default()
                });
            });

            // 8. Articulated Left Leg (Hip Pivot with Negative Space)
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(-0.18, -0.26, 0.0),
                    ..default()
                },
                PlayerLeftLeg,
            )).with_children(|leg| {
                // Thigh cargo pants
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.26, 0.18)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.12, 0.0),
                    ..default()
                });
                // Orange strap buckle
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.17, 0.04, 0.19)),
                    material: orange_accent_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.08, 0.0),
                    ..default()
                });
                // Cyber knee armor plate
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.15, 0.12, 0.08)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.22, -0.08),
                    ..default()
                });
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.06, 0.04, 0.03)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.22, -0.12),
                    ..default()
                });
                // High-top courier boot
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.18, 0.24, 0.32)),
                    material: boots_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.46, 0.03),
                    ..default()
                });
                // Titanium toe cap
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.08, 0.08)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.54, -0.12),
                    ..default()
                });
                // Neon Cyan Sole (faces camera during sprint stride!)
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.05, 0.30)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.58, 0.03),
                    ..default()
                });
                // Neon Cyan Heel Thruster Bar
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.14, 0.08, 0.05)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.48, 0.18),
                    ..default()
                });
            });

            // 9. Articulated Right Leg (Hip Pivot with Negative Space)
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.18, -0.26, 0.0),
                    ..default()
                },
                PlayerRightLeg,
            )).with_children(|leg| {
                // Thigh cargo pants
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.26, 0.18)),
                    material: jacket_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.12, 0.0),
                    ..default()
                });
                // Orange strap buckle
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.17, 0.04, 0.19)),
                    material: orange_accent_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.08, 0.0),
                    ..default()
                });
                // Cyber knee armor plate
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.15, 0.12, 0.08)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.22, -0.08),
                    ..default()
                });
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.06, 0.04, 0.03)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.22, -0.12),
                    ..default()
                });
                // High-top courier boot
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.18, 0.24, 0.32)),
                    material: boots_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.46, 0.03),
                    ..default()
                });
                // Titanium toe cap
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.08, 0.08)),
                    material: armor_trim_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.54, -0.12),
                    ..default()
                });
                // Neon Cyan Sole (faces camera during sprint stride!)
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.16, 0.05, 0.30)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.58, 0.03),
                    ..default()
                });
                // Neon Cyan Heel Thruster Bar
                leg.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.14, 0.08, 0.05)),
                    material: cyan_echo_core_mat.clone(),
                    transform: Transform::from_xyz(0.0, -0.48, 0.18),
                    ..default()
                });
            });

            // 10. Kinetic Powerup & Movement Visual Attachments
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

            // Rail Slide Sparks
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

    // Dynamic banking roll angle during lane change + sprint forward lean pitch
    let target_tilt = (-dx * 0.20).clamp(-0.28, 0.28);
    let target_pitch = if player.is_grounded && !player.is_sliding { 0.12 } else { 0.0 };
    let cur_rot = transform.rotation;
    let target_rot = Quat::from_rotation_z(target_tilt) * Quat::from_rotation_x(target_pitch);
    transform.rotation = cur_rot.slerp(target_rot, (24.0 * dt).min(1.0));
}

fn player_locomotion_animation(
    time: Res<Time>,
    player_q: Query<(&Player, &Transform)>,
    mut left_arm_q: Query<&mut Transform, (With<PlayerLeftArm>, Without<PlayerRightArm>, Without<PlayerLeftLeg>, Without<PlayerRightLeg>, Without<PlayerBackpackStrap>, Without<Player>)>,
    mut right_arm_q: Query<&mut Transform, (With<PlayerRightArm>, Without<PlayerLeftArm>, Without<PlayerLeftLeg>, Without<PlayerRightLeg>, Without<PlayerBackpackStrap>, Without<Player>)>,
    mut left_leg_q: Query<&mut Transform, (With<PlayerLeftLeg>, Without<PlayerLeftArm>, Without<PlayerRightArm>, Without<PlayerRightLeg>, Without<PlayerBackpackStrap>, Without<Player>)>,
    mut right_leg_q: Query<&mut Transform, (With<PlayerRightLeg>, Without<PlayerLeftArm>, Without<PlayerRightArm>, Without<PlayerLeftLeg>, Without<PlayerBackpackStrap>, Without<Player>)>,
    mut strap_q: Query<&mut Transform, (With<PlayerBackpackStrap>, Without<PlayerLeftArm>, Without<PlayerRightArm>, Without<PlayerLeftLeg>, Without<PlayerRightLeg>, Without<Player>)>,
) {
    let (player, _p_trans) = match player_q.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();

    if player.is_sliding {
        // Slide pose: low streamlined lean, right leg extended forward, left knee tucked, arms balancing
        if let Ok(mut arm) = left_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(0.65) * Quat::from_rotation_z(0.35);
            arm.rotation = arm.rotation.slerp(target_rot, 20.0 * dt);
        }
        if let Ok(mut arm) = right_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-0.85) * Quat::from_rotation_z(-0.25);
            arm.rotation = arm.rotation.slerp(target_rot, 20.0 * dt);
        }
        if let Ok(mut leg) = left_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(0.85);
            leg.rotation = leg.rotation.slerp(target_rot, 20.0 * dt);
        }
        if let Ok(mut leg) = right_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-1.10);
            leg.rotation = leg.rotation.slerp(target_rot, 20.0 * dt);
        }
    } else if !player.is_grounded {
        // Jump pose: athletic tuck, knees bent, arms back/out for balance
        let jump_pitch = (player.y_velocity * 0.05).clamp(-0.4, 0.4);
        if let Ok(mut arm) = left_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(0.55 + jump_pitch) * Quat::from_rotation_z(-0.25);
            arm.rotation = arm.rotation.slerp(target_rot, 15.0 * dt);
        }
        if let Ok(mut arm) = right_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(0.55 + jump_pitch) * Quat::from_rotation_z(0.25);
            arm.rotation = arm.rotation.slerp(target_rot, 15.0 * dt);
        }
        if let Ok(mut leg) = left_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-0.65);
            leg.rotation = leg.rotation.slerp(target_rot, 15.0 * dt);
        }
        if let Ok(mut leg) = right_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-0.45);
            leg.rotation = leg.rotation.slerp(target_rot, 15.0 * dt);
        }
    } else {
        // High-speed run stride cycle (running cadence ~15 rad/s)
        let stride_speed = 15.0;
        let stride_sin = (t * stride_speed).sin();

        // Arm swing (opposing leg stride)
        let arm_amp = 0.55;
        if let Ok(mut arm) = left_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-stride_sin * arm_amp) * Quat::from_rotation_z(-0.08);
            arm.rotation = arm.rotation.slerp(target_rot, 25.0 * dt);
        }
        if let Ok(mut arm) = right_arm_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(stride_sin * arm_amp) * Quat::from_rotation_z(0.08);
            arm.rotation = arm.rotation.slerp(target_rot, 25.0 * dt);
        }

        // Leg stride (kicking back reveals cyan glowing thruster soles to camera)
        let leg_amp = 0.65;
        if let Ok(mut leg) = left_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(stride_sin * leg_amp);
            leg.rotation = leg.rotation.slerp(target_rot, 25.0 * dt);
        }
        if let Ok(mut leg) = right_leg_q.get_single_mut() {
            let target_rot = Quat::from_rotation_x(-stride_sin * leg_amp);
            leg.rotation = leg.rotation.slerp(target_rot, 25.0 * dt);
        }

        // Trailing backpack straps flutter
        for mut strap in strap_q.iter_mut() {
            let flutter = (t * 22.0).sin() * 0.18 + 0.22;
            strap.rotation = Quat::from_rotation_x(flutter);
        }
    }
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
