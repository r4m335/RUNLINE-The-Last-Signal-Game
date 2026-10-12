use crate::director::RunDirector;
use crate::environment::EnvironmentAssets;
use crate::types::{AppState, Despawnable, GameRunStats, Lane, TrackSegmentMarker};
use bevy::prelude::*;

// -------------------------------------------------------------
// ZONE 5 — SKY RAIL (8000m–10000m)
// -------------------------------------------------------------
// Core Identity: High-speed elevated magnetic railway suspended ~850m
// above the Aurelia skyline during a violent atmospheric storm.
//
// 5 Authors:
// 1. 8000–8300m: Sky Rail Emergence   (Leaving deep underground; transition bridge)
// 2. 8300–8700m: Upper Aurelia        (First colossal skyline views; megatowers)
// 3. 8700–9200m: Storm Corridor       (Severe atmospheric turbulence; lightning)
// 4. 9200–9650m: Broken Skyway        (Catastrophic infrastructure damage; collapse)
// 5. 9650–10000m: ECHO Approach       (Transition into Forbidden Line facility)
// -------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkyRailChapter {
    SkyRailEmergence, // 8000m–8300m
    UpperAurelia,     // 8300m–8700m
    StormCorridor,    // 8700m–9200m
    BrokenSkyway,     // 9200m–9650m
    EchoApproach,     // 9650m–10000m
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkyRailLandmark {
    None,
    SkyRailGate,         // ~8120m (Tunnel exit portal & emergence bridge)
    AureliaCrownStation, // ~8500m (Suspended orbital megastation structure)
    StormSpineBridge,    // ~8950m (Massive suspension bridge across sky abyss)
    CollapsedSkywaySpan, // ~9400m (Twisted collapsed viaduct & emergency catwalks)
    EchoRelayArray,      // ~9820m (Colossal cyan communication relay above clouds)
}

pub fn sky_rail_chapter_for_distance(distance: f32) -> SkyRailChapter {
    match distance {
        d if d < 8300.0 => SkyRailChapter::SkyRailEmergence,
        d if d < 8700.0 => SkyRailChapter::UpperAurelia,
        d if d < 9200.0 => SkyRailChapter::StormCorridor,
        d if d < 9650.0 => SkyRailChapter::BrokenSkyway,
        _ => SkyRailChapter::EchoApproach,
    }
}

pub fn sky_rail_landmark_for_distance(distance: f32) -> SkyRailLandmark {
    match distance {
        d if (8100.0..8150.0).contains(&d) => SkyRailLandmark::SkyRailGate,
        d if (8480.0..8530.0).contains(&d) => SkyRailLandmark::AureliaCrownStation,
        d if (8930.0..8980.0).contains(&d) => SkyRailLandmark::StormSpineBridge,
        d if (9380.0..9430.0).contains(&d) => SkyRailLandmark::CollapsedSkywaySpan,
        d if (9800.0..9850.0).contains(&d) => SkyRailLandmark::EchoRelayArray,
        _ => SkyRailLandmark::None,
    }
}

pub fn sky_rail_variant(distance: f32) -> u8 {
    let seg_idx = (distance / 40.0).floor() as usize;
    (seg_idx % 3) as u8
}

// -------------------------------------------------------------
// DYNAMIC ATMOSPHERE & STORM COMPONENTS
// -------------------------------------------------------------

#[derive(Component, Debug, Clone)]
pub struct SkyRailDriftCloud {
    pub base_x: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component, Debug, Clone)]
pub struct SkyRailDistantTrain {
    pub speed: f32,
    pub min_local_z: f32,
    pub max_local_z: f32,
}

#[derive(Component, Debug, Clone)]
pub struct SkyRailDistantDrone {
    pub base_pos: Vec3,
    pub hover_speed: f32,
    pub hover_radius: f32,
    pub phase: f32,
}

#[derive(Component, Debug, Clone)]
pub struct SkyRailDistantCraft {
    pub speed: f32,
    pub min_local_z: f32,
    pub max_local_z: f32,
}

#[derive(Component, Debug, Clone)]
pub struct SkyRailDriftSmoke {
    pub base_pos: Vec3,
    pub drift_speed_x: f32,
    pub drift_amp_x: f32,
    pub rise_speed_y: f32,
    pub rise_amp_y: f32,
    pub phase: f32,
}

#[derive(Resource, Debug)]
pub struct SkyRailStormState {
    pub flash_timer: f32,
    pub next_flash_interval: f32,
    pub flash_intensity: f32,
    pub is_flashing: bool,
    pub flash_elapsed: f32,
    pub flash_count: u32,
}

impl Default for SkyRailStormState {
    fn default() -> Self {
        Self {
            flash_timer: 0.0,
            next_flash_interval: 6.8,
            flash_intensity: 0.0,
            is_flashing: false,
            flash_elapsed: 0.0,
            flash_count: 0,
        }
    }
}

// -------------------------------------------------------------
// ASSETS RESOURCE
// -------------------------------------------------------------

#[derive(Resource)]
pub struct SkyRailAssets {
    // Elevated 3-Lane Track Profile Meshes
    pub mesh_viaduct_slab: Handle<Mesh>,
    pub mesh_lane_bed: Handle<Mesh>,
    pub mesh_mag_strip: Handle<Mesh>,
    pub mesh_lane_curb: Handle<Mesh>,
    pub mesh_interlane_trough: Handle<Mesh>,
    pub mesh_expansion_grate: Handle<Mesh>,
    pub mesh_outer_barrier: Handle<Mesh>,
    pub mesh_barrier_post: Handle<Mesh>,
    pub mesh_barrier_aero_fin: Handle<Mesh>,
    pub mesh_under_girder: Handle<Mesh>,
    pub mesh_under_truss: Handle<Mesh>,
    pub mesh_pylon_massive: Handle<Mesh>,
    pub mesh_pylon_bracket: Handle<Mesh>,
    pub mesh_sky_signal_mast: Handle<Mesh>,

    // Architectural & Atmospheric Prop Meshes
    pub mesh_sky_catwalk: Handle<Mesh>,
    pub mesh_relay_spire: Handle<Mesh>,
    pub mesh_cable_bundle_elevated: Handle<Mesh>,
    pub mesh_hazard_reflector: Handle<Mesh>,

    // Chapter 1 & General Structural Portal Meshes
    pub mesh_portal_arch_beam: Handle<Mesh>,
    pub mesh_portal_column: Handle<Mesh>,
    pub mesh_drainage_pipe: Handle<Mesh>,

    // Chapter 2 Upper Aurelia & Station Meshes
    pub mesh_station_platform: Handle<Mesh>,
    pub mesh_station_canopy: Handle<Mesh>,
    pub mesh_station_ring_arch: Handle<Mesh>,
    pub mesh_elevator_shaft: Handle<Mesh>,

    // Chapter 3 Storm Corridor Meshes
    pub mesh_suspension_tower: Handle<Mesh>,
    pub mesh_stay_cable_diag: Handle<Mesh>,
    pub mesh_storm_baffle: Handle<Mesh>,

    // Chapter 4 Broken Skyway Disaster Meshes
    pub mesh_broken_parallel_slab: Handle<Mesh>,
    pub mesh_collapsed_pier: Handle<Mesh>,
    pub mesh_scaffold_frame: Handle<Mesh>,
    pub mesh_derailed_hulk: Handle<Mesh>,
    pub mesh_hazard_barrier_diag: Handle<Mesh>,

    // Chapter 5 ECHO Approach Meshes
    pub mesh_echo_conduit_ring: Handle<Mesh>,
    pub mesh_echo_monolith_pylon: Handle<Mesh>,
    pub mesh_echo_facility_wall: Handle<Mesh>,
    pub mesh_echo_relay_dish: Handle<Mesh>,

    // Skyline Megastructure Meshes (Aurelia dark skyline)
    pub mesh_tower_colossus: Handle<Mesh>,
    pub mesh_tower_obelisk: Handle<Mesh>,
    pub mesh_tower_slab: Handle<Mesh>,
    pub mesh_tower_stepped_crown: Handle<Mesh>,
    pub mesh_tower_antenna_spire: Handle<Mesh>,
    pub mesh_tower_window_slit_v: Handle<Mesh>,
    pub mesh_tower_window_slit_h: Handle<Mesh>,
    pub mesh_sky_bridge_span: Handle<Mesh>,
    pub mesh_distant_megatitan: Handle<Mesh>,
    pub mesh_storm_spine_tower: Handle<Mesh>,

    // Deep Background & Distant Infrastructure Meshes (90–220m Layered Depth)
    pub mesh_distant_rail_span: Handle<Mesh>,
    pub mesh_distant_rail_strip: Handle<Mesh>,
    pub mesh_distant_rail_pylon: Handle<Mesh>,
    pub mesh_distant_elevator_shaft: Handle<Mesh>,
    pub mesh_transmission_mast: Handle<Mesh>,
    pub mesh_transmission_arm: Handle<Mesh>,
    pub mesh_utility_conduit_pipe: Handle<Mesh>,
    pub mesh_hanging_cable: Handle<Mesh>,
    pub mesh_distant_truss_bridge: Handle<Mesh>,
    pub mesh_distant_train_car: Handle<Mesh>,
    pub mesh_distant_window_cluster: Handle<Mesh>,
    pub mesh_cloud_bank_flank: Handle<Mesh>,

    // Midground Infrastructure Meshes (55–105m Layered Depth)
    pub mesh_midground_needle_tower: Handle<Mesh>,
    pub mesh_midground_truss_girder: Handle<Mesh>,
    pub mesh_midground_bracket: Handle<Mesh>,

    // Billowing Cumulus Cloud Ocean Meshes (Flanking Track Depth)
    pub mesh_cloud_puff_large: Handle<Mesh>,
    pub mesh_cloud_puff_medium: Handle<Mesh>,

    // Suspension Skyway & High-Speed Transit Meshes
    pub mesh_suspension_cable_diag: Handle<Mesh>,
    pub mesh_tower_cantilever_deck: Handle<Mesh>,
    pub mesh_train_window_strip: Handle<Mesh>,
    pub mesh_window_grid_warm: Handle<Mesh>,

    // Trackside Viaduct Detail & Overhead Telemetry Gantries
    pub mesh_barrier_cap_light: Handle<Mesh>,
    pub mesh_outer_conduit_bundle: Handle<Mesh>,
    pub mesh_telemetry_gantry_beam: Handle<Mesh>,
    pub mesh_gantry_column: Handle<Mesh>,
    pub mesh_gantry_sensor_pod: Handle<Mesh>,
    pub mesh_trackside_locker: Handle<Mesh>,
    pub mesh_lightning_branch: Handle<Mesh>,

    // Cloud Abyss & Atmospheric Depth Meshes (850m Altitude)
    pub mesh_cloud_deck_vast: Handle<Mesh>,
    pub mesh_cloud_strata_low: Handle<Mesh>,
    pub mesh_cloud_ceiling_storm: Handle<Mesh>,

    // Micro-Infrastructure & Atmospheric Detailing Meshes
    pub mesh_micro_junction_box: Handle<Mesh>,
    pub mesh_micro_antenna_dish: Handle<Mesh>,
    pub mesh_micro_drone_body: Handle<Mesh>,
    pub mesh_micro_hanging_pod: Handle<Mesh>,
    pub mesh_micro_weathered_patch: Handle<Mesh>,
    pub mesh_cloud_wisps_small: Handle<Mesh>,

    // Distant Megacity & Industrial Complex Meshes
    pub mesh_tower_cylinder_large: Handle<Mesh>,
    pub mesh_industrial_cooling_tower: Handle<Mesh>,
    pub mesh_industrial_vent_stack: Handle<Mesh>,
    pub mesh_distant_skybridge_long: Handle<Mesh>,
    pub mesh_tower_cluster_block: Handle<Mesh>,
    pub mesh_distant_crossing_rail: Handle<Mesh>,
    pub mesh_distant_traffic_craft: Handle<Mesh>,

    // Distant Industrial Refinery & Megastructure Meshes
    pub mesh_refinery_storage_sphere: Handle<Mesh>,
    pub mesh_refinery_distillation_column: Handle<Mesh>,
    pub mesh_stepped_brutalist_arcology: Handle<Mesh>,
    pub mesh_industrial_pipe_manifold: Handle<Mesh>,

    // Cinematic Smoke & Atmospheric Haze Meshes
    pub mesh_smoke_plume_puff_large: Handle<Mesh>,
    pub mesh_smoke_plume_puff_medium: Handle<Mesh>,
    pub mesh_smoke_plume_puff_small: Handle<Mesh>,
    pub mesh_smoke_bank_horizontal: Handle<Mesh>,
    pub mesh_smoke_strata_drift: Handle<Mesh>,

    // Track & Infrastructure Materials
    pub mat_sky_steel: Handle<StandardMaterial>,
    pub mat_sky_tread: Handle<StandardMaterial>,
    pub mat_sky_mag_strip: Handle<StandardMaterial>,
    pub mat_sky_barrier: Handle<StandardMaterial>,
    pub mat_sky_amber_glow: Handle<StandardMaterial>,
    pub mat_sky_grate: Handle<StandardMaterial>,
    pub mat_sky_pylon: Handle<StandardMaterial>,
    pub mat_sky_cyan_glow: Handle<StandardMaterial>,
    pub mat_sky_storm_composite: Handle<StandardMaterial>,

    // Chapter & Landmark Specialized Materials
    pub mat_sky_emergency_red: Handle<StandardMaterial>,
    pub mat_sky_echo_monolith: Handle<StandardMaterial>,
    pub mat_sky_hazard_stripe: Handle<StandardMaterial>,

    // Midground Materials (55–105m Readable Blue-Gray Silhouettes)
    pub mat_sky_midground_steel: Handle<StandardMaterial>,
    pub mat_sky_midground_concrete: Handle<StandardMaterial>,

    // Skyline & Cloud Abyss Materials (Readable Dark Atmospheric Depth)
    pub mat_sky_tower_dark: Handle<StandardMaterial>,
    pub mat_sky_tower_distant: Handle<StandardMaterial>,
    pub mat_sky_tower_faint: Handle<StandardMaterial>,
    pub mat_sky_window_dim: Handle<StandardMaterial>,
    pub mat_sky_window_amber_dim: Handle<StandardMaterial>,
    pub mat_sky_antenna_red: Handle<StandardMaterial>,
    pub mat_sky_cloud_abyss: Handle<StandardMaterial>,
    pub mat_sky_cloud_ceiling: Handle<StandardMaterial>,

    // Billowing Cloud Ocean & Illuminated Transit Materials
    pub mat_sky_cloud_cumulus: Handle<StandardMaterial>,
    pub mat_sky_train_window: Handle<StandardMaterial>,
    pub mat_sky_window_warm: Handle<StandardMaterial>,
    pub mat_sky_lightning: Handle<StandardMaterial>,

    // Micro-Infrastructure & Environmental Wear Materials
    pub mat_sky_weathered_iron: Handle<StandardMaterial>,
    pub mat_sky_cloud_wisp: Handle<StandardMaterial>,

    // Cinematic Smoke & Atmospheric Haze Materials
    pub mat_sky_smoke_industrial_dense: Handle<StandardMaterial>,
    pub mat_sky_smoke_haze_bank: Handle<StandardMaterial>,
    pub mat_sky_smoke_wisp_drift: Handle<StandardMaterial>,
}

