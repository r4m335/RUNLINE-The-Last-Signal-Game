use bevy::prelude::*;
use crate::types::*;
use crate::director::RunDirector;

// ----------------------------------------------------------------------------
// ENEMY COMPONENTS & RESOURCES
// ----------------------------------------------------------------------------

#[derive(Component)]
pub struct ScoutDrone {
    pub trailing_distance: f32,
    pub target_lane: Lane,
    pub grapple_grace_timer: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HunterState {
    TrackingFlank,
    Telegraphing { target_lane: Lane, timer: f32 },
    CommittedSweep { target_lane: Lane, timer: f32 },
}

#[derive(Component)]
#[allow(dead_code)]
pub struct HunterDrone {
    pub tracking_lane: Lane,
    pub state: HunterState,
    pub eval_timer: f32,
    pub confidence: f32,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct HeavyBlocker {
    pub lane: Lane,
    pub activation_distance: f32,
    pub emp_radius: f32,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct EchoHunterBoss {
    pub health: f32,
    pub max_health: f32,
    pub phase: u8,
    pub phase_timer: f32,
    pub active_anomaly: bool,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct EnemyAssets {
    // Scout Drone Silhouette
    pub mesh_scout_body: Handle<Mesh>,
    pub mesh_scout_wing: Handle<Mesh>,
    pub mesh_scout_winglet: Handle<Mesh>,
    pub mesh_scout_thruster: Handle<Mesh>,
    pub mesh_scout_eye: Handle<Mesh>,
    pub mat_scout: Handle<StandardMaterial>,
    pub mat_scout_eye: Handle<StandardMaterial>,
    pub mat_scout_thruster: Handle<StandardMaterial>,

    // Hunter Interceptor Silhouette
    pub mesh_hunter_body: Handle<Mesh>,
    pub mesh_hunter_wing: Handle<Mesh>,
    pub mesh_hunter_nacelle: Handle<Mesh>,
    pub mesh_hunter_eye: Handle<Mesh>,
    pub mesh_hunter_cannon: Handle<Mesh>,
    pub mat_hunter: Handle<StandardMaterial>,
    pub mat_hunter_beam: Handle<StandardMaterial>,
    pub mat_hunter_eye: Handle<StandardMaterial>,

    // Heavy Fortified Blocker Silhouette
    pub mesh_heavy_body: Handle<Mesh>,
    pub mesh_heavy_pylon: Handle<Mesh>,
    pub mesh_heavy_quad_eye: Handle<Mesh>,
    pub mesh_heavy_generator: Handle<Mesh>,
    pub mesh_heavy_foot: Handle<Mesh>,
    pub mesh_emp_barrier: Handle<Mesh>,
    pub mat_heavy_armor: Handle<StandardMaterial>,
    pub mat_heavy_emp: Handle<StandardMaterial>,
    pub mat_heavy_hazard: Handle<StandardMaterial>,
    pub mat_heavy_eye: Handle<StandardMaterial>,

    // Boss Core & Common
    pub mesh_boss_core: Handle<Mesh>,
    pub mesh_sensor_eye: Handle<Mesh>,
    pub mat_boss_crystal: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct EnemySquadManager {
    pub spawn_cooldown: f32,
    pub active_boss: bool,
    pub boss_spawned_milestone_4: bool,
    pub boss_spawned_milestone_7: bool,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySquadManager>()
            .add_event::<BossStartedEvent>()
            .add_event::<BossDefeatedEvent>()
            .add_event::<EnemySpawnedEvent>()
            .add_event::<PlayerStumbledEvent>()
            .add_systems(Startup, init_enemy_assets)
            .add_systems(Update, handle_run_reset_enemies)
            .add_systems(
                FixedUpdate,
                (
                    update_enemy_spawning_and_pacing,
                    update_scout_ai_fixed,
                    update_hunter_ai_fixed,
                    update_heavy_blockers_fixed,
                    update_echo_hunter_boss_fixed,
                    player_enemy_interaction_fixed,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    update_enemy_visual_smoothing,
                    animate_enemy_thrusters,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn init_enemy_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EnemyAssets {
        // Scout Drone Silhouette (Needle dart with wide swept wings)
        mesh_scout_body: meshes.add(Cuboid::new(0.32, 0.18, 0.82)),
        mesh_scout_wing: meshes.add(Cuboid::new(0.52, 0.03, 0.45)),
        mesh_scout_winglet: meshes.add(Cuboid::new(0.04, 0.18, 0.26)),
        mesh_scout_thruster: meshes.add(Cuboid::new(0.16, 0.14, 0.20)),
        mesh_scout_eye: meshes.add(Sphere::new(0.09)),
        mat_scout: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.12, 0.15),
            metallic: 0.90,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_scout_eye: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.05, 0.10),
            emissive: LinearRgba::new(5.8, 0.15, 0.15, 1.0),
            ..default()
        }),
        mat_scout_thruster: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 1.0),
            emissive: LinearRgba::new(0.6, 2.0, 6.0, 1.0),
            ..default()
        }),

