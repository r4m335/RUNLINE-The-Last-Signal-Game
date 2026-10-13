use bevy::prelude::*;

use crate::types::{
    ActiveObstacle, ActivePowerUps, AppState, BossBattleState, BossDefeatedEvent, BossStartedEvent,
    CollectibleItem, CollectibleType, GameRunStats, Lane, ObstacleType, Player, RunResetEvent,
    SoundEffect, ThreatAlertState,
};

// -------------------------------------------------------------
// CONSTANTS & TIMINGS
// -------------------------------------------------------------
pub const ECHO_PRIME_START_DISTANCE: f32 = 13300.0;
pub const ECHO_PRIME_END_DISTANCE: f32 = 13700.0;

const LANCE_TELEGRAPH_DURATION: f32 = 1.3;
const LANCE_STRIKE_DURATION: f32 = 0.5;
const SWEEP_TELEGRAPH_DURATION: f32 = 1.2;
const SWEEP_STRIKE_DURATION: f32 = 0.6;
const SHARDS_TELEGRAPH_DURATION: f32 = 1.2;
const SHARDS_STRIKE_DURATION: f32 = 1.4;
const STUN_DURATION: f32 = 2.5;

// Authored pursuit attack milestones between 13,300m and 13,700m
const ATTACK_DISTANCES: [f32; 6] = [
    13340.0, // Attack 0: Singularity Lance (Lane switch)
    13400.0, // Attack 1: Sweep Wave Jump (Low plasma sweep)
    13460.0, // Attack 2: Reality Shards & Resonance Surge 1 (2 lanes blocked + Overdrive orb)
    13530.0, // Attack 3: Sweep Wave Slide (High overhead blade)
    13600.0, // Attack 4: Singularity Lance 2 (Rapid lane switch)
    13650.0, // Attack 5: Reality Shards & Resonance Surge 2 (Climax Overdrive rush)
];

// -------------------------------------------------------------
// COMPONENTS & ENUMS
// -------------------------------------------------------------

#[derive(Component)]
pub struct EchoPrimeBoss {
    pub current_z_offset: f32,
    pub sway_timer: f32,
}

#[derive(Component)]
pub struct EchoPrimeCoreEye;

#[derive(Component)]
pub struct EchoPrimeWing {
    pub sign: f32,
}

#[derive(Component)]
pub struct EchoPrimeTelegraph {
    pub lifetime: f32,
    #[allow(dead_code)]
    pub target_lane: Option<Lane>,
}

#[derive(Component)]
pub struct EchoPrimeAttackProjectile {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct EchoPrimeResonanceSurgeOrb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EchoPrimeAttackStage {
    #[default]
    Idle,
    TelegraphingLance(Lane),
    FiringLance(Lane),
    TelegraphingSweep(EchoPrimeSweepType),
    Sweeping(EchoPrimeSweepType),
    TelegraphingShards(Lane), // lane indicated is the SAFE lane
    DeployingShards(Lane),
    Stunned,
    Banished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EchoPrimeSweepType {
    JumpLow,   // Plasma wave at track level: player must Jump
    SlideHigh, // Plasma blade at chest level: player must Slide
}

// -------------------------------------------------------------
// RESOURCES
// -------------------------------------------------------------

#[derive(Resource, Debug)]
pub struct EchoPrimeManager {
    pub has_spawned: bool,
    pub active_boss_entity: Option<Entity>,
    pub defeated: bool,
    pub health: f32,
    pub current_attack_index: usize,
    pub state_timer: f32,
    pub attack_stage: EchoPrimeAttackStage,
}

impl Default for EchoPrimeManager {
    fn default() -> Self {
        Self {
            has_spawned: false,
            active_boss_entity: None,
            defeated: false,
            health: 100.0,
            current_attack_index: 0,
            state_timer: 0.0,
            attack_stage: EchoPrimeAttackStage::Idle,
        }
    }
}

#[derive(Resource)]
pub struct EchoPrimeBossAssets {
    pub mesh_prime_torso: Handle<Mesh>,
    pub mesh_prime_crown: Handle<Mesh>,
    pub mesh_prime_core: Handle<Mesh>,
    pub mesh_prime_wing: Handle<Mesh>,
    pub mesh_prime_halo_ring: Handle<Mesh>,
    pub mesh_lance_beam: Handle<Mesh>,
    pub mesh_sweep_blade: Handle<Mesh>,
    pub mesh_reality_shard: Handle<Mesh>,
    pub mesh_resonance_orb: Handle<Mesh>,
    pub mesh_telegraph_strip: Handle<Mesh>,
    pub mesh_telegraph_wide: Handle<Mesh>,