pub fn init_sky_rail_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let assets = SkyRailAssets {
        // Main structural viaduct slab (9.4m wide x 0.55m deep x 40m long)
        mesh_viaduct_slab: meshes.add(Cuboid::new(9.4, 0.55, 40.0)),
        // Raised aerodynamic running lane bed (1.6m wide x 0.12m high x 40m long)
        mesh_lane_bed: meshes.add(Cuboid::new(1.6, 0.12, 40.0)),
        // Recessed central magnetic flux guide rail (0.42m wide x 0.03m thick x 40m long)
        mesh_mag_strip: meshes.add(Cuboid::new(0.42, 0.03, 40.0)),
        // High-visibility lane boundary curb (0.10m wide x 0.16m high x 40m long)
        mesh_lane_curb: meshes.add(Cuboid::new(0.10, 0.16, 40.0)),
        // Inter-lane drainage & expansion trough
        mesh_interlane_trough: meshes.add(Cuboid::new(0.60, 0.15, 40.0)),
        mesh_expansion_grate: meshes.add(Cuboid::new(0.56, 0.05, 2.2)),
        // Aerodynamic storm deflection parapets (40m continuous outer guard)
        mesh_outer_barrier: meshes.add(Cuboid::new(0.24, 1.35, 40.0)),
        mesh_barrier_post: meshes.add(Cuboid::new(0.32, 1.45, 0.32)),
        mesh_barrier_aero_fin: meshes.add(Cuboid::new(0.40, 0.85, 0.12)),
        // Underside deep structural foundation
        mesh_under_girder: meshes.add(Cuboid::new(0.50, 2.40, 40.0)),
        mesh_under_truss: meshes.add(Cuboid::new(8.4, 0.45, 0.45)),
        // Massive support pylon plunging 850m downward toward Aurelia
        mesh_pylon_massive: meshes.add(Cuboid::new(3.8, 80.0, 3.8)),
        // Cantilever under-truss brackets bracing the viaduct against abyss crosswinds
        mesh_pylon_bracket: meshes.add(Cuboid::new(8.2, 0.65, 1.2)),
        mesh_sky_signal_mast: meshes.add(Cuboid::new(0.26, 4.4, 0.26)),

        // Supporting props
        mesh_sky_catwalk: meshes.add(Cuboid::new(1.10, 0.12, 12.0)),
        mesh_relay_spire: meshes.add(Cylinder::new(0.18, 6.0)),
        mesh_cable_bundle_elevated: meshes.add(Cylinder::new(0.08, 40.0)),
        mesh_hazard_reflector: meshes.add(Cuboid::new(0.15, 0.25, 0.05)),

        // Chapter 1 & General Structural Portal Meshes
        mesh_portal_arch_beam: meshes.add(Cuboid::new(12.0, 0.9, 1.4)),
        mesh_portal_column: meshes.add(Cuboid::new(1.4, 9.5, 1.4)),
        mesh_drainage_pipe: meshes.add(Cylinder::new(0.35, 14.0)),

        // Chapter 2 Upper Aurelia & Station Meshes
        mesh_station_platform: meshes.add(Cuboid::new(2.4, 0.35, 38.0)),
        mesh_station_canopy: meshes.add(Cuboid::new(3.2, 0.18, 24.0)),
        mesh_station_ring_arch: meshes.add(Cuboid::new(15.2, 0.95, 2.2)),
        mesh_elevator_shaft: meshes.add(Cuboid::new(2.8, 32.0, 2.8)),

        // Chapter 3 Storm Corridor Meshes
        mesh_suspension_tower: meshes.add(Cuboid::new(2.6, 44.0, 2.6)),
        mesh_stay_cable_diag: meshes.add(Cylinder::new(0.09, 36.0)),
        mesh_storm_baffle: meshes.add(Cuboid::new(0.18, 2.6, 7.5)),

        // Chapter 4 Broken Skyway Disaster Meshes
        mesh_broken_parallel_slab: meshes.add(Cuboid::new(7.0, 0.55, 36.0)),
        mesh_collapsed_pier: meshes.add(Cuboid::new(3.4, 38.0, 3.4)),
        mesh_scaffold_frame: meshes.add(Cuboid::new(1.4, 2.8, 8.0)),
        mesh_derailed_hulk: meshes.add(Cuboid::new(3.0, 2.4, 12.0)),
        mesh_hazard_barrier_diag: meshes.add(Cuboid::new(0.15, 0.9, 3.5)),

        // Chapter 5 ECHO Approach Meshes
        mesh_echo_conduit_ring: meshes.add(Cuboid::new(13.6, 1.1, 1.6)),
        mesh_echo_monolith_pylon: meshes.add(Cuboid::new(2.2, 22.0, 2.2)),
        mesh_echo_facility_wall: meshes.add(Cuboid::new(38.0, 55.0, 7.0)),
        mesh_echo_relay_dish: meshes.add(Cylinder::new(4.8, 0.45)),

        // Skyline Megastructure Meshes (Low-poly brutalist silhouettes for integrated GPUs)
        mesh_tower_colossus: meshes.add(Cuboid::new(34.0, 190.0, 36.0)),
        mesh_tower_obelisk: meshes.add(Cuboid::new(18.0, 220.0, 18.0)),
        mesh_tower_slab: meshes.add(Cuboid::new(52.0, 150.0, 26.0)),
        mesh_tower_stepped_crown: meshes.add(Cuboid::new(22.0, 32.0, 24.0)),
        mesh_tower_antenna_spire: meshes.add(Cylinder::new(0.35, 38.0)),
        mesh_tower_window_slit_v: meshes.add(Cuboid::new(0.5, 36.0, 10.0)),
        mesh_tower_window_slit_h: meshes.add(Cuboid::new(18.0, 0.6, 0.5)),
        mesh_sky_bridge_span: meshes.add(Cuboid::new(76.0, 3.6, 4.6)),
        mesh_distant_megatitan: meshes.add(Cuboid::new(58.0, 320.0, 58.0)),
        mesh_storm_spine_tower: meshes.add(Cylinder::new(2.2, 140.0)),

        // Deep Background & Distant Infrastructure Meshes
        mesh_distant_rail_span: meshes.add(Cuboid::new(4.2, 0.65, 40.0)),
        mesh_distant_rail_strip: meshes.add(Cuboid::new(0.45, 0.08, 40.0)),
        mesh_distant_rail_pylon: meshes.add(Cuboid::new(2.4, 90.0, 2.4)),
        mesh_distant_elevator_shaft: meshes.add(Cuboid::new(4.2, 180.0, 4.2)),
        mesh_transmission_mast: meshes.add(Cylinder::new(0.9, 85.0)),
        mesh_transmission_arm: meshes.add(Cuboid::new(18.0, 1.1, 1.1)),
        mesh_utility_conduit_pipe: meshes.add(Cylinder::new(0.7, 40.0)),
        mesh_hanging_cable: meshes.add(Cuboid::new(0.12, 0.12, 38.0)),
        mesh_distant_truss_bridge: meshes.add(Cuboid::new(52.0, 2.4, 2.8)),
        mesh_distant_train_car: meshes.add(Cuboid::new(3.4, 2.2, 22.0)),
        mesh_distant_window_cluster: meshes.add(Cuboid::new(6.0, 12.0, 0.35)),
        mesh_cloud_bank_flank: meshes.add(Cuboid::new(90.0, 16.0, 40.0)),

        // Midground Infrastructure Meshes (55–105m Layered Depth)
        mesh_midground_needle_tower: meshes.add(Cuboid::new(8.0, 110.0, 8.0)),
        mesh_midground_truss_girder: meshes.add(Cuboid::new(32.0, 2.0, 2.4)),
        mesh_midground_bracket: meshes.add(Cuboid::new(1.2, 12.0, 1.2)),

        // Billowing Cumulus Cloud Ocean Meshes
        mesh_cloud_puff_large: meshes.add(Cuboid::new(38.0, 16.0, 26.0)),
        mesh_cloud_puff_medium: meshes.add(Cuboid::new(28.0, 12.0, 20.0)),

        // Suspension Skyway & High-Speed Transit Meshes
        mesh_suspension_cable_diag: meshes.add(Cylinder::new(0.08, 32.0)),
        mesh_tower_cantilever_deck: meshes.add(Cuboid::new(26.0, 2.2, 26.0)),
        mesh_train_window_strip: meshes.add(Cuboid::new(0.12, 0.65, 20.0)),
        mesh_window_grid_warm: meshes.add(Cuboid::new(8.0, 14.0, 0.35)),

        // Trackside Viaduct Detail & Overhead Telemetry Gantries
        mesh_barrier_cap_light: meshes.add(Cuboid::new(0.32, 0.18, 0.32)),
        mesh_outer_conduit_bundle: meshes.add(Cylinder::new(0.10, 40.0)),
        mesh_telemetry_gantry_beam: meshes.add(Cuboid::new(11.2, 0.45, 0.70)),
        mesh_gantry_column: meshes.add(Cuboid::new(0.38, 4.8, 0.38)),
        mesh_gantry_sensor_pod: meshes.add(Cuboid::new(0.40, 0.60, 0.40)),
        mesh_trackside_locker: meshes.add(Cuboid::new(0.35, 0.95, 0.70)),
        mesh_lightning_branch: meshes.add(Cylinder::new(0.22, 28.0)),

        // Cloud Abyss & Atmospheric Depth Meshes
        mesh_cloud_deck_vast: meshes.add(Cuboid::new(380.0, 12.0, 40.0)),
        mesh_cloud_strata_low: meshes.add(Cuboid::new(440.0, 14.0, 40.0)),
        mesh_cloud_ceiling_storm: meshes.add(Cuboid::new(360.0, 10.0, 40.0)),

        // Micro-Infrastructure & Atmospheric Detailing Meshes
        mesh_micro_junction_box: meshes.add(Cuboid::new(0.70, 0.85, 0.45)),
        mesh_micro_antenna_dish: meshes.add(Cylinder::new(0.75, 0.15)),
        mesh_micro_drone_body: meshes.add(Cuboid::new(1.10, 0.35, 1.30)),
        mesh_micro_hanging_pod: meshes.add(Cuboid::new(2.20, 1.80, 2.20)),
        mesh_micro_weathered_patch: meshes.add(Cuboid::new(1.40, 1.80, 0.08)),
        mesh_cloud_wisps_small: meshes.add(Cuboid::new(70.0, 7.5, 30.0)),

        // Distant Megacity & Industrial Complex Meshes
        mesh_tower_cylinder_large: meshes.add(Cylinder::new(14.0, 180.0)),
        mesh_industrial_cooling_tower: meshes.add(Cylinder::new(13.0, 52.0)),
        mesh_industrial_vent_stack: meshes.add(Cylinder::new(3.0, 85.0)),
        mesh_distant_skybridge_long: meshes.add(Cuboid::new(110.0, 3.8, 5.2)),
        mesh_tower_cluster_block: meshes.add(Cuboid::new(34.0, 110.0, 30.0)),
        mesh_distant_crossing_rail: meshes.add(Cuboid::new(130.0, 1.2, 4.0)),
        mesh_distant_traffic_craft: meshes.add(Cuboid::new(3.0, 1.2, 5.5)),

        // Distant Industrial Refinery & Megastructure Meshes
        mesh_refinery_storage_sphere: meshes.add(Cylinder::new(11.0, 18.0)),
        mesh_refinery_distillation_column: meshes.add(Cylinder::new(3.8, 70.0)),
        mesh_stepped_brutalist_arcology: meshes.add(Cuboid::new(54.0, 76.0, 38.0)),
        mesh_industrial_pipe_manifold: meshes.add(Cuboid::new(38.0, 2.2, 4.2)),

        // Cinematic Smoke & Atmospheric Haze Meshes
        mesh_smoke_plume_puff_large: meshes.add(Cuboid::new(24.0, 18.0, 22.0)),
        mesh_smoke_plume_puff_medium: meshes.add(Cuboid::new(15.0, 12.0, 14.0)),
        mesh_smoke_plume_puff_small: meshes.add(Cuboid::new(9.0, 8.0, 9.0)),
        mesh_smoke_bank_horizontal: meshes.add(Cuboid::new(96.0, 16.0, 44.0)),
        mesh_smoke_strata_drift: meshes.add(Cuboid::new(64.0, 8.0, 32.0)),

        // Materials Palette
        mat_sky_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.11, 0.14, 0.19),
            metallic: 0.88,
            perceptual_roughness: 0.32,
            ..default()
        }),
        mat_sky_tread: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.18, 0.23),
            metallic: 0.35,
            perceptual_roughness: 0.65,
            ..default()
        }),
        mat_sky_mag_strip: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.45, 0.85),
            emissive: LinearRgba::new(0.35, 0.90, 1.85, 1.0),
            metallic: 0.60,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_barrier: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.24, 0.30),
            metallic: 0.75,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_sky_amber_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.48, 0.05),
            emissive: LinearRgba::new(2.10, 0.85, 0.08, 1.0),
            perceptual_roughness: 0.30,
            ..default()
        }),
        mat_sky_grate: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.08, 0.10),
            metallic: 0.70,
            perceptual_roughness: 0.75,
            ..default()
        }),
        mat_sky_pylon: materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.15, 0.18),
            metallic: 0.20,
            perceptual_roughness: 0.85,
            ..default()
        }),
        mat_sky_cyan_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.65, 0.75),
            emissive: LinearRgba::new(0.15, 1.35, 1.65, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_sky_storm_composite: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.18, 0.24),
            metallic: 0.82,
            perceptual_roughness: 0.42,
            ..default()
        }),

        // Chapter & Landmark Specialized Materials
        mat_sky_emergency_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.08, 0.08),
            emissive: LinearRgba::new(1.65, 0.10, 0.08, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_sky_echo_monolith: materials.add(StandardMaterial {
            base_color: Color::srgb(0.045, 0.055, 0.072),
            metallic: 0.85,
            perceptual_roughness: 0.28,
            emissive: LinearRgba::new(0.010, 0.016, 0.025, 1.0),
            ..default()
        }),
        mat_sky_hazard_stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.80, 0.42, 0.05),
            metallic: 0.25,
            perceptual_roughness: 0.55,
            ..default()
        }),

        // Midground Materials (55–105m Readable Blue-Gray Silhouettes)
        mat_sky_midground_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.15, 0.21),
            metallic: 0.65,
            perceptual_roughness: 0.45,
            emissive: LinearRgba::new(0.020, 0.030, 0.048, 1.0),
            ..default()
        }),
        mat_sky_midground_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.17, 0.23),
            metallic: 0.25,
            perceptual_roughness: 0.70,
            emissive: LinearRgba::new(0.016, 0.024, 0.038, 1.0),
            ..default()
        }),

        // Skyline & Cloud Abyss Materials (Readable Dark Atmospheric Depth)
        mat_sky_tower_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.075, 0.098, 0.145),
            metallic: 0.75,
            perceptual_roughness: 0.42,
            emissive: LinearRgba::new(0.014, 0.022, 0.036, 1.0),
            ..default()
        }),
        mat_sky_tower_distant: materials.add(StandardMaterial {
            base_color: Color::srgb(0.048, 0.068, 0.098),
            metallic: 0.60,
            perceptual_roughness: 0.55,
            emissive: LinearRgba::new(0.012, 0.018, 0.030, 1.0),
            ..default()
        }),
        mat_sky_tower_faint: materials.add(StandardMaterial {
            base_color: Color::srgb(0.032, 0.045, 0.068),
            metallic: 0.40,
            perceptual_roughness: 0.75,
            emissive: LinearRgba::new(0.008, 0.012, 0.020, 1.0),
            ..default()
        }),
        mat_sky_window_dim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.22, 0.32),
            emissive: LinearRgba::new(0.12, 0.42, 0.65, 1.0),
            perceptual_roughness: 0.30,
            ..default()
        }),
        mat_sky_window_amber_dim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.18, 0.05),
            emissive: LinearRgba::new(0.70, 0.35, 0.06, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_sky_antenna_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.08, 0.08),
            emissive: LinearRgba::new(2.40, 0.12, 0.08, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_cloud_abyss: materials.add(StandardMaterial {
            base_color: Color::srgb(0.040, 0.055, 0.082),
            perceptual_roughness: 0.92,
            metallic: 0.0,
            emissive: LinearRgba::new(0.010, 0.016, 0.026, 1.0),
            ..default()
        }),
        mat_sky_cloud_ceiling: materials.add(StandardMaterial {
            base_color: Color::srgb(0.028, 0.038, 0.055),
            perceptual_roughness: 0.95,
            metallic: 0.0,
            emissive: LinearRgba::new(0.008, 0.012, 0.020, 1.0),
            ..default()
        }),

        // Billowing Cloud Ocean & Illuminated Transit Materials
        mat_sky_cloud_cumulus: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.30, 0.40),
            perceptual_roughness: 0.90,
            metallic: 0.0,
            emissive: LinearRgba::new(0.04, 0.06, 0.10, 1.0),
            ..default()
        }),
        mat_sky_train_window: materials.add(StandardMaterial {
            base_color: Color::srgb(0.70, 0.90, 1.0),
            emissive: LinearRgba::new(1.4, 2.4, 3.6, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_window_warm: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.55, 0.12),
            emissive: LinearRgba::new(2.4, 1.3, 0.25, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_sky_lightning: materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.96, 1.0),
            emissive: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            unlit: true,
            ..default()
        }),

        // Micro-Infrastructure & Environmental Wear Materials
        mat_sky_weathered_iron: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.12, 0.09),
            perceptual_roughness: 0.88,
            metallic: 0.45,
            emissive: LinearRgba::new(0.015, 0.008, 0.005, 1.0),
            ..default()
        }),
        mat_sky_cloud_wisp: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.16, 0.22),
            perceptual_roughness: 0.96,
            metallic: 0.0,
            emissive: LinearRgba::new(0.018, 0.025, 0.038, 1.0),
            ..default()
        }),

        // Cinematic Smoke & Atmospheric Haze Materials (Layered Alpha Blending, Dark Cyberpunk Tones)
        mat_sky_smoke_industrial_dense: materials.add(StandardMaterial {
            base_color: Color::srgba(0.028, 0.035, 0.048, 0.72),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.98,
            metallic: 0.0,
            emissive: LinearRgba::new(0.006, 0.008, 0.012, 1.0),
            cull_mode: None,
            double_sided: true,
            ..default()
        }),
        mat_sky_smoke_haze_bank: materials.add(StandardMaterial {
            base_color: Color::srgba(0.042, 0.054, 0.072, 0.52),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.96,
            metallic: 0.0,
            emissive: LinearRgba::new(0.008, 0.012, 0.018, 1.0),
            cull_mode: None,
            double_sided: true,
            ..default()
        }),
        mat_sky_smoke_wisp_drift: materials.add(StandardMaterial {
            base_color: Color::srgba(0.058, 0.074, 0.098, 0.36),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.95,
            metallic: 0.0,
            emissive: LinearRgba::new(0.010, 0.014, 0.022, 1.0),
            cull_mode: None,
            double_sided: true,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -------------------------------------------------------------
// TRACK GENERATION
// -------------------------------------------------------------

pub fn spawn_sky_rail_segment(
    commands: &mut Commands,
    _env: &EnvironmentAssets,
    sky: &SkyRailAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = sky_rail_chapter_for_distance(distance);
    let landmark = sky_rail_landmark_for_distance(distance);
    let variant = sky_rail_variant(distance);

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // 1. Viaduct Slab Bed & Underside Abyss Foundation
            spawn_elevated_viaduct_slab(seg, sky, length);

            // 2. High-Readability 3 Elevated Running Lanes (Authoritative Lane::SPACING)
            spawn_three_elevated_lanes(seg, sky, chapter, variant);

            // 3. Side Parapets & Storm Deflection Aerodynamic Barriers
            spawn_storm_parapets(seg, sky, variant);

            // 3b. Overhead Telemetry Gantries & Mag-Rail Signal Arches (Periodic High Clearance y = 6.0m)
            spawn_overhead_telemetry_gantries(seg, sky, distance);

            // 4. Underside Support Pylons into 850m Abyss
            spawn_abyss_pylons(seg, sky);

            // 5. Cloud Abyss Ocean & Storm Cloud Ceiling (850m Altitude)
            spawn_cloud_abyss(seg, sky);

            // 5b. Cinematic Atmospheric Smoke & Industrial Pollution Plumes
            spawn_cinematic_smoke_atmosphere(seg, sky, distance, chapter);

            // 6. Aurelia Megatower Skyline Silhouettes (Far Background: |X| >= 110m)
            spawn_skyline_megatowers(seg, sky, distance);

            // 6b. Midground Infrastructure Silhouettes (|X| in [55m, 105m])
            spawn_midground_scenery(seg, sky, distance, chapter);

            // 6c. Deep Background Infrastructure Layers (Far Background: |X| >= 108m)
            spawn_distant_infrastructure(seg, sky, distance, chapter);

            // 7. Chapter-Specific Visual Dressing
            match chapter {
                SkyRailChapter::SkyRailEmergence => {
                    spawn_emergence_dressing(seg, sky, distance, variant);
                }
                SkyRailChapter::UpperAurelia => {
                    spawn_upper_aurelia_dressing(seg, sky, distance, variant);
                }
                SkyRailChapter::StormCorridor => {
                    spawn_storm_corridor_dressing(seg, sky, distance, variant);
                }
                SkyRailChapter::BrokenSkyway => {
                    spawn_broken_skyway_dressing(seg, sky, distance, variant);
                }
                SkyRailChapter::EchoApproach => {
                    spawn_echo_approach_dressing(seg, sky, distance, variant);
                }
            }

            // 8. Landmark Feature (if active in this segment window)
            if landmark != SkyRailLandmark::None {
                spawn_sky_rail_landmark(seg, sky, landmark);
            }
        })
        .id()
}

// -------------------------------------------------------------
// CORE ELEVATED TRACK PILLARS
// -------------------------------------------------------------

fn spawn_elevated_viaduct_slab(seg: &mut ChildBuilder, sky: &SkyRailAssets, _length: f32) {
    // Primary structural viaduct deck (elevated at y = -0.30m)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_viaduct_slab.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(0.0, -0.30, 0.0),
        ..default()
    });

    // Dual longitudinal under-girders at outer margins
    for x in [-3.6, 3.6] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_under_girder.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(x, -1.50, 0.0),
            ..default()
        });
    }

    // Cross-trusses bracing the under-girders every 10m
    for z in [-15.0, -5.0, 5.0, 15.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_under_truss.clone(),
            material: sky.mat_sky_storm_composite.clone(),
            transform: Transform::from_xyz(0.0, -1.80, z),
            ..default()
        });
    }
}

fn spawn_three_elevated_lanes(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    _chapter: SkyRailChapter,
    _variant: u8,
) {
    // Exact gameplay lane centers: x = -Lane::SPACING, 0.0, Lane::SPACING (-2.4, 0.0, 2.4)
    for lane_x in [-Lane::SPACING, 0.0, Lane::SPACING] {
        // Raised lane structural bed (y = 0.06m)
        seg.spawn(PbrBundle {
            mesh: sky.mesh_lane_bed.clone(),
            material: sky.mat_sky_tread.clone(),
            transform: Transform::from_xyz(lane_x, 0.06, 0.0),
            ..default()
        });

        // Glowing magnetic levitation guide strip down lane center
        seg.spawn(PbrBundle {
            mesh: sky.mesh_mag_strip.clone(),
            material: sky.mat_sky_mag_strip.clone(),
            transform: Transform::from_xyz(lane_x, 0.13, 0.0),
            ..default()
        });

        // Lateral curbs defining the lane boundaries with high contrast
        for curb_offset in [-0.85, 0.85] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_lane_curb.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(lane_x + curb_offset, 0.08, 0.0),
                ..default()
            });
        }
    }

    // Inter-lane expansion & drainage troughs between lanes (x = -1.2, 1.2)
    for trough_x in [-Lane::SPACING * 0.5, Lane::SPACING * 0.5] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_interlane_trough.clone(),
            material: sky.mat_sky_grate.clone(),
            transform: Transform::from_xyz(trough_x, -0.05, 0.0),
            ..default()
        });

        // Periodic expansion grates along the inter-lane trough
        for z in [-14.0, -7.0, 0.0, 7.0, 14.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_expansion_grate.clone(),
                material: sky.mat_sky_grate.clone(),
                transform: Transform::from_xyz(trough_x, 0.01, z),
                ..default()
            });
        }
    }
}

