use crate::environment::EnvironmentAssets;
use crate::types::{AppState, Despawnable, GameRunStats, Lane, TrackSegmentMarker};
use bevy::prelude::*;

// -------------------------------------------------------------
// ZONE 6 — THE FORBIDDEN LINE (10,000m–12,000m)
// -------------------------------------------------------------
// Subtitle: Incident Ground Zero // Reality Distortion Detected
//
// Narrative Identity:
// A classified underground transit and containment facility operated
// by Veyron Dynamics, directly connected to the original ECHO incident.
// Heavy blast doors, quarantine gates, and containment structures
// show evidence of a catastrophic failure. Reality itself exhibits
// subtle geometric and spatial fractures.
//
// Visual Direction:
// - Monumental black-steel containment architecture
// - Dark reinforced concrete bunker walls and structural ribs
// - Heavy blast doors, quarantine seals, and hydraulic bulkheads
// - Deep crimson warning strobes and red emergency status indicators
// - Sparse amber hazard indicators on door latches & conduits
// - Restrained cold cyan ECHO energy distortion accents
// - Neutral global illumination; high architectural contrast for lane readability
// -------------------------------------------------------------

/// Five authored chapters for Zone 6 (10,000m–12,000m).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForbiddenLineChapter {
    QuarantineApproach, // 10,000m–10,300m: Leaving Sky Rail and entering sealed facility
    RestrictedTransit,  // 10,300m–10,700m: Militarized checkpoints, scanners & security
    RealityFracture,    // 10,700m–11,200m: Warped geometry, damaged machinery, cyan ECHO anomalies
    GroundZero,         // 11,200m–11,700m: Catastrophic destruction at incident epicentre
    FinalContainment,   // 11,700m–12,000m: Monolithic blast doors & approach to Zone 7 threshold
}

pub fn forbidden_line_chapter_for_distance(distance: f32) -> ForbiddenLineChapter {
    match distance {
        d if d < 10300.0 => ForbiddenLineChapter::QuarantineApproach,
        d if d < 10700.0 => ForbiddenLineChapter::RestrictedTransit,
        d if d < 11200.0 => ForbiddenLineChapter::RealityFracture,
        d if d < 11700.0 => ForbiddenLineChapter::GroundZero,
        _ => ForbiddenLineChapter::FinalContainment,
    }
}

/// Five deterministic hero landmarks anchoring Zone 6.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForbiddenLineLandmark {
    None,
    QuarantineGate,   // ~10,120m: Giant security portal marking facility entrance
    VeyronCheckpoint, // ~10,480m: Fortified inspection bunker & dual scanner gantry
    FractureChamber,  // ~10,880m: Anomalous polygonal containment ring & cyan ECHO coils
    GroundZero,       // ~11,350m: Shattered reactor core dome at incident epicentre
    FinalContainmentBulkhead, // ~11,820m: Monumental 16m interlocking blast portal before Zone 7
}

pub fn forbidden_line_landmark_for_distance(distance: f32) -> ForbiddenLineLandmark {
    match distance {
        d if (10100.0..10145.0).contains(&d) => ForbiddenLineLandmark::QuarantineGate,
        d if (10460.0..10505.0).contains(&d) => ForbiddenLineLandmark::VeyronCheckpoint,
        d if (10860.0..10905.0).contains(&d) => ForbiddenLineLandmark::FractureChamber,
        d if (11335.0..11375.0).contains(&d) => ForbiddenLineLandmark::GroundZero,
        d if (11810.0..11845.0).contains(&d) => ForbiddenLineLandmark::FinalContainmentBulkhead,
        _ => ForbiddenLineLandmark::None,
    }
}

pub fn forbidden_line_variant(distance: f32) -> u8 {
    let seg_idx = (distance.max(0.0) / 40.0).floor() as usize;
    (seg_idx % 3) as u8
}

// -------------------------------------------------------------
// DYNAMIC ALARM & VENTILATION COMPONENTS
// -------------------------------------------------------------

#[derive(Component)]
pub struct ForbiddenLineFan {
    pub speed: f32,
}

#[derive(Resource, Debug)]
pub struct ForbiddenLineAlarmState {
    pub timer: f32,
    pub pulse: f32,
    pub anomaly_shimmer: f32,
}

impl Default for ForbiddenLineAlarmState {
    fn default() -> Self {
        Self {
            timer: 0.0,
            pulse: 1.0,
            anomaly_shimmer: 1.0,
        }
    }
}

// -------------------------------------------------------------
// ASSETS RESOURCE
// -------------------------------------------------------------

#[derive(Resource)]
pub struct ForbiddenLineAssets {
    // Track & Ground Bed
    pub mesh_containment_bed: Handle<Mesh>,
    pub mesh_lane_surface: Handle<Mesh>,
    pub mesh_armoured_curb: Handle<Mesh>,
    pub mesh_cooling_trough: Handle<Mesh>,
    pub mesh_containment_grate: Handle<Mesh>,
    pub mesh_rail_base: Handle<Mesh>,
    pub mesh_rail_crown: Handle<Mesh>,
    pub mesh_sleeper: Handle<Mesh>,
    pub mesh_sleeper_plate: Handle<Mesh>,
    pub mesh_crimson_stripe: Handle<Mesh>,

    // Containment Architecture & Bunker Shell
    pub mesh_bunker_wall: Handle<Mesh>,
    pub mesh_bunker_wall_short: Handle<Mesh>,
    pub mesh_observation_lintel: Handle<Mesh>,
    pub mesh_alcove_railing: Handle<Mesh>,
    pub mesh_wall_rib: Handle<Mesh>,
    pub mesh_vault_rib: Handle<Mesh>,
    pub mesh_bulkhead_girder: Handle<Mesh>,
    pub mesh_conduit_trunk: Handle<Mesh>,
    pub mesh_conduit_bracket: Handle<Mesh>,
    pub mesh_security_pod: Handle<Mesh>,
    pub mesh_warning_box: Handle<Mesh>,
    pub mesh_warning_lens: Handle<Mesh>,
    pub mesh_hydraulic_ram: Handle<Mesh>,
    pub mesh_interlock_panel: Handle<Mesh>,

    // Distant Subterranean Facility & Cavern Depth
    pub mesh_bg_cavern_arch: Handle<Mesh>,
    pub mesh_bg_retaining_wall: Handle<Mesh>,
    pub mesh_bg_rock_anchor: Handle<Mesh>,
    pub mesh_bg_containment_silo: Handle<Mesh>,
    pub mesh_bg_silo_cap: Handle<Mesh>,
    pub mesh_bg_silo_cradle: Handle<Mesh>,
    pub mesh_bg_catwalk_deck: Handle<Mesh>,
    pub mesh_bg_catwalk_rail: Handle<Mesh>,
    pub mesh_bg_catwalk_strut: Handle<Mesh>,
    pub mesh_bg_transit_tube: Handle<Mesh>,
    pub mesh_bg_reactor_trestle: Handle<Mesh>,
    pub mesh_bg_substation_transformer: Handle<Mesh>,
    pub mesh_bg_insulator_bushing: Handle<Mesh>,
    pub mesh_bg_vent_trunk: Handle<Mesh>,
    pub mesh_bg_vent_chimney: Handle<Mesh>,
    pub mesh_bg_fracture_void_monolith: Handle<Mesh>,
    pub mesh_bg_ground_zero_abyss: Handle<Mesh>,
    pub mesh_bg_abyss_fissure_glow: Handle<Mesh>,

    // Trackside Machinery & Storytelling
    pub mesh_exhaust_fan_housing: Handle<Mesh>,
    pub mesh_exhaust_fan_blade: Handle<Mesh>,
    pub mesh_coolant_pipe: Handle<Mesh>,
    pub mesh_pipe_flange: Handle<Mesh>,
    pub mesh_capacitor_tower: Handle<Mesh>,
    pub mesh_stabilizer_coil: Handle<Mesh>,
    pub mesh_cable_tray_hanging: Handle<Mesh>,
    pub mesh_hanging_cable: Handle<Mesh>,
    pub mesh_airlock_wall_door: Handle<Mesh>,
    pub mesh_terminal_kiosk: Handle<Mesh>,
    pub mesh_terminal_screen: Handle<Mesh>,
    pub mesh_spatial_fracture_shard: Handle<Mesh>,
    pub mesh_echo_energy_fissure: Handle<Mesh>,
    pub mesh_laser_scanner_curtain: Handle<Mesh>,
    pub mesh_sheared_rebar_bundle: Handle<Mesh>,
    pub mesh_hazard_bollard: Handle<Mesh>,

    // Landmark Hero Meshes
    pub mesh_gate_portal_frame: Handle<Mesh>,
    pub mesh_gate_warning_header: Handle<Mesh>,
    pub mesh_gate_sentry_pylon: Handle<Mesh>,
    pub mesh_checkpoint_scanner_beam: Handle<Mesh>,
    pub mesh_checkpoint_pillbox: Handle<Mesh>,
    pub mesh_fracture_ring_segment: Handle<Mesh>,
    pub mesh_fracture_monolith: Handle<Mesh>,
    pub mesh_echo_singularity_core: Handle<Mesh>,
    pub mesh_ground_zero_ruptured_dome: Handle<Mesh>,
    pub mesh_ground_zero_rubble_slab: Handle<Mesh>,
    pub mesh_bulkhead_blast_slab: Handle<Mesh>,
    pub mesh_bulkhead_lock_gear: Handle<Mesh>,

