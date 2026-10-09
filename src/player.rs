use std::collections::HashMap;
use std::time::Duration;
use bevy::prelude::*;
use bevy::animation::graph::{AnimationGraph, AnimationNodeIndex};
use crate::types::*;
use crate::director::RunDirector;

#[derive(Component)]
pub struct KaiVisual;

#[derive(Component)]
pub struct KaiArmature;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KaiAnimState {
    Sprint,
    JumpStart,
    JumpLand,
    SlideStart,
    SlideExit,
    Death,
}

impl KaiAnimState {
    pub fn clip_name(&self) -> &'static str {
        match self {
            KaiAnimState::Sprint => "Sprint",
            KaiAnimState::JumpStart => "Jump Start",
            KaiAnimState::JumpLand => "Jump Land",
            KaiAnimState::SlideStart => "Slide Start",
            KaiAnimState::SlideExit => "Slide Exit",
            KaiAnimState::Death => "Death 02",
        }
    }

    pub fn transition_duration(&self) -> Duration {
        match self {
            KaiAnimState::Sprint => Duration::from_secs_f32(0.10),
            KaiAnimState::JumpStart => Duration::from_secs_f32(0.08),
            KaiAnimState::JumpLand => Duration::from_secs_f32(0.06),
            KaiAnimState::SlideStart => Duration::from_secs_f32(0.08),
            KaiAnimState::SlideExit => Duration::from_secs_f32(0.08),
            KaiAnimState::Death => Duration::from_secs_f32(0.12),
        }
    }

    pub fn is_looping(&self) -> bool {
        matches!(self, KaiAnimState::Sprint)
    }
}

#[derive(Component)]
pub struct KaiAnimationController {
    pub current_anim: Option<KaiAnimState>,
    pub was_grounded: bool,
    pub was_sliding: bool,
    pub landing_timer: f32,
    pub slide_exit_timer: f32,
}