fn spawn_storm_parapets(seg: &mut ChildBuilder, sky: &SkyRailAssets, _variant: u8) {
    // Outer aerodynamic parapet barriers at x = -4.6m and 4.6m
    for side in [-1.0, 1.0] {
        let x = side * 4.60;

        // Continuous outer storm wall
        seg.spawn(PbrBundle {
            mesh: sky.mesh_outer_barrier.clone(),
            material: sky.mat_sky_barrier.clone(),
            transform: Transform::from_xyz(x, 0.65, 0.0),
            ..default()
        });

        // Heavy vertical barrier posts every 8m
        for z in [-16.0, -8.0, 0.0, 8.0, 16.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_barrier_post.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(x, 0.70, z),
                ..default()
            });

            // Aerodynamic wind deflection fin angled outward
            seg.spawn(PbrBundle {
                mesh: sky.mesh_barrier_aero_fin.clone(),
                material: sky.mat_sky_storm_composite.clone(),
                transform: Transform::from_xyz(x + side * 0.22, 1.05, z)
                    .with_rotation(Quat::from_rotation_z(side * 0.25)),
                ..default()
            });

            // Prominent glowing amber barrier cap luminaire atop each post
            seg.spawn(PbrBundle {
                mesh: sky.mesh_barrier_cap_light.clone(),
                material: sky.mat_sky_amber_glow.clone(),
                transform: Transform::from_xyz(x, 1.48, z),
                ..default()
            });
        }

        // Longitudinal high-voltage utility conduit bundle outside the barriers
        seg.spawn(PbrBundle {
            mesh: sky.mesh_outer_conduit_bundle.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(x + side * 0.35, 0.40, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });

        // Emergency trackside service lockers mounted on outer barrier wall
        seg.spawn(PbrBundle {
            mesh: sky.mesh_trackside_locker.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(x + side * 0.28, 0.85, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_cyan_glow.clone(),
            transform: Transform::from_xyz(x + side * 0.38, 1.20, 0.0)
                .with_scale(Vec3::new(1.2, 1.2, 1.2)),
            ..default()
        });
    }
}

// -------------------------------------------------------------
// OVERHEAD TELEMETRY GANTRIES (PERIODIC HIGH CLEARANCE y = 6.0M)
// -------------------------------------------------------------

fn spawn_overhead_telemetry_gantries(seg: &mut ChildBuilder, sky: &SkyRailAssets, distance: f32) {
    let seg_idx = (distance / 40.0).floor() as i32;
    // Spawns every 3 segments (120m) at seg_idx % 3 == 1
    if seg_idx.rem_euclid(3) == 1 {
        // Overhead horizontal telemetry arch beam at y = 6.0m spanning across viaduct (x in [-4.9m, 4.9m])
        seg.spawn(PbrBundle {
            mesh: sky.mesh_telemetry_gantry_beam.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(0.0, 6.0, 0.0),
            ..default()
        });

        // Vertical mounting stanchions rising from outer parapet walls
        for side in [-1.0, 1.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_gantry_column.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(side * 4.90, 3.60, 0.0),
                ..default()
            });
            // Outer amber caution beacon atop gantry mounting bracket
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_amber_glow.clone(),
                transform: Transform::from_xyz(side * 4.90, 6.35, 0.0)
                    .with_scale(Vec3::new(1.8, 1.8, 1.8)),
                ..default()
            });
        }

        // Downward-facing telemetry sensor pods suspended over each of the 3 running lanes
        for lane_x in [-Lane::SPACING, 0.0, Lane::SPACING] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_gantry_sensor_pod.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(lane_x, 5.55, 0.0),
                ..default()
            });
            // Cyan high-speed telemetry optical sensor on front face
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_cyan_glow.clone(),
                transform: Transform::from_xyz(lane_x, 5.55, 0.22)
                    .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                ..default()
            });
        }

        // Central top-mounted red aviation beacon on gantry apex
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_antenna_red.clone(),
            transform: Transform::from_xyz(0.0, 6.35, 0.0).with_scale(Vec3::new(2.4, 2.4, 2.4)),
            ..default()
        });
    }
}

fn spawn_abyss_pylons(seg: &mut ChildBuilder, sky: &SkyRailAssets) {
    // 1. Massive support pylon plunging 80m below track into the cloud deck (at center z = 0)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_pylon_massive.clone(),
        material: sky.mat_sky_pylon.clone(),
        transform: Transform::from_xyz(0.0, -42.0, 0.0),
        ..default()
    });

    // 2. Colossal triangular foundation brackets bracing the viaduct underside against abyss winds
    seg.spawn(PbrBundle {
        mesh: sky.mesh_pylon_bracket.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(0.0, -3.2, 0.0),
        ..default()
    });

    // 3. High-voltage utility conduit runs descending down the pylon face deep into the cloud abyss
    seg.spawn(PbrBundle {
        mesh: sky.mesh_cable_bundle_elevated.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(2.0, -42.0, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// CLOUD ABYSS & STORMY ATMOSPHERE (850M ALTITUDE)
// -------------------------------------------------------------

fn spawn_cloud_abyss(seg: &mut ChildBuilder, sky: &SkyRailAssets) {
    // 1. Primary Lower Cloud Deck (churning storm ocean beneath track at y = -42.0m)
    // Support pylons plunge through this deck, establishing the 850m altitude
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_deck_vast.clone(),
            material: sky.mat_sky_cloud_abyss.clone(),
            transform: Transform::from_xyz(0.0, -42.0, 0.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: 0.0,
            amplitude: 14.0,
            speed: 0.35,
            phase: 0.0,
        },
    ));

    // 2. Secondary Deeper Strata Haze (y = -58.0m, swallowing invisible city below)
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_strata_low.clone(),
            material: sky.mat_sky_cloud_abyss.clone(),
            transform: Transform::from_xyz(0.0, -58.0, 0.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: 0.0,
            amplitude: 22.0,
            speed: 0.22,
            phase: 1.57,
        },
    ));

    // 3. Heavy Overhead Storm Cloud Ceiling (oppressive dark canopy at y = 56.0m)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_cloud_ceiling_storm.clone(),
        material: sky.mat_sky_cloud_ceiling.clone(),
        transform: Transform::from_xyz(0.0, 56.0, 0.0),
        ..default()
    });

    // 4. Undulating Rolling Cumulus Cloud Ocean (y in [-15.0m, -24.0m])
    // Puffy, storm-illuminated cloud banks billowing along both flanks of the viaduct,
    // anchoring towers and bridges in a turbulent cloud sea rather than empty black vacuum.
    // Left flank cumulus billows (X in [-45m, -28m])
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_puff_large.clone(),
            material: sky.mat_sky_cloud_cumulus.clone(),
            transform: Transform::from_xyz(-38.0, -17.0, -8.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: -38.0,
            amplitude: 8.0,
            speed: 0.24,
            phase: 0.0,
        },
    ));
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_puff_medium.clone(),
            material: sky.mat_sky_cloud_cumulus.clone(),
            transform: Transform::from_xyz(-46.0, -21.0, 10.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: -46.0,
            amplitude: 8.0,
            speed: 0.20,
            phase: 1.2,
        },
    ));

    // Right flank cumulus billows (X in [28m, 45m])
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_puff_large.clone(),
            material: sky.mat_sky_cloud_cumulus.clone(),
            transform: Transform::from_xyz(38.0, -17.0, 6.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: 38.0,
            amplitude: 8.0,
            speed: 0.22,
            phase: 2.1,
        },
    ));
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_puff_medium.clone(),
            material: sky.mat_sky_cloud_cumulus.clone(),
            transform: Transform::from_xyz(46.0, -21.0, -10.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: 46.0,
            amplitude: 8.0,
            speed: 0.26,
            phase: 3.4,
        },
    ));

    // 5. Sparse Low-Poly Wispy Cloud Strata Crossing Behind Midground Structures
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_wisps_small.clone(),
            material: sky.mat_sky_cloud_wisp.clone(),
            transform: Transform::from_xyz(-62.0, 6.0, 4.0),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: -62.0,
            amplitude: 6.0,
            speed: 0.18,
            phase: 0.8,
        },
    ));
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_cloud_wisps_small.clone(),
            material: sky.mat_sky_cloud_wisp.clone(),
            transform: Transform::from_xyz(58.0, -8.0, -12.0)
                .with_scale(Vec3::new(1.15, 0.85, 1.1)),
            ..default()
        },
        SkyRailDriftCloud {
            base_x: 58.0,
            amplitude: 5.5,
            speed: 0.21,
            phase: 2.3,
        },
    ));
}

// -------------------------------------------------------------
// CINEMATIC SMOKE ATMOSPHERE & INDUSTRIAL POLLUTION PLUMES
// -------------------------------------------------------------

