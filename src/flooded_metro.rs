use crate::environment::EnvironmentAssets;
use crate::types::*;
use bevy::prelude::*;

/// Authored visual chapters for Zone 4 (6,000m–8,000m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodedMetroChapter {
    FloodgateDescent,
    SubmergedStation,
    ReservoirCrossing,
    SunkenTransit,
    DeepLineApproach,
}

pub fn flooded_metro_chapter_for_distance(distance: f32) -> FloodedMetroChapter {
    match distance {
        d if d < 6300.0 => FloodedMetroChapter::FloodgateDescent,
        d if d < 6750.0 => FloodedMetroChapter::SubmergedStation,
        d if d < 7200.0 => FloodedMetroChapter::ReservoirCrossing,
        d if d < 7650.0 => FloodedMetroChapter::SunkenTransit,
        _ => FloodedMetroChapter::DeepLineApproach,
    }
}

pub fn flooded_metro_variant(distance: f32) -> u8 {
    let segment_index = (distance.max(0.0) / 40.0).floor() as i32;
    (segment_index.rem_euclid(3)) as u8
}

/// Authored hero landmarks that anchor each chapter with memorable visual spectacle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodedMetroLandmark {
    None,
    FloodgateDescentSluice,
    SubmergedStationComplex,
    ReservoirCavernCenterpiece,
    SunkenTransitWreckage,
    DeepLineBulkheadPortal,
}