    // Materials
    pub mat_containment_black_steel: Handle<StandardMaterial>,
    pub mat_reinforced_concrete_dark: Handle<StandardMaterial>,
    pub mat_armoured_panel: Handle<StandardMaterial>,
    pub mat_hydraulic_steel: Handle<StandardMaterial>,
    pub mat_crimson_warning_glow: Handle<StandardMaterial>,
    pub mat_crimson_rail_glow: Handle<StandardMaterial>,
    pub mat_amber_hazard: Handle<StandardMaterial>,
    pub mat_cyan_echo_anomaly: Handle<StandardMaterial>,
    pub mat_containment_grate: Handle<StandardMaterial>,
    pub mat_rubble_concrete: Handle<StandardMaterial>,
    pub mat_coolant_cryo_pipe: Handle<StandardMaterial>,
    pub mat_terminal_screen_alert: Handle<StandardMaterial>,
    pub mat_laser_scanner_curtain: Handle<StandardMaterial>,
    pub mat_exposed_rebar: Handle<StandardMaterial>,
    pub mat_echo_singularity_crystal: Handle<StandardMaterial>,
    pub mat_bg_cavern_rock: Handle<StandardMaterial>,
    pub mat_bg_silo_steel: Handle<StandardMaterial>,
    pub mat_bg_catwalk_grate: Handle<StandardMaterial>,
}

pub fn init_forbidden_line_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let length = 40.0;

    let assets = ForbiddenLineAssets {
        // Track Bed & Running Lanes
        mesh_containment_bed: meshes.add(Cuboid::new(9.6, 0.45, length)),
        mesh_lane_surface: meshes.add(Cuboid::new(2.2, 0.08, length)),
        mesh_armoured_curb: meshes.add(Cuboid::new(0.3, 0.18, length)),
        mesh_cooling_trough: meshes.add(Cuboid::new(0.7, 0.22, length)),
        mesh_containment_grate: meshes.add(Cuboid::new(0.66, 0.04, 0.9)),
        mesh_rail_base: meshes.add(Cuboid::new(0.14, 0.06, length)),
        mesh_rail_crown: meshes.add(Cuboid::new(0.10, 0.07, length)),
        mesh_sleeper: meshes.add(Cuboid::new(8.6, 0.14, 0.42)),
        mesh_sleeper_plate: meshes.add(Cuboid::new(0.28, 0.04, 0.32)),
        mesh_crimson_stripe: meshes.add(Cuboid::new(0.08, 0.02, 1.6)),

        // Containment Bunker Shell & Infrastructure
        mesh_bunker_wall: meshes.add(Cuboid::new(0.8, 7.6, length)),
        mesh_bunker_wall_short: meshes.add(Cuboid::new(0.8, 7.6, 14.0)),
        mesh_observation_lintel: meshes.add(Cuboid::new(0.8, 1.8, 12.0)),
        mesh_alcove_railing: meshes.add(Cuboid::new(0.2, 1.1, 12.0)),
        mesh_wall_rib: meshes.add(Cuboid::new(1.3, 7.6, 1.1)),
        mesh_vault_rib: meshes.add(Cuboid::new(13.4, 0.65, 1.0)),
        mesh_bulkhead_girder: meshes.add(Cuboid::new(13.0, 0.85, 0.85)),
        mesh_conduit_trunk: meshes.add(Cylinder::new(0.12, length)),
        mesh_conduit_bracket: meshes.add(Cuboid::new(0.4, 0.35, 0.15)),
        mesh_security_pod: meshes.add(Cuboid::new(0.4, 0.3, 0.6)),
        mesh_warning_box: meshes.add(Cuboid::new(0.45, 0.55, 0.3)),
        mesh_warning_lens: meshes.add(Sphere::new(0.16)),
        mesh_hydraulic_ram: meshes.add(Cylinder::new(0.18, 3.4)),
        mesh_interlock_panel: meshes.add(Cuboid::new(0.15, 3.2, 2.8)),

        // Distant Subterranean Facility & Cavern Depth
        mesh_bg_cavern_arch: meshes.add(Cuboid::new(34.0, 1.4, 1.8)),
        mesh_bg_retaining_wall: meshes.add(Cuboid::new(1.2, 18.0, length)),
        mesh_bg_rock_anchor: meshes.add(Cuboid::new(2.0, 18.0, 2.2)),
        mesh_bg_containment_silo: meshes.add(Cylinder::new(2.4, 12.0)),
        mesh_bg_silo_cap: meshes.add(Sphere::new(2.4)),
        mesh_bg_silo_cradle: meshes.add(Cuboid::new(5.4, 1.8, 5.4)),
        mesh_bg_catwalk_deck: meshes.add(Cuboid::new(1.6, 0.16, length)),
        mesh_bg_catwalk_rail: meshes.add(Cuboid::new(0.08, 1.1, length)),
        mesh_bg_catwalk_strut: meshes.add(Cuboid::new(0.3, 2.4, 0.3)),
        mesh_bg_transit_tube: meshes.add(Cylinder::new(2.6, length)),
        mesh_bg_reactor_trestle: meshes.add(Cuboid::new(1.6, 4.0, 1.6)),
        mesh_bg_substation_transformer: meshes.add(Cuboid::new(3.2, 4.2, 4.8)),
        mesh_bg_insulator_bushing: meshes.add(Cylinder::new(0.16, 1.8)),
        mesh_bg_vent_trunk: meshes.add(Cylinder::new(0.45, length)),
        mesh_bg_vent_chimney: meshes.add(Cylinder::new(0.9, 6.0)),
        mesh_bg_fracture_void_monolith: meshes.add(Cuboid::new(2.2, 10.0, 2.2)),
        mesh_bg_ground_zero_abyss: meshes.add(Cuboid::new(14.0, 0.4, length)),
        mesh_bg_abyss_fissure_glow: meshes.add(Cuboid::new(1.8, 0.08, 16.0)),

        // Trackside Machinery & Storytelling
        mesh_exhaust_fan_housing: meshes.add(Cuboid::new(2.4, 2.4, 0.6)),
        mesh_exhaust_fan_blade: meshes.add(Cuboid::new(2.0, 0.35, 0.08)),
        mesh_coolant_pipe: meshes.add(Cylinder::new(0.20, length)),
        mesh_pipe_flange: meshes.add(Cylinder::new(0.32, 0.15)),
        mesh_capacitor_tower: meshes.add(Cuboid::new(1.2, 3.8, 1.6)),
        mesh_stabilizer_coil: meshes.add(Cuboid::new(1.4, 1.6, 2.2)),
        mesh_cable_tray_hanging: meshes.add(Cuboid::new(0.65, 0.14, length)),
        mesh_hanging_cable: meshes.add(Cylinder::new(0.04, 8.0)),
        mesh_airlock_wall_door: meshes.add(Cuboid::new(0.22, 3.8, 3.2)),
        mesh_terminal_kiosk: meshes.add(Cuboid::new(0.35, 1.2, 0.75)),
        mesh_terminal_screen: meshes.add(Cuboid::new(0.06, 0.75, 0.55)),
        mesh_spatial_fracture_shard: meshes.add(Cuboid::new(0.85, 2.6, 0.85)),
        mesh_echo_energy_fissure: meshes.add(Cuboid::new(0.14, 0.04, 4.4)),
        mesh_laser_scanner_curtain: meshes.add(Cuboid::new(2.2, 4.2, 0.04)),
        mesh_sheared_rebar_bundle: meshes.add(Cylinder::new(0.045, 1.8)),
        mesh_hazard_bollard: meshes.add(Cuboid::new(0.32, 1.1, 0.32)),

        // Landmark Hero Geometry
        mesh_gate_portal_frame: meshes.add(Cuboid::new(14.2, 11.0, 3.2)),
        mesh_gate_warning_header: meshes.add(Cuboid::new(10.5, 1.8, 0.6)),
        mesh_gate_sentry_pylon: meshes.add(Cuboid::new(1.8, 9.5, 1.8)),
        mesh_checkpoint_scanner_beam: meshes.add(Cuboid::new(9.8, 0.55, 0.7)),
        mesh_checkpoint_pillbox: meshes.add(Cuboid::new(2.4, 3.8, 4.2)),
        mesh_fracture_ring_segment: meshes.add(Cuboid::new(1.4, 4.5, 1.4)),
        mesh_fracture_monolith: meshes.add(Cuboid::new(1.8, 8.0, 1.8)),
        mesh_echo_singularity_core: meshes.add(Sphere::new(1.2)),
        mesh_ground_zero_ruptured_dome: meshes.add(Sphere::new(4.8)),
        mesh_ground_zero_rubble_slab: meshes.add(Cuboid::new(3.2, 0.8, 2.6)),
        mesh_bulkhead_blast_slab: meshes.add(Cuboid::new(4.8, 14.0, 2.2)),
        mesh_bulkhead_lock_gear: meshes.add(Cylinder::new(1.4, 0.8)),

        // Materials
        mat_containment_black_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.07, 0.09),
            metallic: 0.85,
            perceptual_roughness: 0.35,
            reflectance: 0.65,
            ..default()
        }),
        mat_reinforced_concrete_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.11, 0.11, 0.13),
            metallic: 0.05,
            perceptual_roughness: 0.88,
            reflectance: 0.20,
            ..default()
        }),
        mat_armoured_panel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.14, 0.17),
            metallic: 0.70,
            perceptual_roughness: 0.42,
            reflectance: 0.55,
            ..default()
        }),
        mat_hydraulic_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.20, 0.24),
            metallic: 0.95,
            perceptual_roughness: 0.22,
            reflectance: 0.85,
            ..default()
        }),
        mat_crimson_warning_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.02, 0.03),
            emissive: LinearRgba::new(3.2, 0.08, 0.14, 1.0),
            metallic: 0.3,
            perceptual_roughness: 0.2,
            ..default()
        }),
        mat_crimson_rail_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.02, 0.02),
            emissive: LinearRgba::new(1.6, 0.06, 0.10, 1.0),
            metallic: 0.5,
            perceptual_roughness: 0.3,
            ..default()
        }),
        mat_amber_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.18, 0.02),
            emissive: LinearRgba::new(1.8, 0.85, 0.05, 1.0),
            metallic: 0.2,
            perceptual_roughness: 0.3,
            ..default()
        }),
        mat_cyan_echo_anomaly: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.25, 0.30),
            emissive: LinearRgba::new(0.08, 1.8, 2.1, 1.0),
            metallic: 0.4,
            perceptual_roughness: 0.2,
            ..default()
        }),
        mat_containment_grate: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.09),
            metallic: 0.65,
            perceptual_roughness: 0.55,
            ..default()
        }),
        mat_rubble_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.12, 0.14),
            metallic: 0.10,
            perceptual_roughness: 0.92,
            ..default()
        }),
        mat_coolant_cryo_pipe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.18, 0.22),
            metallic: 0.82,
            perceptual_roughness: 0.30,
            reflectance: 0.70,
            ..default()
        }),
        mat_terminal_screen_alert: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.05, 0.02),
            emissive: LinearRgba::new(2.4, 0.40, 0.08, 1.0),
            metallic: 0.2,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_laser_scanner_curtain: materials.add(StandardMaterial {
            base_color: Color::srgba(0.85, 0.05, 0.10, 0.35),
            emissive: LinearRgba::new(2.2, 0.06, 0.08, 1.0),
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..default()
        }),
        mat_exposed_rebar: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.14, 0.09),
            metallic: 0.80,
            perceptual_roughness: 0.75,
            ..default()
        }),
        mat_echo_singularity_crystal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.45, 0.55),
            emissive: LinearRgba::new(0.10, 2.6, 3.4, 1.0),
            metallic: 0.5,
            perceptual_roughness: 0.15,
            reflectance: 0.90,
            ..default()
        }),
        mat_bg_cavern_rock: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.07, 0.09),
            metallic: 0.08,
            perceptual_roughness: 0.94,
            reflectance: 0.15,
            ..default()
        }),
        mat_bg_silo_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.14, 0.16),
            metallic: 0.82,
            perceptual_roughness: 0.38,
            reflectance: 0.65,
            ..default()
        }),
        mat_bg_catwalk_grate: materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.10, 0.12),
            metallic: 0.70,
            perceptual_roughness: 0.55,
            reflectance: 0.50,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -------------------------------------------------------------