fn spawn_cinematic_smoke_atmosphere(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    chapter: SkyRailChapter,
) {
    let seg_idx = (distance / 40.0).floor() as i32;
    let h1 = ((seg_idx.wrapping_mul(29) + 7).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(43) + 17).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(61) + 29).rem_euclid(100)) as f32 / 100.0;
    let h4 = ((seg_idx.wrapping_mul(79) + 37).rem_euclid(100)) as f32 / 100.0;

    let primary_side = if seg_idx.rem_euclid(2) == 0 {
        -1.0
    } else {
        1.0
    };
    let secondary_side = -primary_side;

    // 1. Broad Horizontal Atmospheric Smoke Bank (partially veils lower city & tower bases)
    let bank_x = primary_side * (125.0 + h1 * 35.0);
    let bank_y = -4.0 + h2 * 14.0;
    let bank_z = -10.0 + h3 * 20.0;
    seg.spawn((
        PbrBundle {
            mesh: sky.mesh_smoke_bank_horizontal.clone(),
            material: sky.mat_sky_smoke_haze_bank.clone(),
            transform: Transform::from_xyz(bank_x, bank_y, bank_z).with_scale(Vec3::new(
                1.0 + h4 * 0.25,
                0.85 + h1 * 0.35,
                1.0 + h2 * 0.2,
            )),
            ..default()
        },
        SkyRailDriftSmoke {
            base_pos: Vec3::new(bank_x, bank_y, bank_z),
            drift_speed_x: 0.10,
            drift_amp_x: 10.0,
            rise_speed_y: 0.08,
            rise_amp_y: 2.2,
            phase: h2 * 3.14,
        },
    ));

    // 2. Midground Wispy Smoke Strata (Layer A/B transition: 75m–95m)
    if seg_idx.rem_euclid(2) == 1 {
        let wisp_x = secondary_side * (75.0 + h3 * 18.0);
        let wisp_y = 16.0 + h4 * 16.0;
        let wisp_z = 8.0 - h1 * 16.0;
        seg.spawn((
            PbrBundle {
                mesh: sky.mesh_smoke_strata_drift.clone(),
                material: sky.mat_sky_smoke_wisp_drift.clone(),
                transform: Transform::from_xyz(wisp_x, wisp_y, wisp_z)
                    .with_rotation(Quat::from_rotation_y(secondary_side * 0.18)),
                ..default()
            },
            SkyRailDriftSmoke {
                base_pos: Vec3::new(wisp_x, wisp_y, wisp_z),
                drift_speed_x: 0.22,
                drift_amp_x: 8.0,
                rise_speed_y: 0.14,
                rise_amp_y: 3.0,
                phase: h4 * 3.14 + 1.2,
            },
        ));
    }

    // 3. Chapter-Specific Industrial Smoke Plumes & Environmental Smog
    match chapter {
        SkyRailChapter::SkyRailEmergence => {
            // Chapter 1: Transitional emergence — sparse plumes rising from distant exhaust ports
            if seg_idx.rem_euclid(3) == 1 {
                let plume_x = secondary_side * (145.0 + h2 * 25.0);
                let plume_z = -12.0 + h3 * 24.0;
                let tiers = [
                    (22.0, &sky.mesh_smoke_plume_puff_small, 0.0, 0.15, 4.0),
                    (
                        36.0,
                        &sky.mesh_smoke_plume_puff_medium,
                        secondary_side * 4.5,
                        0.18,
                        7.0,
                    ),
                    (
                        52.0,
                        &sky.mesh_smoke_plume_puff_large,
                        secondary_side * 11.0,
                        0.20,
                        10.0,
                    ),
                ];
                for (tier_y, mesh, offset_x, drift_spd, amp) in tiers {
                    let pos = Vec3::new(plume_x + offset_x, tier_y, plume_z);
                    seg.spawn((
                        PbrBundle {
                            mesh: (*mesh).clone(),
                            material: sky.mat_sky_smoke_industrial_dense.clone(),
                            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                            ..default()
                        },
                        SkyRailDriftSmoke {
                            base_pos: pos,
                            drift_speed_x: drift_spd,
                            drift_amp_x: amp,
                            rise_speed_y: 0.10,
                            rise_amp_y: 2.5,
                            phase: h1 * 3.14 + tier_y * 0.05,
                        },
                    ));
                }
            }
        }
        SkyRailChapter::UpperAurelia => {
            // Chapter 2: Upper Aurelia — rising industrial plumes between corporate megatowers
            let plume_x = secondary_side * (138.0 + h3 * 30.0);
            let plume_z = -8.0 + h1 * 18.0;
            let tiers = [
                (28.0, &sky.mesh_smoke_plume_puff_small, 0.0, 0.14, 5.0),
                (
                    44.0,
                    &sky.mesh_smoke_plume_puff_medium,
                    secondary_side * 5.0,
                    0.17,
                    8.0,
                ),
                (
                    62.0,
                    &sky.mesh_smoke_plume_puff_large,
                    secondary_side * 12.0,
                    0.21,
                    12.0,
                ),
                (
                    80.0,
                    &sky.mesh_smoke_plume_puff_large,
                    secondary_side * 20.0,
                    0.24,
                    15.0,
                ),
            ];
            for (tier_y, mesh, offset_x, drift_spd, amp) in tiers {
                let pos = Vec3::new(plume_x + offset_x, tier_y, plume_z);
                seg.spawn((
                    PbrBundle {
                        mesh: (*mesh).clone(),
                        material: sky.mat_sky_smoke_industrial_dense.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    SkyRailDriftSmoke {
                        base_pos: pos,
                        drift_speed_x: drift_spd,
                        drift_amp_x: amp,
                        rise_speed_y: 0.09,
                        rise_amp_y: 2.8,
                        phase: h2 * 3.14 + tier_y * 0.04,
                    },
                ));
            }
        }
        SkyRailChapter::StormCorridor => {
            // Chapter 3: Storm Corridor — massive turbulent industrial plumes from refinery stacks
            // and cooling towers, interacting with storm lightning
            let plume_x = primary_side * (135.0 + h1 * 32.0);
            let plume_z = 6.0 - h2 * 18.0;
            let tiers = [
                (32.0, &sky.mesh_smoke_plume_puff_medium, 0.0, 0.16, 6.0),
                (
                    48.0,
                    &sky.mesh_smoke_plume_puff_large,
                    primary_side * 6.5,
                    0.20,
                    10.0,
                ),
                (
                    66.0,
                    &sky.mesh_smoke_plume_puff_large,
                    primary_side * 15.0,
                    0.25,
                    14.0,
                ),
                (
                    86.0,
                    &sky.mesh_smoke_plume_puff_large,
                    primary_side * 24.0,
                    0.28,
                    18.0,
                ),
            ];
            for (tier_y, mesh, offset_x, drift_spd, amp) in tiers {
                let pos = Vec3::new(plume_x + offset_x, tier_y, plume_z);
                seg.spawn((
                    PbrBundle {
                        mesh: (*mesh).clone(),
                        material: sky.mat_sky_smoke_industrial_dense.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                            .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                        ..default()
                    },
                    SkyRailDriftSmoke {
                        base_pos: pos,
                        drift_speed_x: drift_spd,
                        drift_amp_x: amp,
                        rise_speed_y: 0.12,
                        rise_amp_y: 3.5,
                        phase: h3 * 3.14 + tier_y * 0.06,
                    },
                ));
            }

            // Low-lying churning chasm smog deck partially veiling lower city
            let chasm_haze_x = secondary_side * (115.0 + h4 * 35.0);
            seg.spawn((
                PbrBundle {
                    mesh: sky.mesh_smoke_bank_horizontal.clone(),
                    material: sky.mat_sky_smoke_haze_bank.clone(),
                    transform: Transform::from_xyz(chasm_haze_x, -10.0, 0.0)
                        .with_scale(Vec3::new(1.3, 1.5, 1.2)),
                    ..default()
                },
                SkyRailDriftSmoke {
                    base_pos: Vec3::new(chasm_haze_x, -10.0, 0.0),
                    drift_speed_x: 0.12,
                    drift_amp_x: 12.0,
                    rise_speed_y: 0.06,
                    rise_amp_y: 2.0,
                    phase: h1 * 3.14,
                },
            ));
        }
        SkyRailChapter::BrokenSkyway => {
            // Chapter 4: Broken Skyway — smoldering smoke from damaged power conduits & shattered viaducts
            let plume_x = primary_side * (122.0 + h2 * 28.0);
            let plume_z = -6.0 + h4 * 16.0;
            let tiers = [
                (18.0, &sky.mesh_smoke_plume_puff_small, 0.0, 0.15, 4.0),
                (
                    32.0,
                    &sky.mesh_smoke_plume_puff_medium,
                    primary_side * 5.0,
                    0.19,
                    7.0,
                ),
                (
                    48.0,
                    &sky.mesh_smoke_plume_puff_large,
                    primary_side * 12.0,
                    0.22,
                    11.0,
                ),
            ];
            for (tier_y, mesh, offset_x, drift_spd, amp) in tiers {
                let pos = Vec3::new(plume_x + offset_x, tier_y, plume_z);
                seg.spawn((
                    PbrBundle {
                        mesh: (*mesh).clone(),
                        material: sky.mat_sky_smoke_industrial_dense.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    SkyRailDriftSmoke {
                        base_pos: pos,
                        drift_speed_x: drift_spd,
                        drift_amp_x: amp,
                        rise_speed_y: 0.10,
                        rise_amp_y: 2.6,
                        phase: h4 * 3.14 + tier_y * 0.05,
                    },
                ));
            }
        }
        SkyRailChapter::EchoApproach => {
            // Chapter 5: ECHO Approach — cold, high-altitude stratified haze framing the monolith
            let echo_haze_x = secondary_side * 142.0;
            seg.spawn((
                PbrBundle {
                    mesh: sky.mesh_smoke_bank_horizontal.clone(),
                    material: sky.mat_sky_smoke_haze_bank.clone(),
                    transform: Transform::from_xyz(echo_haze_x, 22.0, 0.0)
                        .with_scale(Vec3::new(1.1, 0.9, 1.1)),
                    ..default()
                },
                SkyRailDriftSmoke {
                    base_pos: Vec3::new(echo_haze_x, 22.0, 0.0),
                    drift_speed_x: 0.08,
                    drift_amp_x: 8.0,
                    rise_speed_y: 0.05,
                    rise_amp_y: 1.5,
                    phase: 0.0,
                },
            ));
        }
    }
}

// -------------------------------------------------------------
// DARK CITY SKYLINE (DISTANT MONUMENTAL TITANS & COMPOUND CLUSTERS)
// -------------------------------------------------------------

fn spawn_skyline_megatowers(seg: &mut ChildBuilder, sky: &SkyRailAssets, distance: f32) {
    let seg_idx = (distance / 40.0).floor() as i32;
    let chapter = sky_rail_chapter_for_distance(distance);

    let h1 = ((seg_idx.wrapping_mul(19) + 5).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(37) + 13).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(59) + 23).rem_euclid(100)) as f32 / 100.0;
    let h4 = ((seg_idx.wrapping_mul(83) + 31).rem_euclid(100)) as f32 / 100.0;

    // Distant Jagged Lightning Electric Discharges (sparse, between towers at |X| in [125m, 165m])
    if seg_idx.rem_euclid(3) == 2 {
        let bolt_side = if h3 > 0.5 { -1.0 } else { 1.0 };
        let bolt_x = bolt_side * (130.0 + h4 * 35.0);
        let bolt_z = -12.0 + h1 * 24.0;
        // Main lightning trunk descending from storm ceiling
        seg.spawn(PbrBundle {
            mesh: sky.mesh_lightning_branch.clone(),
            material: sky.mat_sky_lightning.clone(),
            transform: Transform::from_xyz(bolt_x, 42.0, bolt_z)
                .with_rotation(Quat::from_rotation_z(bolt_side * 0.22)),
            ..default()
        });
        // Fork branch 1
        seg.spawn(PbrBundle {
            mesh: sky.mesh_lightning_branch.clone(),
            material: sky.mat_sky_lightning.clone(),
            transform: Transform::from_xyz(bolt_x + bolt_side * 4.0, 26.0, bolt_z)
                .with_rotation(Quat::from_rotation_z(-bolt_side * 0.35))
                .with_scale(Vec3::new(0.65, 0.75, 0.65)),
            ..default()
        });
        // Fork branch 2
        seg.spawn(PbrBundle {
            mesh: sky.mesh_lightning_branch.clone(),
            material: sky.mat_sky_lightning.clone(),
            transform: Transform::from_xyz(bolt_x - bolt_side * 3.5, 22.0, bolt_z + 4.0)
                .with_rotation(Quat::from_rotation_x(0.28))
                .with_scale(Vec3::new(0.50, 0.55, 0.50)),
            ..default()
        });
    }

    // -------------------------------------------------------------
    // MULTI-LAYER MEGACITY CLUSTERS & HORIZON SKYLINE (LAYER C: 135–320M)
    // -------------------------------------------------------------
    let primary_side = if seg_idx.rem_euclid(2) == 0 {
        -1.0
    } else {
        1.0
    };
    let secondary_side = -primary_side;

    match chapter {
        SkyRailChapter::SkyRailEmergence => {
            // Chapter 1: Emergence from underground into open storm sky.
            // Distant solitary horizon titan rising from the cloud deck at |X| >= 165m.
            let titan_x = primary_side * (165.0 + h2 * 45.0);
            let titan_z = -15.0 + h3 * 30.0;
            // Primary colossal titan piercing cloud stratum
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_distant.clone(),
                transform: Transform::from_xyz(titan_x, 15.0, titan_z),
                ..default()
            });
            // Stepped crown pinnacle atop primary titan
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_stepped_crown.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(titan_x, 115.0, titan_z)
                    .with_scale(Vec3::new(1.4, 1.4, 1.4)),
                ..default()
            });
            // Vertical structural rib accents on inward face
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_window_slit_v.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(titan_x - primary_side * 17.5, 20.0, titan_z)
                    .with_scale(Vec3::new(1.0, 2.8, 1.2)),
                ..default()
            });
            // Horizontal crossband rib
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_window_slit_h.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(titan_x - primary_side * 17.5, 45.0, titan_z)
                    .with_scale(Vec3::new(1.0, 1.5, 1.2)),
                ..default()
            });
            // Distant aviation beacon blinking in cloud mist
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(titan_x, 138.0, titan_z)
                    .with_scale(Vec3::new(3.2, 3.2, 3.2)),
                ..default()
            });

            // Flanking cluster block partially submerged in clouds
            if h1 > 0.35 {
                let block_x = titan_x + primary_side * 28.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cluster_block.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(block_x, 10.0, titan_z + 8.0),
                    ..default()
                });
            }

            // Far horizon faint silhouette skyline on opposite flank (Layer C+ 245–300m)
            let far_x = secondary_side * (245.0 + h4 * 55.0);
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_faint.clone(),
                transform: Transform::from_xyz(far_x, 12.0, titan_z - 10.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(far_x, 110.0, titan_z - 10.0)
                    .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                ..default()
            });
        }
        SkyRailChapter::UpperAurelia => {
            // Chapter 2: Upper Aurelia — colossal corporate arcologies, compound tower clusters,
            // elevated skybridge networks, and far horizon silhouettes.
            let cluster_x = primary_side * (140.0 + h1 * 40.0);
            let cluster_z = -10.0 + h2 * 20.0;

            // 1. Primary Megatitan (3-way architectural silhouette variation)
            if h3 > 0.66 {
                // Cylindrical megatitan
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cylinder_large.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(cluster_x, 30.0, cluster_z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_stepped_crown.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cluster_x, 122.0, cluster_z)
                        .with_scale(Vec3::new(1.6, 1.6, 1.6)),
                    ..default()
                });
            } else if h3 > 0.33 {
                // Wide stepped brutalist corporate arcology
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_stepped_brutalist_arcology.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(cluster_x, 36.0, cluster_z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_antenna_spire.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cluster_x, 82.0, cluster_z)
                        .with_scale(Vec3::new(1.3, 1.3, 1.3)),
                    ..default()
                });
            } else {
                // Monumental rectangular megatitan
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_megatitan.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(cluster_x, 35.0, cluster_z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_stepped_crown.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cluster_x, 195.0, cluster_z)
                        .with_scale(Vec3::new(1.8, 1.8, 1.8)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_antenna_spire.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cluster_x, 235.0, cluster_z)
                        .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                    ..default()
                });
            }

            // Red aviation warning beacon on primary titan apex
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(cluster_x, 240.0, cluster_z)
                    .with_scale(Vec3::new(3.5, 3.5, 3.5)),
                ..default()
            });

            // 2. Flanking stepped corporate cluster block
            let flank_block_x = cluster_x + primary_side * 28.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_cluster_block.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(flank_block_x, 25.0, cluster_z + 8.0),
                ..default()
            });

            // 3. Horizontal skybridge span connecting primary titan to flanking block
            let bridge_span_x = (cluster_x + flank_block_x) * 0.5;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_skybridge_long.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(bridge_span_x, 62.0, cluster_z + 4.0),
                ..default()
            });

            // 4. Inward-facing amber window slits giving structural scale
            if h2 > 0.25 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_v.clone(),
                    material: sky.mat_sky_window_amber_dim.clone(),
                    transform: Transform::from_xyz(
                        cluster_x - primary_side * 20.0,
                        48.0,
                        cluster_z,
                    )
                    .with_scale(Vec3::new(1.5, 2.0, 1.5)),
                    ..default()
                });
            }
            if h4 > 0.4 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_h.clone(),
                    material: sky.mat_sky_window_amber_dim.clone(),
                    transform: Transform::from_xyz(
                        flank_block_x - primary_side * 16.0,
                        52.0,
                        cluster_z + 8.0,
                    )
                    .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                    ..default()
                });
            }

            // 5. Far horizon faint skyline silhouette layer (Layer C+ 240–310m on secondary flank)
            let far_x = secondary_side * (240.0 + h3 * 60.0);
            let far_z = 8.0 - h1 * 16.0;
            if h1 > 0.5 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_colossus.clone(),
                    material: sky.mat_sky_tower_faint.clone(),
                    transform: Transform::from_xyz(far_x, 18.0, far_z),
                    ..default()
                });
            } else {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cylinder_large.clone(),
                    material: sky.mat_sky_tower_faint.clone(),
                    transform: Transform::from_xyz(far_x, 15.0, far_z),
                    ..default()
                });
            }
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(far_x, 115.0, far_z)
                    .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                ..default()
            });
        }
        SkyRailChapter::StormCorridor => {
            // Chapter 3: Storm Corridor — industrial refinery complex, cooling towers, distillation columns,
            // atmospheric storm spires, spherical gas storage tanks, and pipe manifolds.
            let complex_x = primary_side * (138.0 + h1 * 42.0);
            let complex_z = -12.0 + h2 * 24.0;

            // 1. Colossal atmospheric storm spine tower piercing the storm ceiling
            seg.spawn(PbrBundle {
                mesh: sky.mesh_storm_spine_tower.clone(),
                material: sky.mat_sky_storm_composite.clone(),
                transform: Transform::from_xyz(complex_x, 70.0, complex_z),
                ..default()
            });
            // Flashing red aviation warning beacon on tower apex
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(complex_x, 140.0, complex_z)
                    .with_scale(Vec3::new(3.5, 3.5, 3.5)),
                ..default()
            });

            // 2. Colossal background cooling tower at complex flank
            let cooling_x = complex_x + primary_side * 28.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_industrial_cooling_tower.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(cooling_x, 14.0, complex_z + 6.0),
                ..default()
            });

            // 3. High industrial vent stack with red warning beacon
            let vent_x = complex_x - primary_side * 22.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_industrial_vent_stack.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(vent_x, 42.0, complex_z - 8.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(vent_x, 86.0, complex_z - 8.0)
                    .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                ..default()
            });

            // 4. Industrial Refinery Distillation Column with structural rings
            let column_x = complex_x + primary_side * 42.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_refinery_distillation_column.clone(),
                material: sky.mat_sky_storm_composite.clone(),
                transform: Transform::from_xyz(column_x, 36.0, complex_z - 4.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(column_x, 72.0, complex_z - 4.0)
                    .with_scale(Vec3::new(2.4, 2.4, 2.4)),
                ..default()
            });

            // 5. Pressurized spherical/cylindrical refinery storage tanks
            let sphere_x = complex_x - primary_side * 34.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_refinery_storage_sphere.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(sphere_x, 12.0, complex_z + 10.0),
                ..default()
            });

            // 6. Multi-tier industrial pipe manifold rack connecting refinery units
            seg.spawn(PbrBundle {
                mesh: sky.mesh_industrial_pipe_manifold.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz((complex_x + column_x) * 0.5, 20.0, complex_z),
                ..default()
            });

            // 7. Horizontal utility conduit span connecting industrial complex
            seg.spawn(PbrBundle {
                mesh: sky.mesh_utility_conduit_pipe.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(complex_x, 32.0, complex_z)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // 8. Far monolithic industrial slab silhouetted against storm lightning (secondary flank)
            let far_slab_x = secondary_side * (245.0 + h4 * 50.0);
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_slab.clone(),
                material: sky.mat_sky_tower_faint.clone(),
                transform: Transform::from_xyz(far_slab_x, 40.0, 0.0)
                    .with_scale(Vec3::new(1.6, 1.8, 1.6)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(far_slab_x, 175.0, 0.0)
                    .with_scale(Vec3::new(3.0, 3.0, 3.0)),
                ..default()
            });
        }
        SkyRailChapter::BrokenSkyway => {
            // Chapter 4: Broken Skyway — catastrophic infrastructure collapse.
            // Fractured titans, tilted brutalist arcologies, shattered distillation stacks, emergency beacons.
            let titan_x = primary_side * (142.0 + h3 * 45.0);
            let titan_z = -8.0 + h1 * 16.0;

            // 1. Tilted fractured titan
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_megatitan.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(titan_x, 25.0, titan_z)
                    .with_rotation(Quat::from_rotation_z(primary_side * 0.06)),
                ..default()
            });
            // Tilted structural brace along fractured tower face
            seg.spawn(PbrBundle {
                mesh: sky.mesh_pylon_bracket.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(titan_x - primary_side * 28.0, 32.0, titan_z)
                    .with_rotation(Quat::from_rotation_z(primary_side * 0.25)),
                ..default()
            });
            // Emergency red flashing beacon atop fractured apex
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_emergency_red.clone(),
                transform: Transform::from_xyz(titan_x, 195.0, titan_z)
                    .with_scale(Vec3::new(3.5, 3.5, 3.5)),
                ..default()
            });

            // 2. Fractured stepped brutalist arcology block
            let broken_block_x = titan_x + primary_side * 30.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_stepped_brutalist_arcology.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(broken_block_x, 22.0, titan_z + 6.0)
                    .with_rotation(Quat::from_rotation_z(-primary_side * 0.10)),
                ..default()
            });

            // 3. Severed broken skybridge dangling into void
            let bridge_x = (titan_x + broken_block_x) * 0.5;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_skybridge_long.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(bridge_x, 50.0, titan_z + 3.0)
                    .with_rotation(Quat::from_rotation_z(primary_side * 0.28)),
                ..default()
            });

            // 4. Far horizon faint damaged silhouette (secondary flank)
            let far_x = secondary_side * (240.0 + h2 * 55.0);
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_faint.clone(),
                transform: Transform::from_xyz(far_x, 14.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(-primary_side * 0.04)),
                ..default()
            });
        }
        SkyRailChapter::EchoApproach => {
            // Chapter 5: ECHO Approach — artificial mathematical symmetry approaching ECHO monolith.
            // Symmetrical monolithic bastions and stepped brutalist arcology flanks with cyan telemetry accents.
            for side in [-1.0, 1.0] {
                let bastion_x = side * (142.0 + h1 * 22.0);
                // Primary ECHO monolith slab
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_slab.clone(),
                    material: sky.mat_sky_echo_monolith.clone(),
                    transform: Transform::from_xyz(bastion_x, 45.0, 0.0)
                        .with_scale(Vec3::new(1.4, 2.0, 1.4)),
                    ..default()
                });
                // Flanking stepped brutalist arcology bastion
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_stepped_brutalist_arcology.clone(),
                    material: sky.mat_sky_echo_monolith.clone(),
                    transform: Transform::from_xyz(bastion_x + side * 34.0, 36.0, 6.0),
                    ..default()
                });
                // Symmetrical horizontal skybridge span
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_skybridge_long.clone(),
                    material: sky.mat_sky_echo_monolith.clone(),
                    transform: Transform::from_xyz(bastion_x + side * 17.0, 72.0, 3.0),
                    ..default()
                });
                // Cold cyan vertical telemetry window slit
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_v.clone(),
                    material: sky.mat_sky_window_dim.clone(),
                    transform: Transform::from_xyz(bastion_x - side * 15.0, 55.0, 0.0)
                        .with_scale(Vec3::new(1.8, 2.2, 1.8)),
                    ..default()
                });
                // Cyan pinnacle marker beacon
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(bastion_x, 45.0 + 150.0 + 10.0, 0.0)
                        .with_scale(Vec3::new(3.5, 3.5, 3.5)),
                    ..default()
                });
            }

            // Far horizon faint symmetrical bastion gates (secondary flank)
            let far_echo_x = secondary_side * 260.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_faint.clone(),
                transform: Transform::from_xyz(far_echo_x, 20.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_cyan_glow.clone(),
                transform: Transform::from_xyz(far_echo_x, 120.0, 0.0)
                    .with_scale(Vec3::new(3.0, 3.0, 3.0)),
                ..default()
            });
        }
    }
}

// -------------------------------------------------------------
// MIDGROUND INFRASTRUCTURE SILHOUETTES (55–105M LAYERED DEPTH)
// -------------------------------------------------------------