pub fn flooded_metro_landmark_for_distance(distance: f32) -> FloodedMetroLandmark {
    match distance {
        d if (6120.0..6160.0).contains(&d) => FloodedMetroLandmark::FloodgateDescentSluice,
        d if (6480.0..6520.0).contains(&d) => FloodedMetroLandmark::SubmergedStationComplex,
        d if (6960.0..7000.0).contains(&d) => FloodedMetroLandmark::ReservoirCavernCenterpiece,
        d if (7360.0..7400.0).contains(&d) => FloodedMetroLandmark::SunkenTransitWreckage,
        d if (7840.0..7880.0).contains(&d) => FloodedMetroLandmark::DeepLineBulkheadPortal,
        _ => FloodedMetroLandmark::None,
    }
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct FloodedMetroAssets {
    // --- Submerged Track & Drainage Canals ---
    pub mesh_submerged_ballast_bed: Handle<Mesh>,
    pub mesh_wet_ballast_strip: Handle<Mesh>,
    pub mesh_wet_ballast_berm: Handle<Mesh>,
    pub mesh_corroded_rail_base: Handle<Mesh>,
    pub mesh_corroded_rail_crown: Handle<Mesh>,
    pub mesh_submerged_heavy_sleeper: Handle<Mesh>,
    pub mesh_sleeper_water_fillet: Handle<Mesh>,
    pub mesh_sleeper_tie_plate: Handle<Mesh>,
    pub mesh_canal_lip: Handle<Mesh>,
    pub mesh_canal_trough: Handle<Mesh>,
    pub mesh_canal_water: Handle<Mesh>,
    pub mesh_canal_grate: Handle<Mesh>,
    pub mesh_faded_lane_marker: Handle<Mesh>,
    pub mesh_rail_puddle: Handle<Mesh>,
    pub mesh_transit_lane_slab: Handle<Mesh>,
    pub mesh_transit_curb_border: Handle<Mesh>,
    pub mesh_transit_lane_tread: Handle<Mesh>,
    pub mesh_inter_lane_water: Handle<Mesh>,
    pub mesh_flank_water_reservoir: Handle<Mesh>,
    pub mesh_submerged_bed_foundation: Handle<Mesh>,
    pub mesh_submerged_transverse_beam: Handle<Mesh>,
    pub mesh_channel_drain_grate: Handle<Mesh>,
    pub mesh_transit_hazard_marker: Handle<Mesh>,
    pub mesh_transit_puddle_sheet: Handle<Mesh>,

    // --- Structural Geometry ---
    pub mesh_wall_main: Handle<Mesh>,
    pub mesh_wall_reservoir: Handle<Mesh>,
    pub mesh_concrete_rib: Handle<Mesh>,
    pub mesh_ceiling_beam: Handle<Mesh>,
    pub mesh_ceiling_arch: Handle<Mesh>,
    pub mesh_support_column: Handle<Mesh>,
    pub mesh_arch_column: Handle<Mesh>,
    pub mesh_wall_panel_seam: Handle<Mesh>,
    pub mesh_expansion_bracket: Handle<Mesh>,
    pub mesh_steel_crossbrace: Handle<Mesh>,
    pub mesh_maintenance_girder: Handle<Mesh>,

    // --- Wall Modules & Fixtures ---
    pub mesh_bulkhead_frame: Handle<Mesh>,
    pub mesh_bulkhead_door: Handle<Mesh>,
    pub mesh_watertight_hatch: Handle<Mesh>,
    pub mesh_electrical_cabinet: Handle<Mesh>,
    pub mesh_terminal_box: Handle<Mesh>,
    pub mesh_drainage_outlet: Handle<Mesh>,
    pub mesh_water_cascade: Handle<Mesh>,
    pub mesh_conduit_tray: Handle<Mesh>,
    pub mesh_ladder_rail: Handle<Mesh>,
    pub mesh_ladder_rung: Handle<Mesh>,

    // --- Pipe Families ---
    pub mesh_pipe_drain_huge: Handle<Mesh>,
    pub mesh_pipe_medium: Handle<Mesh>,
    pub mesh_pipe_small: Handle<Mesh>,
    pub mesh_pipe_vertical: Handle<Mesh>,
    pub mesh_pipe_flange: Handle<Mesh>,
    pub mesh_pipe_bracket: Handle<Mesh>,
    pub mesh_pipe_elbow: Handle<Mesh>,
    pub mesh_valve_large: Handle<Mesh>,
    pub mesh_pressure_gauge: Handle<Mesh>,

    // --- Electrical & Lighting ---
    pub mesh_cable_bundle: Handle<Mesh>,
    pub mesh_cable_tray: Handle<Mesh>,
    pub mesh_caged_fixture: Handle<Mesh>,
    pub mesh_emergency_light: Handle<Mesh>,
    pub mesh_warning_panel: Handle<Mesh>,
    pub mesh_hazard_sign: Handle<Mesh>,

    // --- Water & Flood Geometry ---
    pub mesh_water_trackside: Handle<Mesh>,
    pub mesh_water_reservoir_wide: Handle<Mesh>,
    pub mesh_water_puddle: Handle<Mesh>,
    pub mesh_waterline_mark: Handle<Mesh>,
    pub mesh_water_stain_vert: Handle<Mesh>,
    pub mesh_drain_channel: Handle<Mesh>,
    pub mesh_floating_debris: Handle<Mesh>,
    pub mesh_floating_drum: Handle<Mesh>,

    // --- Chapter 1: Floodgate Descent ---
    pub mesh_gate_portal_arch: Handle<Mesh>,
    pub mesh_gate_pillar: Handle<Mesh>,
    pub mesh_gate_leaf_heavy: Handle<Mesh>,
    pub mesh_hydraulic_cylinder: Handle<Mesh>,
    pub mesh_hydraulic_rod: Handle<Mesh>,
    pub mesh_flood_level_gauge: Handle<Mesh>,

    // --- Chapter 2: Submerged Station ---
    pub mesh_station_platform: Handle<Mesh>,
    pub mesh_platform_tactile_edge: Handle<Mesh>,
    pub mesh_station_bench: Handle<Mesh>,
    pub mesh_station_sign_board: Handle<Mesh>,
    pub mesh_station_pillar_tile: Handle<Mesh>,
    pub mesh_ticket_turnstile: Handle<Mesh>,
    pub mesh_submerged_stairs: Handle<Mesh>,

    // --- Chapter 3: Reservoir Crossing ---
    pub mesh_reservoir_pier_giant: Handle<Mesh>,
    pub mesh_reservoir_truss_span: Handle<Mesh>,
    pub mesh_catwalk_deck: Handle<Mesh>,
    pub mesh_catwalk_railing: Handle<Mesh>,
    pub mesh_intake_tower: Handle<Mesh>,
    pub mesh_intake_grate: Handle<Mesh>,
    pub mesh_pump_turbine: Handle<Mesh>,
    pub mesh_sluice_pipe_huge: Handle<Mesh>,

    // --- Chapter 4: Sunken Transit (Industrial Freight Consist) ---
    pub mesh_train_body: Handle<Mesh>,
    pub mesh_train_roof: Handle<Mesh>,
    pub mesh_train_window: Handle<Mesh>,
    pub mesh_train_door: Handle<Mesh>,
    pub mesh_train_bogie: Handle<Mesh>,
    pub mesh_mangled_rail: Handle<Mesh>,

    // Industrial Freight Locomotive
    pub mesh_freight_loco_body: Handle<Mesh>,
    pub mesh_freight_loco_cab: Handle<Mesh>,
    pub mesh_freight_cowcatcher: Handle<Mesh>,
    pub mesh_freight_loco_visor: Handle<Mesh>,
    pub mesh_freight_loco_headlight: Handle<Mesh>,
    pub mesh_freight_roof_blister: Handle<Mesh>,
    pub mesh_freight_exhaust_stack: Handle<Mesh>,

    // Modular Cargo Wagons
    pub mesh_freight_boxcar_body: Handle<Mesh>,
    pub mesh_freight_boxcar_door: Handle<Mesh>,
    pub mesh_freight_rib: Handle<Mesh>,
    pub mesh_freight_bulk_wall_long: Handle<Mesh>,
    pub mesh_freight_bulk_wall_end: Handle<Mesh>,
    pub mesh_freight_bulk_floor: Handle<Mesh>,
    pub mesh_freight_bulk_scrap: Handle<Mesh>,
    pub mesh_freight_tank_body: Handle<Mesh>,
    pub mesh_freight_tank_cap: Handle<Mesh>,
    pub mesh_freight_tank_saddle: Handle<Mesh>,
    pub mesh_freight_tank_dome: Handle<Mesh>,
    pub mesh_freight_flatbed_chassis: Handle<Mesh>,
    pub mesh_freight_cable_spool: Handle<Mesh>,
    pub mesh_freight_equipment_crate: Handle<Mesh>,
    pub mesh_freight_coupler: Handle<Mesh>,

    // --- Chapter 5: Deep-Line Approach ---
    pub mesh_deep_portal_arch: Handle<Mesh>,
    pub mesh_deep_bulkhead_leaf: Handle<Mesh>,
    pub mesh_locking_clamp: Handle<Mesh>,
    pub mesh_echo_conduit_ring: Handle<Mesh>,
    pub mesh_echo_sensor_relay: Handle<Mesh>,

    // --- Background Architectural Silhouettes ---
    pub mesh_bg_tunnel_arch: Handle<Mesh>,
    pub mesh_bg_arch_column: Handle<Mesh>,
    pub mesh_bg_vent_shaft_body: Handle<Mesh>,
    pub mesh_bg_vent_shaft_cap: Handle<Mesh>,
    pub mesh_bg_pump_housing: Handle<Mesh>,
    pub mesh_bg_pipe_trunk: Handle<Mesh>,
    pub mesh_bg_maint_bridge_deck: Handle<Mesh>,
    pub mesh_bg_maint_bridge_truss: Handle<Mesh>,
    pub mesh_bg_reservoir_wall_colossal: Handle<Mesh>,
    pub mesh_bg_aqueduct_pillar: Handle<Mesh>,
    pub mesh_bg_distant_train: Handle<Mesh>,
    pub mesh_bg_blast_door_frame: Handle<Mesh>,
    pub mesh_bg_blast_door_panel: Handle<Mesh>,

    // --- WaterDrip Geometry ---
    pub mesh_water_droplet: Handle<Mesh>,

    // --- Materials ---
    pub mat_concrete: Handle<StandardMaterial>,
    pub mat_wet_concrete: Handle<StandardMaterial>,
    pub mat_steel: Handle<StandardMaterial>,
    pub mat_rusted_steel: Handle<StandardMaterial>,
    pub mat_corroded_copper: Handle<StandardMaterial>,
    pub mat_water: Handle<StandardMaterial>,
    pub mat_water_puddle: Handle<StandardMaterial>,
    pub mat_water_stain: Handle<StandardMaterial>,
    pub mat_cable: Handle<StandardMaterial>,
    pub mat_teal_glow: Handle<StandardMaterial>,
    pub mat_warning_amber: Handle<StandardMaterial>,
    pub mat_emergency_red: Handle<StandardMaterial>,
    pub mat_hazard_stripe: Handle<StandardMaterial>,
    pub mat_train_body: Handle<StandardMaterial>,
    pub mat_train_window: Handle<StandardMaterial>,
    pub mat_submerged_wood: Handle<StandardMaterial>,

    // --- Dedicated Submerged Track Materials ---
    pub mat_wet_ballast: Handle<StandardMaterial>,
    pub mat_corroded_rail: Handle<StandardMaterial>,
    pub mat_submerged_sleeper: Handle<StandardMaterial>,
    pub mat_canal_wall: Handle<StandardMaterial>,
    pub mat_faded_lane_marker: Handle<StandardMaterial>,
    pub mat_transit_slab: Handle<StandardMaterial>,
    pub mat_transit_curb: Handle<StandardMaterial>,
    pub mat_transit_tread: Handle<StandardMaterial>,

    // --- Background Silhouette Materials ---
    pub mat_bg_dark_structure: Handle<StandardMaterial>,
    pub mat_bg_metal_structure: Handle<StandardMaterial>,

    // --- WaterDrip Material ---
    pub mat_water_droplet: Handle<StandardMaterial>,
}

pub fn init_flooded_metro_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let assets = FloodedMetroAssets {
        // --- Dedicated Submerged Track & Drainage Canals ---
        mesh_submerged_ballast_bed: meshes.add(Cuboid::new(9.2, 0.44, 40.0)),
        mesh_wet_ballast_strip: meshes.add(Cuboid::new(1.1, 0.16, 40.0)),
        mesh_wet_ballast_berm: meshes.add(Cuboid::new(1.3, 0.22, 40.0)),
        mesh_corroded_rail_base: meshes.add(Cuboid::new(0.22, 0.08, 40.0)),
        mesh_corroded_rail_crown: meshes.add(Cuboid::new(0.14, 0.12, 40.0)),
        mesh_submerged_heavy_sleeper: meshes.add(Cuboid::new(7.8, 0.24, 0.56)),
        mesh_sleeper_water_fillet: meshes.add(Cuboid::new(7.6, 0.025, 0.72)),
        mesh_sleeper_tie_plate: meshes.add(Cuboid::new(0.44, 0.06, 0.46)),
        mesh_canal_lip: meshes.add(Cuboid::new(0.35, 0.28, 40.0)),
        mesh_canal_trough: meshes.add(Cuboid::new(1.10, 0.40, 40.0)),
        mesh_canal_water: meshes.add(Cuboid::new(0.95, 0.05, 40.0)),
        mesh_canal_grate: meshes.add(Cuboid::new(1.05, 0.08, 1.4)),
        mesh_faded_lane_marker: meshes.add(Cuboid::new(0.28, 0.04, 2.2)),
        mesh_rail_puddle: meshes.add(Cuboid::new(2.4, 0.04, 12.0)),
        mesh_transit_lane_slab: meshes.add(Cuboid::new(1.80, 0.26, 40.0)),
        mesh_transit_curb_border: meshes.add(Cuboid::new(0.12, 0.16, 40.0)),
        mesh_transit_lane_tread: meshes.add(Cuboid::new(0.60, 0.03, 40.0)),
        mesh_inter_lane_water: meshes.add(Cuboid::new(0.40, 0.04, 40.0)),
        mesh_flank_water_reservoir: meshes.add(Cuboid::new(2.80, 0.04, 40.0)),
        mesh_submerged_bed_foundation: meshes.add(Cuboid::new(12.0, 0.30, 40.0)),
        mesh_submerged_transverse_beam: meshes.add(Cuboid::new(8.0, 0.22, 0.45)),
        mesh_channel_drain_grate: meshes.add(Cuboid::new(0.40, 0.08, 1.2)),
        mesh_transit_hazard_marker: meshes.add(Cuboid::new(0.35, 0.035, 2.4)),
        mesh_transit_puddle_sheet: meshes.add(Cuboid::new(1.50, 0.02, 10.0)),

        // --- Structural Geometry ---
        mesh_wall_main: meshes.add(Cuboid::new(0.50, 8.6, 40.0)),
        mesh_wall_reservoir: meshes.add(Cuboid::new(0.80, 16.0, 40.0)),
        mesh_concrete_rib: meshes.add(Cuboid::new(0.85, 8.8, 1.4)),
        mesh_ceiling_beam: meshes.add(Cuboid::new(19.0, 0.55, 0.85)),
        mesh_ceiling_arch: meshes.add(Cuboid::new(18.0, 0.65, 1.1)),
        mesh_support_column: meshes.add(Cuboid::new(1.1, 8.6, 1.1)),
        mesh_arch_column: meshes.add(Cuboid::new(0.55, 6.4, 0.55)),
        mesh_wall_panel_seam: meshes.add(Cuboid::new(0.08, 8.6, 0.18)),
        mesh_expansion_bracket: meshes.add(Cuboid::new(0.30, 0.40, 0.30)),
        mesh_steel_crossbrace: meshes.add(Cuboid::new(0.18, 0.18, 6.5)),
        mesh_maintenance_girder: meshes.add(Cuboid::new(0.35, 0.45, 40.0)),

        // --- Wall Modules & Doors ---
        mesh_bulkhead_frame: meshes.add(Cuboid::new(0.40, 7.4, 0.80)),
        mesh_bulkhead_door: meshes.add(Cuboid::new(0.28, 5.8, 4.8)),
        mesh_watertight_hatch: meshes.add(Cuboid::new(0.20, 2.6, 1.5)),
        mesh_electrical_cabinet: meshes.add(Cuboid::new(0.45, 1.9, 1.3)),
        mesh_terminal_box: meshes.add(Cuboid::new(0.25, 0.35, 0.25)),
        mesh_drainage_outlet: meshes.add(Cylinder::new(0.75, 0.60)),
        mesh_water_cascade: meshes.add(Cuboid::new(0.35, 2.4, 0.35)),
        mesh_conduit_tray: meshes.add(Cuboid::new(0.35, 0.14, 20.0)),
        mesh_ladder_rail: meshes.add(Cuboid::new(0.10, 3.6, 0.10)),
        mesh_ladder_rung: meshes.add(Cuboid::new(0.80, 0.08, 0.08)),

        // --- Pipe Families ---
        mesh_pipe_drain_huge: meshes.add(Cylinder::new(0.72, 40.0)),
        mesh_pipe_medium: meshes.add(Cylinder::new(0.28, 40.0)),
        mesh_pipe_small: meshes.add(Cylinder::new(0.12, 40.0)),
        mesh_pipe_vertical: meshes.add(Cylinder::new(0.22, 7.2)),
        mesh_pipe_flange: meshes.add(Cylinder::new(0.88, 0.22)),
        mesh_pipe_bracket: meshes.add(Cuboid::new(1.6, 0.22, 0.35)),
        mesh_pipe_elbow: meshes.add(Cylinder::new(0.30, 0.60)),
        mesh_valve_large: meshes.add(Cylinder::new(0.55, 0.16)),
        mesh_pressure_gauge: meshes.add(Cylinder::new(0.22, 0.08)),

        // --- Electrical & Lighting ---
        mesh_cable_bundle: meshes.add(Cylinder::new(0.12, 24.0)),
        mesh_cable_tray: meshes.add(Cuboid::new(0.45, 0.16, 40.0)),
        mesh_caged_fixture: meshes.add(Cuboid::new(0.22, 0.28, 0.22)),
        mesh_emergency_light: meshes.add(Sphere::new(0.15)),
        mesh_warning_panel: meshes.add(Cuboid::new(0.10, 1.1, 2.4)),
        mesh_hazard_sign: meshes.add(Cuboid::new(0.08, 0.60, 1.4)),

        // --- Water & Flood Geometry ---
        mesh_water_trackside: meshes.add(Cuboid::new(9.5, 0.08, 40.0)),
        mesh_water_reservoir_wide: meshes.add(Cuboid::new(26.0, 0.08, 40.0)),
        mesh_water_puddle: meshes.add(Cuboid::new(3.8, 0.04, 14.0)),
        mesh_waterline_mark: meshes.add(Cuboid::new(0.06, 0.42, 40.0)),
        mesh_water_stain_vert: meshes.add(Cuboid::new(0.06, 2.8, 4.5)),
        mesh_drain_channel: meshes.add(Cuboid::new(0.28, 0.12, 20.0)),
        mesh_floating_debris: meshes.add(Cuboid::new(1.4, 0.18, 1.4)),
        mesh_floating_drum: meshes.add(Cylinder::new(0.34, 0.95)),

        // --- Chapter 1: Floodgate Descent ---
        mesh_gate_portal_arch: meshes.add(Cuboid::new(21.0, 1.8, 2.4)),
        mesh_gate_pillar: meshes.add(Cuboid::new(2.2, 9.6, 2.4)),
        mesh_gate_leaf_heavy: meshes.add(Cuboid::new(5.8, 5.4, 0.65)),
        mesh_hydraulic_cylinder: meshes.add(Cylinder::new(0.32, 5.2)),
        mesh_hydraulic_rod: meshes.add(Cylinder::new(0.18, 4.2)),
        mesh_flood_level_gauge: meshes.add(Cuboid::new(0.12, 4.8, 0.55)),

        // --- Chapter 2: Submerged Station ---
        mesh_station_platform: meshes.add(Cuboid::new(5.2, 0.65, 40.0)),
        mesh_platform_tactile_edge: meshes.add(Cuboid::new(0.42, 0.06, 40.0)),
        mesh_station_bench: meshes.add(Cuboid::new(2.4, 0.65, 0.75)),
        mesh_station_sign_board: meshes.add(Cuboid::new(3.8, 0.95, 0.14)),
        mesh_station_pillar_tile: meshes.add(Cuboid::new(0.75, 6.2, 0.75)),
        mesh_ticket_turnstile: meshes.add(Cuboid::new(1.1, 1.7, 0.85)),
        mesh_submerged_stairs: meshes.add(Cuboid::new(3.2, 1.8, 4.5)),

        // --- Chapter 3: Reservoir Crossing ---
        mesh_reservoir_pier_giant: meshes.add(Cuboid::new(3.2, 12.0, 3.2)),
        mesh_reservoir_truss_span: meshes.add(Cuboid::new(28.0, 1.6, 4.2)),
        mesh_catwalk_deck: meshes.add(Cuboid::new(3.2, 0.18, 14.0)),
        mesh_catwalk_railing: meshes.add(Cuboid::new(0.10, 0.82, 14.0)),
        mesh_intake_tower: meshes.add(Cylinder::new(2.6, 9.2)),
        mesh_intake_grate: meshes.add(Cuboid::new(3.8, 3.8, 0.22)),
        mesh_pump_turbine: meshes.add(Cuboid::new(3.4, 2.6, 3.4)),
        mesh_sluice_pipe_huge: meshes.add(Cylinder::new(1.1, 22.0)),

        // --- Chapter 4: Sunken Transit (Industrial Freight Consist) ---
        mesh_train_body: meshes.add(Cuboid::new(3.2, 2.6, 12.0)),
        mesh_train_roof: meshes.add(Cuboid::new(3.25, 0.32, 11.8)),
        mesh_train_window: meshes.add(Cuboid::new(0.08, 0.85, 1.7)),
        mesh_train_door: meshes.add(Cuboid::new(0.12, 2.1, 2.4)),
        mesh_train_bogie: meshes.add(Cuboid::new(2.6, 0.55, 3.0)),
        mesh_mangled_rail: meshes.add(Cuboid::new(0.16, 0.22, 6.5)),

        // Industrial Freight Locomotive
        mesh_freight_loco_body: meshes.add(Cuboid::new(3.2, 2.6, 12.0)),
        mesh_freight_loco_cab: meshes.add(Cuboid::new(3.25, 1.25, 3.8)),
        mesh_freight_cowcatcher: meshes.add(Cuboid::new(3.3, 0.7, 0.6)),
        mesh_freight_loco_visor: meshes.add(Cuboid::new(2.4, 0.55, 0.15)),
        mesh_freight_loco_headlight: meshes.add(Sphere::new(0.20)),
        mesh_freight_roof_blister: meshes.add(Cuboid::new(1.8, 0.45, 4.2)),
        mesh_freight_exhaust_stack: meshes.add(Cylinder::new(0.25, 0.9)),

        // Modular Cargo Wagons
        mesh_freight_boxcar_body: meshes.add(Cuboid::new(3.2, 2.6, 11.5)),
        mesh_freight_boxcar_door: meshes.add(Cuboid::new(0.12, 2.1, 2.4)),
        mesh_freight_rib: meshes.add(Cuboid::new(0.10, 2.5, 0.14)),
        mesh_freight_bulk_wall_long: meshes.add(Cuboid::new(0.16, 1.5, 11.5)),
        mesh_freight_bulk_wall_end: meshes.add(Cuboid::new(3.1, 1.5, 0.16)),
        mesh_freight_bulk_floor: meshes.add(Cuboid::new(3.0, 0.25, 11.3)),
        mesh_freight_bulk_scrap: meshes.add(Cuboid::new(2.7, 0.75, 9.5)),
        mesh_freight_tank_body: meshes.add(Cylinder::new(1.35, 10.0)),
        mesh_freight_tank_cap: meshes.add(Sphere::new(1.34)),
        mesh_freight_tank_saddle: meshes.add(Cuboid::new(3.1, 0.55, 1.2)),
        mesh_freight_tank_dome: meshes.add(Cylinder::new(0.5, 0.45)),
        mesh_freight_flatbed_chassis: meshes.add(Cuboid::new(3.2, 0.5, 11.5)),
        mesh_freight_cable_spool: meshes.add(Cylinder::new(1.1, 0.8)),
        mesh_freight_equipment_crate: meshes.add(Cuboid::new(1.4, 1.2, 1.8)),
        mesh_freight_coupler: meshes.add(Cuboid::new(0.4, 0.4, 0.9)),

        // --- Chapter 5: Deep-Line Approach ---
        mesh_deep_portal_arch: meshes.add(Cuboid::new(20.0, 2.0, 2.6)),
        mesh_deep_bulkhead_leaf: meshes.add(Cuboid::new(4.6, 5.8, 0.75)),
        mesh_locking_clamp: meshes.add(Cuboid::new(0.55, 0.55, 1.6)),
        mesh_echo_conduit_ring: meshes.add(Cylinder::new(3.8, 0.38)),
        mesh_echo_sensor_relay: meshes.add(Cuboid::new(0.48, 0.75, 0.48)),

        // --- Background Architectural Silhouettes ---
        mesh_bg_tunnel_arch: meshes.add(Cuboid::new(48.0, 3.5, 5.0)),
        mesh_bg_arch_column: meshes.add(Cuboid::new(4.5, 28.0, 4.5)),
        mesh_bg_vent_shaft_body: meshes.add(Cylinder::new(5.0, 32.0)),
        mesh_bg_vent_shaft_cap: meshes.add(Cylinder::new(6.2, 3.0)),
        mesh_bg_pump_housing: meshes.add(Cuboid::new(8.0, 14.0, 8.0)),
        mesh_bg_pipe_trunk: meshes.add(Cylinder::new(1.8, 40.0)),
        mesh_bg_maint_bridge_deck: meshes.add(Cuboid::new(50.0, 1.2, 4.0)),
        mesh_bg_maint_bridge_truss: meshes.add(Cuboid::new(50.0, 3.5, 0.4)),
        mesh_bg_reservoir_wall_colossal: meshes.add(Cuboid::new(2.5, 34.0, 40.0)),
        mesh_bg_aqueduct_pillar: meshes.add(Cuboid::new(5.5, 22.0, 5.5)),
        mesh_bg_distant_train: meshes.add(Cuboid::new(4.2, 3.6, 26.0)),
        mesh_bg_blast_door_frame: meshes.add(Cuboid::new(26.0, 2.4, 3.0)),
        mesh_bg_blast_door_panel: meshes.add(Cuboid::new(2.4, 10.0, 2.2)),

        // --- WaterDrip Geometry ---
        mesh_water_droplet: meshes.add(Cuboid::new(0.045, 0.36, 0.045)),

        // --- Materials ---
        mat_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.15, 0.17),
            perceptual_roughness: 0.88,
            ..default()
        }),
        mat_wet_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.12, 0.14),
            metallic: 0.25,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.22, 0.25),
            metallic: 0.88,
            perceptual_roughness: 0.30,
            ..default()
        }),
        mat_rusted_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.18, 0.12),
            metallic: 0.55,
            perceptual_roughness: 0.68,
            ..default()
        }),
        mat_corroded_copper: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.22, 0.18),
            metallic: 0.35,
            perceptual_roughness: 0.65,
            ..default()
        }),
        mat_water: materials.add(StandardMaterial {
            base_color: Color::srgb(0.015, 0.16, 0.20),
            emissive: LinearRgba::new(0.004, 0.04, 0.05, 1.0),
            metallic: 0.35,
            perceptual_roughness: 0.06,
            ..default()
        }),
        mat_water_puddle: materials.add(StandardMaterial {
            base_color: Color::srgb(0.018, 0.14, 0.18),
            metallic: 0.45,
            perceptual_roughness: 0.03,
            ..default()
        }),
        mat_water_stain: materials.add(StandardMaterial {
            base_color: Color::srgb(0.030, 0.080, 0.095),
            metallic: 0.10,
            perceptual_roughness: 0.85,
            ..default()
        }),
        mat_cable: materials.add(StandardMaterial {
            base_color: Color::srgb(0.045, 0.052, 0.060),
            metallic: 0.40,
            perceptual_roughness: 0.65,
            ..default()
        }),
        mat_teal_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.45, 0.52),
            emissive: LinearRgba::new(0.06, 1.10, 1.35, 1.0),
            metallic: 0.25,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_warning_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.45, 0.06),
            emissive: LinearRgba::new(1.80, 0.55, 0.02, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_emergency_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.12, 0.06),
            emissive: LinearRgba::new(1.60, 0.08, 0.02, 1.0),
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_hazard_stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.65, 0.12),
            metallic: 0.20,
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_train_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.13, 0.15),
            metallic: 0.75,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_train_window: materials.add(StandardMaterial {
            base_color: Color::srgb(0.03, 0.12, 0.15),
            emissive: LinearRgba::new(0.02, 0.28, 0.35, 1.0),
            metallic: 0.45,
            perceptual_roughness: 0.15,
            ..default()
        }),
        mat_submerged_wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.11, 0.08),
            perceptual_roughness: 0.82,
            ..default()
        }),

        // --- Dedicated Submerged Track Materials ---
        mat_wet_ballast: materials.add(StandardMaterial {
            base_color: Color::srgb(0.065, 0.080, 0.090),
            metallic: 0.22,
            perceptual_roughness: 0.32,
            ..default()
        }),
        mat_corroded_rail: materials.add(StandardMaterial {
            base_color: Color::srgb(0.38, 0.24, 0.18),
            metallic: 0.88,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_submerged_sleeper: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.16, 0.17),
            metallic: 0.15,
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_canal_wall: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.085, 0.095),
            metallic: 0.30,
            perceptual_roughness: 0.50,
            ..default()
        }),
        mat_faded_lane_marker: materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.64, 0.14),
            metallic: 0.20,
            perceptual_roughness: 0.48,
            ..default()
        }),
        mat_transit_slab: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.23, 0.25),
            metallic: 0.18,
            perceptual_roughness: 0.34,
            ..default()
        }),
        mat_transit_curb: materials.add(StandardMaterial {
            base_color: Color::srgb(0.38, 0.26, 0.20),
            metallic: 0.90,
            perceptual_roughness: 0.18,
            ..default()
        }),
        mat_transit_tread: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.14, 0.16),
            metallic: 0.35,
            perceptual_roughness: 0.48,
            ..default()
        }),

        // --- Background Silhouette Materials ---
        mat_bg_dark_structure: materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.12, 0.14),
            metallic: 0.35,
            perceptual_roughness: 0.65,
            ..default()
        }),
        mat_bg_metal_structure: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.18, 0.22),
            metallic: 0.65,
            perceptual_roughness: 0.45,
            ..default()
        }),

        // --- WaterDrip Material ---
        mat_water_droplet: materials.add(StandardMaterial {
            base_color: Color::srgba(0.25, 0.68, 0.78, 0.90),
            emissive: LinearRgba::new(0.03, 0.15, 0.18, 1.0),
            metallic: 0.15,
            perceptual_roughness: 0.08,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

pub fn spawn_flooded_metro_segment(
    commands: &mut Commands,
    _env: &EnvironmentAssets,
    flooded: &FloodedMetroAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = flooded_metro_chapter_for_distance(distance);
    let variant = flooded_metro_variant(distance);
    let landmark = flooded_metro_landmark_for_distance(distance);

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // PILLAR B: Dedicated Submerged Track Identity (High Readability)
            spawn_flooded_track(seg, flooded, length);

            // PILLAR D: Lightweight WaterDrip System
            spawn_chapter_water_drip_emitters(seg, flooded, chapter, variant);

            // PILLAR A: Layered Background Depth & Chapter Specific Architecture
            match chapter {
                FloodedMetroChapter::FloodgateDescent => {
                    spawn_floodgate_descent_segment(seg, flooded, variant, landmark);
                }
                FloodedMetroChapter::SubmergedStation => {
                    spawn_submerged_station_segment(seg, flooded, variant, landmark);
                }
                FloodedMetroChapter::ReservoirCrossing => {
                    spawn_reservoir_crossing_segment(seg, flooded, variant, landmark);
                }
                FloodedMetroChapter::SunkenTransit => {
                    spawn_sunken_transit_segment(seg, flooded, variant, landmark);
                }
                FloodedMetroChapter::DeepLineApproach => {
                    spawn_deep_line_approach_segment(seg, flooded, variant, landmark, distance);
                }
            }
        })
        .id()
}