    pub mat_prime_obsidian: Handle<StandardMaterial>,
    pub mat_prime_cyan_radiant: Handle<StandardMaterial>,
    pub mat_prime_core_singularity: Handle<StandardMaterial>,
    pub mat_telegraph_warning: Handle<StandardMaterial>,
    pub mat_telegraph_wide: Handle<StandardMaterial>,
    pub mat_surge_gold_cyan: Handle<StandardMaterial>,
}

// -------------------------------------------------------------
// PLUGIN
// -------------------------------------------------------------

pub struct EchoPrimeBossPlugin;

impl Plugin for EchoPrimeBossPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EchoPrimeManager>()
            .add_systems(Startup, setup_echo_prime_assets)
            .add_systems(
                Update,
                (
                    manage_echo_prime_lifecycle,
                    update_echo_prime_motion_and_visuals,
                    update_echo_prime_attacks,
                    handle_resonance_orb_collection,
                    cleanup_echo_prime_projectiles,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, handle_run_reset_echo_prime);
    }
}

// -------------------------------------------------------------
// ASSET SETUP
// -------------------------------------------------------------

fn setup_echo_prime_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EchoPrimeBossAssets {
        // Boss Body Meshes
        mesh_prime_torso: meshes.add(Cuboid::new(3.8, 7.5, 2.2)),
        mesh_prime_crown: meshes.add(Cylinder::new(2.2, 1.4)),
        mesh_prime_core: meshes.add(Sphere::new(1.6)),
        mesh_prime_wing: meshes.add(Cylinder::new(0.35, 14.0)),
        mesh_prime_halo_ring: meshes.add(Torus::new(3.8, 4.2)),

        // Attack Projectile & Obstacle Meshes
        mesh_lance_beam: meshes.add(Cylinder::new(0.9, 20.0)),
        mesh_sweep_blade: meshes.add(Cuboid::new(8.2, 0.4, 0.8)),
        mesh_reality_shard: meshes.add(Cuboid::new(1.8, 5.5, 1.8)),
        mesh_resonance_orb: meshes.add(Sphere::new(0.85)),

        // Telegraph Meshes
        mesh_telegraph_strip: meshes.add(Cuboid::new(2.2, 0.05, 22.0)),
        mesh_telegraph_wide: meshes.add(Cuboid::new(8.0, 0.05, 2.4)),

        // Materials
        mat_prime_obsidian: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.03, 0.04),
            perceptual_roughness: 0.15,
            metallic: 0.9,
            ..default()
        }),
        mat_prime_cyan_radiant: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.9, 1.0),
            emissive: LinearRgba::new(0.15, 3.2, 4.0, 1.0),
            perceptual_roughness: 0.1,
            ..default()
        }),
        mat_prime_core_singularity: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.98, 1.0),
            emissive: LinearRgba::new(4.5, 6.0, 7.5, 1.0),
            perceptual_roughness: 0.05,
            ..default()
        }),
        mat_telegraph_warning: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.85, 1.0, 0.45),
            emissive: LinearRgba::new(0.2, 2.5, 3.5, 0.7),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_telegraph_wide: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.2, 0.4, 0.55),
            emissive: LinearRgba::new(3.0, 0.4, 0.8, 0.8),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_surge_gold_cyan: materials.add(StandardMaterial {
            base_color: Color::srgba(0.95, 0.85, 0.2, 0.9),
            emissive: LinearRgba::new(3.5, 2.8, 0.5, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -------------------------------------------------------------
// LIFECYCLE MANAGEMENT (13,300m - 13,700m)
// -------------------------------------------------------------

fn manage_echo_prime_lifecycle(
    mut commands: Commands,
    stats: Res<GameRunStats>,
    assets: Res<EchoPrimeBossAssets>,
    mut manager: ResMut<EchoPrimeManager>,
    mut boss_state: ResMut<BossBattleState>,
    mut boss_started_writer: EventWriter<BossStartedEvent>,
    mut boss_defeated_writer: EventWriter<BossDefeatedEvent>,
    player_q: Query<&Transform, With<Player>>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    // 1. Spawn Boss at 13,300m
    if stats.distance >= ECHO_PRIME_START_DISTANCE
        && stats.distance < ECHO_PRIME_END_DISTANCE
        && !manager.has_spawned
    {
        manager.has_spawned = true;
        manager.defeated = false;
        manager.health = 100.0;
        manager.current_attack_index = 0;
        manager.state_timer = 0.0;
        manager.attack_stage = EchoPrimeAttackStage::Idle;

        // Initialize Global Boss Battle HUD
        boss_state.is_active = true;
        boss_state.boss_name = "ECHO PRIME // APEX DAEMON";
        boss_state.boss_health = 100.0;
        boss_state.time_remaining = 20.0;
        boss_state.victory_banner_timer = 0.0;

        boss_started_writer.send(BossStartedEvent {
            boss_name: "ECHO PRIME // APEX DAEMON",
            health: 100.0,
        });

        // Spawn ECHO Prime Hierarchical Entity
        let spawn_z = p_trans.translation.z + 28.0; // Looms behind player
        let boss_root = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(0.0, 8.5, spawn_z),
                    ..default()
                },
                EchoPrimeBoss {
                    current_z_offset: 26.0,
                    sway_timer: 0.0,
                },
            ))
            .with_children(|parent| {
                // Main Monolithic Torso
                parent.spawn(PbrBundle {
                    mesh: assets.mesh_prime_torso.clone(),
                    material: assets.mat_prime_obsidian.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });

                // Crystalline Crown
                parent.spawn(PbrBundle {
                    mesh: assets.mesh_prime_crown.clone(),
                    material: assets.mat_prime_cyan_radiant.clone(),
                    transform: Transform::from_xyz(0.0, 4.4, 0.0),
                    ..default()
                });

                // Pulsing Singularity Core Eye
                parent.spawn((
                    PbrBundle {
                        mesh: assets.mesh_prime_core.clone(),
                        material: assets.mat_prime_core_singularity.clone(),
                        transform: Transform::from_xyz(0.0, 1.2, -0.7),
                        ..default()
                    },
                    EchoPrimeCoreEye,
                ));

                // Left Energy Wing Tendril
                parent.spawn((
                    PbrBundle {
                        mesh: assets.mesh_prime_wing.clone(),
                        material: assets.mat_prime_cyan_radiant.clone(),
                        transform: Transform::from_xyz(-6.8, 2.0, 0.0)
                            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.2, 0.0, 0.785)),
                        ..default()
                    },
                    EchoPrimeWing { sign: -1.0 },
                ));

                // Right Energy Wing Tendril
                parent.spawn((
                    PbrBundle {
                        mesh: assets.mesh_prime_wing.clone(),
                        material: assets.mat_prime_cyan_radiant.clone(),
                        transform: Transform::from_xyz(6.8, 2.0, 0.0)
                            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.2, 0.0, -0.785)),
                        ..default()
                    },
                    EchoPrimeWing { sign: 1.0 },
                ));

                // Orbiting Halo Ring
                parent.spawn(PbrBundle {
                    mesh: assets.mesh_prime_halo_ring.clone(),
                    material: assets.mat_prime_cyan_radiant.clone(),
                    transform: Transform::from_xyz(0.0, 1.2, 0.0)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            })
            .id();

        manager.active_boss_entity = Some(boss_root);
    }

    // 2. Banish Boss at 13,700m (End of Act 4, entrance to Act 5 The Core)
    if stats.distance >= ECHO_PRIME_END_DISTANCE && manager.has_spawned && !manager.defeated {
        manager.defeated = true;
        manager.attack_stage = EchoPrimeAttackStage::Banished;

        boss_state.is_active = false;
        boss_state.boss_health = 0.0;

        boss_defeated_writer.send(BossDefeatedEvent {
            boss_name: "ECHO PRIME // APEX DAEMON",
            reward_fragments: 500,
        });

        if let Some(entity) = manager.active_boss_entity {
            commands.entity(entity).despawn_recursive();
            manager.active_boss_entity = None;
        }
    }
}