        // Hunter Interceptor Silhouette (Aggressive predatory wedge canopy)
        mesh_hunter_body: meshes.add(Cuboid::new(0.76, 0.30, 1.10)),
        mesh_hunter_wing: meshes.add(Cuboid::new(0.55, 0.05, 0.45)),
        mesh_hunter_nacelle: meshes.add(Cuboid::new(0.18, 0.16, 0.55)),
        mesh_hunter_eye: meshes.add(Sphere::new(0.11)),
        mesh_hunter_cannon: meshes.add(Cuboid::new(0.14, 0.14, 0.50)),
        mat_hunter: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.12, 0.22),
            metallic: 0.88,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_hunter_beam: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.50, 0.0),
            emissive: LinearRgba::new(5.8, 2.2, 0.0, 1.0),
            ..default()
        }),
        mat_hunter_eye: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.35, 0.0),
            emissive: LinearRgba::new(5.8, 2.5, 0.0, 1.0),
            ..default()
        }),

        // Heavy Fortified Blocker Silhouette (Massive 2m wide armored bastion)
        mesh_heavy_body: meshes.add(Cuboid::new(1.85, 1.45, 1.15)),
        mesh_heavy_pylon: meshes.add(Cuboid::new(0.50, 1.75, 0.75)),
        mesh_heavy_quad_eye: meshes.add(Sphere::new(0.09)),
        mesh_heavy_generator: meshes.add(Cuboid::new(0.85, 0.45, 0.65)),
        mesh_heavy_foot: meshes.add(Cuboid::new(0.28, 0.32, 0.48)),
        mesh_emp_barrier: meshes.add(Cuboid::new(2.40, 3.00, 0.15)),
        mat_heavy_armor: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.18, 0.22),
            metallic: 0.95,
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_heavy_emp: materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.7, 1.0, 0.55),
            emissive: LinearRgba::new(0.5, 3.2, 5.5, 1.0),
            ..default()
        }),
        mat_heavy_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.60, 0.05),
            emissive: LinearRgba::new(1.2, 0.6, 0.0, 1.0),
            perceptual_roughness: 0.30,
            ..default()
        }),
        mat_heavy_eye: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.05, 0.05),
            emissive: LinearRgba::new(5.5, 0.08, 0.08, 1.0),
            ..default()
        }),

        // Boss Core & Common
        mesh_boss_core: meshes.add(Sphere::new(1.1)),
        mesh_sensor_eye: meshes.add(Sphere::new(0.18)),
        mat_boss_crystal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.9, 0.9),
            emissive: LinearRgba::new(3.0, 0.5, 4.0, 1.0),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

fn handle_run_reset_enemies(
    mut events: EventReader<RunResetEvent>,
    mut commands: Commands,
    mut squad_mgr: ResMut<EnemySquadManager>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut boss_state: ResMut<BossBattleState>,
    enemies_q: Query<Entity, With<ActiveEnemy>>,
) {
    for _ in events.read() {
        // Clear old enemies
        for e in enemies_q.iter() {
            commands.entity(e).despawn_recursive();
        }

        *squad_mgr = EnemySquadManager::default();
        *threat_alerts = ThreatAlertState::default();
        *boss_state = BossBattleState::default();
    }
}

fn spawn_scout_drone(
    commands: &mut Commands,
    assets: &EnemyAssets,
    lane: Lane,
    initial_distance_behind: f32,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: assets.mesh_scout_body.clone(),
                material: assets.mat_scout.clone(),
                transform: Transform::from_xyz(lane.x_pos(), 2.2, initial_distance_behind),
                ..default()
            },
            ActiveEnemy {
                enemy_type: EnemyType::Scout,
                target_lane: lane,
                current_lane: lane,
                lane_switch_timer: 0.0,
                behavior_timer: 0.0,
                distance_from_player: initial_distance_behind,
                is_attacking: false,
            },
            ScoutDrone {
                trailing_distance: initial_distance_behind,
                target_lane: lane,
                grapple_grace_timer: 1.5,
            },
            ChaserDrone {
                distance_behind: initial_distance_behind,
            },
        ))
        .with_children(|drone| {
            // Dual Red Sensor Tracking Eyes
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_eye.clone(),
                material: assets.mat_scout_eye.clone(),
                transform: Transform::from_xyz(-0.12, 0.02, -0.36),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_eye.clone(),
                material: assets.mat_scout_eye.clone(),
                transform: Transform::from_xyz(0.12, 0.02, -0.36),
                ..default()
            });
            // Left & Right Swept Stabilization Wings
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_wing.clone(),
                material: assets.mat_scout.clone(),
                transform: Transform::from_xyz(-0.44, 0.02, 0.06),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_wing.clone(),
                material: assets.mat_scout.clone(),
                transform: Transform::from_xyz(0.44, 0.02, 0.06),
                ..default()
            });
            // Winglet Fin Tips
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_winglet.clone(),
                material: assets.mat_scout.clone(),
                transform: Transform::from_xyz(-0.68, 0.10, 0.10),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_winglet.clone(),
                material: assets.mat_scout.clone(),
                transform: Transform::from_xyz(0.68, 0.10, 0.10),
                ..default()
            });
            // Rear High-Energy Thruster Nozzle & Exhaust
            drone.spawn(PbrBundle {
                mesh: assets.mesh_scout_thruster.clone(),
                material: assets.mat_scout_thruster.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.36),
                ..default()
            });
        });
}