impl Default for KaiAnimationController {
    fn default() -> Self {
        Self {
            current_anim: None,
            was_grounded: true,
            was_sliding: false,
            landing_timer: 0.0,
            slide_exit_timer: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct KaiModelAssets {
    pub gltf: Handle<bevy::gltf::Gltf>,
    pub scene: Handle<Scene>,
    pub graph: Option<Handle<AnimationGraph>>,
    pub animations: HashMap<String, AnimationNodeIndex>,
}

impl FromWorld for KaiModelAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let gltf = asset_server.load("Charecters/Kai/Kai_rigged.glb");
        let scene = asset_server.load("Charecters/Kai/Kai_rigged.glb#Scene0");
        Self {
            gltf,
            scene,
            graph: None,
            animations: HashMap::new(),
        }
    }
}

#[derive(Component)]
pub struct ShieldVisual;

#[derive(Component)]
pub struct MagnetVisual;

#[derive(Component)]
pub struct OverdriveVisual;

#[derive(Component)]
pub struct SlideSparksVisual;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerLeftArm;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerRightArm;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerLeftLeg;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerRightLeg;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerBackpackStrap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KaiModelAssets>()
            .add_systems(OnEnter(AppState::InGame), ensure_player_spawned)
            .add_systems(
                Update,
                (
                    setup_kai_animation_graph,
                    attach_kai_animation_player,
                    kai_animation_controller_system,
                    update_kai_visual_transform,
                    handle_run_reset_player,
                ),
            )
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

fn setup_kai_animation_graph(
    mut kai_assets: ResMut<KaiModelAssets>,
    gltf_assets: Res<Assets<bevy::gltf::Gltf>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if kai_assets.graph.is_some() {
        return;
    }

    if let Some(gltf) = gltf_assets.get(&kai_assets.gltf) {
        let mut graph = AnimationGraph::new();
        let mut anim_map = HashMap::new();

        // Register exact animation clips from Kai_rigged.glb
        for (name, clip_handle) in &gltf.named_animations {
            let node_index = graph.add_clip(clip_handle.clone(), 1.0, graph.root);
            anim_map.insert(name.to_string(), node_index);
        }

        let graph_handle = animation_graphs.add(graph);
        kai_assets.graph = Some(graph_handle);
        kai_assets.animations = anim_map;

        // Apply authentic base color texture and ensure zero unwanted emissive glow
        let mat_handle = gltf.named_materials.get("Material.001")
            .or_else(|| gltf.named_materials.get("Material_0.001"));
        if let Some(mat_handle) = mat_handle {
            if let Some(mat) = materials.get_mut(mat_handle) {
                if mat.base_color_texture.is_none() {
                    mat.base_color_texture = Some(asset_server.load("Charecters/Kai/textures/texture_0.jpg"));
                }
                mat.emissive_texture = None;
                mat.emissive = LinearRgba::BLACK;
                if mat.normal_map_texture.is_none() {
                    mat.normal_map_texture = Some(asset_server.load("Charecters/Kai/textures/texture_2.jpg"));
                }
                mat.perceptual_roughness = 0.85;
                mat.metallic = 0.10;
            }
        }
    }
}

fn attach_kai_animation_player(
    mut commands: Commands,
    kai_assets: Res<KaiModelAssets>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<KaiArmature>>,
    mut controller_q: Query<&mut KaiAnimationController>,
) {
    let Some(ref graph_handle) = kai_assets.graph else {
        return;
    };

    for (entity, mut player) in players.iter_mut() {
        let mut transitions = AnimationTransitions::new();
        if let Some(&sprint_idx) = kai_assets.animations.get("Sprint") {
            transitions.play(&mut player, sprint_idx, Duration::ZERO).repeat();
        }

        commands.entity(entity)
            .insert(graph_handle.clone())
            .insert(transitions)
            .insert(KaiArmature);

        if let Ok(mut controller) = controller_q.get_single_mut() {
            controller.current_anim = Some(KaiAnimState::Sprint);
            controller.was_grounded = true;
            controller.was_sliding = false;
            controller.landing_timer = 0.0;
            controller.slide_exit_timer = 0.0;
        }
    }
}

fn kai_animation_controller_system(
    time: Res<Time>,
    app_state: Res<State<AppState>>,
    kai_assets: Res<KaiModelAssets>,
    mut controller_q: Query<(&Player, &mut KaiAnimationController)>,
    mut player_anim_q: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<KaiArmature>>,
) {
    let (player, mut controller) = match controller_q.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };
    let (mut anim_player, mut transitions) = match player_anim_q.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    // Step 4: Death is a proper final state in GameOver. Once in Death, do not restart.
    if *app_state.get() == AppState::GameOver {
        if controller.current_anim != Some(KaiAnimState::Death) {
            controller.current_anim = Some(KaiAnimState::Death);
            if let Some(&death_idx) = kai_assets.animations.get(KaiAnimState::Death.clip_name()) {
                transitions.play(&mut anim_player, death_idx, KaiAnimState::Death.transition_duration());
            }
        }
        return;
    }

    // Outside active gameplay (Menu / Paused / Story), hold state
    if *app_state.get() != AppState::InGame {
        return;
    }

    // Decrement non-interrupting transient timers
    if controller.landing_timer > 0.0 {
        controller.landing_timer = (controller.landing_timer - dt).max(0.0);
    }
    if controller.slide_exit_timer > 0.0 {
        controller.slide_exit_timer = (controller.slide_exit_timer - dt).max(0.0);
    }

    // Step 2 & Step 3: Determine desired animation based on real gameplay kinematics
    let desired_state = if !player.is_grounded {
        // Airborne: Play Jump Start immediately, cancel slide exit or landing
        controller.was_grounded = false;
        controller.landing_timer = 0.0;
        controller.slide_exit_timer = 0.0;
        KaiAnimState::JumpStart
    } else {
        // Grounded: Detect touchdown from air
        if !controller.was_grounded {
            controller.was_grounded = true;
            if !player.is_sliding {
                controller.landing_timer = 0.25;
            }
        }

        if player.is_sliding {
            controller.was_sliding = true;
            controller.landing_timer = 0.0;
            controller.slide_exit_timer = 0.0;
            KaiAnimState::SlideStart
        } else {
            // Detect transition out of slide
            if controller.was_sliding {
                controller.was_sliding = false;
                controller.slide_exit_timer = 0.25;
            }

            if controller.slide_exit_timer > 0.0 {
                KaiAnimState::SlideExit
            } else if controller.landing_timer > 0.0 {
                KaiAnimState::JumpLand
            } else {
                KaiAnimState::Sprint
            }
        }
    };

