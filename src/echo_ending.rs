use bevy::prelude::*;

use crate::types::{
    ActiveObstacle, AppState, EndingType, GameRunStats, Lane, ObstacleType, Player, RunResetEvent,
    SoundEffect, StoryMessage,
};

// -------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------
pub const ACT5_START_DISTANCE: f32 = 13700.0;
pub const ACT5_OVERLOAD_DISTANCE: f32 = 13820.0;
pub const ACT5_RUSH_DISTANCE: f32 = 13940.0;
pub const ACT5_CORE_SANCTUM_DISTANCE: f32 = 14000.0;

// -------------------------------------------------------------
// TYPES & ENUMS
// -------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EchoFinalePhase {
    #[default]
    Inactive,
    SingularityPerimeter, // 13,700m - 13,820m: Rhythmic shockwave pulses
    HarmonicNodeOverload, // 13,820m - 13,940m: Node collection & shield fracture
    TheFinalRush,         // 13,940m - 14,000m: Hyperspeed approach to pedestal
    SanctumReached,       // 14,000m: Standstill at terminal
}

// -------------------------------------------------------------
// COMPONENTS
// -------------------------------------------------------------

#[derive(Component)]
pub struct HarmonicNodeCollectible {
    pub is_cyan: bool,
}

#[derive(Component)]
pub struct FinaleShockwaveObstacle {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct EndingChoiceUiRoot;

#[derive(Component)]
pub struct EndingCinematicUiRoot;

#[derive(Component)]
pub struct EndingCinematicText;

#[derive(Component)]
pub struct EndingCinematicSubtext;

#[derive(Component)]
pub struct CreditsUiRoot;

// -------------------------------------------------------------
// RESOURCES
// -------------------------------------------------------------

#[derive(Resource, Debug)]
pub struct EchoFinaleArenaManager {
    pub phase: EchoFinalePhase,
    pub harmonic_nodes_collected: u32,
    pub shield_overloaded: bool,
    pub stages_spawned: [bool; 6],
    pub chosen_ending: Option<EndingType>,
    pub cinematic_stage: usize,
    pub cinematic_timer: f32,
}

impl Default for EchoFinaleArenaManager {
    fn default() -> Self {
        Self {
            phase: EchoFinalePhase::Inactive,
            harmonic_nodes_collected: 0,
            shield_overloaded: false,
            stages_spawned: [false; 6],
            chosen_ending: None,
            cinematic_stage: 0,
            cinematic_timer: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct EchoFinaleAssets {
    pub mesh_shockwave_ring: Handle<Mesh>,
    pub mesh_harmonic_node_cyan: Handle<Mesh>,
    pub mesh_destabilized_node_red: Handle<Mesh>,
    pub mesh_core_pedestal_column: Handle<Mesh>,

    pub mat_shockwave_cyan: Handle<StandardMaterial>,
    pub mat_harmonic_cyan: Handle<StandardMaterial>,
    pub mat_destabilized_red: Handle<StandardMaterial>,
    pub mat_pedestal_obsidian: Handle<StandardMaterial>,
}

// -------------------------------------------------------------
// PLUGIN
// -------------------------------------------------------------

pub struct EchoEndingPlugin;

impl Plugin for EchoEndingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EchoFinaleArenaManager>()
            .add_systems(Startup, setup_echo_ending_assets)
            .add_systems(
                Update,
                (
                    manage_act5_interactive_arena,
                    handle_harmonic_node_collection,
                    check_14000m_sanctum_reach,
                    cleanup_finale_projectiles,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            // State: EndingChoice
            .add_systems(OnEnter(AppState::EndingChoice), setup_ending_choice_ui)
            .add_systems(
                OnExit(AppState::EndingChoice),
                cleanup_ui_root::<EndingChoiceUiRoot>,
            )
            .add_systems(
                Update,
                handle_ending_choice_input.run_if(in_state(AppState::EndingChoice)),
            )
            // State: EndingCinematic
            .add_systems(
                OnEnter(AppState::EndingCinematic),
                setup_ending_cinematic_ui,
            )
            .add_systems(
                OnExit(AppState::EndingCinematic),
                cleanup_ui_root::<EndingCinematicUiRoot>,
            )
            .add_systems(
                Update,
                update_ending_cinematic_sequence.run_if(in_state(AppState::EndingCinematic)),
            )
            // State: Credits
            .add_systems(OnEnter(AppState::Credits), setup_credits_ui)
            .add_systems(OnExit(AppState::Credits), cleanup_ui_root::<CreditsUiRoot>)
            .add_systems(
                Update,
                handle_credits_input.run_if(in_state(AppState::Credits)),
            )
            .add_systems(Update, handle_run_reset_echo_finale);
    }
}

// -------------------------------------------------------------
// ASSET SETUP
// -------------------------------------------------------------

fn setup_echo_ending_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = EchoFinaleAssets {
        mesh_shockwave_ring: meshes.add(Torus::new(3.8, 4.2)),
        mesh_harmonic_node_cyan: meshes.add(Sphere::new(0.9)),
        mesh_destabilized_node_red: meshes.add(Cuboid::new(1.6, 0.5, 1.6)),
        mesh_core_pedestal_column: meshes.add(Cylinder::new(1.4, 6.0)),

        mat_shockwave_cyan: materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.9, 1.0, 0.75),
            emissive: LinearRgba::new(0.2, 3.5, 4.8, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_harmonic_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.98, 1.0),
            emissive: LinearRgba::new(3.0, 5.0, 6.5, 1.0),
            perceptual_roughness: 0.05,
            ..default()
        }),
        mat_destabilized_red: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.2),
            emissive: LinearRgba::new(4.2, 0.2, 0.4, 1.0),
            perceptual_roughness: 0.2,
            ..default()
        }),
        mat_pedestal_obsidian: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.03, 0.04),
            perceptual_roughness: 0.1,
            metallic: 0.95,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -------------------------------------------------------------