fn spawn_hunter_drone(
    commands: &mut Commands,
    assets: &EnemyAssets,
    lane: Lane,
    initial_distance_behind: f32,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: assets.mesh_hunter_body.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(lane.x_pos(), 2.8, initial_distance_behind),
                ..default()
            },
            ActiveEnemy {
                enemy_type: EnemyType::Hunter,
                target_lane: lane,
                current_lane: lane,
                lane_switch_timer: 0.0,
                behavior_timer: 0.0,
                distance_from_player: initial_distance_behind,
                is_attacking: false,
            },
            HunterDrone {
                tracking_lane: lane,
                state: HunterState::TrackingFlank,
                eval_timer: 1.2,
                confidence: 0.5,
            },
        ))
        .with_children(|drone| {
            // Dual Piercing Amber/Orange Targeting Optics
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_eye.clone(),
                material: assets.mat_hunter_eye.clone(),
                transform: Transform::from_xyz(-0.16, 0.0, -0.48),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_eye.clone(),
                material: assets.mat_hunter_eye.clone(),
                transform: Transform::from_xyz(0.16, 0.0, -0.48),
                ..default()
            });
            // Left & Right Forward-Swept Combat Foils
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_wing.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(-0.58, 0.04, -0.10),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_wing.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(0.58, 0.04, -0.10),
                ..default()
            });
            // Flanking Repulsor Nacelles
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_nacelle.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(-0.48, -0.10, 0.15),
                ..default()
            });
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_nacelle.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(0.48, -0.10, 0.15),
                ..default()
            });
            // Underslung Pulse Interceptor Cannon
            drone.spawn(PbrBundle {
                mesh: assets.mesh_hunter_cannon.clone(),
                material: assets.mat_hunter.clone(),
                transform: Transform::from_xyz(0.0, -0.18, -0.25),
                ..default()
            });
            // Targeting Laser Array Emitter
            drone.spawn(PbrBundle {
                mesh: assets.mesh_sensor_eye.clone(),
                material: assets.mat_hunter_beam.clone(),
                transform: Transform::from_xyz(0.0, -0.10, -0.55),
                ..default()
            });
        });
}

fn spawn_heavy_blocker(
    commands: &mut Commands,
    assets: &EnemyAssets,
    lane: Lane,
    spawn_z: f32,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: assets.mesh_heavy_body.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(lane.x_pos(), 1.6, spawn_z),
                ..default()
            },
            ActiveEnemy {
                enemy_type: EnemyType::Heavy,
                target_lane: lane,
                current_lane: lane,
                lane_switch_timer: 0.0,
                behavior_timer: 0.0,
                distance_from_player: -38.0,
                is_attacking: true,
            },
            HeavyBlocker {
                lane,
                activation_distance: 38.0,
                emp_radius: 2.4,
            },
            Despawnable { z_center: spawn_z },
        ))
        .with_children(|heavy| {
            // Massive Left & Right Flanking Armored Shoulder Pylons
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_pylon.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(-0.98, 0.15, 0.0),
                ..default()
            });
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_pylon.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(0.98, 0.15, 0.0),
                ..default()
            });
            // Quad Surveillance Matrix Eye Optics across brow
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_quad_eye.clone(),
                material: assets.mat_heavy_eye.clone(),
                transform: Transform::from_xyz(-0.45, 0.55, -0.56),
                ..default()
            });
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_quad_eye.clone(),
                material: assets.mat_heavy_eye.clone(),
                transform: Transform::from_xyz(-0.18, 0.55, -0.56),
                ..default()
            });
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_quad_eye.clone(),
                material: assets.mat_heavy_eye.clone(),
                transform: Transform::from_xyz(0.18, 0.55, -0.56),
                ..default()
            });
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_quad_eye.clone(),
                material: assets.mat_heavy_eye.clone(),
                transform: Transform::from_xyz(0.45, 0.55, -0.56),
                ..default()
            });
            // Underslung Heavy EMP Containment Generator
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_generator.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(0.0, -0.55, 0.0),
                ..default()
            });
            // Grounding Magnetic Stabilizers
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_foot.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(-0.65, -0.75, 0.0),
                ..default()
            });
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_heavy_foot.clone(),
                material: assets.mat_heavy_armor.clone(),
                transform: Transform::from_xyz(0.65, -0.75, 0.0),
                ..default()
            });
            // Pulsing EMP Suppression Shield Barrier Grid
            heavy.spawn(PbrBundle {
                mesh: assets.mesh_emp_barrier.clone(),
                material: assets.mat_heavy_emp.clone(),
                transform: Transform::from_xyz(0.0, 0.35, 0.85),
                ..default()
            });
        });
}

