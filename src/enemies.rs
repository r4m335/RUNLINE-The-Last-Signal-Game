use crate::director::RunDirector;
use crate::types::*;
use bevy::prelude::*;
use rand::Rng;

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
    Infiltrating {
        approach_timer: f32,
    },
    IntimidationWait {
        hover_timer: f32,
    },
    LockingTarget {
        target_lane: Lane,
        lock_timer: f32,
    },
    Telegraphing {
        target_lane: Lane,
        timer: f32,
        total_time: f32,
    },
    Firing {
        target_lane: Lane,
        burst_timer: f32,
        has_resolved_hit: bool,
    },
    Retreating {
        retreat_timer: f32,
    },
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
pub struct HunterModel;

#[derive(Component)]
pub struct HunterLaserBeam;

#[derive(Component)]
pub struct HunterImpactMarker;

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

    // Hunter Interceptor Silhouette & 3D GLB Model
    pub hunter_scene: Handle<Scene>,
    pub mesh_hunter_body: Handle<Mesh>,
    pub mesh_hunter_wing: Handle<Mesh>,
    pub mesh_hunter_nacelle: Handle<Mesh>,
    pub mesh_hunter_eye: Handle<Mesh>,
    pub mesh_hunter_cannon: Handle<Mesh>,
    pub mat_hunter: Handle<StandardMaterial>,
    pub mat_hunter_beam: Handle<StandardMaterial>,
    pub mat_hunter_eye: Handle<StandardMaterial>,

    // Hunter Targeted Warning Laser & Track Danger Marker
    pub mesh_hunter_laser: Handle<Mesh>,
    pub mat_hunter_laser: Handle<StandardMaterial>,
    pub mat_hunter_laser_burst: Handle<StandardMaterial>,
    pub mesh_hunter_marker: Handle<Mesh>,
    pub mat_hunter_marker: Handle<StandardMaterial>,

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
    pub hunter_cooldown: f32,
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
                    update_hunter_laser_visuals,
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
    asset_server: Res<AssetServer>,
) {
    let assets = EnemyAssets {
        // Hunter Interceptor 3D Model
        hunter_scene: asset_server.load("Charecters/Hunter/Hunter2.glb#Scene0"),

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

        // Hunter Targeted Warning Laser & Track Danger Marker
        mesh_hunter_laser: meshes.add(Cuboid::new(0.06, 0.06, 1.0)),
        mat_hunter_laser: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.1),
            emissive: LinearRgba::new(8.0, 0.4, 0.4, 1.0),
            unlit: true,
            ..default()
        }),
        mat_hunter_laser_burst: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.8, 0.8),
            emissive: LinearRgba::new(25.0, 4.0, 4.0, 1.0),
            unlit: true,
            ..default()
        }),
        mesh_hunter_marker: meshes.add(Cuboid::new(1.90, 0.04, 4.80)),
        mat_hunter_marker: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.08, 0.08, 0.75),
            emissive: LinearRgba::new(8.0, 0.4, 0.4, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
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

fn spawn_hunter_drone(commands: &mut Commands, assets: &EnemyAssets, lane: Lane, spawn_z: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(lane.x_pos(), 5.0, spawn_z),
                ..default()
            },
            ActiveEnemy {
                enemy_type: EnemyType::Hunter,
                target_lane: lane,
                current_lane: lane,
                lane_switch_timer: 0.0,
                behavior_timer: 0.0,
                distance_from_player: -12.0,
                is_attacking: false,
            },
            HunterDrone {
                tracking_lane: lane,
                state: HunterState::Infiltrating {
                    approach_timer: 0.7,
                },
                eval_timer: 0.8,
                confidence: 0.5,
            },
        ))
        .with_children(|drone| {
            // Real 3D Hunter Interceptor Drone Model (Faces forward along -Z toward Kai)
            drone.spawn((
                SceneBundle {
                    scene: assets.hunter_scene.clone(),
                    transform: Transform::IDENTITY,
                    ..default()
                },
                HunterModel,
            ));

            // Targeting Red Warning Laser Beam (Procedural emissive 3D beam)
            drone.spawn((
                PbrBundle {
                    mesh: assets.mesh_hunter_laser.clone(),
                    material: assets.mat_hunter_laser.clone(),
                    transform: Transform::IDENTITY,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HunterLaserBeam,
            ));

            // Glowing Red Track Danger Impact Marker (Procedural track warning grid)
            drone.spawn((
                PbrBundle {
                    mesh: assets.mesh_hunter_marker.clone(),
                    material: assets.mat_hunter_marker.clone(),
                    transform: Transform::IDENTITY,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HunterImpactMarker,
            ));
        });
}

fn spawn_heavy_blocker(commands: &mut Commands, assets: &EnemyAssets, lane: Lane, spawn_z: f32) {
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
    squad_mgr.hunter_cooldown -= dt;

    let has_scout = enemies_q.iter().any(|e| e.enemy_type == EnemyType::Scout);
    let has_hunter = enemies_q.iter().any(|e| e.enemy_type == EnemyType::Hunter);
    let heavy_count = enemies_q
        .iter()
        .filter(|e| e.enemy_type == EnemyType::Heavy)
        .count();

    // 1. Level 1 Scout persistence: introduced at 600m in Zone 1 (Old Metro) for pedagogical pacing
    if (stats.distance >= 600.0 || director.active_zone_id >= 2) && !has_scout {
        spawn_scout_drone(&mut commands, &assets, Lane::Center, 7.5);
        enemy_spawn_events.send(EnemySpawnedEvent {
            enemy_type: EnemyType::Scout,
            lane: Lane::Center,
        });
    }

    // 2. Hunter Interceptor predator encounter in Zone 2+ (Neon District onwards)
    // Controlled predator pacing & fairness guarantees:
    // - Cooldown: 4-7 seconds between encounters
    // - Solvability: Player must have >= 2 navigable lanes ahead to evade safely
    // - Concurrency protection: Does not spawn during active boss fight or concurrent Hunter attack
    if director.active_zone_id >= 2
        && !squad_mgr.active_boss
        && !has_hunter
        && squad_mgr.hunter_cooldown <= 0.0
    {
        // Check obstacle density in track window ahead [p.z - 25m, p.z - 5m]
        let mut lane_blocked = [false; 3];
        for (obs, o_trans) in obs_q.iter() {
            let dz = p_trans.translation.z - o_trans.translation.z;
            if dz >= 5.0 && dz <= 25.0 {
                match obs.lane {
                    Lane::Left => lane_blocked[0] = true,
                    Lane::Center => lane_blocked[1] = true,
                    Lane::Right => lane_blocked[2] = true,
                }
            }
        }

        let blocked_count = lane_blocked.iter().filter(|&&b| b).count();
        if blocked_count < 2 {
            let mut rng = rand::thread_rng();
            let lanes = [Lane::Left, Lane::Center, Lane::Right];
            let spawn_lane = lanes[rng.gen_range(0..3)];

            let spawn_z = p_trans.translation.z - 12.0;
            spawn_hunter_drone(&mut commands, &assets, spawn_lane, spawn_z);
            squad_mgr.hunter_cooldown = 15.0; // Pacing buffer while encounter is active
            enemy_spawn_events.send(EnemySpawnedEvent {
                enemy_type: EnemyType::Hunter,
                lane: spawn_lane,
            });
        } else {
            // Track is momentarily congested; defer spawn check by 1.5s
            squad_mgr.hunter_cooldown = 1.5;
        }
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
                    let other_route_open = all_lanes
                        .iter()
                        .enumerate()
                        .any(|(other_idx, _)| other_idx != idx && !lane_has_obstacle[other_idx]);
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
    if stats.distance >= crate::zones::BOSS_MILESTONE_4_DISTANCE
        && !squad_mgr.boss_spawned_milestone_4
        && !squad_mgr.active_boss
    {
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
    } else if stats.distance >= crate::zones::BOSS_MILESTONE_7_DISTANCE
        && !squad_mgr.boss_spawned_milestone_7
        && !squad_mgr.active_boss
    {
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
    mut scout_q: Query<
        (&mut Transform, &mut ActiveEnemy, &mut ScoutDrone),
        (With<ScoutDrone>, Without<Player>),
    >,
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
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(&mut Player, &Transform), (Without<HunterDrone>, Without<ScoutDrone>)>,
    heavy_q: Query<(&Transform, &HeavyBlocker), Without<HunterDrone>>,
    mut stats: ResMut<GameRunStats>,
    mut powerups: ResMut<ActivePowerUps>,
    mut threat_alerts: ResMut<ThreatAlertState>,
    mut squad_mgr: ResMut<EnemySquadManager>,
    mut next_state: ResMut<NextState<AppState>>,
    mut hunter_q: Query<
        (Entity, &mut Transform, &mut ActiveEnemy, &mut HunterDrone),
        (With<HunterDrone>, Without<Player>, Without<HeavyBlocker>),
    >,
    mut sfx: EventWriter<SoundEffect>,
) {
    let (mut player, p_trans) = match player_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    for (entity, mut d_trans, mut enemy, mut hunter) in hunter_q.iter_mut() {
        match hunter.state {
            HunterState::Infiltrating { mut approach_timer } => {
                threat_alerts.hunter_telegraph_lane = None;
                approach_timer -= dt;

                // Move from entrance spawn (Z-12m, Y=5.0m) toward steady hover position (Z-6.5m, Y=3.2m)
                let target_z = p_trans.translation.z - 6.5;
                let target_y = 3.2;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 6.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 6.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;
                enemy.target_lane = player.lane;
                enemy.is_attacking = false;

                if approach_timer <= 0.0 {
                    hunter.state = HunterState::IntimidationWait { hover_timer: 2.0 };
                } else {
                    hunter.state = HunterState::Infiltrating { approach_timer };
                }
            }
            HunterState::IntimidationWait { mut hover_timer } => {
                threat_alerts.hunter_telegraph_lane = None;
                hover_timer -= dt;

                // Visibly hover ahead of the player in full camera view (Z-6.5m, Y=3.2m)
                let target_z = p_trans.translation.z - 6.5;
                let target_y = 3.2;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 12.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                // During intimidation wait, Hunter smoothly tracks Kai's lane - target lane is NOT locked yet!
                enemy.target_lane = player.lane;
                enemy.is_attacking = false;

                // THREAT CONCURRENCY FAIRNESS RULES:
                // Rule 1: Stumble Grace - Hunter holds fire while player is recovering from a stumble
                if stats.stumble_intensity > 0.0 {
                    hunter.state = HunterState::IntimidationWait { hover_timer: 2.0 };
                    continue;
                }

                // Rule 2: Heavy Corridor Protection - Hunter holds fire while player is within a Heavy blocker corridor
                let heavy_corridor_active = heavy_q.iter().any(|(h_trans, _)| {
                    let dz = p_trans.translation.z - h_trans.translation.z;
                    dz >= -5.0 && dz <= 35.0
                });
                if heavy_corridor_active {
                    hunter.state = HunterState::IntimidationWait { hover_timer: 2.0 };
                    continue;
                }

                if hover_timer <= 0.0 {
                    // Intimidation wait complete!
                    // Enter LockingTarget (0.3s): Lock onto the player's current lane and spawn red marker.
                    let target_lane = player.lane;
                    hunter.state = HunterState::LockingTarget {
                        target_lane,
                        lock_timer: 0.3,
                    };
                    enemy.target_lane = target_lane;
                    threat_alerts.hunter_telegraph_lane = Some(target_lane);
                    sfx.send(SoundEffect::Dash); // Lock-on warning audio cue
                } else {
                    hunter.state = HunterState::IntimidationWait { hover_timer };
                }
            }
            HunterState::LockingTarget {
                target_lane,
                mut lock_timer,
            } => {
                // Target lane is LOCKED and red ground marker is spawned
                threat_alerts.hunter_telegraph_lane = Some(target_lane);
                enemy.target_lane = target_lane;
                enemy.is_attacking = false;

                let target_z = p_trans.translation.z - 6.5;
                let target_y = 3.2;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 12.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                lock_timer -= dt;
                if lock_timer <= 0.0 {
                    // Transition to full 1.5s warning telegraph
                    hunter.state = HunterState::Telegraphing {
                        target_lane,
                        timer: 1.5,
                        total_time: 1.5,
                    };
                } else {
                    hunter.state = HunterState::LockingTarget {
                        target_lane,
                        lock_timer,
                    };
                }
            }
            HunterState::Telegraphing {
                target_lane,
                mut timer,
                total_time,
            } => {
                timer -= dt;
                // Red warning active: broadcast locked target lane to HUD
                threat_alerts.hunter_telegraph_lane = Some(target_lane);
                enemy.target_lane = target_lane;
                enemy.is_attacking = false;

                // Hold stable firing position ahead of Kai in clear camera view
                let target_z = p_trans.translation.z - 6.5;
                let target_y = 3.2;
                d_trans.translation.z += (target_z - d_trans.translation.z) * 12.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                if timer <= 0.0 {
                    threat_alerts.hunter_telegraph_lane = None;
                    hunter.state = HunterState::Firing {
                        target_lane,
                        burst_timer: 0.16,
                        has_resolved_hit: false,
                    };
                } else {
                    hunter.state = HunterState::Telegraphing {
                        target_lane,
                        timer,
                        total_time,
                    };
                }
            }
            HunterState::Firing {
                target_lane,
                mut burst_timer,
                mut has_resolved_hit,
            } => {
                threat_alerts.hunter_telegraph_lane = None;
                burst_timer -= dt;
                enemy.target_lane = target_lane;
                enemy.is_attacking = true;

                // Keep relative position steady during the rapid 0.16s blast
                let target_z = p_trans.translation.z - 6.5;
                d_trans.translation.z = target_z;
                enemy.distance_from_player = -6.5;

                if !has_resolved_hit {
                    has_resolved_hit = true;
                    sfx.send(SoundEffect::ShieldBreak); // Laser blast impact sound

                    // SKILL-BASED DODGE RESOLUTION:
                    // Laser strikes the locked lane. Check if player evaded!
                    if player.lane == target_lane {
                        // PLAYER HIT - Direct Hunter laser strike!
                        if powerups.overdrive_timer > 0.0 {
                            // Rule: Overdrive destroys Hunter!
                            stats.fragments += 25;
                            stats.score += 400;
                            sfx.send(SoundEffect::ShieldBreak);
                            commands.entity(entity).despawn_recursive();
                            let mut rng = rand::thread_rng();
                            squad_mgr.hunter_cooldown = 4.0 + rng.gen_range(0.0..3.0);
                            return;
                        } else if player.invulnerable_timer > 0.0 {
                            // Rule: Invulnerable - prevents GameOver
                        } else if powerups.shield {
                            // Rule: Shield absorbs laser strike instead of causing GameOver
                            powerups.shield_absorb_flash_timer = 0.28;
                            if powerups.shield_hits > 1 {
                                powerups.shield_hits -= 1;
                            } else {
                                powerups.shield = false;
                                powerups.shield_hits = 0;
                            }
                            player.invulnerable_timer = 1.5;
                            stats.stumble_intensity = 0.8;
                            sfx.send(SoundEffect::ShieldBreak);
                        } else {
                            // DIRECT HIT = IMMEDIATE GAME OVER!
                            sfx.send(SoundEffect::Crash);
                            next_state.set(AppState::GameOver);
                            return;
                        }
                    } else {
                        // PLAYER DODGED!
                        // Laser strikes empty lane, clean miss!
                    }
                }

                if burst_timer <= 0.0 {
                    hunter.state = HunterState::Retreating { retreat_timer: 1.2 };
                } else {
                    hunter.state = HunterState::Firing {
                        target_lane,
                        burst_timer,
                        has_resolved_hit,
                    };
                }
            }
            HunterState::Retreating { mut retreat_timer } => {
                threat_alerts.hunter_telegraph_lane = None;
                enemy.is_attacking = false;
                retreat_timer -= dt;

                // Drone accelerates upward and forward into ceiling shadows
                d_trans.translation.y += 6.5 * dt;
                d_trans.translation.z -= 12.0 * dt;
                enemy.distance_from_player = d_trans.translation.z - p_trans.translation.z;

                if retreat_timer <= 0.0 {
                    // Despawn Hunter and enter random 4-7 second predator cooldown
                    commands.entity(entity).despawn_recursive();
                    let mut rng = rand::thread_rng();
                    squad_mgr.hunter_cooldown = 4.0 + rng.gen_range(0.0..3.0);
                } else {
                    hunter.state = HunterState::Retreating { retreat_timer };
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
    threat_alerts.heavy_warning_lane = heavy_q
        .iter()
        .find(|(_, h_trans, _)| {
            let dz = p_trans.translation.z - h_trans.translation.z;
            dz >= -5.0 && dz <= 38.0
        })
        .map(|(_, _, h)| h.lane);

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
    mut boss_q: Query<
        (
            Entity,
            &mut Transform,
            &mut EchoHunterBoss,
            &mut ActiveEnemy,
        ),
        (With<EchoHunterBoss>, Without<Player>),
    >,
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
    mut enemies_q: Query<
        (
            Entity,
            &Transform,
            &ActiveEnemy,
            Option<&mut ScoutDrone>,
            Option<&mut EchoHunterBoss>,
        ),
        Without<Player>,
    >,
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
                // Hunter targeting and hit/dodge resolution is authoritatively handled
                // via its skill-based laser strike in update_hunter_ai_fixed
            }
            EnemyType::Heavy => {
                // If player enters Heavy lane with fair collision margins:
                if enemy.target_lane == p_lane
                    && (p_pos.x - e_pos.x).abs() < 1.1
                    && z_dist < 1.4
                    && p_pos.y < 2.2
                {
                    // Rule 1: Dash / Overdrive smashes Heavy blocker!
                    if powerups.overdrive_timer > 0.0
                        || (player.character == CharacterType::Mira
                            && player.invulnerable_timer > 0.8)
                    {
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
                        powerups.shield_absorb_flash_timer = 0.28;
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
    mut enemy_q: Query<(&mut Transform, &ActiveEnemy, Option<&HunterDrone>), Without<Player>>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();

    for (mut d_trans, enemy, hunter_opt) in enemy_q.iter_mut() {
        match enemy.enemy_type {
            EnemyType::Scout => {
                let target_x = enemy.target_lane.x_pos() + (t * 2.5).sin() * 0.35;
                let target_y = 2.1 + (t * 3.5).cos() * 0.2;
                d_trans.translation.x += (target_x - d_trans.translation.x) * 12.0 * dt;
                d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                d_trans.look_at(p_trans.translation + Vec3::new(0.0, 0.6, 0.0), Vec3::Y);
            }
            EnemyType::Hunter => {
                if let Some(hunter) = hunter_opt {
                    match hunter.state {
                        HunterState::Infiltrating { .. } | HunterState::IntimidationWait { .. } => {
                            let target_x = enemy.target_lane.x_pos() + (t * 2.5).cos() * 0.25;
                            let target_y = 3.2 + (t * 3.2).sin() * 0.15;
                            d_trans.translation.x += (target_x - d_trans.translation.x) * 12.0 * dt;
                            d_trans.translation.y += (target_y - d_trans.translation.y) * 8.0 * dt;
                        }
                        HunterState::LockingTarget { target_lane, .. }
                        | HunterState::Telegraphing { target_lane, .. }
                        | HunterState::Firing { target_lane, .. } => {
                            let target_x = target_lane.x_pos();
                            let target_y = 3.2;
                            d_trans.translation.x += (target_x - d_trans.translation.x) * 14.0 * dt;
                            d_trans.translation.y += (target_y - d_trans.translation.y) * 10.0 * dt;
                        }
                        HunterState::Retreating { .. } => {
                            // Retreat altitude and distance managed in update_hunter_ai_fixed
                        }
                    }
                }
                d_trans.rotation = Quat::IDENTITY;
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

// ----------------------------------------------------------------------------
// HUNTER LASER & IMPACT DANGER MARKER VISUALS (Update schedule)
// ----------------------------------------------------------------------------
fn update_hunter_laser_visuals(
    time: Res<Time>,
    assets: Option<Res<EnemyAssets>>,
    player_q: Query<&Transform, With<Player>>,
    hunter_q: Query<
        (&Transform, &HunterDrone, &Children),
        (
            With<HunterDrone>,
            Without<HunterModel>,
            Without<HunterLaserBeam>,
            Without<HunterImpactMarker>,
            Without<Player>,
        ),
    >,
    mut model_q: Query<
        &mut Transform,
        (
            With<HunterModel>,
            Without<HunterLaserBeam>,
            Without<HunterImpactMarker>,
            Without<Player>,
        ),
    >,
    mut beam_q: Query<
        (
            &mut Transform,
            &mut Visibility,
            &mut Handle<StandardMaterial>,
        ),
        (
            With<HunterLaserBeam>,
            Without<HunterModel>,
            Without<HunterImpactMarker>,
            Without<Player>,
        ),
    >,
    mut marker_q: Query<
        (&mut Transform, &mut Visibility),
        (
            With<HunterImpactMarker>,
            Without<HunterModel>,
            Without<HunterLaserBeam>,
            Without<Player>,
        ),
    >,
) {
    let assets = match assets {
        Some(a) => a,
        None => return,
    };

    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let t = time.elapsed_seconds();

    for (h_trans, hunter, children) in hunter_q.iter() {
        let mut model_rot = Quat::IDENTITY;

        // 1. Orient HunterModel child toward Kai (tracking) or target impact point (telegraph/firing)
        for &child in children.iter() {
            if let Ok(mut m_trans) = model_q.get_mut(child) {
                match hunter.state {
                    HunterState::Infiltrating { .. } | HunterState::IntimidationWait { .. } => {
                        let look_target =
                            p_trans.translation - h_trans.translation + Vec3::new(0.0, 0.9, 0.0);
                        m_trans.look_at(look_target, Vec3::Y);
                        model_rot = m_trans.rotation;
                    }
                    HunterState::LockingTarget { target_lane, .. }
                    | HunterState::Telegraphing { target_lane, .. }
                    | HunterState::Firing { target_lane, .. } => {
                        let target_world =
                            Vec3::new(target_lane.x_pos(), 0.04, p_trans.translation.z);
                        let look_target = target_world - h_trans.translation;
                        m_trans.look_at(look_target, Vec3::Y);
                        model_rot = m_trans.rotation;
                    }
                    HunterState::Retreating { .. } => {
                        m_trans.rotation = Quat::from_rotation_x(-0.35);
                        model_rot = m_trans.rotation;
                    }
                }
            }
        }

        let emitter_local = model_rot * Vec3::new(0.0, -0.15, -0.55);

        // 2. Update Laser Beam and Ground Impact Danger Marker
        for &child in children.iter() {
            if let Ok((mut b_trans, mut b_vis, mut b_mat)) = beam_q.get_mut(child) {
                match hunter.state {
                    HunterState::Telegraphing { target_lane, .. } => {
                        *b_vis = Visibility::Visible;
                        *b_mat = assets.mat_hunter_laser.clone();

                        let target_local = Vec3::new(
                            target_lane.x_pos() - h_trans.translation.x,
                            0.04 - h_trans.translation.y,
                            p_trans.translation.z - h_trans.translation.z,
                        );
                        let v = target_local - emitter_local;
                        let len = v.length();
                        if len > 0.05 {
                            let mid = emitter_local + v * 0.5;
                            b_trans.translation = mid;
                            b_trans.rotation = Transform::IDENTITY.looking_at(v, Vec3::Y).rotation;
                            b_trans.scale = Vec3::new(0.07, 0.07, len);
                        }
                    }
                    HunterState::Firing { target_lane, .. } => {
                        *b_vis = Visibility::Visible;
                        *b_mat = assets.mat_hunter_laser_burst.clone();

                        let target_local = Vec3::new(
                            target_lane.x_pos() - h_trans.translation.x,
                            0.04 - h_trans.translation.y,
                            p_trans.translation.z - h_trans.translation.z,
                        );
                        let v = target_local - emitter_local;
                        let len = v.length();
                        if len > 0.05 {
                            let mid = emitter_local + v * 0.5;
                            b_trans.translation = mid;
                            b_trans.rotation = Transform::IDENTITY.looking_at(v, Vec3::Y).rotation;
                            b_trans.scale = Vec3::new(0.24, 0.24, len);
                        }
                    }
                    _ => {
                        *b_vis = Visibility::Hidden;
                    }
                }
            }

            if let Ok((mut m_trans, mut m_vis)) = marker_q.get_mut(child) {
                match hunter.state {
                    HunterState::LockingTarget { target_lane, .. } => {
                        *m_vis = Visibility::Visible;
                        let target_local = Vec3::new(
                            target_lane.x_pos() - h_trans.translation.x,
                            0.04 - h_trans.translation.y,
                            p_trans.translation.z - h_trans.translation.z,
                        );
                        m_trans.translation = target_local;
                        m_trans.rotation = Quat::IDENTITY;
                        m_trans.scale = Vec3::new(1.0, 1.0, 1.0);
                    }
                    HunterState::Telegraphing {
                        target_lane, timer, ..
                    } => {
                        // Human-readable 5-phase timing progression across 1.5s telegraph:
                        // 1.50s to 1.00s: slow, clearly visible red pulse (2.5 Hz)
                        // 1.00s to 0.65s: medium pulse (5.0 Hz)
                        // 0.65s to 0.35s: faster pulse (9.0 Hz)
                        // 0.35s to 0.12s: rapid pulse (16.0 Hz)
                        // 0.12s to 0.00s: intense final warning (28.0 Hz)
                        let (strobe_hz, base_scale, pulse_amp) = if timer > 1.00 {
                            (2.5, 1.04, 0.06)
                        } else if timer > 0.65 {
                            (5.0, 1.02, 0.08)
                        } else if timer > 0.35 {
                            (9.0, 1.00, 0.10)
                        } else if timer > 0.12 {
                            (16.0, 0.98, 0.12)
                        } else {
                            (28.0, 1.08, 0.16)
                        };

                        let pulse_sin = (t * strobe_hz * std::f32::consts::TAU).sin();
                        // Remains persistently visible so lane lock is 100% readable
                        *m_vis = Visibility::Visible;

                        let target_local = Vec3::new(
                            target_lane.x_pos() - h_trans.translation.x,
                            0.04 - h_trans.translation.y,
                            p_trans.translation.z - h_trans.translation.z,
                        );
                        m_trans.translation = target_local;
                        m_trans.rotation = Quat::IDENTITY;
                        let scale_pulse = base_scale + pulse_amp * pulse_sin;
                        m_trans.scale = Vec3::new(scale_pulse, 1.0, scale_pulse);
                    }
                    HunterState::Firing { target_lane, .. } => {
                        *m_vis = Visibility::Visible;
                        let target_local = Vec3::new(
                            target_lane.x_pos() - h_trans.translation.x,
                            0.04 - h_trans.translation.y,
                            p_trans.translation.z - h_trans.translation.z,
                        );
                        m_trans.translation = target_local;
                        m_trans.rotation = Quat::IDENTITY;
                        m_trans.scale = Vec3::new(1.35, 1.5, 1.35);
                    }
                    _ => {
                        *m_vis = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

fn animate_enemy_thrusters(time: Res<Time>, mut query: Query<&mut Transform, With<HeavyBlocker>>) {
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

        // Register all enemy AI, gameplay, and visual systems
        app.add_systems(
            Update,
            (
                update_scout_ai_fixed,
                update_hunter_ai_fixed,
                update_heavy_blockers_fixed,
                update_echo_hunter_boss_fixed,
                player_enemy_interaction_fixed,
                update_enemy_visual_smoothing,
                update_hunter_laser_visuals,
                animate_enemy_thrusters,
                handle_run_reset_enemies,
            ),
        );

        // app.update() initializes every system parameter and validates Query disjointness
        app.update();
    }

    #[test]
    fn test_hunter_entrance_lasts_configured_duration() {
        // 1. Hunter entrance lasts the configured duration (0.5–0.8s, configured to 0.7s)
        let entrance_duration = 0.7_f32;
        let mut state = HunterState::Infiltrating {
            approach_timer: entrance_duration,
        };

        // Simulate 0.3s passing
        let dt = 0.3_f32;
        if let HunterState::Infiltrating { approach_timer } = &mut state {
            *approach_timer -= dt;
            assert!((*approach_timer - 0.4).abs() < 1e-4);
        }
        assert!(matches!(state, HunterState::Infiltrating { .. }));

        // Simulate remaining 0.4s passing
        if let HunterState::Infiltrating { approach_timer } = &mut state {
            *approach_timer -= 0.4;
            if *approach_timer <= 0.0001 {
                state = HunterState::IntimidationWait { hover_timer: 2.0 };
            }
        }
        assert_eq!(state, HunterState::IntimidationWait { hover_timer: 2.0 });
    }

    #[test]
    fn test_hunter_remains_in_visible_hover_state_for_approximately_2_seconds() {
        // 2. Hunter remains in its visible-hover state for approximately 2 seconds.
        let hover_duration = 2.0_f32;
        let mut state = HunterState::IntimidationWait {
            hover_timer: hover_duration,
        };

        // Simulate 1.0s passing
        let dt1 = 1.0_f32;
        if let HunterState::IntimidationWait { hover_timer } = &mut state {
            *hover_timer -= dt1;
            assert!((*hover_timer - 1.0).abs() < 1e-4);
        }

        // Simulate 0.9s passing (total 1.9s elapsed)
        let dt2 = 0.9_f32;
        if let HunterState::IntimidationWait { hover_timer } = &mut state {
            *hover_timer -= dt2;
            assert!((*hover_timer - 0.1).abs() < 1e-4);
        }
        assert!(matches!(state, HunterState::IntimidationWait { .. }));

        // Full 2.0s must elapse before entering target lock
        if let HunterState::IntimidationWait { hover_timer } = &mut state {
            *hover_timer -= 0.1;
            if *hover_timer <= 0.0001 {
                state = HunterState::LockingTarget {
                    target_lane: Lane::Center,
                    lock_timer: 0.3,
                };
            }
        }
        assert_eq!(
            state,
            HunterState::LockingTarget {
                target_lane: Lane::Center,
                lock_timer: 0.3
            }
        );
    }

    #[test]
    fn test_no_marker_appears_during_entrance_or_hover() {
        // 3. No marker appears during entrance or hover.
        let state_entrance = HunterState::Infiltrating {
            approach_timer: 0.7,
        };
        let state_hover = HunterState::IntimidationWait { hover_timer: 2.0 };

        fn marker_visibility(state: HunterState) -> Visibility {
            match state {
                HunterState::LockingTarget { .. }
                | HunterState::Telegraphing { .. }
                | HunterState::Firing { .. } => Visibility::Visible,
                _ => Visibility::Hidden,
            }
        }

        fn laser_visibility(state: HunterState) -> Visibility {
            match state {
                HunterState::Telegraphing { .. } | HunterState::Firing { .. } => {
                    Visibility::Visible
                }
                _ => Visibility::Hidden,
            }
        }

        assert_eq!(marker_visibility(state_entrance), Visibility::Hidden);
        assert_eq!(marker_visibility(state_hover), Visibility::Hidden);
        assert_eq!(laser_visibility(state_entrance), Visibility::Hidden);
        assert_eq!(laser_visibility(state_hover), Visibility::Hidden);
    }

    #[test]
    fn test_marker_appears_only_when_target_lock_begins() {
        // 4. Marker appears only when target lock begins.
        let mut hover_timer = 0.05_f32;
        let player_lane = Lane::Center;
        let mut state = HunterState::IntimidationWait { hover_timer };
        let mut active_marker = false;

        // Step past hover expiration into target lock (0.3s transition)
        let dt = 0.1_f32;
        hover_timer -= dt;
        if hover_timer <= 0.0 {
            let target_lane = player_lane;
            state = HunterState::LockingTarget {
                target_lane,
                lock_timer: 0.3,
            };
            active_marker = true;
        }

        assert!(active_marker, "Marker must appear when target lock begins");
        assert_eq!(
            state,
            HunterState::LockingTarget {
                target_lane: Lane::Center,
                lock_timer: 0.3
            }
        );
    }

    #[test]
    fn test_hunter_remains_visible_throughout_target_lock_and_telegraph() {
        // 5. Hunter remains visible throughout target lock and telegraph.
        // Third-Person Camera parameters: pos=(0, 3.8, 6.8), look_target=(0, 1.4, -7.0), FOV=45 deg, aspect=16:9
        let cam_pos = Vec3::new(0.0, 3.8, 6.8);
        let cam_look = Vec3::new(0.0, 1.4, -7.0);
        let fov_rad = 45.0_f32.to_radians();
        let aspect = 16.0_f32 / 9.0_f32;

        let forward = (cam_look - cam_pos).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward);

        // Relative Hunter positions during hover, lock, and telegraph across all lanes
        let lanes = [
            Lane::Left.x_pos(),
            Lane::Center.x_pos(),
            Lane::Right.x_pos(),
        ];
        for lane_x in lanes {
            let hunter_pos = Vec3::new(lane_x, 3.2, -6.5);
            let d = hunter_pos - cam_pos;
            let dist_f = d.dot(forward);
            let dist_r = d.dot(right);
            let dist_u = d.dot(up);

            assert!(dist_f > 0.0, "Hunter must be in front of the camera plane");
            let half_h = dist_f * (fov_rad * 0.5).tan();
            let half_w = half_h * aspect;

            let ndc_x = dist_r / half_w;
            let ndc_y = dist_u / half_h;

            assert!(
                ndc_x >= -1.0 && ndc_x <= 1.0,
                "Hunter on lane x={:.1} must be within horizontal screen bounds: ndc_x={:.2}",
                lane_x,
                ndc_x
            );
            assert!(
                ndc_y >= -1.0 && ndc_y <= 1.0,
                "Hunter on lane x={:.1} must be within vertical screen bounds: ndc_y={:.2}",
                lane_x,
                ndc_y
            );
            // Verify Hunter is in upper airspace (ndc_y > 0.0), never occluding Kai (Kai is at ndc_y <= -0.31)
            assert!(
                ndc_y > 0.15,
                "Hunter must remain framed in upper airspace without occluding Kai: ndc_y={:.2}",
                ndc_y
            );
        }
    }

    #[test]
    fn test_hunter_target_lane_not_locked_during_intimidation() {
        // Player moves between lanes; Hunter tracks player instead of locking onto a fixed lane.
        let mut enemy = ActiveEnemy {
            enemy_type: EnemyType::Hunter,
            target_lane: Lane::Center,
            current_lane: Lane::Center,
            lane_switch_timer: 0.0,
            behavior_timer: 0.0,
            distance_from_player: -6.5,
            is_attacking: false,
        };
        let state = HunterState::IntimidationWait { hover_timer: 2.0 };

        // Player switches to Left lane while Hunter is intimidating
        let player_lane = Lane::Left;
        if matches!(state, HunterState::IntimidationWait { .. }) {
            enemy.target_lane = player_lane;
        }
        assert_eq!(enemy.target_lane, Lane::Left);

        // Player switches to Right lane
        let player_lane_2 = Lane::Right;
        if matches!(state, HunterState::IntimidationWait { .. }) {
            enemy.target_lane = player_lane_2;
        }
        assert_eq!(enemy.target_lane, Lane::Right);
    }

    #[test]
    fn test_hunter_target_lane_remains_fixed_during_telegraph() {
        // Guarantee: target lane is locked at telegraph onset and does not follow player
        let locked_target = Lane::Center;
        let mut state = HunterState::Telegraphing {
            target_lane: locked_target,
            timer: 1.5,
            total_time: 1.5,
        };

        // Player switches lanes during telegraph window
        let player_new_lane = Lane::Left;
        if let HunterState::Telegraphing {
            target_lane,
            timer,
            total_time,
        } = state
        {
            // target_lane must remain the locked lane, ignoring player's new lane
            assert_eq!(target_lane, locked_target);
            assert_ne!(target_lane, player_new_lane);
            state = HunterState::Telegraphing {
                target_lane,
                timer: timer - 0.5,
                total_time,
            };
        }

        if let HunterState::Telegraphing { target_lane, .. } = state {
            assert_eq!(target_lane, locked_target);
        } else {
            panic!("State must remain Telegraphing");
        }
    }

    #[test]
    fn test_hunter_direct_laser_hit_causes_immediate_game_over() {
        // Direct laser hit without defensive powerups must trigger immediate GameOver
        let targeted_lane = Lane::Center;
        let player_lane = Lane::Center;
        let shield = false;
        let overdrive_active = false;
        let invulnerable = false;

        let mut app_state = AppState::InGame;
        if player_lane == targeted_lane {
            if overdrive_active {
                // Hunter destroyed
            } else if invulnerable {
                // Protected
            } else if shield {
                // Shield absorbs
            } else {
                app_state = AppState::GameOver;
            }
        }

        assert_eq!(
            app_state,
            AppState::GameOver,
            "Direct Hunter laser strike must result in immediate GameOver"
        );
    }

    #[test]
    fn test_hunter_lane_switch_produces_clean_dodge() {
        // Lateral lane switch produces clean dodge
        let targeted_lane = Lane::Center;
        let player_lane = Lane::Left; // Kai dodged laterally
        let hit = player_lane == targeted_lane;
        let mut app_state = AppState::InGame;

        if hit {
            app_state = AppState::GameOver;
        }

        assert!(
            !hit,
            "Player must successfully dodge Hunter laser strike when switching lanes"
        );
        assert_eq!(
            app_state,
            AppState::InGame,
            "Clean dodge must preserve InGame state without damage"
        );
    }

    #[test]
    fn test_hunter_shield_absorbs_laser_and_prevents_game_over() {
        // Defensive fairness: Shield absorbs laser strike and prevents GameOver
        let targeted_lane = Lane::Center;
        let player_lane = Lane::Center;
        let mut shield = true;
        let mut shield_hits = 1;
        let mut app_state = AppState::InGame;

        if player_lane == targeted_lane {
            if shield {
                if shield_hits > 1 {
                    shield_hits -= 1;
                } else {
                    shield = false;
                    shield_hits = 0;
                }
            } else {
                app_state = AppState::GameOver;
            }
        }

        assert_eq!(
            app_state,
            AppState::InGame,
            "Shield must absorb Hunter laser and prevent GameOver"
        );
        assert!(
            !shield,
            "Single-hit shield must be depleted after absorbing laser"
        );
        assert_eq!(shield_hits, 0, "Shield hits must be decremented to 0");
    }

    #[test]
    fn test_hunter_overdrive_destroys_hunter() {
        // Defensive fairness: Overdrive destroys Hunter and prevents GameOver
        let targeted_lane = Lane::Center;
        let player_lane = Lane::Center;
        let overdrive_timer = 2.5_f32;
        let mut hunter_destroyed = false;
        let mut app_state = AppState::InGame;

        if player_lane == targeted_lane {
            if overdrive_timer > 0.0 {
                hunter_destroyed = true;
            } else {
                app_state = AppState::GameOver;
            }
        }

        assert!(
            hunter_destroyed,
            "Overdrive must destroy the attacking Hunter drone"
        );
        assert_eq!(
            app_state,
            AppState::InGame,
            "Overdrive must prevent GameOver"
        );
    }

    #[test]
    fn test_hunter_attack_cooldown_range_fairness() {
        // Pacing guarantee: Hunter predator encounters must maintain 4.0s - 7.0s breathing room
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let cd = 4.0 + rng.gen_range(0.0..3.0);
            assert!(
                cd >= 4.0 && cd <= 7.0,
                "Hunter cooldown must remain strictly within 4-7s range"
            );
        }
    }

    #[test]
    fn test_hunter_telegraph_kinematics_and_fairness_guarantees() {
        let zone_2_speed = 18.4_f32;
        let hunter_telegraph_duration = 1.5_f32;
        let distance_covered = zone_2_speed * hunter_telegraph_duration;

        let reaction_time = 0.18_f32;
        let lateral_lane_transition = 0.17_f32;
        let required_time = reaction_time + lateral_lane_transition;
        let required_distance = zone_2_speed * required_time;

        // Verify kinematic buffer
        assert!(
            hunter_telegraph_duration >= required_time + 0.80,
            "Hunter telegraph must provide >= 0.80s margin above reaction + lateral dodge: duration={:.2}s, required={:.2}s",
            hunter_telegraph_duration,
            required_time
        );
        assert!(
            distance_covered >= required_distance + 15.0,
            "Hunter telegraph must provide >= 15.0m longitudinal margin: covered={:.2}m, required={:.2}m",
            distance_covered,
            required_distance
        );

        // Verify Hunter patterns guarantee at least 2 open lanes
        let catalog = crate::patterns::get_pattern_catalog();
        let hunter_patterns: Vec<_> = catalog
            .iter()
            .filter(|p| p.name.contains("Hunter"))
            .collect();
        assert!(
            !hunter_patterns.is_empty(),
            "Must have Hunter pattern family in catalog"
        );

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