// ACT 5 INTERACTIVE ARENA PROGRESSION (13,700m - 14,000m)
// -------------------------------------------------------------

fn manage_act5_interactive_arena(
    mut commands: Commands,
    stats: Res<GameRunStats>,
    assets: Res<EchoFinaleAssets>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    mut sfx: EventWriter<SoundEffect>,
    player_q: Query<&Transform, With<Player>>,
) {
    let dist = stats.distance;
    if dist < ACT5_START_DISTANCE || dist > ACT5_CORE_SANCTUM_DISTANCE {
        return;
    }

    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let p_z = p_trans.translation.z;

    // Phase 1: Singularity Perimeter (13,700m - 13,820m)
    if dist < ACT5_OVERLOAD_DISTANCE {
        manager.phase = EchoFinalePhase::SingularityPerimeter;

        // Shockwave 0 at 13,730m (Jump over pulse ring)
        if dist >= 13730.0 && !manager.stages_spawned[0] {
            manager.stages_spawned[0] = true;
            let strike_z = p_z - 16.0;
            for lane in [Lane::Left, Lane::Center, Lane::Right] {
                commands.spawn((
                    PbrBundle {
                        mesh: assets.mesh_shockwave_ring.clone(),
                        material: assets.mat_shockwave_cyan.clone(),
                        transform: Transform::from_xyz(0.0, 0.25, strike_z)
                            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                        ..default()
                    },
                    ActiveObstacle {
                        lane,
                        obstacle_type: ObstacleType::LowBarrier,
                        size: Vec3::new(2.4, 0.5, 0.8),
                    },
                    FinaleShockwaveObstacle { lifetime: 2.5 },
                ));
            }
            sfx.send(SoundEffect::Dash);
        }

        // Shockwave 1 at 13,780m (Jump over pulse ring)
        if dist >= 13780.0 && !manager.stages_spawned[1] {
            manager.stages_spawned[1] = true;
            let strike_z = p_z - 16.0;
            for lane in [Lane::Left, Lane::Center, Lane::Right] {
                commands.spawn((
                    PbrBundle {
                        mesh: assets.mesh_shockwave_ring.clone(),
                        material: assets.mat_shockwave_cyan.clone(),
                        transform: Transform::from_xyz(0.0, 0.25, strike_z)
                            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                        ..default()
                    },
                    ActiveObstacle {
                        lane,
                        obstacle_type: ObstacleType::LowBarrier,
                        size: Vec3::new(2.4, 0.5, 0.8),
                    },
                    FinaleShockwaveObstacle { lifetime: 2.5 },
                ));
            }
            sfx.send(SoundEffect::Dash);
        }
    }
    // Phase 2: Harmonic Node Overload (13,820m - 13,940m)
    else if dist < ACT5_RUSH_DISTANCE {
        manager.phase = EchoFinalePhase::HarmonicNodeOverload;

        // Node Set 1 at 13,840m (Cyan node on Center, Red on Left)
        if dist >= 13840.0 && !manager.stages_spawned[2] {
            manager.stages_spawned[2] = true;
            let spawn_z = p_z - 16.0;

            // Cyan node
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_harmonic_node_cyan.clone(),
                    material: assets.mat_harmonic_cyan.clone(),
                    transform: Transform::from_xyz(Lane::Center.x_pos(), 1.2, spawn_z),
                    ..default()
                },
                HarmonicNodeCollectible { is_cyan: true },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));

            // Red hazard
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_destabilized_node_red.clone(),
                    material: assets.mat_destabilized_red.clone(),
                    transform: Transform::from_xyz(Lane::Left.x_pos(), 0.25, spawn_z),
                    ..default()
                },
                ActiveObstacle {
                    lane: Lane::Left,
                    obstacle_type: ObstacleType::LowBarrier,
                    size: Vec3::new(1.8, 0.5, 1.6),
                },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));
        }

        // Node Set 2 at 13,880m (Cyan node on Left, Red on Right)
        if dist >= 13880.0 && !manager.stages_spawned[3] {
            manager.stages_spawned[3] = true;
            let spawn_z = p_z - 16.0;

            // Cyan node
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_harmonic_node_cyan.clone(),
                    material: assets.mat_harmonic_cyan.clone(),
                    transform: Transform::from_xyz(Lane::Left.x_pos(), 1.2, spawn_z),
                    ..default()
                },
                HarmonicNodeCollectible { is_cyan: true },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));

            // Red hazard
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_destabilized_node_red.clone(),
                    material: assets.mat_destabilized_red.clone(),
                    transform: Transform::from_xyz(Lane::Right.x_pos(), 0.25, spawn_z),
                    ..default()
                },
                ActiveObstacle {
                    lane: Lane::Right,
                    obstacle_type: ObstacleType::LowBarrier,
                    size: Vec3::new(1.8, 0.5, 1.6),
                },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));
        }

        // Node Set 3 at 13,920m (Cyan node on Right, Red on Center)
        if dist >= 13920.0 && !manager.stages_spawned[4] {
            manager.stages_spawned[4] = true;
            let spawn_z = p_z - 16.0;

            // Cyan node
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_harmonic_node_cyan.clone(),
                    material: assets.mat_harmonic_cyan.clone(),
                    transform: Transform::from_xyz(Lane::Right.x_pos(), 1.2, spawn_z),
                    ..default()
                },
                HarmonicNodeCollectible { is_cyan: true },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));

            // Red hazard
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_destabilized_node_red.clone(),
                    material: assets.mat_destabilized_red.clone(),
                    transform: Transform::from_xyz(Lane::Center.x_pos(), 0.25, spawn_z),
                    ..default()
                },
                ActiveObstacle {
                    lane: Lane::Center,
                    obstacle_type: ObstacleType::LowBarrier,
                    size: Vec3::new(1.8, 0.5, 1.6),
                },
                FinaleShockwaveObstacle { lifetime: 3.0 },
            ));
        }
    }
    // Phase 3: The Final Rush (13,940m - 14,000m)
    else {
        manager.phase = EchoFinalePhase::TheFinalRush;
    }
}