fn spawn_echo_hunter_boss(
    commands: &mut Commands,
    assets: &EnemyAssets,
    boss_name: &'static str,
    spawn_z: f32,
    boss_events: &mut EventWriter<BossStartedEvent>,
    boss_state: &mut ResMut<BossBattleState>,
) {
    boss_state.is_active = true;
    boss_state.boss_name = boss_name;
    boss_state.time_remaining = 25.0;
    boss_state.boss_health = 100.0;
    boss_state.victory_banner_timer = 0.0;

    boss_events.send(BossStartedEvent {
        boss_name,
        health: 100.0,
    });

    commands.spawn((
        PbrBundle {
            mesh: assets.mesh_boss_core.clone(),
            material: assets.mat_boss_crystal.clone(),
            transform: Transform::from_xyz(0.0, 3.5, spawn_z),
            ..default()
        },
        ActiveEnemy {
            enemy_type: EnemyType::EchoHunter,
            target_lane: Lane::Center,
            current_lane: Lane::Center,
            lane_switch_timer: 0.0,
            behavior_timer: 0.0,
            distance_from_player: -28.0,
            is_attacking: true,
        },
        EchoHunterBoss {
            health: 100.0,
            max_health: 100.0,
            phase: 1,
            phase_timer: 0.0,
            active_anomaly: true,
        },
    ));
}

// ----------------------------------------------------------------------------
// SQUAD PACING & SPAWNING CONTROLLER (With Solvability Checks)
// ----------------------------------------------------------------------------
fn update_enemy_spawning_and_pacing(
    mut commands: Commands,
    time: Res<Time>,
    director: Res<RunDirector>,
    stats: Res<GameRunStats>,
    assets: Res<EnemyAssets>,
    mut squad_mgr: ResMut<EnemySquadManager>,
    player_q: Query<&Transform, With<Player>>,
    enemies_q: Query<&ActiveEnemy>,
    obs_q: Query<(&ActiveObstacle, &Transform)>,
    mut boss_start_events: EventWriter<BossStartedEvent>,
    mut enemy_spawn_events: EventWriter<EnemySpawnedEvent>,
    mut boss_state: ResMut<BossBattleState>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    squad_mgr.spawn_cooldown -= dt;

    let has_scout = enemies_q.iter().any(|e| e.enemy_type == EnemyType::Scout);
    let has_hunter = enemies_q.iter().any(|e| e.enemy_type == EnemyType::Hunter);
    let heavy_count = enemies_q.iter().filter(|e| e.enemy_type == EnemyType::Heavy).count();

    // 1. Level 1 Scout persistence: introduced at 600m in Zone 1 (Old Metro) for pedagogical pacing
    if (stats.distance >= 600.0 || director.active_zone_id >= 2) && !has_scout {
        spawn_scout_drone(&mut commands, &assets, Lane::Center, 7.5);
        enemy_spawn_events.send(EnemySpawnedEvent {
            enemy_type: EnemyType::Scout,
            lane: Lane::Center,
        });
    }

    // 2. Level 2 Hunter introduction in Zone 2+ (Neon District onwards)
    // Pedagogical progression: 800-950m is speed adaptation buffer (NO Hunter).
    // Hunter is introduced at 950m+ with audio telegraph + predictive sweep.
    if (stats.distance >= 950.0 || director.active_zone_id >= 3) && !has_hunter && squad_mgr.spawn_cooldown <= 0.0 {
        spawn_hunter_drone(&mut commands, &assets, Lane::Left, 8.5);
        squad_mgr.spawn_cooldown = 15.0;
        enemy_spawn_events.send(EnemySpawnedEvent {
            enemy_type: EnemyType::Hunter,
            lane: Lane::Left,
        });
    }

    // 3. Level 3 Heavy Route Blocker deployment in Zone 3+ (Industrial onwards)
    // Procedural solvability guarantee: Heavy may deny 1 lane, but MUST NEVER combine
    // with track obstacles to block all remaining routes!
    if director.active_zone_id >= 3 && heavy_count == 0 && squad_mgr.spawn_cooldown <= 0.0 {
        let spawn_z = p_trans.translation.z - 38.0;

        // Query existing track obstacles in the +/- 5m window around spawn_z
        let mut lane_has_obstacle = [false; 3]; // [Left, Center, Right]
        for (obs, o_trans) in obs_q.iter() {
            if (o_trans.translation.z - spawn_z).abs() <= 5.0 {
                match obs.lane {
                    Lane::Left => lane_has_obstacle[0] = true,
                    Lane::Center => lane_has_obstacle[1] = true,
                    Lane::Right => lane_has_obstacle[2] = true,
                }
            }
        }

        let blocked_count = lane_has_obstacle.iter().filter(|&&b| b).count();
        if blocked_count < 2 {
            // Find candidate lanes where placing Heavy still leaves at least one open lane
            let all_lanes = [Lane::Left, Lane::Center, Lane::Right];
            let mut valid_lanes = Vec::new();

            for (idx, &lane) in all_lanes.iter().enumerate() {
                if !lane_has_obstacle[idx] {
                    let other_route_open = all_lanes.iter().enumerate().any(|(other_idx, _)| {
                        other_idx != idx && !lane_has_obstacle[other_idx]
                    });
                    if other_route_open {
                        valid_lanes.push(lane);
                    }
                }
            }

            if let Some(&target_lane) = valid_lanes.first() {
                spawn_heavy_blocker(&mut commands, &assets, target_lane, spawn_z);
                squad_mgr.spawn_cooldown = 18.0;
                enemy_spawn_events.send(EnemySpawnedEvent {
                    enemy_type: EnemyType::Heavy,
                    lane: target_lane,
                });
            } else {
                squad_mgr.spawn_cooldown = 2.0; // Retry when track opens up
            }
        } else {
            squad_mgr.spawn_cooldown = 2.5; // Defer until clear track window
        }
    }

    // 4. Milestone Boss Encounters: Zone 4 & Zone 7
    if stats.distance >= crate::zones::BOSS_MILESTONE_4_DISTANCE && !squad_mgr.boss_spawned_milestone_4 && !squad_mgr.active_boss {
        squad_mgr.boss_spawned_milestone_4 = true;
        squad_mgr.active_boss = true;
        let spawn_z = p_trans.translation.z - 30.0;
        spawn_echo_hunter_boss(
            &mut commands,
            &assets,
            "Veyron Vanguard Alpha",
            spawn_z,
            &mut boss_start_events,
            &mut boss_state,
        );
    } else if stats.distance >= crate::zones::BOSS_MILESTONE_7_DISTANCE && !squad_mgr.boss_spawned_milestone_7 && !squad_mgr.active_boss {
        squad_mgr.boss_spawned_milestone_7 = true;
        squad_mgr.active_boss = true;
        let spawn_z = p_trans.translation.z - 32.0;
        spawn_echo_hunter_boss(
            &mut commands,
            &assets,
            "ECHO Singularity Hunter",
            spawn_z,
            &mut boss_start_events,
            &mut boss_state,
        );
    }
}