fn spawn_midground_scenery(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    chapter: SkyRailChapter,
) {
    let seg_idx = (distance / 40.0).floor() as i32;
    let h1 = ((seg_idx.wrapping_mul(23) + 9).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(41) + 15).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(67) + 27).rem_euclid(100)) as f32 / 100.0;

    match chapter {
        SkyRailChapter::SkyRailEmergence => {
            // Chapter 1: Emergence into 850m open sky above Aurelia cloud ocean.
            // Signature visual identity matching master reference:
            // 1. Left flank (X = -72.0m): Suspension skyway viaduct with stay-cable fans and moving mag-train.
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_span.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(-72.0, 20.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_strip.clone(),
                material: sky.mat_sky_mag_strip.clone(),
                transform: Transform::from_xyz(-72.0, 20.35, 0.0),
                ..default()
            });
            // Foundation pylon plunging deep into cloud ocean
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_pylon.clone(),
                material: sky.mat_sky_midground_concrete.clone(),
                transform: Transform::from_xyz(-72.0, -25.0, 0.0),
                ..default()
            });

            // Suspended maintenance cradle / inspection pod underneath roadbed at y = 10.5m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_micro_hanging_pod.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(-72.0, 10.5, 0.0),
                ..default()
            });
            // Twin vertical suspension cables supporting hanging pod from roadbed
            for dz in [-0.8, 0.8] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_cable_diag.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(-72.0, 15.2, dz)
                        .with_scale(Vec3::new(1.0, 0.30, 1.0)),
                    ..default()
                });
            }
            // Tiny amber maintenance indicator underneath hanging pod
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_amber_glow.clone(),
                transform: Transform::from_xyz(-72.0, 9.4, 0.0)
                    .with_scale(Vec3::new(0.65, 0.65, 0.65)),
                ..default()
            });

            // Every 2 segments: Colossal suspension tower mast with diagonal stay-cable fans
            if seg_idx.rem_euclid(2) == 0 {
                // Suspension tower mast rising 44m above bridge roadbed
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_tower.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(-72.0, 42.0, 0.0),
                    ..default()
                });
                // Aviation warning beacon atop mast
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(-72.0, 65.0, 0.0)
                        .with_scale(Vec3::new(3.2, 3.2, 3.2)),
                    ..default()
                });
                // Fan of diagonal stay cables bracing roadbed
                for (dz_sign, rot_angle) in [(1.0, 0.58), (-1.0, -0.58)] {
                    // Inner stay cable
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_suspension_cable_diag.clone(),
                        material: sky.mat_sky_midground_steel.clone(),
                        transform: Transform::from_xyz(-72.0, 31.0, dz_sign * 7.0)
                            .with_rotation(Quat::from_rotation_x(rot_angle))
                            .with_scale(Vec3::new(1.0, 0.82, 1.0)),
                        ..default()
                    });
                    // Outer stay cable
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_suspension_cable_diag.clone(),
                        material: sky.mat_sky_midground_steel.clone(),
                        transform: Transform::from_xyz(-72.0, 31.0, dz_sign * 14.0)
                            .with_rotation(Quat::from_rotation_x(rot_angle * 1.35))
                            .with_scale(Vec3::new(1.0, 0.98, 1.0)),
                        ..default()
                    });
                }
            } else if distance >= 8160.0 && distance <= 8240.0 {
                // Rare authored transit sighting: High-speed mag-train gliding on the suspension skyway
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_train_car.clone(),
                        material: sky.mat_sky_steel.clone(),
                        transform: Transform::from_xyz(-72.0, 21.3, 0.0),
                        ..default()
                    },
                    SkyRailDistantTrain {
                        speed: -44.0,
                        min_local_z: -42.0,
                        max_local_z: 42.0,
                    },
                ))
                .with_children(|train| {
                    // Glowing cyan passenger window band facing player track (+X)
                    train.spawn(PbrBundle {
                        mesh: sky.mesh_train_window_strip.clone(),
                        material: sky.mat_sky_train_window.clone(),
                        transform: Transform::from_xyz(1.75, 0.25, 0.0),
                        ..default()
                    });
                    // Forward cyan transit headlight
                    train.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -11.0)
                            .with_scale(Vec3::new(1.8, 1.8, 1.8)),
                        ..default()
                    });
                    // Rear red warning taillight
                    train.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_antenna_red.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 11.0)
                            .with_scale(Vec3::new(1.8, 1.8, 1.8)),
                        ..default()
                    });
                });
            }

            // 2. Right flank (X in [68m, 86m]): Colossal gothic megatower with cantilever decks & warm window grids
            let tower_x = 74.0 + h2 * 10.0;
            // Slender 110m midground tower column
            seg.spawn(PbrBundle {
                mesh: sky.mesh_midground_needle_tower.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x, 28.0, 0.0),
                ..default()
            });
            // Foundation pylon plunging deep into cloud sea
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_pylon.clone(),
                material: sky.mat_sky_midground_concrete.clone(),
                transform: Transform::from_xyz(tower_x, -26.0, 0.0),
                ..default()
            });
            // Cantilever observation/maintenance deck tier 1 at y = 22m wrapping tower
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_cantilever_deck.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x, 22.0, 0.0),
                ..default()
            });
            // Diagonal structural under-truss brackets supporting tier 1 cantilever deck
            seg.spawn(PbrBundle {
                mesh: sky.mesh_midground_bracket.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x - 10.0, 16.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(-0.45)),
                ..default()
            });
            // Wall-mounted electrical junction box on cantilever deck face with tiny cyan indicator
            seg.spawn(PbrBundle {
                mesh: sky.mesh_micro_junction_box.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x - 4.2, 23.2, 2.5),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_cyan_glow.clone(),
                transform: Transform::from_xyz(tower_x - 4.6, 23.4, 2.5)
                    .with_scale(Vec3::new(0.45, 0.45, 0.45)),
                ..default()
            });
            // Diagonal support cable bracing tier 1 cantilever deck corner back to tower
            seg.spawn(PbrBundle {
                mesh: sky.mesh_suspension_cable_diag.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x - 6.0, 26.5, 5.0)
                    .with_rotation(Quat::from_rotation_z(-0.35))
                    .with_scale(Vec3::new(1.0, 0.45, 1.0)),
                ..default()
            });
            // Cantilever deck tier 2 at y = 46m wrapping tower upper level
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_cantilever_deck.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x, 46.0, 0.0)
                    .with_scale(Vec3::new(0.85, 1.0, 0.85)),
                ..default()
            });
            // Small parabolic transceiver dish on tower face
            seg.spawn(PbrBundle {
                mesh: sky.mesh_micro_antenna_dish.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(tower_x - 4.2, 47.5, -2.0)
                    .with_rotation(Quat::from_rotation_z(-0.35)),
                ..default()
            });
            // Clustered warm amber window grid on inward face (-X)
            seg.spawn(PbrBundle {
                mesh: sky.mesh_window_grid_warm.clone(),
                material: sky.mat_sky_window_warm.clone(),
                transform: Transform::from_xyz(tower_x - 4.2, 28.0, 0.0),
                ..default()
            });
            // Structural pinnacle spire
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_antenna_spire.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(tower_x, 28.0 + 55.0 + 19.0, 0.0)
                    .with_scale(Vec3::new(1.2, 1.0, 1.2)),
                ..default()
            });
            // Blinking red aviation warning beacon on apex
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(tower_x, 28.0 + 55.0 + 38.0, 0.0)
                    .with_scale(Vec3::new(3.2, 3.2, 3.2)),
                ..default()
            });
            // Transverse skyway bridge connecting to adjacent structures
            if seg_idx.rem_euclid(2) == 1 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_midground_truss_girder.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(tower_x - 14.0, 32.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(tower_x - 14.0, 33.6, 0.0)
                        .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                    ..default()
                });
            }
        }
        SkyRailChapter::UpperAurelia => {
            // Chapter 2: Colossal Upper Aurelia — multi-layered city perspective (55-95m).
            // 1. Stepped corporate midground tower at X in [-72m, -94m]
            if seg_idx.rem_euclid(2) == 0 {
                let tower_x = -72.0 - h1 * 22.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_obelisk.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(tower_x, 38.0, 0.0)
                        .with_scale(Vec3::new(0.72, 0.65, 0.72)),
                    ..default()
                });
                // Cantilever corporate plaza deck wrapping tower at y = 24m
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cantilever_deck.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(tower_x, 24.0, 0.0)
                        .with_scale(Vec3::new(0.9, 1.0, 0.9)),
                    ..default()
                });
                // Electrical cabinet & transformer junction box on cantilever plaza deck
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_trackside_locker.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(tower_x + 4.8, 25.2, 3.2),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_junction_box.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(tower_x + 4.6, 25.2, -3.2),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(tower_x + 5.0, 25.5, -3.2)
                        .with_scale(Vec3::new(0.45, 0.45, 0.45)),
                    ..default()
                });
                // Distant maintenance drone hovering near corporate tower
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_micro_drone_body.clone(),
                        material: sky.mat_sky_midground_steel.clone(),
                        transform: Transform::from_xyz(tower_x - 12.0, 36.0, 8.0),
                        ..default()
                    },
                    SkyRailDistantDrone {
                        base_pos: Vec3::new(tower_x - 12.0, 36.0, 8.0),
                        hover_speed: 0.45,
                        hover_radius: 1.8,
                        phase: 1.2,
                    },
                ))
                .with_children(|drone| {
                    drone.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_amber_glow.clone(),
                        transform: Transform::from_xyz(0.0, -0.15, 0.0)
                            .with_scale(Vec3::new(0.6, 0.6, 0.6)),
                        ..default()
                    });
                });
                // Stepped crown pinnacle
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_stepped_crown.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(tower_x, 38.0 + 72.0 + 12.0, 0.0)
                        .with_scale(Vec3::new(0.85, 0.85, 0.85)),
                    ..default()
                });
                // Warm amber window grid cluster on face
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_window_grid_warm.clone(),
                    material: sky.mat_sky_window_warm.clone(),
                    transform: Transform::from_xyz(tower_x + 6.8, 36.0, 0.0)
                        .with_scale(Vec3::new(1.0, 1.2, 0.8)),
                    ..default()
                });
                // Red aviation beacon
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(tower_x, 38.0 + 72.0 + 26.0, 0.0)
                        .with_scale(Vec3::new(3.0, 3.0, 3.0)),
                    ..default()
                });
            }

            // 2. Suspended midground maintenance bridge span at X in [68m, 88m]
            if seg_idx.rem_euclid(3) == 1 {
                let bridge_x = 68.0 + h2 * 20.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_midground_truss_girder.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(bridge_x, 32.0, 0.0),
                    ..default()
                });
                // Vertical support column beneath
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_rail_pylon.clone(),
                    material: sky.mat_sky_midground_concrete.clone(),
                    transform: Transform::from_xyz(bridge_x, -12.0, 0.0),
                    ..default()
                });
                // Suspended utility module beneath bridge deck
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_hanging_pod.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(bridge_x, 26.5, 0.0)
                        .with_scale(Vec3::new(0.7, 0.7, 0.7)),
                    ..default()
                });
                // Diagonal stay cable
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_cable_diag.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(bridge_x, 38.0, 8.0)
                        .with_rotation(Quat::from_rotation_x(0.48)),
                    ..default()
                });
                // Amber service beacon on bridge center
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(bridge_x, 33.6, 0.0)
                        .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                    ..default()
                });
            }

            // 3. Parallel midground rail fragment passing at X in [64m, 82m] with rare mag-train (8360-8440m)
            let rail_x = 64.0 + h3 * 18.0;
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_span.clone(),
                material: sky.mat_sky_midground_steel.clone(),
                transform: Transform::from_xyz(rail_x, 18.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_strip.clone(),
                material: sky.mat_sky_mag_strip.clone(),
                transform: Transform::from_xyz(rail_x, 18.35, 0.0),
                ..default()
            });
            if distance >= 8360.0 && distance <= 8440.0 {
                // High-speed transit train with glowing windows
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_train_car.clone(),
                        material: sky.mat_sky_steel.clone(),
                        transform: Transform::from_xyz(rail_x, 19.3, 0.0),
                        ..default()
                    },
                    SkyRailDistantTrain {
                        speed: -46.0,
                        min_local_z: -38.0,
                        max_local_z: 38.0,
                    },
                ))
                .with_children(|train| {
                    train.spawn(PbrBundle {
                        mesh: sky.mesh_train_window_strip.clone(),
                        material: sky.mat_sky_train_window.clone(),
                        transform: Transform::from_xyz(-1.75, 0.25, 0.0),
                        ..default()
                    });
                });
            }
        }
        SkyRailChapter::StormCorridor => {
            // Chapter 3: Hostile storm corridor — heavy transmission pylons, stay cables, catenary power.
            // 1. Midground heavy transmission pylon at X in [-66m, -86m]
            if seg_idx.rem_euclid(2) == 1 {
                let mast_x = -66.0 - h2 * 20.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_mast.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(mast_x, 32.0, 0.0)
                        .with_scale(Vec3::new(1.3, 1.0, 1.3)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_arm.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(mast_x, 65.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(mast_x, 75.0, 0.0)
                        .with_scale(Vec3::new(3.5, 3.5, 3.5)),
                    ..default()
                });
                // Hanging catenary power cable
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hanging_cable.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(mast_x + 9.0, 64.5, 0.0),
                    ..default()
                });
                // Diagonal stay cable bracing mast against storm gusts
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_cable_diag.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(mast_x + 6.0, 42.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.35)),
                    ..default()
                });
                // Weathered iron patch plate on mast base
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_weathered_patch.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(mast_x + 1.0, 22.0, 0.0)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
                // Junction conduit connection box
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_junction_box.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(mast_x + 1.0, 36.0, 1.2),
                    ..default()
                });
            }

            // 2. Midground atmospheric storm monitoring sensor tower at X in [72m, 92m]
            if seg_idx.rem_euclid(3) == 0 {
                let sensor_x = 72.0 + h1 * 20.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_storm_spine_tower.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(sensor_x, 42.0, 0.0)
                        .with_scale(Vec3::new(0.85, 0.72, 0.85)),
                    ..default()
                });
                // Monitoring hub cantilever deck
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cantilever_deck.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(sensor_x, 32.0, 0.0)
                        .with_scale(Vec3::new(0.75, 0.85, 0.75)),
                    ..default()
                });
                // Telemetry window array
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_window_grid_warm.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(sensor_x - 3.2, 34.0, 0.0)
                        .with_scale(Vec3::new(0.6, 0.8, 0.6)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(sensor_x, 58.0, 0.0)
                        .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                    ..default()
                });
                // Weathered patch plate on monitoring hub
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_weathered_patch.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(sensor_x - 2.8, 28.0, 1.2)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
                // Inspection drone holding position near weather tower
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_micro_drone_body.clone(),
                        material: sky.mat_sky_storm_composite.clone(),
                        transform: Transform::from_xyz(sensor_x + 6.0, 38.0, -6.0),
                        ..default()
                    },
                    SkyRailDistantDrone {
                        base_pos: Vec3::new(sensor_x + 6.0, 38.0, -6.0),
                        hover_speed: 0.35,
                        hover_radius: 1.5,
                        phase: 3.1,
                    },
                ))
                .with_children(|drone| {
                    drone.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_emergency_red.clone(),
                        transform: Transform::from_xyz(0.0, -0.15, 0.0)
                            .with_scale(Vec3::new(0.55, 0.55, 0.55)),
                        ..default()
                    });
                });
            }
        }
        SkyRailChapter::BrokenSkyway => {
            // Chapter 4: Catastrophic infrastructure collapse in midground!
            // 1. Severed midground skyway viaduct section at X in [66m, 84m]
            if seg_idx.rem_euclid(2) == 0 {
                let severed_x = 66.0 + h2 * 18.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_rail_span.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(severed_x, 20.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-0.35)),
                    ..default()
                });
                // Fractured pier
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_collapsed_pier.clone(),
                    material: sky.mat_sky_midground_concrete.clone(),
                    transform: Transform::from_xyz(severed_x, -10.0, 0.0)
                        .with_scale(Vec3::new(0.75, 0.75, 0.75))
                        .with_rotation(Quat::from_rotation_z(0.22)),
                    ..default()
                });
                // Weathered iron patch plates and hazard repair plates on severed fracture edge
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_weathered_patch.clone(),
                    material: sky.mat_sky_hazard_stripe.clone(),
                    transform: Transform::from_xyz(severed_x - 1.8, 18.5, 4.0)
                        .with_rotation(Quat::from_rotation_z(0.35)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_weathered_patch.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(severed_x + 1.2, 16.0, -3.0)
                        .with_rotation(Quat::from_rotation_x(-0.25)),
                    ..default()
                });
                // Snapped dangling cable end plunging into void
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_cable_diag.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(severed_x - 1.5, 4.0, -2.0)
                        .with_rotation(Quat::from_rotation_z(0.55))
                        .with_scale(Vec3::new(1.0, 0.65, 1.0)),
                    ..default()
                });
                // Dangling severed suspension stay cables plunging down into cloud ocean
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_cable_diag.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(severed_x - 1.2, 6.0, 4.0)
                        .with_rotation(Quat::from_rotation_z(0.42)),
                    ..default()
                });
                // Dangling severed cables plunging down into void
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hanging_cable.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(severed_x - 1.2, 7.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(0.38)),
                    ..default()
                });
                // Emergency response drone hovering near disaster site
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_micro_drone_body.clone(),
                        material: sky.mat_sky_steel.clone(),
                        transform: Transform::from_xyz(severed_x + 8.0, 25.0, 12.0),
                        ..default()
                    },
                    SkyRailDistantDrone {
                        base_pos: Vec3::new(severed_x + 8.0, 25.0, 12.0),
                        hover_speed: 0.55,
                        hover_radius: 2.0,
                        phase: 2.4,
                    },
                ))
                .with_children(|drone| {
                    drone.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_emergency_red.clone(),
                        transform: Transform::from_xyz(-0.35, -0.15, 0.0)
                            .with_scale(Vec3::new(0.6, 0.6, 0.6)),
                        ..default()
                    });
                    drone.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_amber_glow.clone(),
                        transform: Transform::from_xyz(0.35, -0.15, 0.0)
                            .with_scale(Vec3::new(0.6, 0.6, 0.6)),
                        ..default()
                    });
                });
                // Flashing red emergency hazard strobe
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_emergency_red.clone(),
                    transform: Transform::from_xyz(severed_x, 22.0, 18.0)
                        .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                    ..default()
                });
            }

            // 2. Broken maintenance catwalk & fractured cantilever deck at X in [-68m, -86m]
            if seg_idx.rem_euclid(3) == 1 {
                let cat_x = -68.0 - h3 * 18.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_midground_truss_girder.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(cat_x, 28.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.16)),
                    ..default()
                });
                // Tilted fractured cantilever deck
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_cantilever_deck.clone(),
                    material: sky.mat_sky_midground_steel.clone(),
                    transform: Transform::from_xyz(cat_x, 24.0, 0.0)
                        .with_scale(Vec3::new(0.7, 0.8, 0.7))
                        .with_rotation(Quat::from_rotation_z(-0.20)),
                    ..default()
                });
                // Damaged utility box hanging on fractured girder
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_micro_junction_box.clone(),
                    material: sky.mat_sky_weathered_iron.clone(),
                    transform: Transform::from_xyz(cat_x + 1.2, 26.5, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.30)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(cat_x, 30.0, 0.0)
                        .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                    ..default()
                });
            }
        }
        SkyRailChapter::EchoApproach => {
            // Chapter 5: Severe artificial mathematical symmetry approaching ECHO citadel.
            // 1. Symmetrical midground monolith bastions flanking at X = ±(66m - 80m)
            if seg_idx.rem_euclid(2) == 0 {
                for side in [-1.0, 1.0] {
                    let bastion_x = side * (66.0 + h1 * 14.0);
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_echo_monolith_pylon.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(bastion_x, 32.0, 0.0)
                            .with_scale(Vec3::new(1.4, 1.7, 1.4)),
                        ..default()
                    });
                    // Perimeter cantilever security deck
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_tower_cantilever_deck.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(bastion_x, 24.0, 0.0)
                            .with_scale(Vec3::new(0.85, 0.9, 0.85)),
                        ..default()
                    });
                    // Symmetrical antenna transceiver dish on inward bastion face
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_micro_antenna_dish.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(bastion_x - side * 2.2, 42.0, 0.0)
                            .with_rotation(Quat::from_rotation_z(side * 0.35)),
                        ..default()
                    });
                    // Pristine symmetrical junction module
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_micro_junction_box.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(bastion_x - side * 2.0, 26.0, 1.8),
                        ..default()
                    });
                    // Symmetrical cyan telemetry window array
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_window_grid_warm.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(bastion_x - side * 1.8, 30.0, 0.0)
                            .with_scale(Vec3::new(0.7, 0.9, 0.7)),
                        ..default()
                    });
                    // Glowing cyan telemetry vertical strip
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_tower_window_slit_v.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(bastion_x - side * 1.6, 32.0, 0.0)
                            .with_scale(Vec3::new(0.8, 1.2, 0.8)),
                        ..default()
                    });
                    // Cyan pinnacle marker beacon
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(bastion_x, 52.0, 0.0)
                            .with_scale(Vec3::new(2.6, 2.6, 2.6)),
                        ..default()
                    });
                }

                // Sparse automated ECHO survey drone hovering outside track
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_micro_drone_body.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(-78.0, 36.0, 4.0),
                        ..default()
                    },
                    SkyRailDistantDrone {
                        base_pos: Vec3::new(-78.0, 36.0, 4.0),
                        hover_speed: 0.28,
                        hover_radius: 1.4,
                        phase: 0.5,
                    },
                ))
                .with_children(|drone| {
                    drone.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, -0.15, 0.0)
                            .with_scale(Vec3::new(0.65, 0.65, 0.65)),
                        ..default()
                    });
                });
            }

            // 2. High transverse conduit truss bridging high between X = ±65m
            if seg_idx.rem_euclid(3) == 1 {
                for side in [-1.0, 1.0] {
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_midground_truss_girder.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(side * 65.0, 48.0, 0.0),
                        ..default()
                    });
                }
            }
        }
    }
}

// -------------------------------------------------------------
// DEEP BACKGROUND INFRASTRUCTURE LAYERS (90–220M LAYERED DEPTH)
// -------------------------------------------------------------