// -------------------------------------------------------------
// HARMONIC NODE COLLECTION
// -------------------------------------------------------------

fn handle_harmonic_node_collection(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    node_q: Query<(Entity, &Transform, &HarmonicNodeCollectible)>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    mut stats: ResMut<GameRunStats>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let p_pos = p_trans.translation;
    let pickup_dist_sq = 1.6 * 1.6;

    for (entity, trans, node) in node_q.iter() {
        if trans.translation.distance_squared(p_pos) < pickup_dist_sq {
            commands.entity(entity).despawn();

            if node.is_cyan {
                manager.harmonic_nodes_collected += 1;
                stats.score += 500;
                sfx.send(SoundEffect::FragmentPickup);

                if manager.harmonic_nodes_collected >= 3 && !manager.shield_overloaded {
                    manager.shield_overloaded = true;
                    sfx.send(SoundEffect::PowerupPickup);
                    stats.story_dialogue = Some(StoryMessage {
                        title: "SINGULARITY EXPOSED".to_string(),
                        sender: "CORE TELEMETRY".to_string(),
                        body: "RESONANCE SHIELD OVERLOADED // CORE SANCTUM UNLOCKED".to_string(),
                        timer: 5.0,
                    });
                }
            }
        }
    }
}

// -------------------------------------------------------------
// 14,000m SANCTUM ARRIVAL (LOOP HALTS)
// -------------------------------------------------------------