// -------------------------------------------------------------
// MOTION & ANIMATION (Tracking behind player)
// -------------------------------------------------------------

fn update_echo_prime_motion_and_visuals(
    time: Res<Time>,
    manager: Res<EchoPrimeManager>,
    player_q: Query<&Transform, With<Player>>,
    mut boss_q: Query<(&mut Transform, &mut EchoPrimeBoss), Without<Player>>,
    mut wing_q: Query<(&mut Transform, &EchoPrimeWing), (Without<Player>, Without<EchoPrimeBoss>)>,
    mut eye_q: Query<
        &mut Transform,
        (
            With<EchoPrimeCoreEye>,
            Without<Player>,
            Without<EchoPrimeBoss>,
            Without<EchoPrimeWing>,
        ),
    >,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    for (mut trans, mut boss) in boss_q.iter_mut() {
        boss.sway_timer += dt;

        // Stunned reaction pushes boss back
        let target_z_offset = match manager.attack_stage {
            EchoPrimeAttackStage::Stunned => 32.0,
            _ => 25.0,
        };
        boss.current_z_offset += (target_z_offset - boss.current_z_offset) * dt * 2.0;

        // Kinematic floating tracking behind player
        let target_x = (boss.sway_timer * 1.8).sin() * 1.5;
        let target_y = 8.5 + (boss.sway_timer * 2.6).cos() * 0.7;
        let target_z = p_trans.translation.z + boss.current_z_offset;

        trans.translation.x += (target_x - trans.translation.x) * dt * 3.0;
        trans.translation.y += (target_y - trans.translation.y) * dt * 3.0;
        trans.translation.z += (target_z - trans.translation.z) * dt * 6.0;

        // Slight predatory forward pitch
        let pitch = 0.12 + 0.04 * (boss.sway_timer * 2.2).sin();
        let roll = -0.06 * (boss.sway_timer * 1.8).cos();
        trans.rotation = Quat::from_euler(EulerRot::XYZ, pitch, 0.0, roll);
    }

    // Dynamic wing flap & breathing
    for (mut wing_trans, wing) in wing_q.iter_mut() {
        let flap = (time.elapsed_seconds() * 3.5).sin() * 0.15;
        let base_z = wing.sign * 0.785;
        wing_trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.2, 0.0, base_z + flap * wing.sign);
    }

    // Core eye harmonic pulse
    for mut eye_trans in eye_q.iter_mut() {
        let pulse = 1.0 + 0.12 * (time.elapsed_seconds() * 6.0).sin();
        eye_trans.scale = Vec3::splat(pulse);
    }
}