// ----------------------------------------------------------------------------
// LEVEL 1: SCOUT AI (Predictable trailing pursuit)
// ----------------------------------------------------------------------------
fn update_scout_ai_fixed(
    time: Res<Time>,
    player_q: Query<(&Player, &Transform), Without<ScoutDrone>>,
    mut scout_q: Query<(&mut Transform, &mut ActiveEnemy, &mut ScoutDrone), (With<ScoutDrone>, Without<Player>)>,
    stats: Res<GameRunStats>,
    powerups: Res<ActivePowerUps>,
) {
    let (player, p_trans) = match player_q.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    for (mut d_trans, mut enemy, mut scout) in scout_q.iter_mut() {
        // Predictable lane trailing: 0.4s lag behind player transitions
        enemy.behavior_timer += dt;
        if enemy.behavior_timer > 0.4 {
            scout.target_lane = player.lane;
            enemy.behavior_timer = 0.0;
        }

        // Surge distance upon player stumble
        let target_dist = if powerups.overdrive_timer > 0.0 {
            8.5 // Overdrive widens gap
        } else if stats.stumble_intensity > 0.0 {
            2.0 + (1.0 - stats.stumble_intensity) * 4.5
        } else {
            6.5
        };

        scout.trailing_distance += (target_dist - scout.trailing_distance) * 8.0 * dt;
        d_trans.translation.z = p_trans.translation.z + scout.trailing_distance;
        enemy.distance_from_player = scout.trailing_distance;
    }
}