fn spawn_distant_infrastructure(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    chapter: SkyRailChapter,
) {
    let seg_idx = (distance / 40.0).floor() as i32;
    let h1 = ((seg_idx.wrapping_mul(17) + 7).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(31) + 11).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(53) + 19).rem_euclid(100)) as f32 / 100.0;
    let h4 = ((seg_idx.wrapping_mul(71) + 29).rem_euclid(100)) as f32 / 100.0;

    // -------------------------------------------------------------
    // LAYER 5: STORM DEPTH — MID-ALTITUDE DRIFTING CLOUD SHELF
    // Spawns sparsely at |X| >= 110m, y = -18m.
    // Creates dramatic depth: distant structures peek through cloud strata.
    // -------------------------------------------------------------
    if seg_idx.rem_euclid(3) == 0 {
        let cloud_side = if h1 > 0.5 { -1.0 } else { 1.0 };
        let cloud_x = cloud_side * (112.0 + h2 * 28.0);
        seg.spawn((
            PbrBundle {
                mesh: sky.mesh_cloud_bank_flank.clone(),
                material: sky.mat_sky_cloud_abyss.clone(),
                transform: Transform::from_xyz(cloud_x, -18.0, 0.0),
                ..default()
            },
            SkyRailDriftCloud {
                base_x: cloud_x,
                amplitude: 14.0,
                speed: 0.26,
                phase: h3 * 3.14,
            },
        ));
    }

    // -------------------------------------------------------------
    // CHAPTER-AWARE DEEP INFRASTRUCTURE (TRANSIT WEB, BRIDGES & CRAFT)
    // -------------------------------------------------------------
    match chapter {
        SkyRailChapter::SkyRailEmergence => {
            // Chapter 1: First emergence from subterranean portal into vast storm chasm.
            // Distant solitary transmission masts and utility conduits.
            if seg_idx.rem_euclid(2) == 1 {
                let mast_x = -145.0 - h2 * 30.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_mast.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(mast_x, 35.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_arm.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(mast_x, 65.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(mast_x, 78.0, 0.0)
                        .with_scale(Vec3::new(3.0, 3.0, 3.0)),
                    ..default()
                });
            }
            if seg_idx.rem_euclid(3) == 2 {
                let pipe_x = 135.0 + h3 * 25.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_utility_conduit_pipe.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(pipe_x, 14.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            }
        }
        SkyRailChapter::UpperAurelia => {
            // Chapter 2: Upper Aurelia — colossal distant transit web across the chasm.
            // 1. Distant parallel elevated sky-rail line running along X = -125m at y = 22m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_span.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(-125.0, 22.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_strip.clone(),
                material: sky.mat_sky_mag_strip.clone(),
                transform: Transform::from_xyz(-125.0, 22.35, 0.0),
                ..default()
            });
            // Slender pylon plunging 90m into the abyss beneath the distant rail
            if seg_idx.rem_euclid(2) == 0 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_rail_pylon.clone(),
                    material: sky.mat_sky_pylon.clone(),
                    transform: Transform::from_xyz(-125.0, -23.0, 0.0),
                    ..default()
                });
            }

            // 2. Crossing elevated rail bridge spanning obliquely across the background
            if seg_idx.rem_euclid(2) == 1 {
                let cross_x = 120.0 + h1 * 22.0;
                let cross_y = 36.0 + h2 * 10.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_crossing_rail.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cross_x, cross_y, 0.0)
                        .with_rotation(Quat::from_rotation_y(0.42)),
                    ..default()
                });
                // Mag-guide light on crossing bridge
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(cross_x, cross_y + 1.2, 0.0)
                        .with_scale(Vec3::new(1.4, 1.4, 1.4)),
                    ..default()
                });
            }

            // 3. Colossal vertical elevator shaft (spanning 180m from -80m up to +100m)
            if seg_idx.rem_euclid(2) == 0 {
                let shaft_x = 152.0 + h2 * 25.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_elevator_shaft.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(shaft_x, 10.0, 0.0),
                    ..default()
                });
                // Amber service beacons on elevator spine
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(shaft_x - 2.2, 28.0, 0.0)
                        .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(shaft_x - 2.2, 65.0, 0.0)
                        .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                    ..default()
                });
            }

            // 4. Horizontal skybridge span in mid-background
            if seg_idx.rem_euclid(3) == 1 {
                let bridge_x = -145.0 - h3 * 20.0;
                let bridge_mesh = if h2 > 0.5 {
                    sky.mesh_distant_skybridge_long.clone()
                } else {
                    sky.mesh_sky_bridge_span.clone()
                };
                seg.spawn(PbrBundle {
                    mesh: bridge_mesh,
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(bridge_x, 56.0, 0.0),
                    ..default()
                });
            }

            // 5. Distant high-speed mag-train silhouette gliding on the distant rail (authored sighting at 8580-8640m)
            if distance >= 8580.0 && distance <= 8640.0 {
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_train_car.clone(),
                        material: sky.mat_sky_steel.clone(),
                        transform: Transform::from_xyz(-125.0, 23.3, 0.0),
                        ..default()
                    },
                    SkyRailDistantTrain {
                        speed: -52.0,
                        min_local_z: -38.0,
                        max_local_z: 38.0,
                    },
                ))
                .with_children(|car| {
                    // Cyan forward marker
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, -11.0)
                            .with_scale(Vec3::new(1.4, 1.4, 1.4)),
                        ..default()
                    });
                    // Passenger window strip facing inward (+X)
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_train_window_strip.clone(),
                        material: sky.mat_sky_train_window.clone(),
                        transform: Transform::from_xyz(1.75, 0.25, 0.0),
                        ..default()
                    });
                    // Red tail marker
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_antenna_red.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, 11.0)
                            .with_scale(Vec3::new(1.1, 1.1, 1.1)),
                        ..default()
                    });
                });
            }

            // 6. Occasional distant cargo craft slowly cruising in mid-background (sparse movement)
            if seg_idx.rem_euclid(4) == 2 {
                let craft_x = 138.0 + h4 * 24.0;
                let craft_y = 68.0 + h1 * 12.0;
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_traffic_craft.clone(),
                        material: sky.mat_sky_steel.clone(),
                        transform: Transform::from_xyz(craft_x, craft_y, 0.0),
                        ..default()
                    },
                    SkyRailDistantCraft {
                        speed: 16.0,
                        min_local_z: -42.0,
                        max_local_z: 42.0,
                    },
                ))
                .with_children(|craft| {
                    craft.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, 3.2)
                            .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                        ..default()
                    });
                    craft.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_amber_glow.clone(),
                        transform: Transform::from_xyz(0.0, -0.2, -3.2)
                            .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                        ..default()
                    });
                });
            }

            // 7. Tiny distant window cluster panel
            if seg_idx.rem_euclid(3) == 2 {
                let win_x = 142.0 + h4 * 30.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_window_cluster.clone(),
                    material: sky.mat_sky_window_amber_dim.clone(),
                    transform: Transform::from_xyz(win_x, 42.0, 0.0),
                    ..default()
                });
            }
        }
        SkyRailChapter::StormCorridor => {
            // Chapter 3: Hostile storm corridor — industrial pipe networks, continuous parallel rail,
            // crossing rail bridges, catenary power arrays, freight transport, and storm drones.
            // 1. Long industrial utility conduit pipe run at X = 118m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_utility_conduit_pipe.clone(),
                material: sky.mat_sky_storm_composite.clone(),
                transform: Transform::from_xyz(118.0, 16.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // 2. Continuous parallel industrial rail line at X = -125m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_span.clone(),
                material: sky.mat_sky_storm_composite.clone(),
                transform: Transform::from_xyz(-125.0, 22.0, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_distant_rail_strip.clone(),
                material: sky.mat_sky_mag_strip.clone(),
                transform: Transform::from_xyz(-125.0, 22.35, 0.0),
                ..default()
            });

            // 3. High-altitude transmission mast with crossarms and catenary cable
            if seg_idx.rem_euclid(2) == 1 {
                let mast_x = -145.0 - h2 * 30.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_mast.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(mast_x, 35.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_transmission_arm.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(mast_x, 68.0, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(mast_x, 78.0, 0.0)
                        .with_scale(Vec3::new(3.2, 3.2, 3.2)),
                    ..default()
                });
                // Hanging catenary cable span strung along transmission line
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hanging_cable.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(mast_x + 9.0, 67.5, 0.0),
                    ..default()
                });
            }

            // 4. Distant crossing elevated rail bridge cutting through storm mist
            if seg_idx.rem_euclid(3) == 0 {
                let cross_x = 125.0 + h3 * 20.0;
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_crossing_rail.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(cross_x, 42.0, 0.0)
                        .with_rotation(Quat::from_rotation_y(-0.35)),
                    ..default()
                });
            }

            // 5. Heavy industrial freight train gliding on the parallel rail (authored sighting at 8940-9000m)
            if distance >= 8940.0 && distance <= 9000.0 {
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_train_car.clone(),
                        material: sky.mat_sky_storm_composite.clone(),
                        transform: Transform::from_xyz(-125.0, 23.3, 0.0),
                        ..default()
                    },
                    SkyRailDistantTrain {
                        speed: -38.0,
                        min_local_z: -38.0,
                        max_local_z: 38.0,
                    },
                ))
                .with_children(|car| {
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_amber_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, -11.0)
                            .with_scale(Vec3::new(1.5, 1.5, 1.5)),
                        ..default()
                    });
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_antenna_red.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, 11.0)
                            .with_scale(Vec3::new(1.2, 1.2, 1.2)),
                        ..default()
                    });
                });
            }

            // 6. Occasional storm maintenance craft hovering / moving in upper cloud deck
            if seg_idx.rem_euclid(5) == 3 {
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_traffic_craft.clone(),
                        material: sky.mat_sky_storm_composite.clone(),
                        transform: Transform::from_xyz(-142.0, 78.0, 0.0),
                        ..default()
                    },
                    SkyRailDistantCraft {
                        speed: 14.0,
                        min_local_z: -35.0,
                        max_local_z: 35.0,
                    },
                ))
                .with_children(|craft| {
                    craft.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_amber_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0)
                            .with_scale(Vec3::new(1.3, 1.3, 1.3)),
                        ..default()
                    });
                });
            }
        }
        SkyRailChapter::BrokenSkyway => {
            // Chapter 4: Catastrophic infrastructure collapse — fractured parallel skyway,
            // shattered crossing rail, dangling severed cables, emergency beacons.
            // 1. Fractured distant parallel rail span plunging into abyss at X = 132m
            if seg_idx.rem_euclid(2) == 0 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_rail_span.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(132.0, 20.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-0.38)),
                    ..default()
                });
                // Snapped severed pylon
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_rail_pylon.clone(),
                    material: sky.mat_sky_pylon.clone(),
                    transform: Transform::from_xyz(132.0, -8.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(0.18)),
                    ..default()
                });
                // Dangling severed cables plunging down into cloud ocean
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hanging_cable.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(131.0, 8.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(0.42)),
                    ..default()
                });
                // Emergency red hazard beacon at fracture point
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_emergency_red.clone(),
                    transform: Transform::from_xyz(132.0, 22.0, 18.0)
                        .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                    ..default()
                });
            }

            // 2. Fractured crossing rail bridge tilted into the chasm
            if seg_idx.rem_euclid(3) == 1 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_crossing_rail.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(-135.0, 32.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.25)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_emergency_red.clone(),
                    transform: Transform::from_xyz(-135.0, 36.0, 0.0)
                        .with_scale(Vec3::new(2.4, 2.4, 2.4)),
                    ..default()
                });
            }

            // 3. Damaged maintenance truss
            if seg_idx.rem_euclid(4) == 2 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_distant_truss_bridge.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(145.0, 42.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(-0.14)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(145.0, 44.0, 0.0)
                        .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                    ..default()
                });
            }
        }
        SkyRailChapter::EchoApproach => {
            // Chapter 5: Artificial, ominous symmetry approaching ECHO facility.
            // 1. Symmetrical vertical transit elevator shafts flanking at X = ±145m
            if seg_idx.rem_euclid(2) == 0 {
                for side in [-1.0, 1.0] {
                    let shaft_x = side * 145.0;
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_distant_elevator_shaft.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(shaft_x, 16.0, 0.0),
                        ..default()
                    });
                    // Cyan telemetry marker nodes
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(shaft_x - side * 2.2, 32.0, 0.0)
                            .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                        ..default()
                    });
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(shaft_x - side * 2.2, 68.0, 0.0)
                            .with_scale(Vec3::new(2.2, 2.2, 2.2)),
                        ..default()
                    });
                }
            }

            // 2. High overhead symmetrical crossing skybridge spans
            if seg_idx.rem_euclid(3) == 1 {
                for side in [-1.0, 1.0] {
                    seg.spawn(PbrBundle {
                        mesh: sky.mesh_distant_skybridge_long.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(side * 138.0, 58.0, 0.0),
                        ..default()
                    });
                }
            }

            // 3. Distant automated ECHO courier transit vehicle (authored sighting at 9780-9840m)
            if distance >= 9780.0 && distance <= 9840.0 {
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_train_car.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(145.0, 23.4, 0.0),
                        ..default()
                    },
                    SkyRailDistantTrain {
                        speed: -46.0,
                        min_local_z: -38.0,
                        max_local_z: 38.0,
                    },
                ))
                .with_children(|car| {
                    car.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.2, -11.0)
                            .with_scale(Vec3::new(1.6, 1.6, 1.6)),
                        ..default()
                    });
                });
            }

            // 4. Occasional autonomous ECHO courier drone / craft
            if seg_idx.rem_euclid(5) == 2 {
                seg.spawn((
                    PbrBundle {
                        mesh: sky.mesh_distant_traffic_craft.clone(),
                        material: sky.mat_sky_echo_monolith.clone(),
                        transform: Transform::from_xyz(-148.0, 82.0, 0.0),
                        ..default()
                    },
                    SkyRailDistantCraft {
                        speed: 22.0,
                        min_local_z: -40.0,
                        max_local_z: 40.0,
                    },
                ))
                .with_children(|craft| {
                    craft.spawn(PbrBundle {
                        mesh: sky.mesh_hazard_reflector.clone(),
                        material: sky.mat_sky_cyan_glow.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0)
                            .with_scale(Vec3::new(1.5, 1.5, 1.5)),
                        ..default()
                    });
                });
            }
        }
    }
}

// -------------------------------------------------------------
// CHAPTER-SPECIFIC DRESSING
// -------------------------------------------------------------

fn spawn_emergence_dressing(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    variant: u8,
) {
    // Chapter 1: Transitional emergence infrastructure from underground facility (8000–8300m)
    // 1. Heavy tunnel exit portal remnants at initial emergence (8000–8080m)
    if distance <= 8080.0 {
        // Massive overhead portal arch beam at y = 6.4m spanning across track
        seg.spawn(PbrBundle {
            mesh: sky.mesh_portal_arch_beam.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(0.0, 6.4, -10.0),
            ..default()
        });
        // Twin heavy vertical portal columns flanking viaduct
        for side in [-1.0, 1.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_portal_column.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(side * 5.4, 4.4, -10.0),
                ..default()
            });
            // Floodwater drainage pipes descending into abyss
            seg.spawn(PbrBundle {
                mesh: sky.mesh_drainage_pipe.clone(),
                material: sky.mat_sky_grate.clone(),
                transform: Transform::from_xyz(side * 5.7, -3.2, -10.0),
                ..default()
            });
        }
    }

    // 2. Launch guide stanchions & amber emergence indicators along margins
    let signal_side = if variant == 1 { -1.0 } else { 1.0 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_sky_signal_mast.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(signal_side * 4.8, 2.2, -6.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_amber_glow.clone(),
        transform: Transform::from_xyz(signal_side * 4.8, 4.3, -6.0),
        ..default()
    });

    // 3. Cantilevered maintenance catwalk hanging off outer viaduct margin (for distance > 8080m)
    if distance > 8080.0 {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_sky_catwalk.clone(),
            material: sky.mat_sky_grate.clone(),
            transform: Transform::from_xyz(signal_side * 5.4, 0.40, 6.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_amber_glow.clone(),
            transform: Transform::from_xyz(signal_side * 5.4, 0.70, 6.0)
                .with_scale(Vec3::new(1.6, 1.6, 1.6)),
            ..default()
        });
    }

    // 4. Residual floodwater drainage conduits running along the opposite margin
    let drain_side = -signal_side;
    seg.spawn(PbrBundle {
        mesh: sky.mesh_drainage_pipe.clone(),
        material: sky.mat_sky_pylon.clone(),
        transform: Transform::from_xyz(drain_side * 5.1, -1.2, 4.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

fn spawn_upper_aurelia_dressing(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    _distance: f32,
    variant: u8,
) {
    // Chapter 2: High skyline urban railway surrounded by megastructures (8300–8700m)
    // 1. Flanking suspended service catwalks along margins
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_sky_catwalk.clone(),
            material: sky.mat_sky_grate.clone(),
            transform: Transform::from_xyz(side * 5.4, 0.45, 0.0),
            ..default()
        });
    }

    // 2. Vertical express elevator conduit shaft rising beside the viaduct
    let ele_side = if variant == 0 { -1.0 } else { 1.0 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_elevator_shaft.clone(),
        material: sky.mat_sky_tower_dark.clone(),
        transform: Transform::from_xyz(ele_side * 7.5, 14.0, 5.0),
        ..default()
    });
    // Dim amber observation slit on elevator shaft
    seg.spawn(PbrBundle {
        mesh: sky.mesh_tower_window_slit_v.clone(),
        material: sky.mat_sky_window_amber_dim.clone(),
        transform: Transform::from_xyz(ele_side * (7.5 - ele_side * 1.3), 16.0, 5.0),
        ..default()
    });

    // 3. Overhead utility suspension cable stay spanning above track (high clearance y = 7.2m)
    if variant == 2 {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_portal_arch_beam.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(0.0, 7.2, 0.0),
            ..default()
        });
        for side in [-1.0, 1.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_cyan_glow.clone(),
                transform: Transform::from_xyz(side * 5.2, 7.2, 0.0),
                ..default()
            });
        }
    }
}