// -------------------------------------------------------------
// PILLAR B — DEDICATED SUBMERGED TRANSIT LANES & WATER CHANNELS
// -------------------------------------------------------------

fn spawn_flooded_track(seg: &mut ChildBuilder, a: &FloodedMetroAssets, _length: f32) {
    // 1. Submerged route foundation base underneath the flooded bed
    seg.spawn(PbrBundle {
        mesh: a.mesh_submerged_bed_foundation.clone(),
        material: a.mat_wet_ballast.clone(),
        transform: Transform::from_xyz(0.0, -0.32, 0.0),
        ..default()
    });

    // 2. Three Dedicated Raised Submerged Transit Lanes (-1, 0, +1)
    for lane_idx in -1..=1 {
        let x = Lane::from_index(lane_idx).x_pos(); // -2.2, 0.0, +2.2

        // A. Raised concrete/composite transit causeway slab
        seg.spawn(PbrBundle {
            mesh: a.mesh_transit_lane_slab.clone(),
            material: a.mat_transit_slab.clone(),
            transform: Transform::from_xyz(x, -0.07, 0.0),
            ..default()
        });

        // B. Central directional traction runner tread
        seg.spawn(PbrBundle {
            mesh: a.mesh_transit_lane_tread.clone(),
            material: a.mat_transit_tread.clone(),
            transform: Transform::from_xyz(x, 0.065, 0.0),
            ..default()
        });

        // C. Metallic corroded guide curbs flanking each individual lane
        for side in [-1.0, 1.0] {
            let curb_x = x + side * 0.90;
            seg.spawn(PbrBundle {
                mesh: a.mesh_transit_curb_border.clone(),
                material: a.mat_transit_curb.clone(),
                transform: Transform::from_xyz(curb_x, 0.10, 0.0),
                ..default()
            });
        }

        // D. Directional industrial hazard markers down lane center
        for i in -4..=4 {
            if (i + lane_idx).abs() % 4 != 0 {
                seg.spawn(PbrBundle {
                    mesh: a.mesh_transit_hazard_marker.clone(),
                    material: a.mat_faded_lane_marker.clone(),
                    transform: Transform::from_xyz(x, 0.075, i as f32 * 4.5),
                    ..default()
                });
            }
        }

        // E. Shallow wet puddle accumulation on the transit slab
        for z in [-11.0, 7.0] {
            seg.spawn(PbrBundle {
                mesh: a.mesh_transit_puddle_sheet.clone(),
                material: a.mat_water_puddle.clone(),
                transform: Transform::from_xyz(x, 0.072, z),
                ..default()
            });
        }
    }

    // 3. Flooded inter-lane water channels (in the 0.40m gaps between slabs)
    for gap_x in [-1.1, 1.1] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_inter_lane_water.clone(),
            material: a.mat_water.clone(),
            transform: Transform::from_xyz(gap_x, -0.04, 0.0),
            ..default()
        });

        // Periodic overflow drainage grates spanning the gap
        for z in [-12.0, 0.0, 12.0] {
            seg.spawn(PbrBundle {
                mesh: a.mesh_channel_drain_grate.clone(),
                material: a.mat_rusted_steel.clone(),
                transform: Transform::from_xyz(gap_x, 0.04, z),
                ..default()
            });
        }
    }

    // 4. Wide flooded flank reservoirs extending to the tunnel walls
    for flank_side in [-1.0, 1.0] {
        let flank_x = flank_side * 4.6;
        seg.spawn(PbrBundle {
            mesh: a.mesh_flank_water_reservoir.clone(),
            material: a.mat_water.clone(),
            transform: Transform::from_xyz(flank_x, -0.04, 0.0),
            ..default()
        });
    }

    // 5. Heavy transverse structural tie beams connecting the causeways beneath the water
    for z in [-18.0, -10.0, -2.0, 6.0, 14.0] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_submerged_transverse_beam.clone(),
            material: a.mat_steel.clone(),
            transform: Transform::from_xyz(0.0, -0.09, z),
            ..default()
        });
    }
}