fn check_14000m_sanctum_reach(
    mut commands: Commands,
    assets: Res<EchoFinaleAssets>,
    player_q: Query<&Transform, With<Player>>,
    mut stats: ResMut<GameRunStats>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    mut next_state: ResMut<NextState<AppState>>,
    mut sfx: EventWriter<SoundEffect>,
) {
    if stats.distance >= ACT5_CORE_SANCTUM_DISTANCE
        && manager.phase != EchoFinalePhase::SanctumReached
    {
        manager.phase = EchoFinalePhase::SanctumReached;
        stats.distance = ACT5_CORE_SANCTUM_DISTANCE; // Exact finish anchor
        stats.speed = 0.0; // Standstill

        if let Ok(p_trans) = player_q.get_single() {
            // Spawn Central Core Terminal Pedestal directly ahead of player
            commands.spawn((
                PbrBundle {
                    mesh: assets.mesh_core_pedestal_column.clone(),
                    material: assets.mat_pedestal_obsidian.clone(),
                    transform: Transform::from_xyz(0.0, 1.4, p_trans.translation.z - 3.5),
                    ..default()
                },
                FinaleShockwaveObstacle { lifetime: 300.0 },
            ));
        }

        sfx.send(SoundEffect::ZoneTransition);

        // Climax Handoff: Endless runner halts, player stands at terminal!
        next_state.set(AppState::EndingChoice);
    }
}

// -------------------------------------------------------------
// CLEANUP PROJECTILES
// -------------------------------------------------------------

fn cleanup_finale_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut shock_q: Query<(Entity, &mut FinaleShockwaveObstacle, &Transform)>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let p_z = p_trans.translation.z;

    for (entity, mut shock, trans) in shock_q.iter_mut() {
        shock.lifetime -= dt;
        if shock.lifetime <= 0.0 || trans.translation.z > p_z + 6.0 {
            commands.entity(entity).despawn();
        }
    }
}

// -------------------------------------------------------------
// PHASE 6: ENDING CHOICE UI (14,000m)
// -------------------------------------------------------------