    // Step 5: Smooth AnimationTransitions blending
    if controller.current_anim != Some(desired_state) {
        if let Some(&node_idx) = kai_assets.animations.get(desired_state.clip_name()) {
            let active = transitions.play(
                &mut anim_player,
                node_idx,
                desired_state.transition_duration(),
            );
            if desired_state.is_looping() {
                active.repeat();
            }
            controller.current_anim = Some(desired_state);
        }
    }
}

fn update_kai_visual_transform(
    player_q: Query<&Player>,
    mut visual_q: Query<&mut Transform, With<KaiVisual>>,
) {
    let Ok(player) = player_q.get_single() else { return };
    let Ok(mut trans) = visual_q.get_single_mut() else { return };

    if player.is_sliding {
        // Counteract parent root scale during slide so Kai mesh maintains 1.0 natural scale
        // and aligns feet with ground level at Y=0.0
        trans.scale = Vec3::new(1.0 / 1.1, 1.0 / 0.45, 1.0 / 1.2);
        trans.translation.y = -0.35 / 0.45;
    } else {
        trans.scale = Vec3::ONE;
        trans.translation.y = -0.65;
    }
}

fn ensure_player_spawned(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stats: Res<GameRunStats>,
    kai_assets: Res<KaiModelAssets>,
    player_q: Query<Entity, With<Player>>,
) {
    if player_q.is_empty() {
        spawn_player_entity(&mut commands, &mut meshes, &mut materials, stats.selected_character, &kai_assets);
    }
}

fn handle_run_reset_player(
    mut events: EventReader<RunResetEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_q: Query<(&mut Player, &mut Transform, Option<&mut KaiAnimationController>)>,
    mut anim_q: Query<(&mut AnimationPlayer, &mut AnimationTransitions), With<KaiArmature>>,
    mut history: ResMut<PlayerMovementHistory>,
    stats: Res<GameRunStats>,
    kai_assets: Res<KaiModelAssets>,
) {
    for _ in events.read() {
        *history = PlayerMovementHistory::default();

        if let Ok((mut player, mut transform, opt_ctrl)) = player_q.get_single_mut() {
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

            if let Some(mut ctrl) = opt_ctrl {
                ctrl.current_anim = None;
                ctrl.was_grounded = true;
                ctrl.was_sliding = false;
                ctrl.landing_timer = 0.0;
                ctrl.slide_exit_timer = 0.0;
            }

            if let Ok((mut anim_player, mut transitions)) = anim_q.get_single_mut() {
                if let Some(&sprint_idx) = kai_assets.animations.get("Sprint") {
                    transitions.play(&mut anim_player, sprint_idx, Duration::ZERO).repeat();
                }
            }
        } else {
            spawn_player_entity(&mut commands, &mut meshes, &mut materials, stats.selected_character, &kai_assets);
        }
    }
}

#[allow(dead_code)]
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stats: Res<GameRunStats>,
    kai_assets: Res<KaiModelAssets>,
) {
    spawn_player_entity(&mut commands, &mut meshes, &mut materials, stats.selected_character, &kai_assets);
}

pub fn spawn_player_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    selected_character: CharacterType,
    kai_assets: &KaiModelAssets,
) {
    let cyan_echo_core_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.90, 1.0),
        emissive: LinearRgba::new(0.6, 4.5, 6.0, 1.0),
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

    let sparks_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.65, 0.1),
        emissive: LinearRgba::new(4.5, 2.0, 0.2, 1.0),
        ..default()
    });

    // Spawn Root Courier entity
    commands
        .spawn((
            SpatialBundle {
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
            KaiAnimationController::default(),
        ))
        .with_children(|parent| {
            // Real 3D Kai GLB Model (Facing forward along -Z, feet aligned with track at Y=0.0)
            parent.spawn((
                SceneBundle {
                    scene: kai_assets.scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.65, 0.0)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                    ..default()
                },
                KaiVisual,
            ));

            // Powerup & Movement Visual Attachments
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
            player.y_velocity = 9.0;
            player.is_grounded = false;
            player.is_sliding = false;
            player.slide_timer = 0.0;
            sfx.send(SoundEffect::Jump);
        } else if powerups.double_jump_timer > 0.0 && !player.has_double_jumped {
            player.y_velocity = 8.5;
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