// -------------------------------------------------------------
// PILLAR A — REUSABLE BACKGROUND ARCHITECTURE SILHOUETTES
// -------------------------------------------------------------

fn spawn_background_tunnel_arch(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
    scale: Vec3,
) {
    // Monumental transverse arch lintel spanning the tunnel ceiling
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_tunnel_arch.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 8.4 * scale.y, z).with_scale(scale),
        ..default()
    });
    // Massive architectural buttress columns embedded in the tunnel sidewalls
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_bg_arch_column.clone(),
            material: a.mat_bg_dark_structure.clone(),
            transform: Transform::from_xyz(x + side * 9.2 * scale.x, y + 4.2 * scale.y, z)
                .with_scale(scale),
            ..default()
        });
    }
}

fn spawn_background_vent_shaft(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // Colossal vertical shaft ascending from the ceiling into subterranean darkness
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_vent_shaft_body.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 24.4, z),
        ..default()
    });
    // Heavy reinforced rim collar at the ceiling opening
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_vent_shaft_cap.clone(),
        material: a.mat_bg_metal_structure.clone(),
        transform: Transform::from_xyz(x, y + 8.4, z),
        ..default()
    });
    // High-visibility amber warning beacon illuminating the shaft opening
    seg.spawn(PbrBundle {
        mesh: a.mesh_emergency_light.clone(),
        material: a.mat_warning_amber.clone(),
        transform: Transform::from_xyz(x, y + 7.2, z + 4.8).with_scale(Vec3::splat(2.0)),
        ..default()
    });
}

fn spawn_background_pump_tower(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // 14m high industrial pump machinery housing placed along side water margin
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_pump_housing.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 5.5, z),
        ..default()
    });
    // Vertical high-pressure suction trunk ascending from water to ceiling
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_pipe_trunk.clone(),
        material: a.mat_rusted_steel.clone(),
        transform: Transform::from_xyz(x - x.signum() * 2.6, y + 8.0, z),
        ..default()
    });
    // High-mounted amber machinery beacon
    seg.spawn(PbrBundle {
        mesh: a.mesh_emergency_light.clone(),
        material: a.mat_warning_amber.clone(),
        transform: Transform::from_xyz(x, y + 12.6, z).with_scale(Vec3::splat(2.2)),
        ..default()
    });
}

fn spawn_background_maintenance_bridge(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // Overhead bridge deck crossing directly above the transit corridor
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_maint_bridge_deck.clone(),
        material: a.mat_bg_metal_structure.clone(),
        transform: Transform::from_xyz(x, y + 7.6, z),
        ..default()
    });
    // Industrial truss railing
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_maint_bridge_truss.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 9.2, z),
        ..default()
    });
}

fn spawn_background_pipe_bank(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // High-level pipe trunks spanning across the ceiling transversely
    for y_off in [7.5, 8.8] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_bg_pipe_trunk.clone(),
            material: a.mat_rusted_steel.clone(),
            transform: Transform::from_xyz(x, y + y_off, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
}

fn spawn_distant_train_silhouette(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
) {
    let rot = Quat::from_rotation_y(yaw);
    // Industrial freight consist silhouette: heavy locomotive + trailing cargo wagon
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_distant_train.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 1.2, z).with_rotation(rot),
        ..default()
    });
    // Distinct locomotive cab hump silhouette
    let cab_offset = rot * Vec3::new(0.0, 1.8, -8.0);
    seg.spawn(PbrBundle {
        mesh: a.mesh_freight_loco_cab.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x + cab_offset.x, y + 1.2 + cab_offset.y, z + cab_offset.z)
            .with_rotation(rot),
        ..default()
    });
    // Distinct tanker saddle / equipment silhouette
    let tank_offset = rot * Vec3::new(0.0, 0.4, 7.5);
    seg.spawn(PbrBundle {
        mesh: a.mesh_freight_tank_saddle.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(
            x + tank_offset.x,
            y + 1.2 + tank_offset.y,
            z + tank_offset.z,
        )
        .with_rotation(rot),
        ..default()
    });
}

fn spawn_background_reservoir_wall(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // Colossal reservoir retention wall
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_reservoir_wall_colossal.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 14.0, z),
        ..default()
    });
    // Heavy aqueduct buttress pillars flanking the dam wall
    for z_off in [-14.0, 14.0] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_bg_aqueduct_pillar.clone(),
            material: a.mat_bg_dark_structure.clone(),
            transform: Transform::from_xyz(x - (x.signum() * 2.5), y + 10.0, z + z_off),
            ..default()
        });
    }
}

fn spawn_background_blast_door_portal(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    // 26m wide reinforced blast portal overhead header lintel (elevated at ceiling level)
    seg.spawn(PbrBundle {
        mesh: a.mesh_bg_blast_door_frame.clone(),
        material: a.mat_bg_dark_structure.clone(),
        transform: Transform::from_xyz(x, y + 8.4, z),
        ..default()
    });
    // Massive vertical jamb stanchions embedded into the tunnel sidewalls (center corridor remains 100% open)
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: a.mesh_bg_blast_door_panel.clone(),
            material: a.mat_bg_metal_structure.clone(),
            transform: Transform::from_xyz(x + side * 8.8, y + 4.5, z),
            ..default()
        });
    }
}

// -------------------------------------------------------------
// REUSABLE ARCHITECTURAL & ENVIRONMENT DRESSING MODULES
// -------------------------------------------------------------