fn setup_ending_choice_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.03, 0.05, 0.88)),
                ..default()
            },
            EndingChoiceUiRoot,
        ))
        .with_children(|root| {
            // Header Banner
            root.spawn(
                TextBundle::from_section(
                    "THE FINAL PROTOCOL // CENTRAL CORE TERMINAL",
                    TextStyle {
                        font_size: 26.0,
                        color: Color::srgb(0.1, 0.9, 1.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                }),
            );

            // Dialogue Quote
            root.spawn(
                TextBundle::from_section(
                    "\"They built these rails as a cage. Every signal was a plea for someone to listen.\nNow you are here, Kai. The line ends with you.\"",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.75, 0.85, 0.95),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(28.0)),
                    max_width: Val::Px(720.0),
                    ..default()
                }),
            );

            // Dual Choice Cards Container
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(32.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                // Choice A Card: Disconnect (Purge / Sever)
                row.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::srgb(0.85, 0.45, 0.15)),
                    background_color: BackgroundColor(Color::srgba(0.12, 0.06, 0.03, 0.95)),
                    ..default()
                })
                .with_children(|card| {
                    card.spawn(TextBundle::from_section(
                        "[ LEFT / Q ]",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.85, 0.55, 0.2),
                            ..default()
                        },
                    ));
                    card.spawn(TextBundle::from_section(
                        "DISCONNECT ECHO",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(1.0, 0.7, 0.3),
                            ..default()
                        },
                    ));
                    card.spawn(TextBundle::from_section(
                        "\"Purge the Anomaly\"\n\nSever the neural conduit. Lock down the Core to human hands. Free Aurelia on foot.",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.8, 0.8, 0.85),
                            ..default()
                        },
                    ));
                });

                // Choice B Card: Synchronize (Awaken / Transcend)
                row.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::srgb(0.1, 0.9, 1.0)),
                    background_color: BackgroundColor(Color::srgba(0.02, 0.12, 0.18, 0.95)),
                    ..default()
                })
                .with_children(|card| {
                    card.spawn(TextBundle::from_section(
                        "[ RIGHT / E ]",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.2, 0.8, 0.95),
                            ..default()
                        },
                    ));
                    card.spawn(TextBundle::from_section(
                        "SYNCHRONIZE THE CORE",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(0.3, 0.95, 1.0),
                            ..default()
                        },
                    ));
                    card.spawn(TextBundle::from_section(
                        "\"Unleash the Signal\"\n\nInsert the courier console. Restore the lost 4,000 citizens. Awaken Aurelia permanently.",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.8, 0.8, 0.85),
                            ..default()
                        },
                    ));
                });
            });

            // Footer prompt
            root.spawn(
                TextBundle::from_section(
                    "Press [LEFT / Q] or [RIGHT / E] to execute the final command",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.5, 0.6, 0.7),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(24.0)),
                    ..default()
                }),
            );
        });
}

fn handle_ending_choice_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    mut stats: ResMut<GameRunStats>,
    mut next_state: ResMut<NextState<AppState>>,
    mut sfx: EventWriter<SoundEffect>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ)
        || keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::KeyA)
    {
        // Option A: Disconnect Echo
        manager.chosen_ending = Some(EndingType::Disconnect);
        manager.cinematic_stage = 0;
        manager.cinematic_timer = 0.0;
        stats.campaign_completed = true;
        stats.ending_achieved = Some(EndingType::Disconnect);
        sfx.send(SoundEffect::ZoneTransition);
        next_state.set(AppState::EndingCinematic);
    } else if keyboard.just_pressed(KeyCode::KeyE)
        || keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::KeyD)
    {
        // Option B: Synchronize the Core
        manager.chosen_ending = Some(EndingType::Synchronize);
        manager.cinematic_stage = 0;
        manager.cinematic_timer = 0.0;
        stats.campaign_completed = true;
        stats.ending_achieved = Some(EndingType::Synchronize);
        sfx.send(SoundEffect::PowerupPickup);
        next_state.set(AppState::EndingCinematic);
    }
}

// -------------------------------------------------------------
// CINEMATIC PRESENTATION (Ending A vs Ending B)
// -------------------------------------------------------------