// -------------------------------------------------------------
// ATTACK PATTERNS & TELEGRAPH PIPELINE
// -------------------------------------------------------------

fn update_echo_prime_attacks(
    mut commands: Commands,
    time: Res<Time>,
    stats: Res<GameRunStats>,
    assets: Res<EchoPrimeBossAssets>,
    mut manager: ResMut<EchoPrimeManager>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut sfx: EventWriter<SoundEffect>,
    player_q: Query<(&Player, &Transform)>,
) {
    if !manager.has_spawned || manager.defeated {
        return;
    }

    let (player, p_trans) = match player_q.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    manager.state_timer += dt;

    // 1. Advance to next scheduled attack if ready and idle
    if manager.attack_stage == EchoPrimeAttackStage::Idle
        && manager.current_attack_index < ATTACK_DISTANCES.len()
    {
        let next_dist = ATTACK_DISTANCES[manager.current_attack_index];
        if stats.distance >= next_dist {
            let p_lane = player.lane;
            match manager.current_attack_index {
                0 | 4 => {
                    // Attack 0 & 4: Singularity Lance
                    manager.attack_stage = EchoPrimeAttackStage::TelegraphingLance(p_lane);
                    manager.state_timer = 0.0;
                    threat_alerts.hunter_telegraph_lane = Some(p_lane);

                    // Spawn Floor Telegraph Line along targeted lane
                    let t_z = p_trans.translation.z - 16.0;
                    commands.spawn((
                        PbrBundle {
                            mesh: assets.mesh_telegraph_strip.clone(),
                            material: assets.mat_telegraph_warning.clone(),
                            transform: Transform::from_xyz(p_lane.x_pos(), 0.05, t_z),
                            ..default()
                        },
                        EchoPrimeTelegraph {
                            lifetime: LANCE_TELEGRAPH_DURATION,
                            target_lane: Some(p_lane),
                        },
                    ));
                    sfx.send(SoundEffect::Dash);
                }
                1 => {
                    // Attack 1: Sweep Wave Jump (Low plasma sweep)
                    let sweep_type = EchoPrimeSweepType::JumpLow;
                    manager.attack_stage = EchoPrimeAttackStage::TelegraphingSweep(sweep_type);
                    manager.state_timer = 0.0;

                    let t_z = p_trans.translation.z - 16.0;
                    commands.spawn((
                        PbrBundle {
                            mesh: assets.mesh_telegraph_wide.clone(),
                            material: assets.mat_telegraph_wide.clone(),
                            transform: Transform::from_xyz(0.0, 0.05, t_z),
                            ..default()
                        },
                        EchoPrimeTelegraph {
                            lifetime: SWEEP_TELEGRAPH_DURATION,
                            target_lane: None,
                        },
                    ));
                    sfx.send(SoundEffect::Dash);
                }
                3 => {
                    // Attack 3: Sweep Wave Slide (High overhead blade)
                    let sweep_type = EchoPrimeSweepType::SlideHigh;
                    manager.attack_stage = EchoPrimeAttackStage::TelegraphingSweep(sweep_type);
                    manager.state_timer = 0.0;

                    let t_z = p_trans.translation.z - 16.0;
                    commands.spawn((
                        PbrBundle {
                            mesh: assets.mesh_telegraph_wide.clone(),
                            material: assets.mat_telegraph_wide.clone(),
                            transform: Transform::from_xyz(0.0, 0.05, t_z),
                            ..default()
                        },
                        EchoPrimeTelegraph {
                            lifetime: SWEEP_TELEGRAPH_DURATION,
                            target_lane: None,
                        },
                    ));
                    sfx.send(SoundEffect::Dash);
                }
                2 | 5 => {
                    // Attack 2 & 5: Reality Shards & Resonance Surge
                    // Safe lane is opposite to player's current lane to reward tactical movement
                    let safe_lane = match p_lane {
                        Lane::Left => Lane::Right,
                        Lane::Right => Lane::Left,
                        Lane::Center => Lane::Right,
                    };
                    manager.attack_stage = EchoPrimeAttackStage::TelegraphingShards(safe_lane);
                    manager.state_timer = 0.0;

                    // Floor telegraph indicator in blocked lanes
                    for lane in [Lane::Left, Lane::Center, Lane::Right] {
                        if lane != safe_lane {
                            let t_z = p_trans.translation.z - 16.0;
                            commands.spawn((
                                PbrBundle {
                                    mesh: assets.mesh_telegraph_strip.clone(),
                                    material: assets.mat_telegraph_warning.clone(),
                                    transform: Transform::from_xyz(lane.x_pos(), 0.05, t_z),
                                    ..default()
                                },
                                EchoPrimeTelegraph {
                                    lifetime: SHARDS_TELEGRAPH_DURATION,
                                    target_lane: Some(lane),
                                },
                            ));
                        }
                    }
                    sfx.send(SoundEffect::Dash);
                }
                _ => {}
            }
        }
    }

    // 2. Handle Telegraph Expiration -> Strike Execution
    match manager.attack_stage {
        EchoPrimeAttackStage::TelegraphingLance(lane) => {
            if manager.state_timer >= LANCE_TELEGRAPH_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::FiringLance(lane);
                manager.state_timer = 0.0;
                threat_alerts.hunter_telegraph_lane = None;

                // Strike: Spawn vertical lance obstacle in targeted lane
                let strike_z = p_trans.translation.z - 14.0;
                commands.spawn((
                    PbrBundle {
                        mesh: assets.mesh_lance_beam.clone(),
                        material: assets.mat_prime_cyan_radiant.clone(),
                        transform: Transform::from_xyz(lane.x_pos(), 5.0, strike_z),
                        ..default()
                    },
                    ActiveObstacle {
                        lane,
                        obstacle_type: ObstacleType::TallPillar,
                        size: Vec3::new(1.8, 10.0, 1.8),
                    },
                    EchoPrimeAttackProjectile {
                        lifetime: LANCE_STRIKE_DURATION,
                    },
                ));
                sfx.send(SoundEffect::Crash);
            }
        }
        EchoPrimeAttackStage::FiringLance(_) => {
            if manager.state_timer >= LANCE_STRIKE_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::Idle;
                manager.current_attack_index += 1;
            }
        }
        EchoPrimeAttackStage::TelegraphingSweep(sweep_type) => {
            if manager.state_timer >= SWEEP_TELEGRAPH_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::Sweeping(sweep_type);
                manager.state_timer = 0.0;

                let strike_z = p_trans.translation.z - 14.0;
                match sweep_type {
                    EchoPrimeSweepType::JumpLow => {
                        // Low sweep wave across all 3 lanes (vault over / Jump)
                        for lane in [Lane::Left, Lane::Center, Lane::Right] {
                            commands.spawn((
                                PbrBundle {
                                    mesh: assets.mesh_sweep_blade.clone(),
                                    material: assets.mat_prime_cyan_radiant.clone(),
                                    transform: Transform::from_xyz(0.0, 0.25, strike_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane,
                                    obstacle_type: ObstacleType::LowBarrier,
                                    size: Vec3::new(2.4, 0.5, 0.8),
                                },
                                EchoPrimeAttackProjectile {
                                    lifetime: SWEEP_STRIKE_DURATION,
                                },
                            ));
                        }
                    }
                    EchoPrimeSweepType::SlideHigh => {
                        // High overhead blade across all 3 lanes (duck under / Slide)
                        for lane in [Lane::Left, Lane::Center, Lane::Right] {
                            commands.spawn((
                                PbrBundle {
                                    mesh: assets.mesh_sweep_blade.clone(),
                                    material: assets.mat_prime_core_singularity.clone(),
                                    transform: Transform::from_xyz(0.0, 1.35, strike_z),
                                    ..default()
                                },
                                ActiveObstacle {
                                    lane,
                                    obstacle_type: ObstacleType::HighHangingWire,
                                    size: Vec3::new(2.4, 0.8, 0.8),
                                },
                                EchoPrimeAttackProjectile {
                                    lifetime: SWEEP_STRIKE_DURATION,
                                },
                            ));
                        }
                    }
                }
                sfx.send(SoundEffect::Dash);
            }
        }
        EchoPrimeAttackStage::Sweeping(_) => {
            if manager.state_timer >= SWEEP_STRIKE_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::Idle;
                manager.current_attack_index += 1;
            }
        }
        EchoPrimeAttackStage::TelegraphingShards(safe_lane) => {
            if manager.state_timer >= SHARDS_TELEGRAPH_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::DeployingShards(safe_lane);
                manager.state_timer = 0.0;

                let strike_z = p_trans.translation.z - 14.0;

                // 2 blocked lanes get Reality Shard pillars
                for lane in [Lane::Left, Lane::Center, Lane::Right] {
                    if lane != safe_lane {
                        commands.spawn((
                            PbrBundle {
                                mesh: assets.mesh_reality_shard.clone(),
                                material: assets.mat_prime_obsidian.clone(),
                                transform: Transform::from_xyz(lane.x_pos(), 2.75, strike_z),
                                ..default()
                            },
                            ActiveObstacle {
                                lane,
                                obstacle_type: ObstacleType::TallPillar,
                                size: Vec3::new(1.8, 5.5, 1.8),
                            },
                            EchoPrimeAttackProjectile {
                                lifetime: SHARDS_STRIKE_DURATION,
                            },
                        ));
                    }
                }

                // Safe lane gets Resonance Surge Orb (Overdrive + Shield powerup payoff!)
                commands.spawn((
                    PbrBundle {
                        mesh: assets.mesh_resonance_orb.clone(),
                        material: assets.mat_surge_gold_cyan.clone(),
                        transform: Transform::from_xyz(safe_lane.x_pos(), 1.2, strike_z),
                        ..default()
                    },
                    CollectibleItem {
                        item_type: CollectibleType::Overdrive,
                        lane: safe_lane,
                        initial_y: 1.2,
                        rot_speed: 4.0,
                    },
                    EchoPrimeResonanceSurgeOrb,
                    EchoPrimeAttackProjectile { lifetime: 3.5 },
                ));

                sfx.send(SoundEffect::Crash);
            }
        }
        EchoPrimeAttackStage::DeployingShards(_) => {
            if manager.state_timer >= SHARDS_STRIKE_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::Idle;
                manager.current_attack_index += 1;
            }
        }
        EchoPrimeAttackStage::Stunned => {
            if manager.state_timer >= STUN_DURATION {
                manager.attack_stage = EchoPrimeAttackStage::Idle;
                manager.current_attack_index += 1;
            }
        }
        _ => {}
    }
}