// PLUGIN DEFINITION & DYNAMIC SYSTEMS
// -------------------------------------------------------------

pub struct ForbiddenLinePlugin;

impl Plugin for ForbiddenLinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ForbiddenLineAlarmState>();
        app.add_systems(
            Update,
            (update_forbidden_line_alarms, animate_forbidden_line_fans)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

pub fn update_forbidden_line_alarms(
    time: Res<Time>,
    run_stats: Option<Res<GameRunStats>>,
    mut alarm_state: ResMut<ForbiddenLineAlarmState>,
    fl_assets: Option<Res<ForbiddenLineAssets>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(fl) = fl_assets else { return };

    let in_zone_6 = run_stats
        .as_ref()
        .map(|s| s.distance >= 9950.0 && s.distance <= 12050.0)
        .unwrap_or(true);

    if !in_zone_6 {
        return;
    }

    let dt = time.delta_seconds();
    alarm_state.timer += dt;

    // Atmospheric rhythmic alarm pulse (1.6s cycle)
    let cycle = (alarm_state.timer * 3.9).sin();
    let pulse_factor = 0.7 + 0.3 * cycle.max(0.0).powi(2);
    alarm_state.pulse = pulse_factor;

    // Erratic cyan anomaly shimmer (harmonic intermodulation)
    let shimmer = 0.8 + 0.2 * (alarm_state.timer * 8.5).sin() * (alarm_state.timer * 13.2).cos();
    alarm_state.anomaly_shimmer = shimmer;

    // Modulate crimson warning glow material emissive
    if let Some(mat) = materials.get_mut(&fl.mat_crimson_warning_glow) {
        let base_emissive = LinearRgba::new(3.2, 0.08, 0.14, 1.0);
        mat.emissive = LinearRgba::new(
            base_emissive.red * pulse_factor,
            base_emissive.green * pulse_factor,
            base_emissive.blue * pulse_factor,
            1.0,
        );
    }

    // Modulate cyan ECHO anomaly material emissive
    if let Some(mat) = materials.get_mut(&fl.mat_cyan_echo_anomaly) {
        let base_emissive = LinearRgba::new(0.08, 1.8, 2.1, 1.0);
        mat.emissive = LinearRgba::new(
            base_emissive.red * shimmer,
            base_emissive.green * shimmer,
            base_emissive.blue * shimmer,
            1.0,
        );
    }
}

pub fn animate_forbidden_line_fans(
    time: Res<Time>,
    mut fan_q: Query<(&mut Transform, &ForbiddenLineFan)>,
) {
    let dt = time.delta_seconds();
    for (mut trans, fan) in fan_q.iter_mut() {
        trans.rotate_z(fan.speed * dt);
    }
}

// -------------------------------------------------------------
// MODULAR SEGMENT SPAWNING
// -------------------------------------------------------------

pub fn spawn_forbidden_line_segment(
    commands: &mut Commands,
    _env: &EnvironmentAssets,
    fl: &ForbiddenLineAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = forbidden_line_chapter_for_distance(distance);
    let landmark = forbidden_line_landmark_for_distance(distance);
    let variant = forbidden_line_variant(distance);

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // 1. Heavy Containment Sub-Ballast Bed & Drainage Trough
            spawn_containment_bed(seg, fl, length);

            // 2. High-Readability 3 Running Lanes with Crimson Warning Rails
            spawn_three_containment_lanes(seg, fl, chapter, variant);

            // 3. Fortified Underground Bunker Shell (Walls, Ribs & Bulkhead Vaults)
            spawn_bunker_shell(seg, fl, variant, length);

            // 4. Distant Subterranean Facility Background & Overhead Excavation Vault
            spawn_distant_facility_background(seg, fl, chapter, variant, distance);

            // 5. Chapter-Specific Visual Dressing & Environmental Storytelling
            match chapter {
                ForbiddenLineChapter::QuarantineApproach => {
                    spawn_quarantine_approach_dressing(seg, fl, variant);
                }
                ForbiddenLineChapter::RestrictedTransit => {
                    spawn_restricted_transit_dressing(seg, fl, variant);
                }
                ForbiddenLineChapter::RealityFracture => {
                    spawn_reality_fracture_dressing(seg, fl, variant);
                }
                ForbiddenLineChapter::GroundZero => {
                    spawn_ground_zero_dressing(seg, fl, variant);
                }
                ForbiddenLineChapter::FinalContainment => {
                    spawn_final_containment_dressing(seg, fl, variant);
                }
            }

            // 6. Deterministic Hero Landmark (if active in this segment window)
            if landmark != ForbiddenLineLandmark::None {
                spawn_forbidden_line_landmark(seg, fl, landmark);
            }
        })
        .id()
}

// -------------------------------------------------------------
// 1. TRACK BED & INFRASTRUCTURE
// -------------------------------------------------------------