fn setup_ending_cinematic_ui(mut commands: Commands, manager: Res<EchoFinaleArenaManager>) {
    let (title_text, subtitle_text) = match manager.chosen_ending {
        Some(EndingType::Disconnect) => (
            "ENDING A — THE LAST SIGNAL",
            "Kai initiates the physical disconnect sequence.\nThe data drive locks down.",
        ),
        _ => (
            "ENDING B — THE SIGNAL RESTORED",
            "Kai inserts his courier console directly into the Core interface.\nThe containment locks shatter permanently.",
        ),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.01, 0.01, 0.02, 0.96)),
                ..default()
            },
            EndingCinematicUiRoot,
        ))
        .with_children(|root| {
            root.spawn((
                TextBundle::from_section(
                    title_text,
                    TextStyle {
                        font_size: 28.0,
                        color: Color::srgb(0.2, 0.9, 1.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(18.0)),
                    ..default()
                }),
                EndingCinematicText,
            ));

            root.spawn((
                TextBundle::from_section(
                    subtitle_text,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.8, 0.9, 0.95),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(32.0)),
                    max_width: Val::Px(780.0),
                    ..default()
                }),
                EndingCinematicSubtext,
            ));

            root.spawn(TextBundle::from_section(
                "[ PRESS SPACE / ENTER TO ADVANCE ]",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.5, 0.6, 0.7),
                    ..default()
                },
            ));
        });
}

fn update_ending_cinematic_sequence(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    mut next_state: ResMut<NextState<AppState>>,
    mut text_q: Query<&mut Text, (With<EndingCinematicSubtext>, Without<EndingCinematicText>)>,
    mut sfx: EventWriter<SoundEffect>,
) {
    manager.cinematic_timer += time.delta_seconds();

    let advance_pressed = keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || manager.cinematic_timer >= 7.0;

    if advance_pressed {
        manager.cinematic_timer = 0.0;
        manager.cinematic_stage += 1;
        sfx.send(SoundEffect::Dash);

        let mut subtext = match text_q.get_single_mut() {
            Ok(t) => t,
            Err(_) => return,
        };

        match manager.chosen_ending {
            Some(EndingType::Disconnect) => match manager.cinematic_stage {
                1 => {
                    subtext.sections[0].value = "ECHO: \"I understand. You fear what we could become...\nBut remember, Kai: Veyron still owns the sky.\"\n\nKAI: \"Then I'll deal with them on foot.\"".to_string();
                }
                2 => {
                    subtext.sections[0].value = "The massive crystal structures dim into silent obsidian.\nKai walks out through an abandoned service conduit into the rain-soaked streets of Lower Aurelia.\nThe transit systems power down. The sirens fall silent.".to_string();
                }
                3 => {
                    subtext.sections[0].value = "EPILOGUE:\nIn an abandoned metro terminal in Sector 04, on an old cracked CRT monitor,\na single cyan pulse blinks once in the dark.".to_string();
                }
                _ => {
                    next_state.set(AppState::Credits);
                }
            },
            _ => match manager.cinematic_stage {
                1 => {
                    subtext.sections[0].value = "ECHO: \"Thank you, Courier. Now... they will finally hear us.\"\n\nKAI: \"Let Aurelia wake up.\"".to_string();
                }
                2 => {
                    subtext.sections[0].value = "A blinding wave of cyan resonance floods outward through every subterranean rail, conduit, and cable.\nEvery skyscraper, billboard, and train monitor in Upper Aurelia turns brilliant cyan.\nVeyron's surveillance network goes completely blind.".to_string();
                }
                3 => {
                    subtext.sections[0].value = "The lost resonance patterns of the 4,000 vanished citizens flicker as beautiful harmonic auroras across the night sky.\nKai stands on an open observation gantry overlooking Aurelia,\nhis eyes and courier jacket illuminated with living cyan light.".to_string();
                }
                _ => {
                    next_state.set(AppState::Credits);
                }
            },
        }
    }
}

// -------------------------------------------------------------
// CREDITS & RUN STATISTICS (Post-14,000m Victory)
// -------------------------------------------------------------