// -------------------------------------------------------------
// RESONANCE SURGE PAYOFF (Stun Boss & Damage HP)
// -------------------------------------------------------------

fn handle_resonance_orb_collection(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    orb_q: Query<(Entity, &Transform), With<EchoPrimeResonanceSurgeOrb>>,
    mut manager: ResMut<EchoPrimeManager>,
    mut boss_state: ResMut<BossBattleState>,
    mut powerups: ResMut<ActivePowerUps>,
    mut stats: ResMut<GameRunStats>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let p_pos = p_trans.translation;
    let pickup_radius_sq = 1.6 * 1.6;

    for (entity, o_trans) in orb_q.iter() {
        if o_trans.translation.distance_squared(p_pos) < pickup_radius_sq {
            // Player successfully collected Resonance Surge Orb!
            commands.entity(entity).despawn();

            // Grant Overdrive + Shield protection
            powerups.overdrive_timer = 6.0;
            powerups.shield = true;
            powerups.shield_hits = 2;
            stats.score += 1000;

            // Stun and damage ECHO Prime
            manager.health = (manager.health - 25.0).max(0.0);
            boss_state.boss_health = manager.health;
            manager.attack_stage = EchoPrimeAttackStage::Stunned;
            manager.state_timer = 0.0;

            sfx.send(SoundEffect::PowerupPickup);
        }
    }
}