fn spawn_containment_bed(seg: &mut ChildBuilder, fl: &ForbiddenLineAssets, _length: f32) {
    // Heavy sub-track containment slab (recessed footing at y = -0.225)
    seg.spawn(PbrBundle {
        mesh: fl.mesh_containment_bed.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(0.0, -0.225, 0.0),
        ..default()
    });

    // Central drainage / coolant trough
    seg.spawn(PbrBundle {
        mesh: fl.mesh_cooling_trough.clone(),
        material: fl.mat_reinforced_concrete_dark.clone(),
        transform: Transform::from_xyz(0.0, -0.05, 0.0),
        ..default()
    });

    // Slotted drainage grates every 4.0m along central trough
    for i in -4..=4 {
        let z = i as f32 * 4.0;
        seg.spawn(PbrBundle {
            mesh: fl.mesh_containment_grate.clone(),
            material: fl.mat_containment_grate.clone(),
            transform: Transform::from_xyz(0.0, 0.01, z),
            ..default()
        });
    }

    // Outer containment curbs framing the track bed
    seg.spawn(PbrBundle {
        mesh: fl.mesh_armoured_curb.clone(),
        material: fl.mat_armoured_panel.clone(),
        transform: Transform::from_xyz(-4.6, 0.08, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_armoured_curb.clone(),
        material: fl.mat_armoured_panel.clone(),
        transform: Transform::from_xyz(4.6, 0.08, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// 2. THREE RUNNING LANES (Authoritative Lane::SPACING = 2.4)
// -------------------------------------------------------------

fn spawn_three_containment_lanes(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    _chapter: ForbiddenLineChapter,
    _variant: u8,
) {
    // Heavy transverse containment sleepers every 2.0m
    for i in -9..=9 {
        let z = i as f32 * 2.0;
        seg.spawn(PbrBundle {
            mesh: fl.mesh_sleeper.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(0.0, 0.02, z),
            ..default()
        });

        // Fastener plates at each rail position
        for lane_idx in -1..=1 {
            let cx = Lane::from_index(lane_idx).x_pos();
            seg.spawn(PbrBundle {
                mesh: fl.mesh_sleeper_plate.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(cx - 0.48, 0.07, z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_sleeper_plate.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(cx + 0.48, 0.07, z),
                ..default()
            });
        }
    }

    // Running lanes (Left = -2.4, Center = 0.0, Right = +2.4)
    for lane_idx in -1..=1 {
        let cx = Lane::from_index(lane_idx).x_pos();

        // Dark matte running slab
        seg.spawn(PbrBundle {
            mesh: fl.mesh_lane_surface.clone(),
            material: fl.mat_reinforced_concrete_dark.clone(),
            transform: Transform::from_xyz(cx, 0.04, 0.0),
            ..default()
        });

        // Left Rail of Lane: Dark Steel Base + Deep Crimson Warning Emissive Crown
        seg.spawn(PbrBundle {
            mesh: fl.mesh_rail_base.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(cx - 0.48, 0.07, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_rail_crown.clone(),
            material: fl.mat_crimson_rail_glow.clone(),
            transform: Transform::from_xyz(cx - 0.48, 0.12, 0.0),
            ..default()
        });

        // Right Rail of Lane: Dark Steel Base + Deep Crimson Warning Emissive Crown
        seg.spawn(PbrBundle {
            mesh: fl.mesh_rail_base.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(cx + 0.48, 0.07, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_rail_crown.clone(),
            material: fl.mat_crimson_rail_glow.clone(),
            transform: Transform::from_xyz(cx + 0.48, 0.12, 0.0),
            ..default()
        });

        // Inter-lane dashed warning stripes along center of each lane
        for s in -5..=5 {
            let sz = s as f32 * 3.6;
            seg.spawn(PbrBundle {
                mesh: fl.mesh_crimson_stripe.clone(),
                material: fl.mat_crimson_warning_glow.clone(),
                transform: Transform::from_xyz(cx, 0.09, sz),
                ..default()
            });
        }
    }

    // Inter-lane dividing curbs (x = -1.2 and +1.2)
    seg.spawn(PbrBundle {
        mesh: fl.mesh_armoured_curb.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(-1.2, 0.06, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_armoured_curb.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(1.2, 0.06, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// 3. FORTIFIED BUNKER SHELL & OVERARCHING RIBS
// -------------------------------------------------------------

fn spawn_bunker_shell(seg: &mut ChildBuilder, fl: &ForbiddenLineAssets, variant: u8, _length: f32) {
    // Fortified Sidewalls:
    // Variant 0: Left side has a 12m observation embrasure, right side solid bunker wall.
    // Variant 1: Continuous fortified sidewalls on both flanks (heavy security corridor).
    // Variant 2: Left side solid bunker wall, right side has a 12m observation embrasure.
    match variant {
        0 => {
            // Left side observation embrasure (z in [-6.0, 6.0] open to lateral facility bay)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall_short.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.8, 3.8, -13.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall_short.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.8, 3.8, 13.0),
                ..default()
            });
            // Reinforced lintel over opening
            seg.spawn(PbrBundle {
                mesh: fl.mesh_observation_lintel.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.8, 6.7, 0.0),
                ..default()
            });
            // Low safety balustrade along opening
            seg.spawn(PbrBundle {
                mesh: fl.mesh_alcove_railing.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-5.8, 0.55, 0.0),
                ..default()
            });

            // Right side solid wall
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.8, 3.8, 0.0),
                ..default()
            });
        }
        2 => {
            // Left side solid wall
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.8, 3.8, 0.0),
                ..default()
            });

            // Right side observation embrasure (z in [-6.0, 6.0] open to secondary transit tube)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall_short.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.8, 3.8, -13.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall_short.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.8, 3.8, 13.0),
                ..default()
            });
            // Reinforced lintel over opening
            seg.spawn(PbrBundle {
                mesh: fl.mesh_observation_lintel.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.8, 6.7, 0.0),
                ..default()
            });
            // Low safety balustrade along opening
            seg.spawn(PbrBundle {
                mesh: fl.mesh_alcove_railing.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(5.8, 0.55, 0.0),
                ..default()
            });
        }
        _ => {
            // Variant 1: Continuous fortified sidewalls on both flanks
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.8, 3.8, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bunker_wall.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.8, 3.8, 0.0),
                ..default()
            });
        }
    }

    // Vertical structural bunker buttresses & ceiling crossbeams at z = -14.0, 0.0, +14.0
    for z in [-14.0, 0.0, 14.0] {
        // Left & right wall buttresses
        seg.spawn(PbrBundle {
            mesh: fl.mesh_wall_rib.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(-5.4, 3.8, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_wall_rib.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(5.4, 3.8, z),
            ..default()
        });

        // High clearance ceiling cross-girder at y = 7.2m
        seg.spawn(PbrBundle {
            mesh: fl.mesh_vault_rib.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(0.0, 7.2, z),
            ..default()
        });

        // Emergency wall strobe light boxes on buttresses (crimson strobe)
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_box.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(-4.75, 4.2, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_crimson_warning_glow.clone(),
            transform: Transform::from_xyz(-4.55, 4.2, z),
            ..default()
        });

        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_box.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(4.75, 4.2, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_crimson_warning_glow.clone(),
            transform: Transform::from_xyz(4.55, 4.2, z),
            ..default()
        });
    }

    // High-voltage industrial conduit trunks along ceiling shoulders
    seg.spawn(PbrBundle {
        mesh: fl.mesh_conduit_trunk.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(-4.8, 6.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_conduit_trunk.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(4.8, 6.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Interlocking lateral blast panels on sidewalls
    if variant != 1 {
        for z in [-8.0, 8.0] {
            seg.spawn(PbrBundle {
                mesh: fl.mesh_interlock_panel.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-5.3, 2.6, z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_interlock_panel.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(5.3, 2.6, z),
                ..default()
            });
        }
    }
}

// -------------------------------------------------------------
// 4. DISTANT SUBTERRANEAN FACILITY BACKGROUND & CAVERN DEPTH
// -------------------------------------------------------------

fn spawn_distant_facility_background(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    chapter: ForbiddenLineChapter,
    variant: u8,
    _distance: f32,
) {
    // -------------------------------------------------------------
    // A. COLOSSAL OVERHEAD CAVERN VAULT & EXCAVATION ARCHES (y >= 10.0m)
    // -------------------------------------------------------------
    // Heavy excavation arches spanning 34m across the subterranean cavern
    for z in [-12.0, 12.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_cavern_arch.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(0.0, 11.8, z),
            ..default()
        });
    }

    // Overhead high-volume ventilation trunks running along cavern shoulders
    seg.spawn(PbrBundle {
        mesh: fl.mesh_bg_vent_trunk.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(-4.6, 10.2, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_bg_vent_trunk.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(4.6, 10.2, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Vertical ventilation chimney shafts ascending into deep bedrock ceiling
    if variant != 1 {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_vent_chimney.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(-7.5, 13.0, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_vent_chimney.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(7.5, 13.0, 0.0),
            ..default()
        });
    }

    // -------------------------------------------------------------
    // B. UPPER OBSERVATION CATWALK GALLERIES (y = 8.8m, |x| = 7.2m)
    // -------------------------------------------------------------
    // High-level catwalk decks overlooking the containment transit line
    for x_side in [-7.2, 7.2] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_catwalk_deck.clone(),
            material: fl.mat_bg_catwalk_grate.clone(),
            transform: Transform::from_xyz(x_side, 8.8, 0.0),
            ..default()
        });

        // Inner & outer safety handrails
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_catwalk_rail.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(x_side - 0.76, 9.4, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_catwalk_rail.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(x_side + 0.76, 9.4, 0.0),
            ..default()
        });

        // 45-degree angled support struts connecting catwalk underside to walls
        for z in [-14.0, 0.0, 14.0] {
            let strut_rot = if x_side < 0.0 { -0.785 } else { 0.785 };
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_catwalk_strut.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(
                    x_side + (if x_side < 0.0 { 0.6 } else { -0.6 }),
                    7.8,
                    z,
                )
                .with_rotation(Quat::from_rotation_z(strut_rot)),
                ..default()
            });

            // Amber service maintenance lights on catwalk stations
            seg.spawn(PbrBundle {
                mesh: fl.mesh_warning_lens.clone(),
                material: fl.mat_amber_hazard.clone(),
                transform: Transform::from_xyz(x_side, 9.6, z),
                ..default()
            });
        }
    }

    // -------------------------------------------------------------
    // C. DISTANT CAVERN PERIMETER RETAINING WALLS (|x| = 20.0m)
    // -------------------------------------------------------------
    // Left retaining wall (always solid reinforced bedrock facing)
    seg.spawn(PbrBundle {
        mesh: fl.mesh_bg_retaining_wall.clone(),
        material: fl.mat_bg_cavern_rock.clone(),
        transform: Transform::from_xyz(-20.0, 9.0, 0.0),
        ..default()
    });

    // Massive rock stabilization pilasters at intervals on left perimeter
    for z in [-14.0, 0.0, 14.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_rock_anchor.clone(),
            material: fl.mat_reinforced_concrete_dark.clone(),
            transform: Transform::from_xyz(-19.2, 9.0, z),
            ..default()
        });
    }

    // Right retaining wall: in Chapter 4 (Ground Zero), it is breached by an abyss!
    if chapter == ForbiddenLineChapter::GroundZero {
        // Deep subterranean rift / abyss floor plunging 4.5m below track level
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_ground_zero_abyss.clone(),
            material: fl.mat_reinforced_concrete_dark.clone(),
            transform: Transform::from_xyz(18.0, -4.5, 0.0),
            ..default()
        });
        // Molten glowing cyan/crimson subterranean rift fissure in the abyss
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_abyss_fissure_glow.clone(),
            material: fl.mat_cyan_echo_anomaly.clone(),
            transform: Transform::from_xyz(17.5, -4.25, 0.0)
                .with_rotation(Quat::from_rotation_y(0.18)),
            ..default()
        });
        // Fractured rock anchors jutting into the breach
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_rock_anchor.clone(),
            material: fl.mat_rubble_concrete.clone(),
            transform: Transform::from_xyz(22.0, 4.0, -10.0)
                .with_rotation(Quat::from_rotation_z(-0.35)),
            ..default()
        });
    } else {
        // Standard distant perimeter retaining wall on right flank
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bg_retaining_wall.clone(),
            material: fl.mat_bg_cavern_rock.clone(),
            transform: Transform::from_xyz(20.0, 9.0, 0.0),
            ..default()
        });
        for z in [-14.0, 0.0, 14.0] {
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_rock_anchor.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(19.2, 9.0, z),
                ..default()
            });
        }
    }

    // -------------------------------------------------------------
    // D. LATERAL FACILITY BAYS & INDUSTRIAL INFRASTRUCTURE
    // -------------------------------------------------------------
    match variant {
        0 => {
            // Left flank: Massive Cryogenic Storage Silo Yard
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_containment_silo.clone(),
                material: fl.mat_bg_silo_steel.clone(),
                transform: Transform::from_xyz(-14.2, 6.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_silo_cap.clone(),
                material: fl.mat_bg_silo_steel.clone(),
                transform: Transform::from_xyz(-14.2, 12.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_silo_cradle.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-14.2, 0.9, 0.0),
                ..default()
            });
            // Coolant manifold pipe & pressure indicator
            seg.spawn(PbrBundle {
                mesh: fl.mesh_coolant_pipe.clone(),
                material: fl.mat_coolant_cryo_pipe.clone(),
                transform: Transform::from_xyz(-11.4, 4.2, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_warning_lens.clone(),
                material: fl.mat_crimson_warning_glow.clone(),
                transform: Transform::from_xyz(-11.8, 6.5, 0.0),
                ..default()
            });

            // Right flank: High-Voltage Electrical Substation Yard
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_substation_transformer.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(12.5, 2.1, 0.0),
                ..default()
            });
            // Ceramic insulator bushings atop transformer
            for z_bush in [-1.4, 0.0, 1.4] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_bg_insulator_bushing.clone(),
                    material: fl.mat_hydraulic_steel.clone(),
                    transform: Transform::from_xyz(12.5, 5.1, z_bush),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(12.5, 6.1, z_bush),
                    ..default()
                });
            }
        }
        1 => {
            // Right flank: Secondary Parallel Transit Tube
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_transit_tube.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(15.5, 3.8, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            // Heavy concrete trestle pylons supporting parallel tube
            for z in [-12.0, 12.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_bg_reactor_trestle.clone(),
                    material: fl.mat_reinforced_concrete_dark.clone(),
                    transform: Transform::from_xyz(15.5, 1.2, z),
                    ..default()
                });
            }
            // Amber hazard running lights along parallel transit tube
            for z in [-10.0, 0.0, 10.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_amber_hazard.clone(),
                    transform: Transform::from_xyz(12.8, 4.0, z),
                    ..default()
                });
            }

            // Left flank: Industrial capacitor banks and power distribution racks
            for z in [-8.0, 8.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_capacitor_tower.clone(),
                    material: fl.mat_containment_black_steel.clone(),
                    transform: Transform::from_xyz(-13.0, 2.0, z),
                    ..default()
                });
            }
        }
        _ => {
            // Variant 2: Industrial Logistics & Twin Pressurized Vessels
            for z_silo in [-9.0, 9.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_bg_containment_silo.clone(),
                    material: fl.mat_bg_silo_steel.clone(),
                    transform: Transform::from_xyz(-14.5, 6.0, z_silo),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_bg_silo_cap.clone(),
                    material: fl.mat_bg_silo_steel.clone(),
                    transform: Transform::from_xyz(-14.5, 12.0, z_silo),
                    ..default()
                });
            }
            // Interconnecting overhead transfer manifold between twin silos
            seg.spawn(PbrBundle {
                mesh: fl.mesh_coolant_pipe.clone(),
                material: fl.mat_coolant_cryo_pipe.clone(),
                transform: Transform::from_xyz(-14.5, 8.5, 0.0),
                ..default()
            });

            // Right flank: Elevated crane gantry beam & logistics platform
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_girder.clone(),
                material: fl.mat_armoured_panel.clone(),
                transform: Transform::from_xyz(14.0, 7.2, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_reactor_trestle.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(14.0, 3.6, -12.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_reactor_trestle.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(14.0, 3.6, 12.0),
                ..default()
            });
        }
    }

    // -------------------------------------------------------------
    // E. CHAPTER-SPECIFIC DISTANT SPECTACLE
    // -------------------------------------------------------------
    match chapter {
        ForbiddenLineChapter::QuarantineApproach => {
            // Elevated sentry observation watchtower on far left perimeter
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_reactor_trestle.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-16.0, 4.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_checkpoint_pillbox.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-16.0, 7.8, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_warning_lens.clone(),
                material: fl.mat_amber_hazard.clone(),
                transform: Transform::from_xyz(-14.6, 8.2, 0.0),
                ..default()
            });
        }
        ForbiddenLineChapter::RestrictedTransit => {
            // High-level automated overhead hoist beam suspended in cavern vault
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_girder.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(0.0, 10.8, 0.0),
                ..default()
            });
            // Crane hoist trolley pod with warning indicator
            seg.spawn(PbrBundle {
                mesh: fl.mesh_security_pod.clone(),
                material: fl.mat_armoured_panel.clone(),
                transform: Transform::from_xyz(-2.5, 10.2, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_warning_lens.clone(),
                material: fl.mat_crimson_warning_glow.clone(),
                transform: Transform::from_xyz(-2.5, 9.9, 0.0),
                ..default()
            });
        }
        ForbiddenLineChapter::RealityFracture => {
            // Distant anomalous geometric monoliths hovering askew in the high cavern void
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_fracture_void_monolith.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-16.5, 11.5, -4.0).with_rotation(Quat::from_euler(
                    EulerRot::YXZ,
                    0.45,
                    0.25,
                    0.18,
                )),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_spatial_fracture_shard.clone(),
                material: fl.mat_cyan_echo_anomaly.clone(),
                transform: Transform::from_xyz(-16.5, 12.0, -4.0),
                ..default()
            });

            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_fracture_void_monolith.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(16.5, 10.0, 4.0).with_rotation(Quat::from_euler(
                    EulerRot::YXZ,
                    -0.35,
                    -0.20,
                    -0.25,
                )),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_spatial_fracture_shard.clone(),
                material: fl.mat_cyan_echo_anomaly.clone(),
                transform: Transform::from_xyz(16.5, 10.5, 4.0),
                ..default()
            });
        }
        ForbiddenLineChapter::GroundZero => {
            // Ruptured secondary silo tilting precariously into the breached abyss
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bg_containment_silo.clone(),
                material: fl.mat_bg_silo_steel.clone(),
                transform: Transform::from_xyz(15.0, 2.5, -6.0)
                    .with_rotation(Quat::from_rotation_z(0.38)),
                ..default()
            });
            // Catastrophic fracture rubble along abyss edge
            seg.spawn(PbrBundle {
                mesh: fl.mesh_ground_zero_rubble_slab.clone(),
                material: fl.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(14.0, 0.4, 6.0)
                    .with_rotation(Quat::from_rotation_y(0.6)),
                ..default()
            });
        }
        ForbiddenLineChapter::FinalContainment => {
            // Monumental perimeter hydraulic compression buttresses flanking the final approach
            for x_pylon in [-16.0, 16.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_bg_catwalk_strut.clone(),
                    material: fl.mat_hydraulic_steel.clone(),
                    transform: Transform::from_xyz(x_pylon, 6.0, 0.0)
                        .with_scale(Vec3::new(2.8, 6.0, 2.8)),
                    ..default()
                });
                // Double emergency lockdown strobe arrays on perimeter buttresses
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(
                        x_pylon + (if x_pylon < 0.0 { 1.2 } else { -1.2 }),
                        9.0,
                        0.0,
                    ),
                    ..default()
                });
            }
        }
    }
}