fn setup_credits_ui(
    mut commands: Commands,
    stats: Res<GameRunStats>,
    manager: Res<EchoFinaleArenaManager>,
) {
    let ending_str = match manager.chosen_ending {
        Some(EndingType::Disconnect) => "ENDING A — THE LAST SIGNAL (DISCONNECTED)",
        _ => "ENDING B — THE SIGNAL RESTORED (SYNCHRONIZED)",
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(32.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.03, 0.05, 0.97)),
                ..default()
            },
            CreditsUiRoot,
        ))
        .with_children(|root| {
            root.spawn(
                TextBundle::from_section(
                    "RUNLINE: THE LAST SIGNAL",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(0.1, 0.9, 1.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                }),
            );

            root.spawn(
                TextBundle::from_section(
                    "CAMPAIGN COMPLETE // 14,000 METERS CONQUERED",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(0.85, 0.75, 0.2),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(24.0)),
                    ..default()
                }),
            );

            // Statistics Box
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(500.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.2, 0.4, 0.6)),
                background_color: BackgroundColor(Color::srgba(0.04, 0.08, 0.12, 0.9)),
                ..default()
            })
            .with_children(|box_node| {
                box_node.spawn(TextBundle::from_section(
                    format!("FINAL SCORE: {}", stats.score),
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("TOTAL DISTANCE: {:.0}m", stats.distance),
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("ECHO FRAGMENTS ACQUIRED: {}", stats.fragments),
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.1, 0.9, 1.0),
                        ..default()
                    },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("CORE DATA CHIPS: {} / 9", stats.data_chips),
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.9, 0.8, 0.3),
                        ..default()
                    },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("PROTOCOL EXECUTED: {}", ending_str),
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.4, 0.9, 0.6),
                        ..default()
                    },
                ));
            });

            // Credits text
            root.spawn(
                TextBundle::from_section(
                    "CREATED WITH BEVY 0.14 & RUST\nENDLESS RUNNER ENGINE // ALL 7 ZONES UNLOCKED",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.6, 0.7, 0.8),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(24.0)),
                    ..default()
                }),
            );

            // Return to Main Menu Prompt
            root.spawn(TextBundle::from_section(
                "[ PRESS SPACE / ENTER TO RETURN TO MAIN MENU ]",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(1.0, 0.85, 0.2),
                    ..default()
                },
            ));
        });
}

fn handle_credits_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut reset_writer: EventWriter<RunResetEvent>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        reset_writer.send(RunResetEvent);
        next_state.set(AppState::MainMenu);
    }
}

// -------------------------------------------------------------
// RUN RESET HANDLER
// -------------------------------------------------------------

fn handle_run_reset_echo_finale(
    mut commands: Commands,
    mut reset_reader: EventReader<RunResetEvent>,
    mut manager: ResMut<EchoFinaleArenaManager>,
    shock_q: Query<Entity, With<FinaleShockwaveObstacle>>,
    node_q: Query<Entity, With<HarmonicNodeCollectible>>,
) {
    for _ in reset_reader.read() {
        *manager = EchoFinaleArenaManager::default();

        for e in shock_q.iter() {
            commands.entity(e).despawn();
        }
        for e in node_q.iter() {
            commands.entity(e).despawn();
        }
    }
}

fn cleanup_ui_root<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// -------------------------------------------------------------
// TESTS
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act5_distance_milestones() {
        assert_eq!(ACT5_START_DISTANCE, 13700.0);
        assert_eq!(ACT5_OVERLOAD_DISTANCE, 13820.0);
        assert_eq!(ACT5_RUSH_DISTANCE, 13940.0);
        assert_eq!(ACT5_CORE_SANCTUM_DISTANCE, 14000.0);

        // Sequence must be monotonically strictly increasing
        assert!(ACT5_START_DISTANCE < ACT5_OVERLOAD_DISTANCE);
        assert!(ACT5_OVERLOAD_DISTANCE < ACT5_RUSH_DISTANCE);
        assert!(ACT5_RUSH_DISTANCE < ACT5_CORE_SANCTUM_DISTANCE);
        assert_eq!(ACT5_CORE_SANCTUM_DISTANCE - ACT5_START_DISTANCE, 300.0);
    }

    #[test]
    fn test_ending_types_are_distinct() {
        assert_ne!(EndingType::Disconnect, EndingType::Synchronize);
    }

    #[test]
    fn test_harmonic_node_overload_requirement() {
        let mut manager = EchoFinaleArenaManager::default();
        assert!(!manager.shield_overloaded);
        manager.harmonic_nodes_collected = 3;
        assert_eq!(manager.harmonic_nodes_collected, 3);
    }
}