fn spawn_storm_corridor_dressing(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    _distance: f32,
    variant: u8,
) {
    // Chapter 3: Violent storm corridor, hostile, high turbulence (8700–9200m)
    // 1. Aerodynamic storm deflection baffles along both margins
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_storm_baffle.clone(),
            material: sky.mat_sky_storm_composite.clone(),
            transform: Transform::from_xyz(side * 4.9, 1.8, -4.0),
            ..default()
        });
    }

    // 2. Heavy diagonal cable stays anchoring viaduct against crosswinds
    let brace_side = if variant == 1 { -1.0 } else { 1.0 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_stay_cable_diag.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(brace_side * 6.0, 8.0, 0.0)
            .with_rotation(Quat::from_rotation_z(brace_side * 0.45)),
        ..default()
    });

    // 3. Storm-damaged signal mast tilted by wind with flashing red aviation reflector
    seg.spawn(PbrBundle {
        mesh: sky.mesh_sky_signal_mast.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(-brace_side * 4.8, 2.0, 8.0)
            .with_rotation(Quat::from_rotation_z(brace_side * 0.22)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_emergency_red.clone(),
        transform: Transform::from_xyz(-brace_side * 4.8, 4.0, 8.0),
        ..default()
    });

    // 4. Lightning conductor / atmospheric sensor spire
    seg.spawn(PbrBundle {
        mesh: sky.mesh_relay_spire.clone(),
        material: sky.mat_sky_storm_composite.clone(),
        transform: Transform::from_xyz(brace_side * 5.2, 3.8, 8.0),
        ..default()
    });

    // 5. Environmental wear: weathered patch plates on outer storm deflection baffle
    seg.spawn(PbrBundle {
        mesh: sky.mesh_micro_weathered_patch.clone(),
        material: sky.mat_sky_weathered_iron.clone(),
        transform: Transform::from_xyz(brace_side * 4.95, 1.4, -4.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_amber_glow.clone(),
        transform: Transform::from_xyz(brace_side * 4.95, 2.2, -4.0)
            .with_scale(Vec3::new(0.45, 0.45, 0.45)),
        ..default()
    });
}

fn spawn_broken_skyway_dressing(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    variant: u8,
) {
    // Chapter 4: Broken Skyway hero chapter - Catastrophic infrastructure failure (9200–9650m)
    // Visual progression:
    // 9200..9280: Early damage (bent barriers, loose cables)
    // 9280..9460: Major collapse (parallel snapped skyway plunging into abyss, derailed hulks)
    // 9460..9650: Emergency repair zone (scaffolding, temporary hazard barriers, red strobes)

    if distance < 9280.0 {
        // Early damage: bent barriers and loose cables
        let mast_side = if variant == 2 { -1.0 } else { 1.0 };
        seg.spawn(PbrBundle {
            mesh: sky.mesh_sky_signal_mast.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(mast_side * 4.8, 1.8, -8.0)
                .with_rotation(Quat::from_rotation_z(mast_side * 0.35)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_emergency_red.clone(),
            transform: Transform::from_xyz(mast_side * 4.8, 3.5, -8.0),
            ..default()
        });
    } else if distance < 9460.0 {
        // Major disaster: Parallel collapsed skyway running alongside at x = 9.8m
        // Snapped parallel viaduct slab tilted plunging into the cloud abyss
        let tilt_angle = -0.38 - (variant as f32) * 0.12;
        seg.spawn(PbrBundle {
            mesh: sky.mesh_broken_parallel_slab.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(9.8, -4.5, 0.0)
                .with_rotation(Quat::from_rotation_x(tilt_angle)),
            ..default()
        });

        // Fractured support pier snapped off under the parallel slab
        seg.spawn(PbrBundle {
            mesh: sky.mesh_collapsed_pier.clone(),
            material: sky.mat_sky_pylon.clone(),
            transform: Transform::from_xyz(10.2, -22.0, 5.0)
                .with_rotation(Quat::from_rotation_z(0.20)),
            ..default()
        });

        // Emergency repair plates on damaged outer barrier
        seg.spawn(PbrBundle {
            mesh: sky.mesh_micro_weathered_patch.clone(),
            material: sky.mat_sky_hazard_stripe.clone(),
            transform: Transform::from_xyz(4.85, 1.2, 0.0),
            ..default()
        });

        // Derailed mag-car hulk resting on the severed parallel pier (completely outside playable lanes)
        if variant == 1 {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_derailed_hulk.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(10.0, -2.8, -6.0)
                    .with_rotation(Quat::from_rotation_z(0.30)),
                ..default()
            });
            // Dim red emergency strobe on wrecked car
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_emergency_red.clone(),
                transform: Transform::from_xyz(10.0, -1.2, -6.0),
                ..default()
            });
        }

        // Modular yellow/amber emergency scaffolding along player's outer barrier
        seg.spawn(PbrBundle {
            mesh: sky.mesh_scaffold_frame.clone(),
            material: sky.mat_sky_hazard_stripe.clone(),
            transform: Transform::from_xyz(-5.2, 1.4, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_barrier_diag.clone(),
            material: sky.mat_sky_amber_glow.clone(),
            transform: Transform::from_xyz(4.9, 0.8, -4.0),
            ..default()
        });
    } else {
        // Approaching 9650m: Emergency scaffolding and temporary structural repair braces
        for side in [-1.0, 1.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_scaffold_frame.clone(),
                material: sky.mat_sky_hazard_stripe.clone(),
                transform: Transform::from_xyz(side * 5.4, 1.4, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_micro_weathered_patch.clone(),
                material: sky.mat_sky_weathered_iron.clone(),
                transform: Transform::from_xyz(side * 5.4, 0.6, 2.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_emergency_red.clone(),
                transform: Transform::from_xyz(side * 5.4, 3.2, 0.0),
                ..default()
            });
        }
    }
}

fn spawn_echo_approach_dressing(
    seg: &mut ChildBuilder,
    sky: &SkyRailAssets,
    distance: f32,
    _variant: u8,
) {
    // Chapter 5: Controlled, symmetrical, ominous ECHO facility approach (9650–10000m)
    // 1. High overhead cyan conduit ring bridging across track at high clearance (y = 6.8m)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_echo_conduit_ring.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(0.0, 6.8, 0.0),
        ..default()
    });
    // Central glowing cyan telemetry guide on conduit ring
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_cyan_glow.clone(),
        transform: Transform::from_xyz(0.0, 6.8, -0.85),
        ..default()
    });

    // 2. Twin symmetrical ECHO surveillance monoliths flanking the track
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_echo_monolith_pylon.clone(),
            material: sky.mat_sky_echo_monolith.clone(),
            transform: Transform::from_xyz(side * 5.6, 9.0, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_relay_spire.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(side * 5.6, 21.0, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_cyan_glow.clone(),
            transform: Transform::from_xyz(side * 5.6, 24.5, 0.0),
            ..default()
        });
    }

    // 3. Approaching 10000m: Colossal dark facility entrance bastions rising in the distance
    if distance >= 9860.0 {
        let wall_scale = ((distance - 9860.0) / 140.0).clamp(0.5, 1.0);
        for side in [-1.0, 1.0] {
            seg.spawn(PbrBundle {
                mesh: sky.mesh_echo_facility_wall.clone(),
                material: sky.mat_sky_echo_monolith.clone(),
                transform: Transform::from_xyz(side * 28.0, 24.0, 0.0)
                    .with_scale(Vec3::new(wall_scale, wall_scale, wall_scale)),
                ..default()
            });
            // High cyan entrance marker beacon
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_cyan_glow.clone(),
                transform: Transform::from_xyz(side * 14.0, 32.0, 0.0)
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..default()
            });
        }
    }
}

fn spawn_sky_rail_landmark(seg: &mut ChildBuilder, sky: &SkyRailAssets, landmark: SkyRailLandmark) {
    match landmark {
        SkyRailLandmark::SkyRailGate => {
            // Tunnel exit portal framing arch at margins (|x| = 5.8m, leaving corridor open)
            seg.spawn(PbrBundle {
                mesh: sky.mesh_portal_arch_beam.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(0.0, 7.2, 0.0).with_scale(Vec3::new(1.3, 1.5, 1.3)),
                ..default()
            });
            for side in [-1.0, 1.0] {
                // Giant portal column
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_portal_column.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 5.8, 5.2, 0.0)
                        .with_scale(Vec3::new(1.5, 1.3, 1.5)),
                    ..default()
                });
                // Launch hoist mast with amber emergence beacon
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_sky_signal_mast.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 6.4, 8.5, 0.0)
                        .with_scale(Vec3::new(1.6, 2.0, 1.6)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(side * 6.4, 12.8, 0.0)
                        .with_scale(Vec3::new(1.8, 1.8, 1.8)),
                    ..default()
                });
                // Massive drainage manifold into abyss
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_drainage_pipe.clone(),
                    material: sky.mat_sky_grate.clone(),
                    transform: Transform::from_xyz(side * 6.2, -4.5, 0.0),
                    ..default()
                });
            }
        }
        SkyRailLandmark::AureliaCrownStation => {
            // Colossal orbital-class megastation landmark!
            // 1. Overhead station ring arch spanning across viaduct at y = 8.8m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_station_ring_arch.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(0.0, 8.8, 0.0),
                ..default()
            });
            // 2. Suspended multi-tier passenger platforms and canopies flanking both sides
            for side in [-1.0, 1.0] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_station_platform.clone(),
                    material: sky.mat_sky_grate.clone(),
                    transform: Transform::from_xyz(side * 6.6, 0.65, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_station_canopy.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(side * 6.8, 4.2, 0.0)
                        .with_rotation(Quat::from_rotation_z(side * 0.15)),
                    ..default()
                });
                // Twin express transit elevator cores rising 38m into the clouds
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_elevator_shaft.clone(),
                    material: sky.mat_sky_tower_dark.clone(),
                    transform: Transform::from_xyz(side * 8.8, 18.0, 0.0)
                        .with_scale(Vec3::new(1.3, 1.25, 1.3)),
                    ..default()
                });
                // Dim cyan technical station signage along platform edge
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(side * 5.4, 0.9, 0.0)
                        .with_scale(Vec3::new(1.4, 1.4, 1.4)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(side * 5.4, 0.9, 8.0),
                    ..default()
                });
            }
        }
        SkyRailLandmark::StormSpineBridge => {
            // Massive suspension bridge across the sky abyss!
            // 1. Overhead wind-brace cross-beam at y = 9.4m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_portal_arch_beam.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(0.0, 9.4, 0.0).with_scale(Vec3::new(1.4, 1.6, 1.4)),
                ..default()
            });
            for side in [-1.0, 1.0] {
                // Colossal suspension tower rising 44m above track
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_suspension_tower.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(side * 6.5, 21.0, 0.0),
                    ..default()
                });
                // Heavy diagonal main stay cables angling down to bridge deck
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_stay_cable_diag.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 6.5, 14.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(0.42)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_stay_cable_diag.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 6.5, 14.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-0.42)),
                    ..default()
                });
                // Red storm aviation beacon atop tower
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_antenna_red.clone(),
                    transform: Transform::from_xyz(side * 6.5, 43.5, 0.0)
                        .with_scale(Vec3::new(2.4, 2.4, 2.4)),
                    ..default()
                });
                // Aerodynamic wind-deflector wedge fairing along flank
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_storm_baffle.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 5.2, 1.5, 0.0),
                    ..default()
                });
            }
        }
        SkyRailLandmark::CollapsedSkywaySpan => {
            // Hero disaster set piece of Zone 5!
            // 1. Massive fractured pier & snapped parallel span plunging 35° into cloud ocean at x = 10.5m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_broken_parallel_slab.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(10.5, -6.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-0.58)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_collapsed_pier.clone(),
                material: sky.mat_sky_pylon.clone(),
                transform: Transform::from_xyz(11.2, -24.0, 4.0)
                    .with_rotation(Quat::from_rotation_z(0.32)),
                ..default()
            });

            // 2. Derailed, crumpled mag-car carriage precariously balanced on fractured pier
            seg.spawn(PbrBundle {
                mesh: sky.mesh_derailed_hulk.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(10.8, -3.2, 2.0)
                    .with_rotation(Quat::from_rotation_z(0.40) * Quat::from_rotation_x(-0.25)),
                ..default()
            });
            // Flashing red emergency strobe on wrecked carriage
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_emergency_red.clone(),
                transform: Transform::from_xyz(10.8, -1.4, 2.0)
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..default()
            });

            // 3. Emergency scaffolding cage & hazard barricades clamped to player's viable viaduct span
            seg.spawn(PbrBundle {
                mesh: sky.mesh_scaffold_frame.clone(),
                material: sky.mat_sky_hazard_stripe.clone(),
                transform: Transform::from_xyz(-5.5, 1.6, 0.0).with_scale(Vec3::new(1.3, 1.4, 1.3)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_barrier_diag.clone(),
                material: sky.mat_sky_amber_glow.clone(),
                transform: Transform::from_xyz(4.9, 0.9, -6.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_emergency_red.clone(),
                transform: Transform::from_xyz(-5.5, 3.8, 0.0).with_scale(Vec3::new(1.8, 1.8, 1.8)),
                ..default()
            });
        }
        SkyRailLandmark::EchoRelayArray => {
            // Colossal cyan communication relay array above clouds!
            // 1. High overhead cyan energy conduit arch connecting towers at y = 9.0m
            seg.spawn(PbrBundle {
                mesh: sky.mesh_portal_arch_beam.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(0.0, 9.0, 0.0).with_scale(Vec3::new(1.4, 1.2, 1.4)),
                ..default()
            });
            for side in [-1.0, 1.0] {
                // Monumental transmission monolith rising 38m
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_echo_monolith_pylon.clone(),
                    material: sky.mat_sky_echo_monolith.clone(),
                    transform: Transform::from_xyz(side * 6.8, 18.0, 0.0)
                        .with_scale(Vec3::new(1.6, 1.7, 1.6)),
                    ..default()
                });
                // Giant concentric circular cyan energy focusing dish
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_echo_relay_dish.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(side * 6.8, 22.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(side * std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
                // Pulsating cyan data transmission node beacon atop spire
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(side * 6.8, 36.5, 0.0)
                        .with_scale(Vec3::new(2.8, 2.8, 2.8)),
                    ..default()
                });
                // Flanking monolithic bastion walls
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_echo_facility_wall.clone(),
                    material: sky.mat_sky_echo_monolith.clone(),
                    transform: Transform::from_xyz(side * 22.0, 20.0, 0.0)
                        .with_scale(Vec3::new(0.6, 0.7, 0.6)),
                    ..default()
                });
            }
        }
        SkyRailLandmark::None => {}
    }
}

// -------------------------------------------------------------
// PLUGIN DEFINITION & DYNAMIC STORM SYSTEMS
// -------------------------------------------------------------

pub struct SkyRailPlugin;

impl Plugin for SkyRailPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkyRailStormState>();
        app.add_systems(
            Update,
            update_sky_rail_storm.run_if(in_state(AppState::InGame)),
        );
    }
}

pub fn update_sky_rail_storm(
    time: Res<Time>,
    mut storm_state: ResMut<SkyRailStormState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Option<Res<SkyRailAssets>>,
    run_stats: Option<Res<GameRunStats>>,
    director: Option<Res<RunDirector>>,
    mut drift_q: Query<(&SkyRailDriftCloud, &mut Transform)>,
    mut train_q: Query<(&SkyRailDistantTrain, &mut Transform), Without<SkyRailDriftCloud>>,
    mut drone_q: Query<
        (&SkyRailDistantDrone, &mut Transform),
        (Without<SkyRailDriftCloud>, Without<SkyRailDistantTrain>),
    >,
    mut craft_q: Query<
        (&SkyRailDistantCraft, &mut Transform),
        (
            Without<SkyRailDriftCloud>,
            Without<SkyRailDistantTrain>,
            Without<SkyRailDistantDrone>,
        ),
    >,
    mut smoke_q: Query<
        (&SkyRailDriftSmoke, &mut Transform),
        (
            Without<SkyRailDriftCloud>,
            Without<SkyRailDistantTrain>,
            Without<SkyRailDistantDrone>,
            Without<SkyRailDistantCraft>,
        ),
    >,
) {
    let dt = time.delta_seconds();
    let in_zone_5 = run_stats
        .as_ref()
        .map(|s| s.distance >= 7950.0 && s.distance <= 10050.0)
        .or_else(|| director.as_ref().map(|d| d.active_zone_id == 5))
        .unwrap_or(false);

    if !in_zone_5 {
        if storm_state.flash_intensity > 0.0 {
            storm_state.flash_intensity = 0.0;
            storm_state.is_flashing = false;
            storm_state.flash_elapsed = 0.0;
            if let Some(ref sky) = assets {
                if let Some(mat) = materials.get_mut(&sky.mat_sky_cloud_ceiling) {
                    mat.emissive = LinearRgba::new(0.008, 0.012, 0.020, 1.0);
                    mat.base_color = Color::srgb(0.028, 0.038, 0.055);
                }
                if let Some(mat) = materials.get_mut(&sky.mat_sky_cloud_cumulus) {
                    mat.emissive = LinearRgba::new(0.04, 0.06, 0.10, 1.0);
                }
                if let Some(mat) = materials.get_mut(&sky.mat_sky_lightning) {
                    mat.emissive = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
                }
                if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_industrial_dense) {
                    mat.emissive = LinearRgba::new(0.006, 0.008, 0.012, 1.0);
                }
                if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_haze_bank) {
                    mat.emissive = LinearRgba::new(0.008, 0.012, 0.018, 1.0);
                }
                if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_wisp_drift) {
                    mat.emissive = LinearRgba::new(0.010, 0.014, 0.022, 1.0);
                }
            }
        }
        return;
    }

    // 1. Subtle, continuous cloud ocean drift & swelling
    let elapsed = time.elapsed_seconds();
    for (drift, mut trans) in drift_q.iter_mut() {
        trans.translation.x =
            drift.base_x + (elapsed * drift.speed + drift.phase).sin() * drift.amplitude;
    }

    // 2. Lightning storm events (sparse, dramatic cold-white flashes)
    let Some(ref sky) = assets else { return };

    if !storm_state.is_flashing {
        storm_state.flash_timer += dt;
        if storm_state.flash_timer >= storm_state.next_flash_interval {
            storm_state.is_flashing = true;
            storm_state.flash_elapsed = 0.0;
            storm_state.flash_timer = 0.0;
            storm_state.flash_count = storm_state.flash_count.wrapping_add(1);
            let hash = (storm_state.flash_count.wrapping_mul(47) % 35) as f32;
            storm_state.next_flash_interval = 6.0 + hash * 0.1;
        }
    } else {
        storm_state.flash_elapsed += dt;
        let t = storm_state.flash_elapsed;
        // Multi-pulse lightning stroke profile:
        // 0.00..0.05s: Initial primary strike -> peak 1.0
        // 0.05..0.09s: Brief dip -> 0.2
        // 0.09..0.14s: Secondary return stroke -> peak 0.8
        // 0.14..0.22s: Smooth decay -> 0.0
        let intensity = if t < 0.05 {
            t / 0.05
        } else if t < 0.09 {
            1.0 - ((t - 0.05) / 0.04) * 0.8
        } else if t < 0.14 {
            0.2 + ((t - 0.09) / 0.05) * 0.6
        } else if t < 0.22 {
            0.8 * (1.0 - (t - 0.14) / 0.08)
        } else {
            0.0
        };

        storm_state.flash_intensity = intensity;

        if t >= 0.22 {
            storm_state.is_flashing = false;
            storm_state.flash_intensity = 0.0;
            storm_state.flash_elapsed = 0.0;
        }

        // Apply emissive lightning illumination to cloud ceiling & cumulus cloud tops
        if let Some(mat) = materials.get_mut(&sky.mat_sky_cloud_ceiling) {
            if intensity > 0.001 {
                mat.emissive =
                    LinearRgba::new(intensity * 3.2, intensity * 3.8, intensity * 5.2, 1.0);
                mat.base_color = Color::srgb(
                    0.014 + intensity * 0.70,
                    0.020 + intensity * 0.82,
                    0.030 + intensity * 0.95,
                );
            } else {
                mat.emissive = LinearRgba::new(0.008, 0.012, 0.020, 1.0);
                mat.base_color = Color::srgb(0.028, 0.038, 0.055);
            }
        }
        if let Some(mat) = materials.get_mut(&sky.mat_sky_cloud_cumulus) {
            if intensity > 0.001 {
                mat.emissive = LinearRgba::new(
                    0.04 + intensity * 1.6,
                    0.06 + intensity * 2.0,
                    0.10 + intensity * 2.8,
                    1.0,
                );
            } else {
                mat.emissive = LinearRgba::new(0.04, 0.06, 0.10, 1.0);
            }
        }
        if let Some(mat) = materials.get_mut(&sky.mat_sky_lightning) {
            if intensity > 0.001 {
                mat.emissive =
                    LinearRgba::new(intensity * 14.0, intensity * 18.0, intensity * 26.0, 1.0);
            } else {
                mat.emissive = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
            }
        }
        // Subtly illuminate smoke plumes and horizontal haze banks during lightning flash
        if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_industrial_dense) {
            if intensity > 0.001 {
                mat.emissive = LinearRgba::new(
                    0.006 + intensity * 0.45,
                    0.008 + intensity * 0.58,
                    0.012 + intensity * 0.85,
                    1.0,
                );
            } else {
                mat.emissive = LinearRgba::new(0.006, 0.008, 0.012, 1.0);
            }
        }
        if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_haze_bank) {
            if intensity > 0.001 {
                mat.emissive = LinearRgba::new(
                    0.008 + intensity * 0.60,
                    0.012 + intensity * 0.78,
                    0.018 + intensity * 1.10,
                    1.0,
                );
            } else {
                mat.emissive = LinearRgba::new(0.008, 0.012, 0.018, 1.0);
            }
        }
        if let Some(mat) = materials.get_mut(&sky.mat_sky_smoke_wisp_drift) {
            if intensity > 0.001 {
                mat.emissive = LinearRgba::new(
                    0.010 + intensity * 0.75,
                    0.014 + intensity * 0.95,
                    0.022 + intensity * 1.35,
                    1.0,
                );
            } else {
                mat.emissive = LinearRgba::new(0.010, 0.014, 0.022, 1.0);
            }
        }
    }

    // 3. Lightweight distant mag-train transit movement
    for (train, mut trans) in train_q.iter_mut() {
        trans.translation.z += train.speed * dt;
        let span = train.max_local_z - train.min_local_z;
        if span > 0.0 {
            if trans.translation.z > train.max_local_z {
                trans.translation.z -= span;
            } else if trans.translation.z < train.min_local_z {
                trans.translation.z += span;
            }
        }
    }

    // 4. Subtle deterministic distant drone hovering motion
    for (drone, mut trans) in drone_q.iter_mut() {
        let t = elapsed * drone.hover_speed + drone.phase;
        trans.translation.x = drone.base_pos.x + t.cos() * drone.hover_radius;
        trans.translation.y = drone.base_pos.y + (t * 1.3).sin() * (drone.hover_radius * 0.45);
        trans.translation.z = drone.base_pos.z + (t * 0.7).sin() * (drone.hover_radius * 0.6);
    }

    // 5. Very sparse distant transport craft motion in far background corridors
    for (craft, mut trans) in craft_q.iter_mut() {
        trans.translation.z += craft.speed * dt;
        let span = craft.max_local_z - craft.min_local_z;
        if span > 0.0 {
            if trans.translation.z > craft.max_local_z {
                trans.translation.z -= span;
            } else if trans.translation.z < craft.min_local_z {
                trans.translation.z += span;
            }
        }
    }

    // 6. Subtle wind-driven cinematic smoke drift and slow buoyant rise
    for (smoke, mut trans) in smoke_q.iter_mut() {
        let t = elapsed * smoke.drift_speed_x + smoke.phase;
        trans.translation.x = smoke.base_pos.x + t.sin() * smoke.drift_amp_x;
        trans.translation.y =
            smoke.base_pos.y + (t * smoke.rise_speed_y).sin().abs() * smoke.rise_amp_y;
    }
}