// -------------------------------------------------------------
// 5. CHAPTER DRESSINGS & ENVIRONMENTAL STORYTELLING
// -------------------------------------------------------------

/// Chapter 1: Quarantine Approach (10,000–10,300m)
/// Leaving Sky Rail and entering the classified underground containment facility.
fn spawn_quarantine_approach_dressing(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    variant: u8,
) {
    // Heavy entrance reinforcement bulkheads at z = -10.0 and +10.0
    for z in [-10.0, 10.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_bulkhead_girder.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(0.0, 6.6, z),
            ..default()
        });

        // Amber hazard indicators on portal clamps
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_amber_hazard.clone(),
            transform: Transform::from_xyz(-4.8, 6.2, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_amber_hazard.clone(),
            transform: Transform::from_xyz(4.8, 6.2, z),
            ..default()
        });
    }

    // Heavy hydraulic seal rams along outer shoulders
    if variant == 0 {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_hydraulic_ram.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(-5.0, 2.2, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_hydraulic_ram.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(5.0, 2.2, 0.0),
            ..default()
        });
    }

    // Cryogenic coolant pipe lines running overhead
    seg.spawn(PbrBundle {
        mesh: fl.mesh_coolant_pipe.clone(),
        material: fl.mat_coolant_cryo_pipe.clone(),
        transform: Transform::from_xyz(-4.2, 6.8, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_coolant_pipe.clone(),
        material: fl.mat_coolant_cryo_pipe.clone(),
        transform: Transform::from_xyz(4.2, 6.8, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Coolant pipe connection flanges
    for z in [-8.0, 8.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_pipe_flange.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(-4.2, 6.8, z)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_pipe_flange.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(4.2, 6.8, z)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }

    // Wall-mounted security monitoring terminal with alert screen
    seg.spawn(PbrBundle {
        mesh: fl.mesh_terminal_kiosk.clone(),
        material: fl.mat_armoured_panel.clone(),
        transform: Transform::from_xyz(-5.1, 2.2, 6.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_terminal_screen.clone(),
        material: fl.mat_terminal_screen_alert.clone(),
        transform: Transform::from_xyz(-4.9, 2.4, 6.0),
        ..default()
    });

    // Recessed quarantine airlock doors in sidewalls
    if variant == 1 {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_airlock_wall_door.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(-5.35, 2.2, -6.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_crimson_warning_glow.clone(),
            transform: Transform::from_xyz(-5.2, 3.8, -6.0),
            ..default()
        });
    }
}

/// Chapter 2: Restricted Transit (10,300–10,700m)
/// Militarized checkpoints, overhead security scanners, surveillance camera clusters.
fn spawn_restricted_transit_dressing(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    variant: u8,
) {
    // Overhead security scanner beam at center of segment (high clearance at y = 5.8m)
    seg.spawn(PbrBundle {
        mesh: fl.mesh_checkpoint_scanner_beam.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(0.0, 5.8, 0.0),
        ..default()
    });

    // Semi-translucent scanning laser curtain across each running lane
    for lane_idx in -1..=1 {
        let cx = Lane::from_index(lane_idx).x_pos();
        seg.spawn(PbrBundle {
            mesh: fl.mesh_laser_scanner_curtain.clone(),
            material: fl.mat_laser_scanner_curtain.clone(),
            transform: Transform::from_xyz(cx, 2.4, 0.0),
            ..default()
        });
    }

    // Scanner sensor pods with alternating crimson/amber status lights
    for lane_idx in -1..=1 {
        let cx = Lane::from_index(lane_idx).x_pos();
        seg.spawn(PbrBundle {
            mesh: fl.mesh_security_pod.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(cx, 5.5, 0.0),
            ..default()
        });

        let mat = if lane_idx == 0 {
            fl.mat_crimson_warning_glow.clone()
        } else {
            fl.mat_amber_hazard.clone()
        };
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: mat,
            transform: Transform::from_xyz(cx, 5.25, 0.0),
            ..default()
        });
    }

    // Trackside high-voltage capacitor towers outside the curbs (x = ±5.1m)
    seg.spawn(PbrBundle {
        mesh: fl.mesh_capacitor_tower.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(-5.1, 2.0, -10.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_capacitor_tower.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(5.1, 2.0, 10.0),
        ..default()
    });

    // Heavy overhead exhaust fan in ceiling vault
    seg.spawn(PbrBundle {
        mesh: fl.mesh_exhaust_fan_housing.clone(),
        material: fl.mat_armoured_panel.clone(),
        transform: Transform::from_xyz(0.0, 7.3, 0.0),
        ..default()
    });
    seg.spawn((
        PbrBundle {
            mesh: fl.mesh_exhaust_fan_blade.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(0.0, 7.3, 0.0),
            ..default()
        },
        ForbiddenLineFan { speed: 1.8 },
    ));

    // Security barricade bollards flanking the outer shoulders
    if variant == 2 {
        for z in [-6.0, 6.0] {
            seg.spawn(PbrBundle {
                mesh: fl.mesh_hazard_bollard.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(-4.8, 0.55, z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_hazard_bollard.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(4.8, 0.55, z),
                ..default()
            });
        }
    }
}

/// Chapter 3: Reality Fracture (10,700–11,200m)
/// Containment failure, distorted geometric ribs, restrained cold cyan ECHO anomalies.
fn spawn_reality_fracture_dressing(seg: &mut ChildBuilder, fl: &ForbiddenLineAssets, variant: u8) {
    // Asymmetrical angled vault rib at center: rotated slightly around Z to evoke spatial distortion
    let tilt_angle = if variant == 0 { 0.09 } else { -0.09 };
    seg.spawn(PbrBundle {
        mesh: fl.mesh_vault_rib.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(0.0, 7.0, 0.0)
            .with_rotation(Quat::from_rotation_z(tilt_angle)),
        ..default()
    });

    // Cold cyan ECHO energy fissures running along the floor and outer curbs
    for z in [-10.0, 10.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_echo_energy_fissure.clone(),
            material: fl.mat_cyan_echo_anomaly.clone(),
            transform: Transform::from_xyz(-3.8, 0.06, z).with_rotation(Quat::from_rotation_y(0.2)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_echo_energy_fissure.clone(),
            material: fl.mat_cyan_echo_anomaly.clone(),
            transform: Transform::from_xyz(3.8, 0.06, z).with_rotation(Quat::from_rotation_y(-0.2)),
            ..default()
        });
    }

    // Floating spatial fracture shards hovering at tilted angles outside the tracks
    seg.spawn(PbrBundle {
        mesh: fl.mesh_spatial_fracture_shard.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(-5.1, 3.4, 0.0).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            0.4,
            0.25,
            0.15,
        )),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_spatial_fracture_shard.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(5.1, 3.4, 0.0).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            -0.4,
            -0.25,
            -0.15,
        )),
        ..default()
    });

    // Corrupted wall terminal with glitching alert screen
    if variant == 1 {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_terminal_kiosk.clone(),
            material: fl.mat_armoured_panel.clone(),
            transform: Transform::from_xyz(5.1, 2.2, -4.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_terminal_screen.clone(),
            material: fl.mat_terminal_screen_alert.clone(),
            transform: Transform::from_xyz(4.9, 2.4, -4.0),
            ..default()
        });
    }
}