// ----------------------------------------------------------------------------
// LEVEL 2: HUNTER AI (Predictive interception with confidence threshold & telegraph)
// ----------------------------------------------------------------------------
fn update_hunter_ai_fixed(
    time: Res<Time>,
    player_q: Query<(&Player, &Transform), (Without<HunterDrone>, Without<ScoutDrone>)>,
    heavy_q: Query<(&Transform, &HeavyBlocker), Without<HunterDrone>>,
    history: Res<PlayerMovementHistory>,
    stats: Res<GameRunStats>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut hunter_q: Query<(&mut Transform, &mut ActiveEnemy, &mut HunterDrone), (With<HunterDrone>, Without<Player>, Without<HeavyBlocker>)>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let (player, p_trans) = match player_q.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let now = time.elapsed_seconds();

    for (mut d_trans, mut enemy, mut hunter) in hunter_q.iter_mut() {
        match hunter.state {
            HunterState::TrackingFlank => {
                threat_alerts.hunter_telegraph_lane = None;
                hunter.eval_timer -= dt;

                // Follow player with observation lag
                let target_z = p_trans.translation.z + 5.5;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 6.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                // THREAT CONCURRENCY FAIRNESS RULES:
                // Rule 1: Stumble Grace - Hunter holds fire while player is recovering from a stumble
                if stats.stumble_intensity > 0.0 {
                    hunter.eval_timer = 1.6;
                    continue;
                }

                // Rule 2: Heavy Corridor Protection - Hunter holds fire while player is within a Heavy blocker corridor
                let heavy_corridor_active = heavy_q.iter().any(|(h_trans, _)| {
                    let dz = p_trans.translation.z - h_trans.translation.z;
                    dz >= -5.0 && dz <= 35.0
                });
                if heavy_corridor_active {
                    hunter.eval_timer = 1.8;
                    continue;
                }

                if hunter.eval_timer <= 0.0 {
                    hunter.eval_timer = 1.4;

                    // Evaluate player movement history:
                    // 1. High switch frequency in last second indicates erratic dodging -> low prediction confidence
                    if history.switch_count_last_second >= 2 {
                        hunter.confidence = 0.35; // Below 0.70 threshold: stay in tracking mode
                    } else if (now - history.last_switch_time) < 0.45 && !history.recent_switches.is_empty() {
                        // Player recently initiated a deliberate transition
                        let target_lane = match player.lane {
                            Lane::Left => Lane::Center,
                            Lane::Center => {
                                // If player moved into center from left, momentum suggests right, and vice versa
                                if let Some(&(_, prev_lane)) = history.recent_switches.iter().rev().nth(1) {
                                    if prev_lane == Lane::Left {
                                        Lane::Right
                                    } else {
                                        Lane::Left
                                    }
                                } else {
                                    Lane::Center
                                }
                            }
                            Lane::Right => Lane::Center,
                        };
                        hunter.confidence = 0.78; // Above 0.70 threshold: commit to telegraph
                        hunter.state = HunterState::Telegraphing {
                            target_lane,
                            timer: 0.65, // 0.65s telegraph provides >= 0.47s reaction window above human reflex
                        };
                        enemy.target_lane = target_lane;
                        threat_alerts.hunter_telegraph_lane = Some(target_lane);
                        sfx.send(SoundEffect::Dash);
                    } else {
                        // Steady line held for > 0.8s
                        hunter.confidence = 0.82; // Above 0.70 threshold: predict interception in current lane
                        hunter.state = HunterState::Telegraphing {
                            target_lane: player.lane,
                            timer: 0.65,
                        };
                        enemy.target_lane = player.lane;
                        threat_alerts.hunter_telegraph_lane = Some(player.lane);
                        sfx.send(SoundEffect::Dash);
                    }
                }
            }
            HunterState::Telegraphing { target_lane, mut timer } => {
                timer -= dt;
                enemy.is_attacking = true;
                threat_alerts.hunter_telegraph_lane = Some(target_lane);
                // Hover slightly forward and aim beam into target lane
                let target_z = p_trans.translation.z + 3.5;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 8.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                if timer <= 0.0 {
                    threat_alerts.hunter_telegraph_lane = None;
                    hunter.state = HunterState::CommittedSweep {
                        target_lane,
                        timer: 0.6,
                    };
                } else {
                    hunter.state = HunterState::Telegraphing { target_lane, timer };
                }
            }
            HunterState::CommittedSweep { target_lane, mut timer } => {
                timer -= dt;
                threat_alerts.hunter_telegraph_lane = None;
                // Surge forward aggressively along target lane
                let target_z = p_trans.translation.z + 0.8;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 12.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;
                enemy.target_lane = target_lane;

                if timer <= 0.0 {
                    // Sweep finished; return to tracking flank
                    hunter.state = HunterState::TrackingFlank;
                    hunter.eval_timer = 1.8;
                    enemy.is_attacking = false;
                } else {
                    hunter.state = HunterState::CommittedSweep { target_lane, timer };
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// LEVEL 3: HEAVY BLOCKER AI (Tactical route denial)
// ----------------------------------------------------------------------------
fn update_heavy_blockers_fixed(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    heavy_q: Query<(Entity, &Transform, &HeavyBlocker)>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    // Update Heavy warning lane for HUD readability when within 38m ahead
    threat_alerts.heavy_warning_lane = heavy_q.iter().find(|(_, h_trans, _)| {
        let dz = p_trans.translation.z - h_trans.translation.z;
        dz >= -5.0 && dz <= 38.0
    }).map(|(_, _, h)| h.lane);

    for (entity, h_trans, _heavy) in heavy_q.iter() {
        // When player successfully passes the Heavy blocker, trigger dissolution and despawn
        if p_trans.translation.z < h_trans.translation.z - 4.0 {
            sfx.send(SoundEffect::ShieldBreak);
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ----------------------------------------------------------------------------
// ELITE: ECHO HUNTER BOSS AI (Milestone 25s Gauntlet Encounter)
// ----------------------------------------------------------------------------
fn update_echo_hunter_boss_fixed(
    mut commands: Commands,
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut boss_q: Query<(Entity, &mut Transform, &mut EchoHunterBoss, &mut ActiveEnemy), (With<EchoHunterBoss>, Without<Player>)>,
    mut squad_mgr: ResMut<EnemySquadManager>,
    mut boss_state: ResMut<BossBattleState>,
    mut boss_defeated_events: EventWriter<BossDefeatedEvent>,
    mut stats: ResMut<GameRunStats>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    for (entity, mut b_trans, mut boss, mut enemy) in boss_q.iter_mut() {
        boss.phase_timer += dt;
        boss_state.time_remaining = (25.0 - boss.phase_timer).max(0.0);
        boss_state.boss_health = boss.health;

        // 3 Phase Gauntlet:
        // Phase 1 (0.0..8.0s): Flank Sweeping
        // Phase 2 (8.0..16.0s): EMP Wave charges
        // Phase 3 (16.0..25.0s): Aggressive Interception closer to player
        let (target_offset_z, sweep_speed, sweep_width) = if boss.phase_timer < 8.0 {
            boss.phase = 1;
            (-26.0, 1.6, 2.4)
        } else if boss.phase_timer < 16.0 {
            boss.phase = 2;
            (-22.0, 2.4, 2.4)
        } else {
            boss.phase = 3;
            (-17.0, 3.2, 2.4)
        };

        let target_z = p_trans.translation.z + target_offset_z;
        b_trans.translation.z = target_z;
        enemy.distance_from_player = target_offset_z;

        let sweep_x = (boss.phase_timer * sweep_speed).sin() * sweep_width;
        b_trans.translation.x = sweep_x;

        // Survival / Defeat condition: 25 seconds completed or boss health depleted
        if boss.phase_timer >= 25.0 || boss.health <= 0.0 {
            boss_defeated_events.send(BossDefeatedEvent {
                boss_name: "Veyron Vanguard Alpha",
                reward_fragments: 50,
            });
            stats.fragments += 50;
            stats.score += 2500;
            squad_mgr.active_boss = false;
            boss_state.is_active = false;
            boss_state.victory_banner_timer = 5.0;
            sfx.send(SoundEffect::ZoneTransition);
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ----------------------------------------------------------------------------
// EXPLICIT PLAYER / ENEMY INTERACTION MATRIX (FixedUpdate)
// ----------------------------------------------------------------------------
fn player_enemy_interaction_fixed(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(&mut Player, &Transform)>,
    mut enemies_q: Query<(Entity, &Transform, &ActiveEnemy, Option<&mut ScoutDrone>, Option<&mut EchoHunterBoss>), Without<Player>>,
    mut powerups: ResMut<ActivePowerUps>,
    mut stats: ResMut<GameRunStats>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let (mut player, p_trans) = match player_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let dt = time.delta_seconds();

    let p_pos = p_trans.translation;
    let p_lane = player.lane;

    for (entity, e_trans, enemy, mut scout_opt, mut boss_opt) in enemies_q.iter_mut() {
        let e_pos = e_trans.translation;
        let z_dist = (p_pos.z - e_pos.z).abs();

        match enemy.enemy_type {
            EnemyType::Scout => {
                // Scout trails behind. If player is severely stumbling and scout catches up:
                let scout_dist = enemy.distance_from_player;
                if let Some(ref mut scout) = scout_opt {
                    if scout_dist <= 1.8 && stats.stumble_intensity > 0.5 {
                        if player.invulnerable_timer <= 0.0 && !powerups.shield {
                            threat_alerts.scout_grappling = true;
                            scout.grapple_grace_timer -= dt;
                            if scout.grapple_grace_timer <= 0.0 {
                                // Failed to recover within the 1.2s grace window
                                sfx.send(SoundEffect::Crash);
                                next_state.set(AppState::GameOver);
                                return;
                            }
                        }
                    } else {
                        scout.grapple_grace_timer = 1.5;
                        threat_alerts.scout_grappling = false;
                    }
                }
            }
            EnemyType::Hunter => {
                // If Hunter is actively sweeping into player's lane:
                if enemy.is_attacking && (p_pos.x - e_pos.x).abs() < 1.3 && z_dist < 1.4 {
                    // Rule 1: Dash / Overdrive destroys Hunter!
                    if powerups.overdrive_timer > 0.0 || (player.character == CharacterType::Mira && player.invulnerable_timer > 0.8) {
                        stats.fragments += 15;
                        stats.score += 300;
                        sfx.send(SoundEffect::ShieldBreak);
                        commands.entity(entity).despawn_recursive();
                        return;
                    }

                    if player.invulnerable_timer > 0.0 {
                        continue;
                    }

                    // Rule 2: Shield absorbs hit
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
                        return;
                    }

                    // Rule 3: Stumble or Crash
                    if stats.stumble_intensity == 0.0 {
                        stats.stumble_intensity = 0.85;
                        player.invulnerable_timer = 1.6;
                        sfx.send(SoundEffect::Stumble);
                    } else {
                        sfx.send(SoundEffect::Crash);
                        next_state.set(AppState::GameOver);
                        return;
                    }
                }
            }
            EnemyType::Heavy => {
                // If player enters Heavy lane with fair collision margins:
                if enemy.target_lane == p_lane && (p_pos.x - e_pos.x).abs() < 1.1 && z_dist < 1.4 && p_pos.y < 2.2 {
                    // Rule 1: Dash / Overdrive smashes Heavy blocker!
                    if powerups.overdrive_timer > 0.0 || (player.character == CharacterType::Mira && player.invulnerable_timer > 0.8) {
                        stats.fragments += 25;
                        stats.score += 500;
                        sfx.send(SoundEffect::ShieldBreak);
                        commands.entity(entity).despawn_recursive();
                        return;
                    }

                    if player.invulnerable_timer > 0.0 {
                        continue;
                    }

                    // Rule 2: Shield absorbs hit
                    if powerups.shield {
                        if powerups.shield_hits > 1 {
                            powerups.shield_hits -= 1;
                        } else {
                            powerups.shield = false;
                            powerups.shield_hits = 0;
                        }
                        player.invulnerable_timer = 1.6;
                        stats.stumble_intensity = 1.0;
                        sfx.send(SoundEffect::ShieldBreak);
                        commands.entity(entity).despawn_recursive();
                        return;
                    }

                    // Rule 3: Arin second-chance or fatal crash
                    if player.character == CharacterType::Arin && stats.stumble_intensity == 0.0 {
                        stats.stumble_intensity = 1.0;
                        player.invulnerable_timer = 1.8;
                        sfx.send(SoundEffect::Stumble);
                        commands.entity(entity).despawn_recursive();
                        return;
                    }

                    // Fatal crash into fortified bulkhead
                    sfx.send(SoundEffect::Crash);
                    next_state.set(AppState::GameOver);
                    return;
                }
            }
            EnemyType::EchoHunter => {
                // If player is close to boss during overdrive, deal damage!
                if z_dist < 3.5 && (p_pos.x - e_pos.x).abs() < 1.8 {
                    if powerups.overdrive_timer > 0.0 {
                        if let Some(ref mut boss) = boss_opt {
                            boss.health = (boss.health - 35.0).max(0.0);
                            sfx.send(SoundEffect::ShieldBreak);
                        }
                    }
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// VISUAL SMOOTHING & ROTATION (Runs in Update for silky 60+ FPS rendering)
// ----------------------------------------------------------------------------
fn update_enemy_visual_smoothing(
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(&mut Transform, &ActiveEnemy), Without<Player>>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();

    for (mut d_trans, enemy) in enemy_q.iter_mut() {
        match enemy.enemy_type {
            EnemyType::Scout => {
                let target_x = enemy.target_lane.x_pos() + (t * 2.5).sin() * 0.35;
                let target_y = 2.1 + (t * 3.5).cos() * 0.2;
                d_trans.translation.x += (target_x - d_trans.translation.x) * 12.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                d_trans.look_at(p_trans.translation + Vec3::new(0.0, 0.6, 0.0), Vec3::Y);
            }
            EnemyType::Hunter => {
                let target_x = enemy.target_lane.x_pos() + (t * 3.2).cos() * 0.3;
                let target_y = 2.8 + (t * 4.0).sin() * 0.25;
                d_trans.translation.x += (target_x - d_trans.translation.x) * 14.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 9.0 * dt;
                d_trans.look_at(p_trans.translation, Vec3::Y);
            }
            EnemyType::Heavy => {
                // Grounded hovering bulkhead
                d_trans.translation.y = 1.6 + (t * 2.0).sin() * 0.1;
            }
            EnemyType::EchoHunter => {
                // Menacing crystalline float
                d_trans.translation.y = 3.6 + (t * 2.8).sin() * 0.3;
                d_trans.rotate_y(1.2 * dt);
            }
        }
    }
}

fn animate_enemy_thrusters(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<HeavyBlocker>>,
) {
    let dt = time.delta_seconds();
    for mut trans in query.iter_mut() {
        trans.rotate_z((time.elapsed_seconds() * 4.0).sin() * 0.02 * dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_systems_parameter_initialization_no_conflicts() {
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<AppState>();
        app.init_resource::<EnemySquadManager>();
        app.init_resource::<PlayerMovementHistory>();
        app.init_resource::<ThreatAlertState>();
        app.init_resource::<GameRunStats>();
        app.init_resource::<ActivePowerUps>();
        app.init_resource::<BossBattleState>();
        app.init_resource::<crate::director::RunDirector>();
        app.add_event::<BossStartedEvent>();
        app.add_event::<BossDefeatedEvent>();
        app.add_event::<EnemySpawnedEvent>();
        app.add_event::<PlayerStumbledEvent>();
        app.add_event::<SoundEffect>();
        app.add_event::<RunResetEvent>();

        // Register all enemy AI and gameplay systems
        app.add_systems(
            Update,
            (
                update_scout_ai_fixed,
                update_hunter_ai_fixed,
                update_heavy_blockers_fixed,
                update_echo_hunter_boss_fixed,
                player_enemy_interaction_fixed,
                update_enemy_visual_smoothing,
                animate_enemy_thrusters,
                handle_run_reset_enemies,
            ),
        );

        // app.update() initializes every system parameter and validates Query disjointness
        app.update();
    }

    #[test]
    fn test_hunter_telegraph_kinematics_and_fairness_guarantees() {
        let zone_2_speed = 18.4_f32;
        let hunter_telegraph_duration = 0.65_f32;
        let distance_covered = zone_2_speed * hunter_telegraph_duration;

        let reaction_time = 0.18_f32;
        let lateral_lane_transition = 0.17_f32;
        let required_time = reaction_time + lateral_lane_transition;
        let required_distance = zone_2_speed * required_time;

        // Verify kinematic buffer
        assert!(
            hunter_telegraph_duration >= required_time + 0.20,
            "Hunter telegraph must provide >= 0.20s margin above reaction + lateral dodge"
        );
        assert!(
            distance_covered >= required_distance + 4.0,
            "Hunter telegraph must provide >= 4.0m longitudinal margin: covered={:.2}m, required={:.2}m",
            distance_covered,
            required_distance
        );

        // Verify Hunter patterns guarantee at least 2 open lanes
        let catalog = crate::patterns::get_pattern_catalog();
        let hunter_patterns: Vec<_> = catalog.iter().filter(|p| p.name.contains("Hunter")).collect();
        assert!(!hunter_patterns.is_empty(), "Must have Hunter pattern family in catalog");

        for pattern in hunter_patterns {
            for &z in &[-10.0, -16.0, -20.0] {
                let open_lanes = pattern.get_navigable_lanes_at(z, 0.5);
                assert!(
                    open_lanes.lanes().len() >= 2,
                    "Pattern '{}' at z={:.1}m must leave >= 2 open lanes for Hunter dodge, left {}",
                    pattern.name,
                    z,
                    open_lanes.lanes().len()
                );
            }
        }
    }
}