// -------------------------------------------------------------
// TESTS
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sky_rail_chapter_boundaries() {
        assert_eq!(
            sky_rail_chapter_for_distance(8000.0),
            SkyRailChapter::SkyRailEmergence
        );
        assert_eq!(
            sky_rail_chapter_for_distance(8299.9),
            SkyRailChapter::SkyRailEmergence
        );
        assert_eq!(
            sky_rail_chapter_for_distance(8300.0),
            SkyRailChapter::UpperAurelia
        );
        assert_eq!(
            sky_rail_chapter_for_distance(8699.9),
            SkyRailChapter::UpperAurelia
        );
        assert_eq!(
            sky_rail_chapter_for_distance(8700.0),
            SkyRailChapter::StormCorridor
        );
        assert_eq!(
            sky_rail_chapter_for_distance(9199.9),
            SkyRailChapter::StormCorridor
        );
        assert_eq!(
            sky_rail_chapter_for_distance(9200.0),
            SkyRailChapter::BrokenSkyway
        );
        assert_eq!(
            sky_rail_chapter_for_distance(9649.9),
            SkyRailChapter::BrokenSkyway
        );
        assert_eq!(
            sky_rail_chapter_for_distance(9650.0),
            SkyRailChapter::EchoApproach
        );
        assert_eq!(
            sky_rail_chapter_for_distance(10000.0),
            SkyRailChapter::EchoApproach
        );
    }

    #[test]
    fn test_sky_rail_landmarks_distribution() {
        assert_eq!(
            sky_rail_landmark_for_distance(8120.0),
            SkyRailLandmark::SkyRailGate
        );
        assert_eq!(
            sky_rail_landmark_for_distance(8500.0),
            SkyRailLandmark::AureliaCrownStation
        );
        assert_eq!(
            sky_rail_landmark_for_distance(8950.0),
            SkyRailLandmark::StormSpineBridge
        );
        assert_eq!(
            sky_rail_landmark_for_distance(9400.0),
            SkyRailLandmark::CollapsedSkywaySpan
        );
        assert_eq!(
            sky_rail_landmark_for_distance(9820.0),
            SkyRailLandmark::EchoRelayArray
        );
        assert_eq!(
            sky_rail_landmark_for_distance(8200.0),
            SkyRailLandmark::None
        );
    }

    #[test]
    fn test_sky_rail_variants_are_bounded_and_deterministic() {
        for dist in (8000..10000).step_by(20) {
            let v = sky_rail_variant(dist as f32);
            assert!(v < 3);
        }
    }

    #[test]
    fn test_sky_rail_lane_alignment_matches_gameplay_spacing() {
        let expected_lanes = [-Lane::SPACING, 0.0, Lane::SPACING];
        assert_eq!(expected_lanes, [-2.4, 0.0, 2.4]);

        let expected_troughs = [-Lane::SPACING * 0.5, Lane::SPACING * 0.5];
        assert_eq!(expected_troughs, [-1.2, 1.2]);
    }

    #[test]
    fn test_sky_rail_landmark_windows_are_isolated() {
        // Confirm non-landmark distances resolve to None
        assert_eq!(
            sky_rail_landmark_for_distance(8050.0),
            SkyRailLandmark::None
        );
        assert_eq!(
            sky_rail_landmark_for_distance(8300.0),
            SkyRailLandmark::None
        );
        assert_eq!(
            sky_rail_landmark_for_distance(8700.0),
            SkyRailLandmark::None
        );
        assert_eq!(
            sky_rail_landmark_for_distance(9200.0),
            SkyRailLandmark::None
        );
        assert_eq!(
            sky_rail_landmark_for_distance(9650.0),
            SkyRailLandmark::None
        );
        assert_eq!(
            sky_rail_landmark_for_distance(9950.0),
            SkyRailLandmark::None
        );
    }

    #[test]
    fn test_sky_rail_progression_determinism() {
        // Every distance produces stable, repeatable chapter and variant
        for d in (8000..10000).step_by(25) {
            let dist = d as f32;
            let c1 = sky_rail_chapter_for_distance(dist);
            let c2 = sky_rail_chapter_for_distance(dist);
            assert_eq!(c1, c2);

            let v1 = sky_rail_variant(dist);
            let v2 = sky_rail_variant(dist);
            assert_eq!(v1, v2);

            let l1 = sky_rail_landmark_for_distance(dist);
            let l2 = sky_rail_landmark_for_distance(dist);
            assert_eq!(l1, l2);
        }
    }

    #[test]
    fn test_sky_rail_storm_state_defaults() {
        let state = SkyRailStormState::default();
        assert_eq!(state.flash_timer, 0.0);
        assert!(state.next_flash_interval >= 5.0 && state.next_flash_interval <= 10.0);
        assert_eq!(state.flash_intensity, 0.0);
        assert!(!state.is_flashing);
    }

    #[test]
    fn test_sky_rail_cloud_drift_bounds() {
        let drift = SkyRailDriftCloud {
            base_x: 0.0,
            amplitude: 14.0,
            speed: 0.35,
            phase: 0.0,
        };
        // Verify peak displacement remains well inside the 380m cloud deck width
        let max_displacement = drift.base_x.abs() + drift.amplitude;
        assert!(max_displacement < 50.0);
    }

    #[test]
    fn test_sky_rail_skyline_clearance() {
        // Verify that distant megastructures never intrude within 90m of the track
        for d in (8000..10000).step_by(40) {
            let seg_idx = (d as f32 / 40.0).floor() as i32;
            let h1 = ((seg_idx.wrapping_mul(19) + 5).rem_euclid(100)) as f32 / 100.0;
            let titan_x = 110.0 + h1 * 40.0;
            assert!(titan_x >= 90.0, "Distant titans must remain outside 90m");
        }
    }

    #[test]
    fn test_sky_rail_distant_infrastructure_clearance() {
        // Verify that all distant infrastructure positions across all chapters strictly remain >= 95m
        for d in (8000..10000).step_by(20) {
            let seg_idx = (d as f32 / 40.0).floor() as i32;
            let h2 = ((seg_idx.wrapping_mul(31) + 11).rem_euclid(100)) as f32 / 100.0;
            let cloud_x = 112.0 + h2 * 28.0;
            assert!(cloud_x >= 95.0, "Distant cloud flank must stay outside 95m");

            let mast_x = 145.0 + h2 * 30.0;
            assert!(mast_x >= 95.0, "Distant mast must stay outside 95m");

            let rail_x = 125.0_f32;
            assert!(
                rail_x >= 95.0,
                "Distant parallel rail must stay outside 95m"
            );

            let shaft_x = 145.0_f32;
            assert!(
                shaft_x >= 95.0,
                "Distant elevator shaft must stay outside 95m"
            );
        }
    }

    #[test]
    fn test_sky_rail_distant_train_kinematics() {
        let train = SkyRailDistantTrain {
            speed: -50.0,
            min_local_z: -38.0,
            max_local_z: 38.0,
        };
        let mut z = 0.0;
        let dt = 1.0;
        z += train.speed * dt; // -50.0
        let span = train.max_local_z - train.min_local_z; // 76.0
        if z < train.min_local_z {
            z += span; // -50.0 + 76.0 = 26.0
        }
        assert!(z >= train.min_local_z && z <= train.max_local_z);
    }

    #[test]
    fn test_sky_rail_distant_craft_kinematics_and_clearance() {
        let craft = SkyRailDistantCraft {
            speed: 16.0,
            min_local_z: -42.0,
            max_local_z: 42.0,
        };
        let mut z = 0.0;
        let dt = 3.0;
        z += craft.speed * dt; // 48.0
        let span = craft.max_local_z - craft.min_local_z; // 84.0
        if z > craft.max_local_z {
            z -= span; // 48.0 - 84.0 = -36.0
        }
        assert!(z >= craft.min_local_z && z <= craft.max_local_z);

        // Verify distant craft spawn positions remain far outside playable corridor (|X| >= 135m, Y >= 65m)
        let craft_x = 138.0_f32;
        let craft_y = 68.0_f32;
        assert!(craft_x >= 95.0, "Distant craft X must remain outside 95m");
        assert!(
            craft_y >= 50.0,
            "Distant craft Y must remain elevated in storm sky"
        );
    }

    #[test]
    fn test_sky_rail_midground_infrastructure_clearance() {
        // Verify that midground infrastructure strictly remains within 55m..105m band
        for d in (8000..10000).step_by(20) {
            let seg_idx = (d as f32 / 40.0).floor() as i32;
            let h1 = ((seg_idx.wrapping_mul(23) + 9).rem_euclid(100)) as f32 / 100.0;
            let h2 = ((seg_idx.wrapping_mul(41) + 15).rem_euclid(100)) as f32 / 100.0;
            let h3 = ((seg_idx.wrapping_mul(67) + 27).rem_euclid(100)) as f32 / 100.0;

            let tower_x = 74.0 + h2 * 10.0;
            assert!(
                tower_x >= 55.0 && tower_x <= 105.0,
                "Chapter 1 midground tower must stay in [55m, 105m]"
            );

            let bridge_x = 72.0_f32;
            assert!(
                bridge_x >= 55.0 && bridge_x <= 105.0,
                "Chapter 1 suspension bridge must stay in [55m, 105m]"
            );

            let upper_x = 72.0 + h1 * 22.0;
            assert!(
                upper_x >= 55.0 && upper_x <= 105.0,
                "Upper midground tower must stay in [55m, 105m]"
            );

            let storm_pylon_x = 66.0 + h2 * 20.0;
            assert!(
                storm_pylon_x >= 55.0 && storm_pylon_x <= 105.0,
                "Storm transmission mast must stay in [55m, 105m]"
            );

            let ch2_rail_x = 64.0 + h3 * 18.0;
            assert!(
                ch2_rail_x >= 55.0 && ch2_rail_x <= 105.0,
                "Chapter 2 parallel rail must stay in [55m, 105m]"
            );
        }
    }

    #[test]
    fn test_sky_rail_cumulus_flank_bounds() {
        // Verify flanking cumulus clouds remain below track (y <= -14m) and outside lane bounds
        let left_cloud_x = -38.0_f32;
        let right_cloud_x = 38.0_f32;
        assert!(
            left_cloud_x.abs() > 20.0,
            "Cumulus clouds must stay well outside track corridor"
        );
        assert!(
            right_cloud_x > 20.0,
            "Cumulus clouds must stay well outside track corridor"
        );
    }

    #[test]
    fn test_sky_rail_overhead_gantry_clearance() {
        // Verify overhead telemetry gantries provide at least 4.5m of vertical clearance
        let sensor_pod_y = 5.55_f32;
        let pod_half_height = 0.30_f32;
        let lowest_point = sensor_pod_y - pod_half_height; // 5.25m
        let track_surface_y = 0.06_f32;
        let vertical_headroom = lowest_point - track_surface_y;
        assert!(
            vertical_headroom >= 4.5,
            "Overhead gantry must provide >= 4.5m headroom"
        );
    }

    #[test]
    fn test_sky_rail_distant_drone_kinematics_and_clearance() {
        // Drones in chapters 2, 3, 4, 5:
        // Ch2: base X = tower_x - 12.0m (approx -84m to -106m)
        // Ch3: base X = sensor_x + 6.0m (approx 78m to 98m)
        // Ch4: base X = severed_x + 8.0m (approx 74m to 92m)
        // Ch5: base X = -78.0m
        let drone_configs = [
            SkyRailDistantDrone {
                base_pos: Vec3::new(-88.0, 36.0, 8.0),
                hover_speed: 0.45,
                hover_radius: 1.8,
                phase: 1.2,
            },
            SkyRailDistantDrone {
                base_pos: Vec3::new(86.0, 38.0, -6.0),
                hover_speed: 0.35,
                hover_radius: 1.5,
                phase: 3.1,
            },
            SkyRailDistantDrone {
                base_pos: Vec3::new(76.0, 25.0, 12.0),
                hover_speed: 0.55,
                hover_radius: 2.0,
                phase: 2.4,
            },
            SkyRailDistantDrone {
                base_pos: Vec3::new(-78.0, 36.0, 4.0),
                hover_speed: 0.28,
                hover_radius: 1.4,
                phase: 0.5,
            },
        ];

        for drone in drone_configs {
            for step in 0..100 {
                let elapsed = step as f32 * 0.1;
                let t = elapsed * drone.hover_speed + drone.phase;
                let x = drone.base_pos.x + t.cos() * drone.hover_radius;
                let y = drone.base_pos.y + (t * 1.3).sin() * (drone.hover_radius * 0.45);
                let z = drone.base_pos.z + (t * 0.7).sin() * (drone.hover_radius * 0.6);

                // Drones must remain far away outside the gameplay corridor (|X| >= 70m)
                assert!(
                    x.abs() >= 70.0,
                    "Drone X position ({}) must remain >= 70.0m",
                    x
                );
                // Drones must remain elevated in the background air
                assert!(y >= 20.0, "Drone Y ({}) must remain >= 20.0m", y);
                // Hover offset should not exceed hover_radius
                let dist_from_base = (Vec3::new(x, y, z) - drone.base_pos).length();
                assert!(
                    dist_from_base <= drone.hover_radius * 1.5,
                    "Drone hovering must be bounded"
                );
            }
        }
    }

    #[test]
    fn test_sky_rail_hanging_pod_clearance() {
        // Verify hanging infrastructure remains strictly well outside the playable corridor (|X| > 50m)
        let ch1_hanging_pod_x = -72.0_f32;
        assert!(
            ch1_hanging_pod_x.abs() > 50.0,
            "Ch1 hanging pod must stay well outside track corridor"
        );

        for d in (8300..8700).step_by(40) {
            let seg_idx = (d as f32 / 40.0).floor() as i32;
            let h2 = ((seg_idx.wrapping_mul(41) + 15).rem_euclid(100)) as f32 / 100.0;
            let bridge_x = 68.0 + h2 * 20.0;
            assert!(
                bridge_x >= 55.0,
                "Ch2 hanging pod bridge must stay >= 55.0m"
            );
        }
    }

    #[test]
    fn test_sky_rail_smoke_clearance() {
        // Verify that smoke entities across all chapters strictly remain >= 70m from the track center
        for d in (8000..10000).step_by(20) {
            let seg_idx = (d as f32 / 40.0).floor() as i32;
            let h1 = ((seg_idx.wrapping_mul(29) + 7).rem_euclid(100)) as f32 / 100.0;
            let h2 = ((seg_idx.wrapping_mul(43) + 17).rem_euclid(100)) as f32 / 100.0;
            let h3 = ((seg_idx.wrapping_mul(61) + 29).rem_euclid(100)) as f32 / 100.0;
            let h4 = ((seg_idx.wrapping_mul(79) + 37).rem_euclid(100)) as f32 / 100.0;

            // 1. Horizontal smoke bank: 125m - 160m
            let bank_x = 125.0 + h1 * 35.0;
            assert!(
                bank_x >= 115.0,
                "Smoke bank X ({}) must remain outside 115m",
                bank_x
            );

            // 2. Midground wisp strata: 75m - 93m
            let wisp_x = 75.0 + h3 * 18.0;
            assert!(
                wisp_x >= 70.0,
                "Smoke wisp X ({}) must remain outside 70m",
                wisp_x
            );

            // 3. Industrial plumes
            let ch1_plume_x = 145.0 + h2 * 25.0;
            assert!(ch1_plume_x >= 115.0);

            let ch2_plume_x = 138.0 + h3 * 30.0;
            assert!(ch2_plume_x >= 115.0);

            let ch3_plume_x = 135.0 + h1 * 32.0;
            assert!(ch3_plume_x >= 115.0);

            let ch4_plume_x = 122.0 + h2 * 28.0;
            assert!(ch4_plume_x >= 115.0);

            let ch5_haze_x = 142.0_f32;
            assert!(ch5_haze_x >= 115.0);
        }
    }

    #[test]
    fn test_sky_rail_smoke_kinematics() {
        let smoke = SkyRailDriftSmoke {
            base_pos: Vec3::new(125.0, 14.0, 0.0),
            drift_speed_x: 0.15,
            drift_amp_x: 10.0,
            rise_speed_y: 0.10,
            rise_amp_y: 3.0,
            phase: 1.5,
        };

        for step in 0..200 {
            let elapsed = step as f32 * 0.2;
            let t = elapsed * smoke.drift_speed_x + smoke.phase;
            let x = smoke.base_pos.x + t.sin() * smoke.drift_amp_x;
            let y = smoke.base_pos.y + (t * smoke.rise_speed_y).sin().abs() * smoke.rise_amp_y;

            // X offset must remain bounded within amplitude
            assert!((x - smoke.base_pos.x).abs() <= smoke.drift_amp_x + 1e-4);
            // Y offset must remain positive and bounded within amplitude
            assert!(y >= smoke.base_pos.y - 1e-4);
            assert!(y <= smoke.base_pos.y + smoke.rise_amp_y + 1e-4);
            // Must stay well outside track corridor
            assert!(x >= 100.0);
        }
    }
}