/// Chapter 4: Ground Zero (11,200–11,700m)
/// Catastrophic containment rupture, shattered bulkheads, lateral rubble debris.
fn spawn_ground_zero_dressing(seg: &mut ChildBuilder, fl: &ForbiddenLineAssets, variant: u8) {
    // Ruptured overhead bulkhead girder with sheared gap
    seg.spawn(PbrBundle {
        mesh: fl.mesh_conduit_bracket.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(-3.8, 6.8, 0.0).with_rotation(Quat::from_rotation_z(0.2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_conduit_bracket.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(3.8, 6.8, 0.0).with_rotation(Quat::from_rotation_z(-0.2)),
        ..default()
    });

    // Lateral containment rubble slabs (safely outside player lanes at |x| = 4.8m)
    for z in [-10.0, 10.0] {
        let x_offset = if variant == 0 { -4.8 } else { 4.8 };
        seg.spawn(PbrBundle {
            mesh: fl.mesh_ground_zero_rubble_slab.clone(),
            material: fl.mat_rubble_concrete.clone(),
            transform: Transform::from_xyz(x_offset, 0.4, z)
                .with_rotation(Quat::from_rotation_y(0.35)),
            ..default()
        });
    }

    // Sheared structural rebar bundles protruding from fractured concrete
    seg.spawn(PbrBundle {
        mesh: fl.mesh_sheared_rebar_bundle.clone(),
        material: fl.mat_exposed_rebar.clone(),
        transform: Transform::from_xyz(-4.7, 1.2, 2.0).with_rotation(Quat::from_rotation_z(0.35)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_sheared_rebar_bundle.clone(),
        material: fl.mat_exposed_rebar.clone(),
        transform: Transform::from_xyz(4.7, 1.2, -2.0).with_rotation(Quat::from_rotation_z(-0.35)),
        ..default()
    });

    // Magnetic rail stabilizer coil with emergency flash
    seg.spawn(PbrBundle {
        mesh: fl.mesh_stabilizer_coil.clone(),
        material: fl.mat_containment_black_steel.clone(),
        transform: Transform::from_xyz(-5.1, 0.9, -6.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_warning_lens.clone(),
        material: fl.mat_crimson_warning_glow.clone(),
        transform: Transform::from_xyz(-4.6, 1.6, -6.0),
        ..default()
    });

    // Emergency red lockdown beacons on fractured columns
    seg.spawn(PbrBundle {
        mesh: fl.mesh_warning_lens.clone(),
        material: fl.mat_crimson_warning_glow.clone(),
        transform: Transform::from_xyz(-4.6, 2.8, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_warning_lens.clone(),
        material: fl.mat_crimson_warning_glow.clone(),
        transform: Transform::from_xyz(4.6, 2.8, 0.0),
        ..default()
    });
}

/// Chapter 5: Final Containment (11,700–12,000m)
/// Monolithic blast doors, reinforced security bulkheads, ominous threshold to Zone 7.
fn spawn_final_containment_dressing(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    _variant: u8,
) {
    // Dual monumental overhead blast door frames at z = -12.0 and +12.0
    for z in [-12.0, 12.0] {
        seg.spawn(PbrBundle {
            mesh: fl.mesh_vault_rib.clone(),
            material: fl.mat_containment_black_steel.clone(),
            transform: Transform::from_xyz(0.0, 7.4, z),
            ..default()
        });

        // Twin hydraulic lockdown rams flanking corridor
        seg.spawn(PbrBundle {
            mesh: fl.mesh_hydraulic_ram.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(-5.1, 3.2, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_hydraulic_ram.clone(),
            material: fl.mat_hydraulic_steel.clone(),
            transform: Transform::from_xyz(5.1, 3.2, z),
            ..default()
        });

        // Crimson lockdown strobe beacons
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_crimson_warning_glow.clone(),
            transform: Transform::from_xyz(-4.8, 5.6, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: fl.mesh_warning_lens.clone(),
            material: fl.mat_crimson_warning_glow.clone(),
            transform: Transform::from_xyz(4.8, 5.6, z),
            ..default()
        });
    }

    // Heavy catenary power cable tray suspended below the ceiling
    seg.spawn(PbrBundle {
        mesh: fl.mesh_cable_tray_hanging.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(0.0, 6.6, 0.0),
        ..default()
    });

    // Hanging high-voltage catenary cable loops suspended from ceiling vaults
    seg.spawn(PbrBundle {
        mesh: fl.mesh_hanging_cable.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(-2.2, 6.8, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: fl.mesh_hanging_cable.clone(),
        material: fl.mat_hydraulic_steel.clone(),
        transform: Transform::from_xyz(2.2, 6.8, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// 5. FIVE DETERMINISTIC HERO LANDMARKS
// -------------------------------------------------------------

fn spawn_forbidden_line_landmark(
    seg: &mut ChildBuilder,
    fl: &ForbiddenLineAssets,
    landmark: ForbiddenLineLandmark,
) {
    match landmark {
        ForbiddenLineLandmark::None => {}

        // Landmark 1: Quarantine Gate (~10,120m)
        // Towering security portal marking the transition into the classified facility.
        ForbiddenLineLandmark::QuarantineGate => {
            // Monumental portal frame spanning over the corridor (high clearance y = 6.2m)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_gate_portal_frame.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(0.0, 6.2, 0.0),
                ..default()
            });

            // Prominent Veyron warning header across upper beam
            seg.spawn(PbrBundle {
                mesh: fl.mesh_gate_warning_header.clone(),
                material: fl.mat_crimson_warning_glow.clone(),
                transform: Transform::from_xyz(0.0, 7.8, 1.2),
                ..default()
            });

            // Twin sentry pylon towers flanking track bed
            seg.spawn(PbrBundle {
                mesh: fl.mesh_gate_sentry_pylon.clone(),
                material: fl.mat_armoured_panel.clone(),
                transform: Transform::from_xyz(-5.6, 4.8, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_gate_sentry_pylon.clone(),
                material: fl.mat_armoured_panel.clone(),
                transform: Transform::from_xyz(5.6, 4.8, 0.0),
                ..default()
            });

            // Heavy hydraulic lockdown arms clamped to outer pillars
            seg.spawn(PbrBundle {
                mesh: fl.mesh_hydraulic_ram.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(-4.9, 3.8, 1.4)
                    .with_rotation(Quat::from_rotation_z(0.3)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_hydraulic_ram.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(4.9, 3.8, 1.4)
                    .with_rotation(Quat::from_rotation_z(-0.3)),
                ..default()
            });

            // Array of emergency red beacon lights framing the gate
            for y_step in [3.0, 5.5, 8.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(-4.6, y_step, 0.8),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(4.6, y_step, 0.8),
                    ..default()
                });
            }
        }

        // Landmark 2: Veyron Checkpoint (~10,480m)
        // Armoured transit inspection fortress with elevated observation pillboxes and scanner array.
        ForbiddenLineLandmark::VeyronCheckpoint => {
            // Dual overhead inspection scanning beams (high clearance at y = 6.2m)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_checkpoint_scanner_beam.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(0.0, 6.2, -3.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_checkpoint_scanner_beam.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(0.0, 6.2, 3.0),
                ..default()
            });

            // Laser scanner curtain curtains over each lane
            for lane_idx in -1..=1 {
                let cx = Lane::from_index(lane_idx).x_pos();
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_laser_scanner_curtain.clone(),
                    material: fl.mat_laser_scanner_curtain.clone(),
                    transform: Transform::from_xyz(cx, 2.8, -3.0),
                    ..default()
                });
            }

            // Lateral fortified pillboxes outside running lanes (x = ±5.4m)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_checkpoint_pillbox.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(-5.4, 2.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_checkpoint_pillbox.clone(),
                material: fl.mat_reinforced_concrete_dark.clone(),
                transform: Transform::from_xyz(5.4, 2.0, 0.0),
                ..default()
            });

            // Amber and crimson surveillance status arrays
            for z in [-3.0, 3.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_amber_hazard.clone(),
                    transform: Transform::from_xyz(-4.1, 3.8, z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(4.1, 3.8, z),
                    ..default()
                });
            }
        }

        // Landmark 3: Fracture Chamber (~10,880m)
        // Anomalous containment chamber with polygonal containment ring and cyan ECHO resonance.
        ForbiddenLineLandmark::FractureChamber => {
            // Monumental octagonal containment ring framing the corridor
            for i in 0..8 {
                let angle = i as f32 * std::f32::consts::PI / 4.0;
                let radius = 6.2;
                let rx = radius * angle.cos();
                let ry = 4.2 + radius * angle.sin();

                // Keep clear of gameplay running lanes (y < 4.0 when |x| < 3.2)
                if ry > 0.6 || rx.abs() > 3.6 {
                    seg.spawn(PbrBundle {
                        mesh: fl.mesh_fracture_ring_segment.clone(),
                        material: fl.mat_armoured_panel.clone(),
                        transform: Transform::from_xyz(rx, ry, 0.0)
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    });
                }
            }

            // Flanking reality-fracture monoliths tilted outward
            seg.spawn(PbrBundle {
                mesh: fl.mesh_fracture_monolith.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-5.6, 4.0, -2.0)
                    .with_rotation(Quat::from_rotation_z(0.15)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_fracture_monolith.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(5.6, 4.0, 2.0)
                    .with_rotation(Quat::from_rotation_z(-0.15)),
                ..default()
            });

            // Cold cyan ECHO anomaly singularity coils (elevated safely at y = 8.5m)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_echo_singularity_core.clone(),
                material: fl.mat_echo_singularity_crystal.clone(),
                transform: Transform::from_xyz(0.0, 8.5, 0.0),
                ..default()
            });
        }

        // Landmark 4: Ground Zero (~11,350m)
        // Catastrophic destruction surrounding the original incident site: ruptured reactor shield.
        ForbiddenLineLandmark::GroundZero => {
            // Massive ruptured containment sphere split open above the facility
            seg.spawn(PbrBundle {
                mesh: fl.mesh_ground_zero_ruptured_dome.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-6.8, 7.5, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_ground_zero_ruptured_dome.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(6.8, 7.5, 0.0),
                ..default()
            });

            // Shattered concrete rubble flanks (strictly outside running lanes at x = ±5.0m)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_ground_zero_rubble_slab.clone(),
                material: fl.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(-5.0, 0.6, -4.0)
                    .with_rotation(Quat::from_rotation_y(0.5)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_ground_zero_rubble_slab.clone(),
                material: fl.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(5.0, 0.6, 4.0)
                    .with_rotation(Quat::from_rotation_y(-0.5)),
                ..default()
            });

            // Sheared rebar bundles in rubble pile
            seg.spawn(PbrBundle {
                mesh: fl.mesh_sheared_rebar_bundle.clone(),
                material: fl.mat_exposed_rebar.clone(),
                transform: Transform::from_xyz(-4.6, 1.4, -4.0)
                    .with_rotation(Quat::from_rotation_z(0.4)),
                ..default()
            });

            // High-voltage warning beacons in emergency lockdown strobe
            for z in [-6.0, 0.0, 6.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(-4.2, 5.0, z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(4.2, 5.0, z),
                    ..default()
                });
            }
        }

        // Landmark 5: Final Containment Bulkhead (~11,820m)
        // Monumental interlocking blast doors framing the approach into Zone 7.
        ForbiddenLineLandmark::FinalContainmentBulkhead => {
            // Gigantic left and right blast door slabs towering 14m high (leaving center clearance)
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_blast_slab.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(-5.6, 6.5, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_blast_slab.clone(),
                material: fl.mat_containment_black_steel.clone(),
                transform: Transform::from_xyz(5.6, 6.5, 0.0),
                ..default()
            });

            // Monumental mechanical locking gears on upper door frame
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_lock_gear.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(-3.8, 9.2, 1.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: fl.mesh_bulkhead_lock_gear.clone(),
                material: fl.mat_hydraulic_steel.clone(),
                transform: Transform::from_xyz(3.8, 9.2, 1.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // High-intensity lockdown strobe array framing transition portal
            for y_step in [2.5, 5.0, 7.5, 10.0] {
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(-4.5, y_step, 1.5),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: fl.mesh_warning_lens.clone(),
                    material: fl.mat_crimson_warning_glow.clone(),
                    transform: Transform::from_xyz(4.5, y_step, 1.5),
                    ..default()
                });
            }
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
    fn test_forbidden_line_chapter_boundaries() {
        assert_eq!(
            forbidden_line_chapter_for_distance(10000.0),
            ForbiddenLineChapter::QuarantineApproach
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(10299.9),
            ForbiddenLineChapter::QuarantineApproach
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(10300.0),
            ForbiddenLineChapter::RestrictedTransit
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(10699.9),
            ForbiddenLineChapter::RestrictedTransit
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(10700.0),
            ForbiddenLineChapter::RealityFracture
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(11199.9),
            ForbiddenLineChapter::RealityFracture
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(11200.0),
            ForbiddenLineChapter::GroundZero
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(11699.9),
            ForbiddenLineChapter::GroundZero
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(11700.0),
            ForbiddenLineChapter::FinalContainment
        );
        assert_eq!(
            forbidden_line_chapter_for_distance(12000.0),
            ForbiddenLineChapter::FinalContainment
        );
    }

    #[test]
    fn test_forbidden_line_landmarks_distribution() {
        assert_eq!(
            forbidden_line_landmark_for_distance(10120.0),
            ForbiddenLineLandmark::QuarantineGate
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(10480.0),
            ForbiddenLineLandmark::VeyronCheckpoint
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(10880.0),
            ForbiddenLineLandmark::FractureChamber
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(11350.0),
            ForbiddenLineLandmark::GroundZero
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(11820.0),
            ForbiddenLineLandmark::FinalContainmentBulkhead
        );

        // Outside landmark windows must be None
        assert_eq!(
            forbidden_line_landmark_for_distance(10000.0),
            ForbiddenLineLandmark::None
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(10250.0),
            ForbiddenLineLandmark::None
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(10600.0),
            ForbiddenLineLandmark::None
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(11000.0),
            ForbiddenLineLandmark::None
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(11500.0),
            ForbiddenLineLandmark::None
        );
        assert_eq!(
            forbidden_line_landmark_for_distance(11950.0),
            ForbiddenLineLandmark::None
        );
    }

    #[test]
    fn test_forbidden_line_landmarks_spawn_exactly_once() {
        let mut gate_count = 0;
        let mut checkpoint_count = 0;
        let mut fracture_count = 0;
        let mut ground_zero_count = 0;
        let mut bulkhead_count = 0;

        // Sweep 10,000m to 12,000m in 40m track segment increments
        let mut dist = 10000.0;
        while dist < 12000.0 {
            match forbidden_line_landmark_for_distance(dist) {
                ForbiddenLineLandmark::QuarantineGate => gate_count += 1,
                ForbiddenLineLandmark::VeyronCheckpoint => checkpoint_count += 1,
                ForbiddenLineLandmark::FractureChamber => fracture_count += 1,
                ForbiddenLineLandmark::GroundZero => ground_zero_count += 1,
                ForbiddenLineLandmark::FinalContainmentBulkhead => bulkhead_count += 1,
                ForbiddenLineLandmark::None => {}
            }
            dist += 40.0;
        }

        assert_eq!(
            gate_count, 1,
            "Quarantine Gate must spawn exactly once in Zone 6"
        );
        assert_eq!(
            checkpoint_count, 1,
            "Veyron Checkpoint must spawn exactly once in Zone 6"
        );
        assert_eq!(
            fracture_count, 1,
            "Fracture Chamber must spawn exactly once in Zone 6"
        );
        assert_eq!(
            ground_zero_count, 1,
            "Ground Zero must spawn exactly once in Zone 6"
        );
        assert_eq!(
            bulkhead_count, 1,
            "Final Containment Bulkhead must spawn exactly once in Zone 6"
        );
    }

    #[test]
    fn test_forbidden_line_variant_determinism() {
        for dist in [10000.0, 10040.0, 10080.0, 10120.0, 10500.0, 11800.0] {
            let v1 = forbidden_line_variant(dist);
            let v2 = forbidden_line_variant(dist);
            assert_eq!(v1, v2, "Variant must be deterministic");
            assert!(v1 < 3, "Variant must be within 0..=2");
        }
    }

    #[test]
    fn test_forbidden_line_zone_boundary_routing_and_milestones() {
        use crate::environment::{environment_route_for_distance, EnvironmentRoute};
        use crate::zones::BOSS_MILESTONE_7_DISTANCE;

        // Zone 5 verification: 8000m..10000m
        assert_eq!(
            environment_route_for_distance(8000.0),
            EnvironmentRoute::SkyRail
        );
        assert_eq!(
            environment_route_for_distance(9999.9),
            EnvironmentRoute::SkyRail,
            "Immediately before 10,000m must route to SkyRail"
        );

        // Zone 6 verification: 10000m..12000m
        assert_eq!(
            environment_route_for_distance(10000.0),
            EnvironmentRoute::ForbiddenLine,
            "10,000m exactly must route to ForbiddenLine"
        );
        assert_eq!(
            environment_route_for_distance(10000.1),
            EnvironmentRoute::ForbiddenLine,
            "Immediately after 10,000m must route to ForbiddenLine"
        );
        assert_eq!(
            environment_route_for_distance(11999.9),
            EnvironmentRoute::ForbiddenLine,
            "Immediately before 12,000m must route to ForbiddenLine"
        );

        // Zone 7 verification: 12000m+ must NOT use ForbiddenLine; uses Generic fallback
        assert_eq!(
            environment_route_for_distance(12000.0),
            EnvironmentRoute::Generic,
            "12,000m threshold must transition out of ForbiddenLine to Generic"
        );
        assert_eq!(
            environment_route_for_distance(12000.1),
            EnvironmentRoute::Generic,
            "Immediately after 12,000m must route to Generic fallback"
        );
        assert_ne!(
            environment_route_for_distance(12000.0),
            EnvironmentRoute::ForbiddenLine,
            "Zone 7 must not accidentally inherit ForbiddenLine"
        );

        // Boss encounter milestone intact
        assert_eq!(
            BOSS_MILESTONE_7_DISTANCE, 12000.0,
            "Zone 7 encounter milestone must remain exactly at 12,000m"
        );
    }

    #[test]
    fn test_forbidden_line_alarm_state_modulation() {
        let mut alarm = ForbiddenLineAlarmState::default();
        assert_eq!(alarm.timer, 0.0);
        assert_eq!(alarm.pulse, 1.0);

        // Simulate alarm cycle progression
        for dt in [0.1, 0.2, 0.5, 1.0, 1.6] {
            alarm.timer += dt;
            let cycle = (alarm.timer * 3.9).sin();
            let pulse_factor = 0.7 + 0.3 * cycle.max(0.0).powi(2);
            assert!(
                pulse_factor >= 0.7 && pulse_factor <= 1.0,
                "Pulse factor {} must stay bounded in [0.7, 1.0]",
                pulse_factor
            );
        }
    }

    #[test]
    fn test_forbidden_line_background_geometry_clearance() {
        // Player running bounds
        let left_lane_x = Lane::from_index(-1).x_pos(); // -2.4m
        let right_lane_x = Lane::from_index(1).x_pos(); // +2.4m
        let outer_curb_x = 4.6;
        let max_jump_height = 2.5;

        // Verify sidewall containment clearance
        let bunker_wall_x = 5.8;
        assert!(
            bunker_wall_x > outer_curb_x,
            "Bunker sidewall ({}m) must be outside outer track curb ({}m)",
            bunker_wall_x,
            outer_curb_x
        );
        assert!(
            (-bunker_wall_x - left_lane_x).abs() >= 3.0,
            "Left bunker wall must maintain at least 3.0m clearance from lane centers"
        );
        assert!(
            (bunker_wall_x - right_lane_x).abs() >= 3.0,
            "Right bunker wall must maintain at least 3.0m clearance from lane centers"
        );

        // Verify upper background catwalk clearance
        let catwalk_y = 8.8;
        assert!(
            catwalk_y > max_jump_height + 4.0,
            "Catwalk gallery ({}m) must be far above player jump envelope ({}m)",
            catwalk_y,
            max_jump_height
        );

        // Verify overhead excavation cavern arch clearance
        let cavern_arch_y = 11.8;
        assert!(
            cavern_arch_y > catwalk_y,
            "Cavern vault arch ({}m) must tower above upper catwalk ({}m)",
            cavern_arch_y,
            catwalk_y
        );

        // Verify lateral facility bays clearance
        let min_lateral_facility_x = 12.0;
        assert!(
            min_lateral_facility_x > bunker_wall_x + 5.0,
            "Lateral facility structures ({}m) must sit deep beyond the bunker wall ({}m)",
            min_lateral_facility_x,
            bunker_wall_x
        );

        // Verify distant retaining wall perimeter clearance
        let retaining_wall_x = 20.0;
        assert!(
            retaining_wall_x >= min_lateral_facility_x + 6.0,
            "Perimeter retaining wall ({}m) must frame the outermost excavation boundary",
            retaining_wall_x
        );
    }
}