fn spawn_tunnel_shell(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    wall_x: f32,
    wall_height: f32,
    use_reservoir_wall: bool,
) {
    let mesh = if use_reservoir_wall {
        &a.mesh_wall_reservoir
    } else {
        &a.mesh_wall_main
    };

    // Main structural sidewall slabs and panel seams on both sides
    for side in [-1.0, 1.0] {
        let x = side * wall_x;
        spawn_mesh(
            seg,
            mesh,
            &a.mat_wet_concrete,
            Transform::from_xyz(x, wall_height * 0.5, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_wall_panel_seam,
            &a.mat_steel,
            Transform::from_xyz(x - side * 0.22, wall_height * 0.5, 0.0),
        );
    }

    // PILLAR E & ASYMMETRY: Lived-in industrial wear & asymmetric wall dressing
    // ------------------------------------------------------------------------
    // LEFT WALL (side = -1.0): Conduit trays, historical waterline marks, and water seepage
    let left_x = -wall_x;
    spawn_mesh(
        seg,
        &a.mesh_waterline_mark,
        &a.mat_water_stain,
        Transform::from_xyz(left_x + 0.24, 1.85, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_conduit_tray,
        &a.mat_steel,
        Transform::from_xyz(left_x + 0.26, 3.4, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_cable_bundle,
        &a.mat_cable,
        Transform::from_xyz(left_x + 0.30, 2.1, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
    spawn_mesh(
        seg,
        &a.mesh_water_stain_vert,
        &a.mat_water_stain,
        Transform::from_xyz(left_x + 0.23, 2.4, -8.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_terminal_box,
        &a.mat_rusted_steel,
        Transform::from_xyz(left_x + 0.32, 2.8, 10.0),
    );
    for z in [-16.0, 16.0] {
        spawn_mesh(
            seg,
            &a.mesh_support_column,
            &a.mat_concrete,
            Transform::from_xyz(left_x + 0.45, 4.3, z),
        );
    }

    // RIGHT WALL (side = 1.0): Maintenance girders, higher flood mark, drainage outlet & hazard sign
    let right_x = wall_x;
    spawn_mesh(
        seg,
        &a.mesh_waterline_mark,
        &a.mat_water_stain,
        Transform::from_xyz(right_x - 0.24, 2.05, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_maintenance_girder,
        &a.mat_steel,
        Transform::from_xyz(right_x - 0.28, 4.2, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_drainage_outlet,
        &a.mat_rusted_steel,
        Transform::from_xyz(right_x - 0.32, 2.2, 6.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
    );
    spawn_mesh(
        seg,
        &a.mesh_water_stain_vert,
        &a.mat_water_stain,
        Transform::from_xyz(right_x - 0.23, 1.4, 6.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_hazard_sign,
        &a.mat_hazard_stripe,
        Transform::from_xyz(right_x - 0.25, 3.2, -6.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_support_column,
        &a.mat_concrete,
        Transform::from_xyz(right_x - 0.45, 4.3, 0.0),
    );
}

fn spawn_concrete_arch_ribs(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    width: f32,
    z_positions: &[f32],
) {
    for &z in z_positions {
        for side in [-1.0, 1.0] {
            spawn_mesh(
                seg,
                &a.mesh_concrete_rib,
                &a.mat_concrete,
                Transform::from_xyz(side * width, 4.3, z),
            );
            spawn_mesh(
                seg,
                &a.mesh_expansion_bracket,
                &a.mat_rusted_steel,
                Transform::from_xyz(side * (width - 0.4), 1.2, z),
            );
            spawn_mesh(
                seg,
                &a.mesh_steel_crossbrace,
                &a.mat_rusted_steel,
                Transform::from_xyz(side * (width - 0.15), 5.4, z)
                    .with_rotation(Quat::from_rotation_z(side * 0.42)),
            );
        }
        spawn_mesh(
            seg,
            &a.mesh_ceiling_beam,
            &a.mat_concrete,
            Transform::from_xyz(0.0, 7.3, z),
        );
        spawn_mesh(
            seg,
            &a.mesh_ceiling_arch,
            &a.mat_concrete,
            Transform::from_xyz(0.0, 7.6, z),
        );
    }
}

fn spawn_ceiling_infrastructure(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    is_cavern: bool,
) {
    let y_base = if is_cavern { 9.5 } else { 6.4 };

    spawn_mesh(
        seg,
        &a.mesh_pipe_medium,
        &a.mat_steel,
        Transform::from_xyz(-5.6, y_base, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
    spawn_mesh(
        seg,
        &a.mesh_cable_tray,
        &a.mat_steel,
        Transform::from_xyz(5.4, y_base + 0.1, 0.0),
    );

    let cable_offset: f32 = match variant {
        0 => -3.2,
        1 => 3.2,
        _ => 0.0,
    };
    if cable_offset.abs() > 0.1 {
        spawn_mesh(
            seg,
            &a.mesh_cable_bundle,
            &a.mat_cable,
            Transform::from_xyz(cable_offset, y_base - 0.7, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
    }

    for z in [-12.0, 12.0] {
        spawn_mesh(
            seg,
            &a.mesh_caged_fixture,
            &a.mat_teal_glow,
            Transform::from_xyz(0.0, y_base - 0.2, z),
        );
    }
}

fn spawn_pipe_runs(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    spawn_mesh(
        seg,
        &a.mesh_pipe_drain_huge,
        &a.mat_rusted_steel,
        Transform::from_xyz(-7.4, 4.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
    for z in [-15.0, 0.0, 15.0] {
        spawn_mesh(
            seg,
            &a.mesh_pipe_flange,
            &a.mat_steel,
            Transform::from_xyz(-7.4, 4.4, z)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
        spawn_mesh(
            seg,
            &a.mesh_pipe_bracket,
            &a.mat_steel,
            Transform::from_xyz(-8.1, 4.4, z),
        );
    }

    if variant != 1 {
        spawn_mesh(
            seg,
            &a.mesh_valve_large,
            &a.mat_warning_amber,
            Transform::from_xyz(-7.4, 5.2, -6.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
        spawn_mesh(
            seg,
            &a.mesh_pressure_gauge,
            &a.mat_steel,
            Transform::from_xyz(-7.4, 5.2, 6.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
    }

    for (y_offset, z_offset) in [(3.2, 0.0), (3.7, 0.0)] {
        spawn_mesh(
            seg,
            &a.mesh_pipe_small,
            &a.mat_steel,
            Transform::from_xyz(7.5, y_offset, z_offset)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
    }
}

fn spawn_water_elements(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    is_wide_reservoir: bool,
) {
    if is_wide_reservoir {
        for s in [-1.0, 1.0] {
            spawn_mesh(
                seg,
                &a.mesh_water_reservoir_wide,
                &a.mat_water,
                Transform::from_xyz(s * 14.5, -0.06, 0.0),
            );
        }
        // Floating industrial drums and timber in wide reservoir expanse
        spawn_mesh(
            seg,
            &a.mesh_floating_drum,
            &a.mat_rusted_steel,
            Transform::from_xyz(-13.5, 0.02, 10.0).with_rotation(Quat::from_rotation_x(0.35)),
        );
        spawn_mesh(
            seg,
            &a.mesh_floating_debris,
            &a.mat_submerged_wood,
            Transform::from_xyz(14.0, 0.02, -8.0),
        );
    } else {
        spawn_mesh(
            seg,
            &a.mesh_water_trackside,
            &a.mat_water,
            Transform::from_xyz(side * 8.5, -0.02, 0.0),
        );
        // Subtle floating debris along the flooded side margin
        spawn_mesh(
            seg,
            &a.mesh_floating_drum,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 7.6, 0.02, 8.0)
                .with_rotation(Quat::from_rotation_z(side * 0.25)),
        );
        spawn_mesh(
            seg,
            &a.mesh_floating_debris,
            &a.mat_submerged_wood,
            Transform::from_xyz(side * 8.0, 0.03, -10.0),
        );
    }

    for s in [-1.0, 1.0] {
        spawn_mesh(
            seg,
            &a.mesh_water_puddle,
            &a.mat_water_puddle,
            Transform::from_xyz(s * 3.75, 0.02, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_drain_channel,
            &a.mat_steel,
            Transform::from_xyz(s * 3.85, 0.14, 0.0),
        );
    }
}

fn spawn_culvert_drain(seg: &mut ChildBuilder, a: &FloodedMetroAssets, side: f32, z: f32) {
    let x = side * 8.6;
    spawn_mesh(
        seg,
        &a.mesh_drainage_outlet,
        &a.mat_rusted_steel,
        Transform::from_xyz(x, 2.4, z)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
    );
    spawn_mesh(
        seg,
        &a.mesh_water_cascade,
        &a.mat_water,
        Transform::from_xyz(side * 8.1, 1.2, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_water_stain_vert,
        &a.mat_water_stain,
        Transform::from_xyz(side * 8.7, 1.4, z),
    );
}

fn spawn_ladder(seg: &mut ChildBuilder, a: &FloodedMetroAssets, side: f32, z: f32) {
    let x = side * 8.55;
    for z_off in [-0.40, 0.40] {
        spawn_mesh(
            seg,
            &a.mesh_ladder_rail,
            &a.mat_steel,
            Transform::from_xyz(x, 2.4, z + z_off),
        );
    }
    for y in [1.0, 1.8, 2.6, 3.4, 4.2] {
        spawn_mesh(
            seg,
            &a.mesh_ladder_rung,
            &a.mat_rusted_steel,
            Transform::from_xyz(x, y, z)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        );
    }
}

fn spawn_electrical_box(seg: &mut ChildBuilder, a: &FloodedMetroAssets, side: f32, z: f32) {
    let x = side * 8.2;
    spawn_mesh(
        seg,
        &a.mesh_electrical_cabinet,
        &a.mat_steel,
        Transform::from_xyz(x, 1.6, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_terminal_box,
        &a.mat_rusted_steel,
        Transform::from_xyz(side * 8.35, 2.8, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_emergency_light,
        &a.mat_warning_amber,
        Transform::from_xyz(side * 7.95, 2.45, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_warning_panel,
        &a.mat_hazard_stripe,
        Transform::from_xyz(side * 8.35, 3.4, z),
    );
}

fn spawn_pump_station(seg: &mut ChildBuilder, a: &FloodedMetroAssets, side: f32, z: f32) {
    let x = side * 6.8;
    spawn_mesh(
        seg,
        &a.mesh_pump_turbine,
        &a.mat_rusted_steel,
        Transform::from_xyz(x, 1.2, z).with_scale(Vec3::new(0.65, 0.65, 0.65)),
    );
    spawn_mesh(
        seg,
        &a.mesh_pipe_vertical,
        &a.mat_steel,
        Transform::from_xyz(side * 7.6, 3.2, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_valve_large,
        &a.mat_steel,
        Transform::from_xyz(side * 7.6, 4.4, z)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
}

// -------------------------------------------------------------
// CHAPTER 1 — FLOODGATE DESCENT (6000m–6300m)
// -------------------------------------------------------------

fn spawn_floodgate_descent_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
) {
    spawn_tunnel_shell(seg, a, 9.0, 8.6, false);
    spawn_concrete_arch_ribs(seg, a, 8.6, &[-14.0, 14.0]);
    spawn_ceiling_infrastructure(seg, a, variant, false);
    spawn_pipe_runs(seg, a, variant);
    spawn_water_elements(seg, a, 1.0, false);

    // PILLAR A: Background Architectural Silhouettes (Scale & Subterranean Depth)
    match variant {
        0 => {
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, -16.0, Vec3::new(1.0, 1.0, 1.0));
            spawn_background_pipe_bank(seg, a, 0.0, 0.0, 8.0);
            spawn_culvert_drain(seg, a, -1.0, -8.0);
            spawn_pump_station(seg, a, 1.0, 6.0);
            spawn_ladder(seg, a, 1.0, -12.0);
            // Environmental Signage: Flood Level Indicator gauge
            spawn_mesh(
                seg,
                &a.mesh_flood_level_gauge,
                &a.mat_hazard_stripe,
                Transform::from_xyz(-8.65, 2.6, 2.0),
            );
            // Signage: Pump Room / Hydraulic Control Warning
            spawn_mesh(
                seg,
                &a.mesh_warning_panel,
                &a.mat_hazard_stripe,
                Transform::from_xyz(8.65, 3.4, -4.0),
            );
        }
        1 => {
            spawn_background_pump_tower(seg, a, 7.8, 0.0, 0.0);
            spawn_background_maintenance_bridge(seg, a, 0.0, 0.0, 12.0);
            spawn_electrical_box(seg, a, -1.0, 4.0);
            spawn_culvert_drain(seg, a, 1.0, 10.0);
            spawn_mesh(
                seg,
                &a.mesh_watertight_hatch,
                &a.mat_steel,
                Transform::from_xyz(-8.7, 1.6, -4.0),
            );
            // Wall hydraulic actuation infrastructure
            spawn_mesh(
                seg,
                &a.mesh_hydraulic_cylinder,
                &a.mat_steel,
                Transform::from_xyz(8.4, 4.2, -6.0),
            );
            spawn_mesh(
                seg,
                &a.mesh_hydraulic_rod,
                &a.mat_steel,
                Transform::from_xyz(8.4, 6.2, -6.0),
            );
        }
        _ => {
            spawn_background_vent_shaft(seg, a, 0.0, 0.0, -8.0);
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, 16.0, Vec3::new(1.0, 1.0, 1.0));
            spawn_pump_station(seg, a, -1.0, -4.0);
            spawn_electrical_box(seg, a, 1.0, 8.0);
            spawn_culvert_drain(seg, a, -1.0, 12.0);
            spawn_mesh(
                seg,
                &a.mesh_flood_level_gauge,
                &a.mat_hazard_stripe,
                Transform::from_xyz(8.65, 2.6, -2.0),
            );
        }
    }

    if landmark == FloodedMetroLandmark::FloodgateDescentSluice {
        spawn_landmark_floodgate(seg, a);
    }
}

fn spawn_landmark_floodgate(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    spawn_mesh(
        seg,
        &a.mesh_gate_portal_arch,
        &a.mat_concrete,
        Transform::from_xyz(0.0, 7.8, 0.0),
    );
    for side in [-1.0, 1.0] {
        let x = side * 7.4;
        spawn_mesh(
            seg,
            &a.mesh_gate_pillar,
            &a.mat_concrete,
            Transform::from_xyz(x, 4.8, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_hydraulic_cylinder,
            &a.mat_rusted_steel,
            Transform::from_xyz(x - side * 0.9, 5.2, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_hydraulic_rod,
            &a.mat_steel,
            Transform::from_xyz(x - side * 0.9, 3.4, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_emergency_light,
            &a.mat_warning_amber,
            Transform::from_xyz(side * 6.2, 7.2, 0.2),
        );
    }
    for side in [-1.0, 1.0] {
        spawn_mesh(
            seg,
            &a.mesh_gate_leaf_heavy,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 3.8, 5.4, 0.0),
        );
    }
    spawn_mesh(
        seg,
        &a.mesh_flood_level_gauge,
        &a.mat_hazard_stripe,
        Transform::from_xyz(-6.8, 2.4, 0.4),
    );
}

// -------------------------------------------------------------
// CHAPTER 2 — SUBMERGED STATION (6300m–6750m)
// -------------------------------------------------------------

fn spawn_submerged_station_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
) {
    let platform_side = if variant == 1 { -1.0 } else { 1.0 };
    spawn_tunnel_shell(seg, a, 9.0, 8.6, false);
    spawn_concrete_arch_ribs(seg, a, 8.6, &[-14.0, 0.0, 14.0]);
    spawn_ceiling_infrastructure(seg, a, variant, false);
    spawn_water_elements(seg, a, -platform_side, false);

    // PILLAR A: Background Silhouettes (Vast Station Hall & Distant Platforms)
    match variant {
        0 => {
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, -12.0, Vec3::new(1.05, 1.0, 1.0));
            spawn_background_maintenance_bridge(seg, a, 0.0, 0.0, 10.0);
        }
        1 => {
            spawn_distant_train_silhouette(seg, a, -platform_side * 6.6, 0.0, 2.0, 0.0);
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, 14.0, Vec3::new(1.05, 1.0, 1.0));
        }
        _ => {
            spawn_background_vent_shaft(seg, a, 0.0, 0.0, 0.0);
            spawn_background_maintenance_bridge(seg, a, 0.0, 0.0, -12.0);
        }
    }

    let plat_x = platform_side * 6.5;
    spawn_mesh(
        seg,
        &a.mesh_station_platform,
        &a.mat_wet_concrete,
        Transform::from_xyz(plat_x, 0.42, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_platform_tactile_edge,
        &a.mat_hazard_stripe,
        Transform::from_xyz(platform_side * 3.9, 0.74, 0.0),
    );

    for z in [-14.0, 0.0, 14.0] {
        spawn_mesh(
            seg,
            &a.mesh_station_pillar_tile,
            &a.mat_concrete,
            Transform::from_xyz(platform_side * 5.2, 3.2, z),
        );
    }

    spawn_mesh(
        seg,
        &a.mesh_station_bench,
        &a.mat_steel,
        Transform::from_xyz(platform_side * 6.8, 0.95, -6.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_station_sign_board,
        &a.mat_teal_glow,
        Transform::from_xyz(platform_side * 5.2, 5.2, 0.0),
    );

    match variant {
        0 => {
            spawn_electrical_box(seg, a, platform_side, 8.0);
            spawn_ladder(seg, a, -platform_side, -8.0);
            // Station track & line identifier placard
            spawn_mesh(
                seg,
                &a.mesh_hazard_sign,
                &a.mat_hazard_stripe,
                Transform::from_xyz(platform_side * 8.65, 3.2, -4.0),
            );
        }
        1 => {
            spawn_mesh(
                seg,
                &a.mesh_ticket_turnstile,
                &a.mat_steel,
                Transform::from_xyz(plat_x, 1.25, 6.0),
            );
            spawn_pump_station(seg, a, -platform_side, 4.0);
        }
        _ => {
            spawn_mesh(
                seg,
                &a.mesh_floating_debris,
                &a.mat_submerged_wood,
                Transform::from_xyz(-platform_side * 7.5, 0.05, -5.0),
            );
            spawn_mesh(
                seg,
                &a.mesh_floating_drum,
                &a.mat_rusted_steel,
                Transform::from_xyz(-platform_side * 6.8, 0.05, 7.0)
                    .with_rotation(Quat::from_rotation_z(0.35)),
            );
        }
    }

    if landmark == FloodedMetroLandmark::SubmergedStationComplex {
        spawn_landmark_submerged_station(seg, a);
    }
}

fn spawn_landmark_submerged_station(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    spawn_mesh(
        seg,
        &a.mesh_submerged_stairs,
        &a.mat_concrete,
        Transform::from_xyz(-6.8, 0.25, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_station_sign_board,
        &a.mat_teal_glow,
        Transform::from_xyz(0.0, 5.8, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_ticket_turnstile,
        &a.mat_rusted_steel,
        Transform::from_xyz(6.8, 1.25, -2.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_station_bench,
        &a.mat_steel,
        Transform::from_xyz(7.0, 0.95, 6.0),
    );
}

// -------------------------------------------------------------
// CHAPTER 3 — RESERVOIR CROSSING (6750m–7200m)
// -------------------------------------------------------------

fn spawn_reservoir_crossing_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
) {
    spawn_tunnel_shell(seg, a, 21.0, 16.0, true);
    spawn_ceiling_infrastructure(seg, a, variant, true);
    spawn_water_elements(seg, a, 1.0, true);

    // PILLAR A: Background Silhouettes (Colossal Scale & Distant Aqueduct Arches)
    match variant {
        0 => {
            spawn_background_reservoir_wall(seg, a, -19.5, 0.0, 0.0);
            spawn_background_reservoir_wall(seg, a, 19.5, 0.0, 0.0);
        }
        1 => {
            spawn_background_tunnel_arch(seg, a, 0.0, 3.5, 0.0, Vec3::new(2.1, 1.4, 1.2));
            spawn_background_maintenance_bridge(seg, a, 0.0, 4.5, 12.0);
        }
        _ => {
            spawn_background_pump_tower(seg, a, -14.0, 0.0, -8.0);
            spawn_background_pump_tower(seg, a, 14.0, 0.0, 8.0);
            spawn_background_maintenance_bridge(seg, a, 0.0, 4.5, -12.0);
        }
    }

    for side in [-1.0, 1.0] {
        for z in [-14.0, 14.0] {
            spawn_mesh(
                seg,
                &a.mesh_reservoir_pier_giant,
                &a.mat_concrete,
                Transform::from_xyz(side * 6.8, -2.0, z),
            );
        }
    }

    let catwalk_side = if variant == 0 { -1.0 } else { 1.0 };
    spawn_mesh(
        seg,
        &a.mesh_catwalk_deck,
        &a.mat_steel,
        Transform::from_xyz(catwalk_side * 14.0, 6.4, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_catwalk_railing,
        &a.mat_rusted_steel,
        Transform::from_xyz(catwalk_side * 12.5, 6.8, 0.0),
    );
    // Industrial signage: SECTOR C // AQUEDUCT RESERVOIR warning panel
    spawn_mesh(
        seg,
        &a.mesh_warning_panel,
        &a.mat_hazard_stripe,
        Transform::from_xyz(catwalk_side * 13.8, 7.8, -4.0),
    );

    match variant {
        0 => {
            spawn_mesh(
                seg,
                &a.mesh_intake_tower,
                &a.mat_concrete,
                Transform::from_xyz(-14.0, 3.8, 0.0),
            );
            spawn_mesh(
                seg,
                &a.mesh_intake_grate,
                &a.mat_steel,
                Transform::from_xyz(-11.5, 3.8, 0.0),
            );
        }
        1 => {
            spawn_mesh(
                seg,
                &a.mesh_pump_turbine,
                &a.mat_rusted_steel,
                Transform::from_xyz(13.5, 1.4, 0.0),
            );
            spawn_mesh(
                seg,
                &a.mesh_sluice_pipe_huge,
                &a.mat_steel,
                Transform::from_xyz(14.0, 3.4, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            );
        }
        _ => {
            spawn_mesh(
                seg,
                &a.mesh_floating_debris,
                &a.mat_submerged_wood,
                Transform::from_xyz(-12.0, 0.05, 6.0),
            );
            spawn_mesh(
                seg,
                &a.mesh_floating_drum,
                &a.mat_corroded_copper,
                Transform::from_xyz(12.5, 0.05, -8.0).with_rotation(Quat::from_rotation_x(0.4)),
            );
        }
    }

    if landmark == FloodedMetroLandmark::ReservoirCavernCenterpiece {
        spawn_landmark_reservoir_aqueduct(seg, a);
    }
}

fn spawn_landmark_reservoir_aqueduct(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    spawn_mesh(
        seg,
        &a.mesh_reservoir_truss_span,
        &a.mat_steel,
        Transform::from_xyz(0.0, 10.8, 0.0),
    );
    for side in [-1.0, 1.0] {
        let x = side * 15.0;
        spawn_mesh(
            seg,
            &a.mesh_intake_tower,
            &a.mat_concrete,
            Transform::from_xyz(x, 4.4, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_intake_grate,
            &a.mat_teal_glow,
            Transform::from_xyz(side * 12.4, 4.4, 0.0),
        );
    }
}

// -------------------------------------------------------------
// CHAPTER 4 — SUNKEN TRANSIT (7200m–7650m)
// -------------------------------------------------------------

fn spawn_sunken_transit_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
) {
    let siding_side = if variant == 2 { -1.0 } else { 1.0 };
    spawn_tunnel_shell(seg, a, 9.0, 8.6, false);
    spawn_concrete_arch_ribs(seg, a, 8.6, &[-14.0, 14.0]);
    spawn_ceiling_infrastructure(seg, a, variant, false);
    spawn_water_elements(seg, a, siding_side, false);

    // PILLAR A: Background Silhouettes (Distant Derailed Carriages & Transit Yard Hall)
    match variant {
        0 => {
            spawn_distant_train_silhouette(seg, a, siding_side * 6.5, 0.0, -12.0, 0.12);
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, 14.0, Vec3::new(1.0, 1.0, 1.0));
        }
        1 => {
            spawn_background_maintenance_bridge(seg, a, 0.0, 0.0, 0.0);
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, -16.0, Vec3::new(1.0, 1.0, 1.0));
        }
        _ => {
            spawn_distant_train_silhouette(seg, a, -siding_side * 6.5, 0.0, 8.0, -0.15);
            spawn_background_vent_shaft(seg, a, 0.0, 0.0, -6.0);
        }
    }

    // Modular Freight Consists in siding margins based on chapter variant
    match variant {
        0 => {
            // Variant 0: Heavy industrial locomotive + closed cargo boxcar
            spawn_freight_locomotive(seg, a, siding_side, -6.0, false);
            spawn_freight_boxcar(seg, a, siding_side, 6.5, false, false);
            spawn_culvert_drain(seg, a, -siding_side, 10.0);
            spawn_electrical_box(seg, a, -siding_side, -4.0);
        }
        1 => {
            // Variant 1: Industrial chemical tanker + maintenance utility flatbed wagon
            spawn_freight_tanker(seg, a, siding_side, -6.0, false);
            spawn_freight_maintenance_wagon(seg, a, siding_side, 6.5, false);
            spawn_freight_boxcar(seg, a, -siding_side, 12.0, false, false);
        }
        _ => {
            // Variant 2: Damaged tilted locomotive + open bulk scrap wagon
            spawn_freight_locomotive(seg, a, siding_side, -5.0, true);
            spawn_freight_bulk_wagon(seg, a, siding_side, 7.5, false);
            spawn_ladder(seg, a, siding_side, -14.0);
            spawn_pump_station(seg, a, -siding_side, 8.0);
        }
    }

    spawn_mesh(
        seg,
        &a.mesh_mangled_rail,
        &a.mat_rusted_steel,
        Transform::from_xyz(-siding_side * 6.5, 0.12, -8.0)
            .with_rotation(Quat::from_rotation_y(0.25)),
    );
    spawn_mesh(
        seg,
        &a.mesh_train_bogie,
        &a.mat_steel,
        Transform::from_xyz(-siding_side * 7.2, 0.28, 6.0),
    );
    // Industrial signage: MAINTENANCE YARD // DANGER COLLAPSE placard
    spawn_mesh(
        seg,
        &a.mesh_hazard_sign,
        &a.mat_hazard_stripe,
        Transform::from_xyz(-siding_side * 8.65, 3.4, 0.0),
    );

    if landmark == FloodedMetroLandmark::SunkenTransitWreckage {
        spawn_landmark_sunken_train_wreck(seg, a);
    }
}

fn spawn_landmark_sunken_train_wreck(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    // 7360m–7400m HERO LANDMARK: Derailed industrial freight consist partially submerged in canal siding
    // Lead industrial locomotive heavily tilted into flooded canal siding
    spawn_freight_locomotive(seg, a, -1.0, -8.0, true);
    // Trailing derailed closed boxcar with damaged/slid-open cargo door
    spawn_freight_boxcar(seg, a, -1.0, 4.5, true, true);
    // Trailing partially submerged chemical tanker wagon
    spawn_freight_tanker(seg, a, -1.0, 16.5, true);
    // Opposite siding maintenance flatbed wagon with tilted industrial cable reels
    spawn_freight_maintenance_wagon(seg, a, 1.0, 2.0, false);

    // Detached bogies on mangled track
    spawn_mesh(
        seg,
        &a.mesh_train_bogie,
        &a.mat_rusted_steel,
        Transform::from_xyz(-6.5, 0.35, -1.0).with_rotation(Quat::from_rotation_z(0.35)),
    );
    spawn_mesh(
        seg,
        &a.mesh_train_bogie,
        &a.mat_steel,
        Transform::from_xyz(7.0, 0.35, 10.0).with_rotation(Quat::from_rotation_z(-0.25)),
    );
    // Floating debris & submerged timber cargo pallets
    spawn_mesh(
        seg,
        &a.mesh_floating_debris,
        &a.mat_submerged_wood,
        Transform::from_xyz(-6.6, 0.05, 11.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_floating_debris,
        &a.mat_submerged_wood,
        Transform::from_xyz(6.8, 0.05, -6.0),
    );
    // Mangled rail section protruding from the water
    spawn_mesh(
        seg,
        &a.mesh_mangled_rail,
        &a.mat_rusted_steel,
        Transform::from_xyz(-6.3, 0.15, -14.0).with_rotation(Quat::from_rotation_y(0.40)),
    );
}

fn spawn_freight_locomotive(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    let x = side * 8.2;
    let base_rot = if side < 0.0 {
        Quat::from_rotation_y(std::f32::consts::PI)
    } else {
        Quat::IDENTITY
    };
    let tilt_rot = if tilted {
        Quat::from_rotation_z(side * 0.24) * Quat::from_rotation_x(0.10)
    } else {
        Quat::IDENTITY
    };
    let rot = tilt_rot * base_rot;
    let y = if tilted { 1.35 } else { 1.85 };

    // Heavy industrial locomotive main body
    spawn_mesh(
        seg,
        &a.mesh_freight_loco_body,
        &a.mat_steel,
        Transform::from_xyz(x, y, z).with_rotation(rot),
    );

    // Raised engineer control cab at rear of locomotive
    let cab_offset = rot * Vec3::new(0.0, 1.45, -3.2);
    spawn_mesh(
        seg,
        &a.mesh_freight_loco_cab,
        &a.mat_steel,
        Transform::from_xyz(x + cab_offset.x, y + cab_offset.y, z + cab_offset.z)
            .with_rotation(rot),
    );

    // Armored windshield visor
    let visor_offset = rot * Vec3::new(0.0, 1.45, -1.25);
    spawn_mesh(
        seg,
        &a.mesh_freight_loco_visor,
        &a.mat_rusted_steel,
        Transform::from_xyz(x + visor_offset.x, y + visor_offset.y, z + visor_offset.z)
            .with_rotation(rot),
    );

    // Heavy wedge cowcatcher / plow bumper with hazard stripes at front
    let cowcatcher_offset = rot * Vec3::new(0.0, -0.65, 5.8);
    spawn_mesh(
        seg,
        &a.mesh_freight_cowcatcher,
        &a.mat_hazard_stripe,
        Transform::from_xyz(
            x + cowcatcher_offset.x,
            y + cowcatcher_offset.y,
            z + cowcatcher_offset.z,
        )
        .with_rotation(rot),
    );

    // Twin piercing halogen headlights
    for lamp_x in [-0.9, 0.9] {
        let lamp_offset = rot * Vec3::new(lamp_x, 0.15, 5.95);
        spawn_mesh(
            seg,
            &a.mesh_freight_loco_headlight,
            &a.mat_warning_amber,
            Transform::from_xyz(x + lamp_offset.x, y + lamp_offset.y, z + lamp_offset.z)
                .with_rotation(rot),
        );
    }

    // Dynamic brake blister / radiator cowl on long hood
    let blister_offset = rot * Vec3::new(0.0, 1.45, 1.5);
    spawn_mesh(
        seg,
        &a.mesh_freight_roof_blister,
        &a.mat_rusted_steel,
        Transform::from_xyz(
            x + blister_offset.x,
            y + blister_offset.y,
            z + blister_offset.z,
        )
        .with_rotation(rot),
    );

    // Industrial exhaust stack
    let stack_offset = rot * Vec3::new(0.45, 1.85, 3.2);
    spawn_mesh(
        seg,
        &a.mesh_freight_exhaust_stack,
        &a.mat_rusted_steel,
        Transform::from_xyz(x + stack_offset.x, y + stack_offset.y, z + stack_offset.z)
            .with_rotation(rot),
    );

    // Bogies / wheel assemblies
    for bogie_z in [-3.8, 3.8] {
        let bogie_offset = rot * Vec3::new(0.0, -1.15, bogie_z);
        spawn_mesh(
            seg,
            &a.mesh_train_bogie,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + bogie_offset.x, y + bogie_offset.y, z + bogie_offset.z)
                .with_rotation(rot),
        );
    }

    // Heavy knuckle coupler
    let coupler_offset = rot * Vec3::new(0.0, -0.70, -6.1);
    spawn_mesh(
        seg,
        &a.mesh_freight_coupler,
        &a.mat_steel,
        Transform::from_xyz(
            x + coupler_offset.x,
            y + coupler_offset.y,
            z + coupler_offset.z,
        )
        .with_rotation(rot),
    );
}

fn spawn_freight_boxcar(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
    damaged_door: bool,
) {
    let x = side * 8.2;
    let rot = if tilted {
        Quat::from_rotation_z(side * 0.20) * Quat::from_rotation_x(0.08)
    } else {
        Quat::IDENTITY
    };
    let y = if tilted { 1.45 } else { 1.85 };

    // Heavy corrugated steel freight container / boxcar body
    spawn_mesh(
        seg,
        &a.mesh_freight_boxcar_body,
        &a.mat_rusted_steel,
        Transform::from_xyz(x, y, z).with_rotation(rot),
    );

    // Exterior vertical reinforcement ribs
    for rib_z in [-4.6, -2.3, 0.0, 2.3, 4.6] {
        for side_sign in [-1.0, 1.0] {
            let rib_offset = rot * Vec3::new(side_sign * 1.62, 0.0, rib_z);
            spawn_mesh(
                seg,
                &a.mesh_freight_rib,
                &a.mat_steel,
                Transform::from_xyz(x + rib_offset.x, y + rib_offset.y, z + rib_offset.z)
                    .with_rotation(rot),
            );
        }
    }

    // Heavy sliding cargo door facing track corridor
    let door_inward = -side;
    let door_z_offset = if damaged_door { 1.2 } else { 0.0 };
    let door_rot = if damaged_door {
        rot * Quat::from_rotation_y(0.25)
    } else {
        rot
    };
    let door_offset = rot * Vec3::new(door_inward * 1.66, -0.15, door_z_offset);
    spawn_mesh(
        seg,
        &a.mesh_freight_boxcar_door,
        &a.mat_steel,
        Transform::from_xyz(x + door_offset.x, y + door_offset.y, z + door_offset.z)
            .with_rotation(door_rot),
    );

    // Bogies / wheel assemblies
    for bogie_z in [-3.6, 3.6] {
        let bogie_offset = rot * Vec3::new(0.0, -1.15, bogie_z);
        spawn_mesh(
            seg,
            &a.mesh_train_bogie,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + bogie_offset.x, y + bogie_offset.y, z + bogie_offset.z)
                .with_rotation(rot),
        );
    }

    // Couplers at both ends
    for end_z in [-5.8, 5.8] {
        let coup_offset = rot * Vec3::new(0.0, -0.70, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_coupler,
            &a.mat_steel,
            Transform::from_xyz(x + coup_offset.x, y + coup_offset.y, z + coup_offset.z)
                .with_rotation(rot),
        );
    }
}

fn spawn_freight_tanker(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    let x = side * 8.2;
    let rot = if tilted {
        Quat::from_rotation_z(side * 0.22) * Quat::from_rotation_x(-0.08)
    } else {
        Quat::IDENTITY
    };
    let y = if tilted { 1.40 } else { 1.85 };

    // Horizontal cylindrical pressurized chemical / fuel tank
    let tank_rot = rot * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    spawn_mesh(
        seg,
        &a.mesh_freight_tank_body,
        &a.mat_steel,
        Transform::from_xyz(x, y + 0.35, z).with_rotation(tank_rot),
    );

    // Hemispherical rounded end caps
    for end_z in [-5.0, 5.0] {
        let cap_offset = rot * Vec3::new(0.0, 0.35, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_tank_cap,
            &a.mat_steel,
            Transform::from_xyz(x + cap_offset.x, y + cap_offset.y, z + cap_offset.z)
                .with_rotation(rot),
        );
    }

    // Saddle cradle mounts
    for saddle_z in [-3.5, 3.5] {
        let saddle_offset = rot * Vec3::new(0.0, -0.65, saddle_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_tank_saddle,
            &a.mat_rusted_steel,
            Transform::from_xyz(
                x + saddle_offset.x,
                y + saddle_offset.y,
                z + saddle_offset.z,
            )
            .with_rotation(rot),
        );
    }

    // Top inspection manhole dome
    let dome_offset = rot * Vec3::new(0.0, 1.70, 0.0);
    spawn_mesh(
        seg,
        &a.mesh_freight_tank_dome,
        &a.mat_rusted_steel,
        Transform::from_xyz(x + dome_offset.x, y + dome_offset.y, z + dome_offset.z)
            .with_rotation(rot),
    );

    // Hazardous chemical warning stripe placard
    let placard_inward = -side;
    let placard_offset = rot * Vec3::new(placard_inward * 1.42, 0.35, 0.0);
    spawn_mesh(
        seg,
        &a.mesh_hazard_sign,
        &a.mat_hazard_stripe,
        Transform::from_xyz(
            x + placard_offset.x,
            y + placard_offset.y,
            z + placard_offset.z,
        )
        .with_rotation(rot),
    );

    // Bogies
    for bogie_z in [-3.6, 3.6] {
        let bogie_offset = rot * Vec3::new(0.0, -1.15, bogie_z);
        spawn_mesh(
            seg,
            &a.mesh_train_bogie,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + bogie_offset.x, y + bogie_offset.y, z + bogie_offset.z)
                .with_rotation(rot),
        );
    }

    // Couplers
    for end_z in [-5.5, 5.5] {
        let coup_offset = rot * Vec3::new(0.0, -0.70, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_coupler,
            &a.mat_steel,
            Transform::from_xyz(x + coup_offset.x, y + coup_offset.y, z + coup_offset.z)
                .with_rotation(rot),
        );
    }
}

fn spawn_freight_bulk_wagon(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    let x = side * 8.2;
    let rot = if tilted {
        Quat::from_rotation_z(side * 0.18) * Quat::from_rotation_x(0.06)
    } else {
        Quat::IDENTITY
    };
    let y = if tilted { 1.35 } else { 1.75 };

    // Flatbed bed floor
    let floor_offset = rot * Vec3::new(0.0, -0.65, 0.0);
    spawn_mesh(
        seg,
        &a.mesh_freight_bulk_floor,
        &a.mat_steel,
        Transform::from_xyz(x + floor_offset.x, y + floor_offset.y, z + floor_offset.z)
            .with_rotation(rot),
    );

    // Long side walls
    for side_sign in [-1.0, 1.0] {
        let wall_offset = rot * Vec3::new(side_sign * 1.52, 0.10, 0.0);
        spawn_mesh(
            seg,
            &a.mesh_freight_bulk_wall_long,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + wall_offset.x, y + wall_offset.y, z + wall_offset.z)
                .with_rotation(rot),
        );
    }

    // End bulkhead walls
    for end_z in [-5.65, 5.65] {
        let end_offset = rot * Vec3::new(0.0, 0.10, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_bulk_wall_end,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + end_offset.x, y + end_offset.y, z + end_offset.z)
                .with_rotation(rot),
        );
    }

    // Heavy irregular dark scrap metal / mineral load
    let scrap_offset = rot * Vec3::new(0.0, -0.20, 0.0);
    spawn_mesh(
        seg,
        &a.mesh_freight_bulk_scrap,
        &a.mat_bg_dark_structure,
        Transform::from_xyz(x + scrap_offset.x, y + scrap_offset.y, z + scrap_offset.z)
            .with_rotation(rot),
    );

    // Bogies
    for bogie_z in [-3.6, 3.6] {
        let bogie_offset = rot * Vec3::new(0.0, -1.15, bogie_z);
        spawn_mesh(
            seg,
            &a.mesh_train_bogie,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + bogie_offset.x, y + bogie_offset.y, z + bogie_offset.z)
                .with_rotation(rot),
        );
    }

    // Couplers
    for end_z in [-5.8, 5.8] {
        let coup_offset = rot * Vec3::new(0.0, -0.70, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_coupler,
            &a.mat_steel,
            Transform::from_xyz(x + coup_offset.x, y + coup_offset.y, z + coup_offset.z)
                .with_rotation(rot),
        );
    }
}

fn spawn_freight_maintenance_wagon(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    let x = side * 8.2;
    let rot = if tilted {
        Quat::from_rotation_z(side * 0.16) * Quat::from_rotation_x(0.05)
    } else {
        Quat::IDENTITY
    };
    let y = if tilted { 1.25 } else { 1.65 };

    // Heavy channel flatbed chassis
    spawn_mesh(
        seg,
        &a.mesh_freight_flatbed_chassis,
        &a.mat_steel,
        Transform::from_xyz(x, y - 0.40, z).with_rotation(rot),
    );

    // Heavy industrial cable spools on deck
    for (spool_z, spool_rot_tilt) in [(-2.4, 0.0), (2.0, 0.20)] {
        let spool_rot = rot
            * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_z(spool_rot_tilt);
        let spool_offset = rot * Vec3::new(0.0, 0.70, spool_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_cable_spool,
            &a.mat_submerged_wood,
            Transform::from_xyz(x + spool_offset.x, y + spool_offset.y, z + spool_offset.z)
                .with_rotation(spool_rot),
        );
    }

    // Utility generator / equipment crate
    let crate_offset = rot * Vec3::new(0.0, 0.65, -4.5);
    spawn_mesh(
        seg,
        &a.mesh_freight_equipment_crate,
        &a.mat_rusted_steel,
        Transform::from_xyz(x + crate_offset.x, y + crate_offset.y, z + crate_offset.z)
            .with_rotation(rot),
    );

    // Bogies
    for bogie_z in [-3.6, 3.6] {
        let bogie_offset = rot * Vec3::new(0.0, -1.05, bogie_z);
        spawn_mesh(
            seg,
            &a.mesh_train_bogie,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + bogie_offset.x, y + bogie_offset.y, z + bogie_offset.z)
                .with_rotation(rot),
        );
    }

    // Couplers
    for end_z in [-5.8, 5.8] {
        let coup_offset = rot * Vec3::new(0.0, -0.60, end_z);
        spawn_mesh(
            seg,
            &a.mesh_freight_coupler,
            &a.mat_steel,
            Transform::from_xyz(x + coup_offset.x, y + coup_offset.y, z + coup_offset.z)
                .with_rotation(rot),
        );
    }
}

#[allow(dead_code)]
fn spawn_train_carriage(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    spawn_freight_boxcar(seg, a, side, z, tilted, false);
}

// -------------------------------------------------------------
// CHAPTER 5 — DEEP-LINE APPROACH (7650m–8000m)
// -------------------------------------------------------------

fn spawn_deep_line_approach_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
    distance: f32,
) {
    spawn_tunnel_shell(seg, a, 9.0, 8.6, false);
    spawn_concrete_arch_ribs(seg, a, 8.6, &[-14.0, -7.0, 7.0, 14.0]);
    spawn_ceiling_infrastructure(seg, a, variant, false);
    spawn_pipe_runs(seg, a, variant);
    spawn_water_elements(seg, a, -1.0, false);

    // PILLAR A: Background Silhouettes (Fortified Deep Bulkheads & ECHO Portals)
    match variant {
        0 => {
            spawn_background_blast_door_portal(seg, a, 0.0, 0.0, -16.0);
        }
        1 => {
            spawn_background_tunnel_arch(seg, a, 0.0, 0.0, 0.0, Vec3::new(1.05, 1.0, 1.0));
            spawn_background_pipe_bank(seg, a, 0.0, 0.0, 12.0);
        }
        _ => {
            spawn_background_maintenance_bridge(seg, a, 0.0, 0.0, -10.0);
            spawn_background_vent_shaft(seg, a, 0.0, 0.0, 10.0);
        }
    }

    for side in [-1.0, 1.0] {
        spawn_mesh(
            seg,
            &a.mesh_watertight_hatch,
            &a.mat_steel,
            Transform::from_xyz(side * 8.7, 1.8, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_locking_clamp,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 8.35, 1.8, 0.0),
        );
    }

    spawn_mesh(
        seg,
        &a.mesh_hazard_sign,
        &a.mat_warning_amber,
        Transform::from_xyz(-7.8, 4.2, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_emergency_light,
        &a.mat_emergency_red,
        Transform::from_xyz(7.8, 4.2, 0.0),
    );

    match variant {
        0 => {
            spawn_electrical_box(seg, a, -1.0, -8.0);
            spawn_pump_station(seg, a, 1.0, 8.0);
        }
        1 => {
            spawn_mesh(
                seg,
                &a.mesh_echo_sensor_relay,
                &a.mat_teal_glow,
                Transform::from_xyz(-8.2, 2.6, 6.0),
            );
            spawn_culvert_drain(seg, a, 1.0, -6.0);
        }
        _ => {
            spawn_ladder(seg, a, -1.0, 10.0);
            spawn_pump_station(seg, a, -1.0, -8.0);
            spawn_electrical_box(seg, a, 1.0, 6.0);
        }
    }

    // PILLAR F: 8000m ZONE 5 APPROACH TRANSITION
    // In the final 120m after the hero bulkhead (7880m-8000m), infrastructure becomes cleaner,
    // fortified, and controlled as Kai approaches the deep ECHO research facility.
    if distance >= 7880.0 {
        spawn_mesh(
            seg,
            &a.mesh_echo_conduit_ring,
            &a.mat_teal_glow,
            Transform::from_xyz(0.0, 7.4, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
        for side in [-1.0, 1.0] {
            spawn_mesh(
                seg,
                &a.mesh_echo_sensor_relay,
                &a.mat_teal_glow,
                Transform::from_xyz(side * 8.4, 3.4, -6.0),
            );
        }
        spawn_mesh(
            seg,
            &a.mesh_hazard_sign,
            &a.mat_emergency_red,
            Transform::from_xyz(-8.65, 3.8, 6.0),
        );
    }

    if landmark == FloodedMetroLandmark::DeepLineBulkheadPortal {
        spawn_landmark_deep_bulkhead_portal(seg, a);
    }
}

fn spawn_landmark_deep_bulkhead_portal(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    spawn_mesh(
        seg,
        &a.mesh_deep_portal_arch,
        &a.mat_steel,
        Transform::from_xyz(0.0, 7.2, 0.0),
    );
    for side in [-1.0, 1.0] {
        let x = side * 6.8;
        spawn_mesh(
            seg,
            &a.mesh_bulkhead_frame,
            &a.mat_steel,
            Transform::from_xyz(x, 4.0, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_deep_bulkhead_leaf,
            &a.mat_wet_concrete,
            Transform::from_xyz(x, 3.8, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_locking_clamp,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 4.2, 3.8, 0.4),
        );
        spawn_mesh(
            seg,
            &a.mesh_emergency_light,
            &a.mat_emergency_red,
            Transform::from_xyz(side * 5.8, 6.4, 0.3),
        );
    }
    spawn_mesh(
        seg,
        &a.mesh_echo_conduit_ring,
        &a.mat_teal_glow,
        Transform::from_xyz(0.0, 6.8, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    );
    spawn_mesh(
        seg,
        &a.mesh_hazard_sign,
        &a.mat_hazard_stripe,
        Transform::from_xyz(0.0, 7.8, 0.3),
    );
}

// -------------------------------------------------------------
// PILLAR D — LIGHTWEIGHT WATERDRIP SYSTEM (ROBUST BEVY 0.14)
// -------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WaterDripEmitterType {
    CeilingCrack,
    PipeLeak,
    DrainageOutlet,
    StructuralJoint,
    BrokenVent,
    PumpHousingLeak,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct WaterDripEmitter {
    pub emitter_type: WaterDripEmitterType,
    pub timer: f32,
    pub interval: f32,
    pub droplet_speed: f32,
    pub floor_y: f32,
    pub spread: f32,
    pub active_count: u32,
    pub max_active: u32,
}

#[derive(Component)]
pub struct WaterDripDroplet {
    pub speed: f32,
    pub floor_y: f32,
    pub emitter: Entity,
}

pub fn spawn_water_drip_emitter(
    parent: &mut ChildBuilder,
    _flooded: &FloodedMetroAssets,
    emitter_type: WaterDripEmitterType,
    pos: Vec3,
    floor_y: f32,
) {
    let (interval, droplet_speed, spread, max_active) = match emitter_type {
        WaterDripEmitterType::CeilingCrack => (0.32, 12.5, 0.35, 14),
        WaterDripEmitterType::PipeLeak => (0.20, 14.0, 0.15, 18),
        WaterDripEmitterType::DrainageOutlet => (0.16, 15.0, 0.20, 20),
        WaterDripEmitterType::StructuralJoint => (0.40, 11.5, 0.30, 12),
        WaterDripEmitterType::BrokenVent => (0.26, 13.0, 0.45, 15),
        WaterDripEmitterType::PumpHousingLeak => (0.24, 13.5, 0.18, 16),
    };

    parent.spawn((
        SpatialBundle::from_transform(Transform::from_translation(pos)),
        WaterDripEmitter {
            emitter_type,
            timer: 0.0,
            interval,
            droplet_speed,
            floor_y,
            spread,
            active_count: 0,
            max_active,
        },
    ));
}

fn spawn_chapter_water_drip_emitters(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    chapter: FloodedMetroChapter,
    variant: u8,
) {
    match chapter {
        FloodedMetroChapter::FloodgateDescent => {
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::StructuralJoint,
                Vec3::new(0.0, 7.2, -10.0),
                0.0,
            );
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::PipeLeak,
                Vec3::new(-7.4, 4.4, 4.0),
                0.0,
            );
            if variant == 0 {
                spawn_water_drip_emitter(
                    seg,
                    a,
                    WaterDripEmitterType::PumpHousingLeak,
                    Vec3::new(6.8, 2.8, 6.0),
                    0.0,
                );
            }
        }
        FloodedMetroChapter::SubmergedStation => {
            let plat_x = if variant == 1 { -6.5 } else { 6.5 };
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::CeilingCrack,
                Vec3::new(plat_x, 6.2, 0.0),
                0.42,
            );
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::PipeLeak,
                Vec3::new(-plat_x * 0.8, 6.4, -8.0),
                0.0,
            );
        }
        FloodedMetroChapter::ReservoirCrossing => {
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::CeilingCrack,
                Vec3::new(0.0, 9.5, 0.0),
                0.0,
            );
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::DrainageOutlet,
                Vec3::new(12.0, 4.5, -10.0),
                -0.06,
            );
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::StructuralJoint,
                Vec3::new(-6.8, 5.0, 8.0),
                0.0,
            );
        }
        FloodedMetroChapter::SunkenTransit => {
            let siding_side = if variant == 2 { -1.0 } else { 1.0 };
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::BrokenVent,
                Vec3::new(siding_side * 8.0, 6.0, 0.0),
                1.6,
            );
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::CeilingCrack,
                Vec3::new(-siding_side * 3.0, 6.4, 12.0),
                0.0,
            );
        }
        FloodedMetroChapter::DeepLineApproach => {
            spawn_water_drip_emitter(
                seg,
                a,
                WaterDripEmitterType::StructuralJoint,
                Vec3::new(-6.5, 5.8, 0.0),
                0.0,
            );
            if variant == 1 {
                spawn_water_drip_emitter(
                    seg,
                    a,
                    WaterDripEmitterType::CeilingCrack,
                    Vec3::new(4.0, 6.8, 8.0),
                    0.0,
                );
            }
        }
    }
}

pub fn update_water_drip_emitters(
    mut commands: Commands,
    time: Res<Time>,
    flooded: Option<Res<FloodedMetroAssets>>,
    mut emitter_query: Query<(Entity, &GlobalTransform, &mut WaterDripEmitter)>,
) {
    let Some(flooded) = flooded else { return };
    let dt = time.delta_seconds();

    for (emitter_entity, global_transform, mut emitter) in emitter_query.iter_mut() {
        emitter.timer += dt;
        if emitter.timer >= emitter.interval {
            emitter.timer = 0.0;
            if emitter.active_count < emitter.max_active {
                emitter.active_count += 1;
                let offset_x =
                    ((emitter.active_count as f32 * 1.618).fract() - 0.5) * 2.0 * emitter.spread;
                let offset_z =
                    ((emitter.active_count as f32 * 2.718).fract() - 0.5) * 2.0 * emitter.spread;
                let drop_mesh = flooded.mesh_water_droplet.clone();
                let drop_mat = flooded.mat_water_droplet.clone();
                let droplet_speed = emitter.droplet_speed;
                let floor_y = emitter.floor_y;
                let origin = global_transform.translation();
                let spawn_pos = origin + Vec3::new(offset_x, 0.0, offset_z);

                commands.spawn((
                    PbrBundle {
                        mesh: drop_mesh,
                        material: drop_mat,
                        transform: Transform::from_translation(spawn_pos),
                        ..default()
                    },
                    WaterDripDroplet {
                        speed: droplet_speed,
                        floor_y,
                        emitter: emitter_entity,
                    },
                    Despawnable {
                        z_center: spawn_pos.z,
                    },
                ));
            }
        }
    }
}

pub fn update_water_drips(
    mut commands: Commands,
    time: Res<Time>,
    mut droplet_query: Query<(Entity, &mut Transform, &WaterDripDroplet)>,
    mut emitter_query: Query<&mut WaterDripEmitter>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, droplet) in droplet_query.iter_mut() {
        transform.translation.y -= droplet.speed * dt;
        if transform.translation.y <= droplet.floor_y {
            if let Ok(mut emitter) = emitter_query.get_mut(droplet.emitter) {
                emitter.active_count = emitter.active_count.saturating_sub(1);
            }
            if let Some(mut entity_cmd) = commands.get_entity(entity) {
                entity_cmd.despawn();
            }
        }
    }
}

// -------------------------------------------------------------
// GENERIC UTILITY HELPER
// -------------------------------------------------------------

fn spawn_mesh(
    seg: &mut ChildBuilder,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    transform: Transform,
) {
    seg.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform,
        ..default()
    });
}

// -------------------------------------------------------------
// UNIT TESTS
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flooded_metro_chapters_follow_authored_boundaries() {
        assert_eq!(
            flooded_metro_chapter_for_distance(6000.0),
            FloodedMetroChapter::FloodgateDescent
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6299.99),
            FloodedMetroChapter::FloodgateDescent
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6300.0),
            FloodedMetroChapter::SubmergedStation
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6750.0),
            FloodedMetroChapter::ReservoirCrossing
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(7200.0),
            FloodedMetroChapter::SunkenTransit
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(7650.0),
            FloodedMetroChapter::DeepLineApproach
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(8000.0),
            FloodedMetroChapter::DeepLineApproach
        );
    }

    #[test]
    fn flooded_metro_variants_are_deterministic_and_bounded() {
        for distance in [6000.0, 6300.0, 6750.0, 7200.0, 7650.0, 7999.0] {
            assert_eq!(
                flooded_metro_variant(distance),
                flooded_metro_variant(distance)
            );
            assert!(flooded_metro_variant(distance) < 3);
        }
    }

    #[test]
    fn test_flooded_metro_landmarks_appear_in_each_chapter() {
        assert_eq!(
            flooded_metro_landmark_for_distance(6130.0),
            FloodedMetroLandmark::FloodgateDescentSluice
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(6490.0),
            FloodedMetroLandmark::SubmergedStationComplex
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(6980.0),
            FloodedMetroLandmark::ReservoirCavernCenterpiece
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7380.0),
            FloodedMetroLandmark::SunkenTransitWreckage
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7860.0),
            FloodedMetroLandmark::DeepLineBulkheadPortal
        );

        assert_eq!(
            flooded_metro_landmark_for_distance(6050.0),
            FloodedMetroLandmark::None
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(6600.0),
            FloodedMetroLandmark::None
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7100.0),
            FloodedMetroLandmark::None
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7500.0),
            FloodedMetroLandmark::None
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7700.0),
            FloodedMetroLandmark::None
        );
    }

    #[test]
    fn test_flooded_metro_water_drip_emitter_types() {
        let emitters = [
            WaterDripEmitterType::CeilingCrack,
            WaterDripEmitterType::PipeLeak,
            WaterDripEmitterType::DrainageOutlet,
            WaterDripEmitterType::StructuralJoint,
            WaterDripEmitterType::BrokenVent,
            WaterDripEmitterType::PumpHousingLeak,
        ];
        assert_eq!(emitters.len(), 6);
    }

    #[test]
    fn test_flooded_metro_background_composition_chapters() {
        let checkpoints = [
            (6000.0, FloodedMetroChapter::FloodgateDescent),
            (6300.0, FloodedMetroChapter::SubmergedStation),
            (6750.0, FloodedMetroChapter::ReservoirCrossing),
            (7200.0, FloodedMetroChapter::SunkenTransit),
            (7650.0, FloodedMetroChapter::DeepLineApproach),
            (8000.0, FloodedMetroChapter::DeepLineApproach),
        ];

        for (dist, expected_ch) in checkpoints {
            assert_eq!(flooded_metro_chapter_for_distance(dist), expected_ch);
            let var = flooded_metro_variant(dist);
            assert!(var < 3);
        }
    }

    #[test]
    fn test_flooded_metro_sunken_transit_freight_landmark() {
        assert_eq!(
            flooded_metro_chapter_for_distance(7380.0),
            FloodedMetroChapter::SunkenTransit
        );
        assert_eq!(
            flooded_metro_landmark_for_distance(7380.0),
            FloodedMetroLandmark::SunkenTransitWreckage
        );
        // Non-landmark distance within Chapter 4
        assert_eq!(
            flooded_metro_landmark_for_distance(7300.0),
            FloodedMetroLandmark::None
        );
    }
}