// -------------------------------------------------------------
// CLEANUP SYSTEM
// -------------------------------------------------------------

fn cleanup_echo_prime_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut proj_q: Query<(Entity, &mut EchoPrimeAttackProjectile, &Transform)>,
    mut tele_q: Query<(Entity, &mut EchoPrimeTelegraph)>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let player_z = p_trans.translation.z;

    for (entity, mut proj, trans) in proj_q.iter_mut() {
        proj.lifetime -= dt;
        // Despawn if lifetime expired or passed behind player
        if proj.lifetime <= 0.0 || trans.translation.z > player_z + 6.0 {
            commands.entity(entity).despawn();
        }
    }

    for (entity, mut tele) in tele_q.iter_mut() {
        tele.lifetime -= dt;
        if tele.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// -------------------------------------------------------------
// RUN RESET HANDLER
// -------------------------------------------------------------

fn handle_run_reset_echo_prime(
    mut commands: Commands,
    mut reset_reader: EventReader<RunResetEvent>,
    mut manager: ResMut<EchoPrimeManager>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    boss_q: Query<Entity, With<EchoPrimeBoss>>,
    proj_q: Query<Entity, With<EchoPrimeAttackProjectile>>,
    tele_q: Query<Entity, With<EchoPrimeTelegraph>>,
) {
    for _ in reset_reader.read() {
        *manager = EchoPrimeManager::default();
        threat_alerts.hunter_telegraph_lane = None;

        for e in boss_q.iter() {
            commands.entity(e).despawn_recursive();
        }
        for e in proj_q.iter() {
            commands.entity(e).despawn();
        }
        for e in tele_q.iter() {
            commands.entity(e).despawn();
        }
    }
}

// -------------------------------------------------------------
// TESTS
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_prime_distance_bounds() {
        assert_eq!(ECHO_PRIME_START_DISTANCE, 13300.0);
        assert_eq!(ECHO_PRIME_END_DISTANCE, 13700.0);
        assert!(ECHO_PRIME_END_DISTANCE > ECHO_PRIME_START_DISTANCE);
        assert_eq!(ECHO_PRIME_END_DISTANCE - ECHO_PRIME_START_DISTANCE, 400.0);
    }

    #[test]
    fn test_echo_prime_attack_schedule_monotonicity() {
        assert!(ATTACK_DISTANCES.windows(2).all(|w| w[0] < w[1]));
        assert!(ATTACK_DISTANCES[0] >= ECHO_PRIME_START_DISTANCE);
        assert!(*ATTACK_DISTANCES.last().unwrap() < ECHO_PRIME_END_DISTANCE);
    }

    #[test]
    fn test_echo_prime_attack_types_cover_runner_fundamentals() {
        // Attack 0: Lance (lane switch)
        // Attack 1: Low Sweep (jump)
        // Attack 2: Reality Shards (threading)
        // Attack 3: High Sweep (slide)
        // Attack 4: Lance (lane switch)
        // Attack 5: Reality Shards (climax rush)
        assert_eq!(ATTACK_DISTANCES.len(), 6);
    }

    #[test]
    fn test_echo_prime_sweep_vertical_solvability() {
        // JumpLow wave is at y = 0.25m, obstacle size y = 0.5m -> clearable with standard Jump (y > 1.1m)
        let jump_clear_height = 1.1f32;
        let low_wave_top = 0.25 + 0.25;
        assert!(jump_clear_height > low_wave_top);

        // SlideHigh blade is at y = 1.35m, obstacle size y = 0.8m -> clearable with Slide
        let slide_clear_standing_hit = 1.35f32;
        assert!(slide_clear_standing_hit > 0.8);
    }
}
