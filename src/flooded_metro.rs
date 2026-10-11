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

    // --- Chapter 4: Sunken Transit ---
    pub mesh_train_body: Handle<Mesh>,
    pub mesh_train_roof: Handle<Mesh>,
    pub mesh_train_window: Handle<Mesh>,
    pub mesh_train_door: Handle<Mesh>,
    pub mesh_train_bogie: Handle<Mesh>,
    pub mesh_mangled_rail: Handle<Mesh>,

    // --- Chapter 5: Deep-Line Approach ---
    pub mesh_deep_portal_arch: Handle<Mesh>,
    pub mesh_deep_bulkhead_leaf: Handle<Mesh>,
    pub mesh_locking_clamp: Handle<Mesh>,
    pub mesh_echo_conduit_ring: Handle<Mesh>,
    pub mesh_echo_sensor_relay: Handle<Mesh>,

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
}

pub fn init_flooded_metro_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let assets = FloodedMetroAssets {
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

        // --- Chapter 4: Sunken Transit ---
        mesh_train_body: meshes.add(Cuboid::new(3.3, 2.8, 13.5)),
        mesh_train_roof: meshes.add(Cuboid::new(3.5, 0.32, 13.2)),
        mesh_train_window: meshes.add(Cuboid::new(0.08, 0.85, 1.7)),
        mesh_train_door: meshes.add(Cuboid::new(0.10, 2.0, 1.15)),
        mesh_train_bogie: meshes.add(Cuboid::new(2.6, 0.55, 3.0)),
        mesh_mangled_rail: meshes.add(Cuboid::new(0.16, 0.22, 6.5)),

        // --- Chapter 5: Deep-Line Approach ---
        mesh_deep_portal_arch: meshes.add(Cuboid::new(20.0, 2.0, 2.6)),
        mesh_deep_bulkhead_leaf: meshes.add(Cuboid::new(4.6, 5.8, 0.75)),
        mesh_locking_clamp: meshes.add(Cuboid::new(0.55, 0.55, 1.6)),
        mesh_echo_conduit_ring: meshes.add(Cylinder::new(3.8, 0.38)),
        mesh_echo_sensor_relay: meshes.add(Cuboid::new(0.48, 0.75, 0.48)),

        // --- Materials ---
        mat_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.15, 0.17),
            perceptual_roughness: 0.88,
            ..default()
        }),
        mat_wet_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.085, 0.10),
            metallic: 0.18,
            perceptual_roughness: 0.36,
            ..default()
        }),
        mat_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.20, 0.22),
            metallic: 0.88,
            perceptual_roughness: 0.32,
            ..default()
        }),
        mat_rusted_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.15, 0.11),
            metallic: 0.55,
            perceptual_roughness: 0.78,
            ..default()
        }),
        mat_corroded_copper: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.18, 0.16),
            metallic: 0.32,
            perceptual_roughness: 0.72,
            ..default()
        }),
        mat_water: materials.add(StandardMaterial {
            base_color: Color::srgb(0.012, 0.14, 0.18),
            emissive: LinearRgba::new(0.003, 0.07, 0.09, 1.0),
            metallic: 0.22,
            perceptual_roughness: 0.14,
            ..default()
        }),
        mat_water_puddle: materials.add(StandardMaterial {
            base_color: Color::srgb(0.018, 0.11, 0.14),
            metallic: 0.30,
            perceptual_roughness: 0.06,
            ..default()
        }),
        mat_water_stain: materials.add(StandardMaterial {
            base_color: Color::srgb(0.022, 0.065, 0.075),
            metallic: 0.06,
            perceptual_roughness: 0.94,
            ..default()
        }),
        mat_cable: materials.add(StandardMaterial {
            base_color: Color::srgb(0.032, 0.040, 0.045),
            metallic: 0.35,
            perceptual_roughness: 0.74,
            ..default()
        }),
        mat_teal_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.45, 0.52),
            emissive: LinearRgba::new(0.04, 0.85, 1.05, 1.0),
            metallic: 0.25,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_warning_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.36, 0.08),
            emissive: LinearRgba::new(0.95, 0.22, 0.02, 1.0),
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_emergency_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.10, 0.05),
            emissive: LinearRgba::new(1.10, 0.05, 0.02, 1.0),
            perceptual_roughness: 0.45,
            ..default()
        }),
        mat_hazard_stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.70, 0.56, 0.08),
            perceptual_roughness: 0.52,
            ..default()
        }),
        mat_train_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.085, 0.115, 0.135),
            metallic: 0.70,
            perceptual_roughness: 0.44,
            ..default()
        }),
        mat_train_window: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.08, 0.10),
            emissive: LinearRgba::new(0.01, 0.18, 0.22, 1.0),
            metallic: 0.30,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_submerged_wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.09, 0.065),
            perceptual_roughness: 0.88,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

pub fn spawn_flooded_metro_segment(
    commands: &mut Commands,
    env: &EnvironmentAssets,
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
            spawn_flooded_track(seg, env, length);

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
                    spawn_deep_line_approach_segment(seg, flooded, variant, landmark);
                }
            }
        })
        .id()
}

fn spawn_flooded_track(seg: &mut ChildBuilder, env: &EnvironmentAssets, length: f32) {
    seg.spawn(PbrBundle {
        mesh: env.mesh_bed.clone(),
        material: env.mat_concrete_bed.clone(),
        transform: Transform::from_xyz(0.0, -0.225, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_drain_trough.clone(),
        material: env.mat_drainage.clone(),
        transform: Transform::from_xyz(0.0, -0.05, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_curb_shoulder.clone(),
        material: env.mat_ballast_curb.clone(),
        transform: Transform::from_xyz(-4.2, 0.12, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_curb_shoulder.clone(),
        material: env.mat_ballast_curb.clone(),
        transform: Transform::from_xyz(4.2, 0.12, 0.0),
        ..default()
    });

    let sleeper_count = (length / 4.0) as i32;
    for i in -sleeper_count / 2..=sleeper_count / 2 {
        seg.spawn(PbrBundle {
            mesh: env.mesh_sleeper_heavy.clone(),
            material: env.mat_sleeper_stained.clone(),
            transform: Transform::from_xyz(0.0, 0.01, i as f32 * 4.0),
            ..default()
        });
    }

    for lane_idx in -1..=1 {
        let x = Lane::from_index(lane_idx).x_pos();
        for rail_x in [x - 0.48, x + 0.48] {
            seg.spawn(PbrBundle {
                mesh: env.mesh_rail_base.clone(),
                material: env.mat_rail_base.clone(),
                transform: Transform::from_xyz(rail_x, 0.04, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_rail_crown.clone(),
                material: env.mat_rail_crown.clone(),
                transform: Transform::from_xyz(rail_x, 0.09, 0.0),
                ..default()
            });
        }
        for i in -4..=4 {
            seg.spawn(PbrBundle {
                mesh: env.mesh_lane_stripe.clone(),
                material: env.mat_echo_conduit.clone(),
                transform: Transform::from_xyz(x, 0.13, i as f32 * 4.5),
                ..default()
            });
        }
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
        spawn_mesh(
            seg,
            &a.mesh_waterline_mark,
            &a.mat_water_stain,
            Transform::from_xyz(x - side * 0.24, 1.85, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_conduit_tray,
            &a.mat_steel,
            Transform::from_xyz(x - side * 0.26, 3.4, 0.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_support_column,
            &a.mat_concrete,
            Transform::from_xyz(x - side * 0.45, 4.3, -16.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_support_column,
            &a.mat_concrete,
            Transform::from_xyz(x - side * 0.45, 4.3, 16.0),
        );
    }
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

    // Continuous drainage conduits and structural girders overhead
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

    // Caged utility fixtures at ceiling level
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
    // Left shoulder large drainage main with flange joints and valve wheels
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

    // Right shoulder smaller multi-conduit cluster
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
    } else {
        spawn_mesh(
            seg,
            &a.mesh_water_trackside,
            &a.mat_water,
            Transform::from_xyz(side * 8.5, -0.02, 0.0),
        );
    }

    // Standing water puddles along track curb shoulders
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

    match variant {
        0 => {
            spawn_culvert_drain(seg, a, -1.0, -8.0);
            spawn_pump_station(seg, a, 1.0, 6.0);
            spawn_ladder(seg, a, 1.0, -12.0);
        }
        1 => {
            spawn_electrical_box(seg, a, -1.0, 4.0);
            spawn_culvert_drain(seg, a, 1.0, 10.0);
            spawn_mesh(
                seg,
                &a.mesh_watertight_hatch,
                &a.mat_steel,
                Transform::from_xyz(-8.7, 1.6, -4.0),
            );
        }
        _ => {
            spawn_pump_station(seg, a, -1.0, -4.0);
            spawn_electrical_box(seg, a, 1.0, 8.0);
            spawn_culvert_drain(seg, a, -1.0, 12.0);
        }
    }

    if landmark == FloodedMetroLandmark::FloodgateDescentSluice {
        spawn_landmark_floodgate(seg, a);
    }
}

fn spawn_landmark_floodgate(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    // Massive Reinforced Sluice Floodgate Arch
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
        // Hydraulic actuator assemblies
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
        // Emergency beacon indicators on gate towers
        spawn_mesh(
            seg,
            &a.mesh_emergency_light,
            &a.mat_warning_amber,
            Transform::from_xyz(side * 6.2, 7.2, 0.2),
        );
    }

    // Heavy reinforced steel gate leaves suspended overhead with safe clearance for Kai
    for side in [-1.0, 1.0] {
        spawn_mesh(
            seg,
            &a.mesh_gate_leaf_heavy,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 3.8, 5.4, 0.0),
        );
    }

    // Vertical flood level gauge board
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

    // Elevated passenger station platform on one flank
    let plat_x = platform_side * 6.5;
    spawn_mesh(
        seg,
        &a.mesh_station_platform,
        &a.mat_wet_concrete,
        Transform::from_xyz(plat_x, 0.42, 0.0),
    );
    // Tactile hazard edge stripe along platform edge
    spawn_mesh(
        seg,
        &a.mesh_platform_tactile_edge,
        &a.mat_hazard_stripe,
        Transform::from_xyz(platform_side * 3.9, 0.74, 0.0),
    );

    // Metro tile arch columns supporting station roof
    for z in [-14.0, 0.0, 14.0] {
        spawn_mesh(
            seg,
            &a.mesh_station_pillar_tile,
            &a.mat_concrete,
            Transform::from_xyz(platform_side * 5.2, 3.2, z),
        );
    }

    // Station passenger furniture & fittings
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
            // Floating debris and submerged pallets in flooded opposite lane
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
    // Grand Station Platform Complex with submerged passenger stairs
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
    // Vast Cavern Shell: pushed out to 21 meters with 16m high walls!
    spawn_tunnel_shell(seg, a, 21.0, 16.0, true);
    spawn_ceiling_infrastructure(seg, a, variant, true);
    spawn_water_elements(seg, a, 1.0, true);

    // Massive concrete bridge piers plunging down into deep water
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

    // High overhead maintenance catwalks along cavern sides
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

    match variant {
        0 => {
            // Cylindrical intake tower rising from the water
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
            // Turbine pump housing island
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
            // Floating debris and maintenance ladders
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
    // Colossal Overhead Truss Aqueduct Bridge Spanning Across the Cavern
    spawn_mesh(
        seg,
        &a.mesh_reservoir_truss_span,
        &a.mat_steel,
        Transform::from_xyz(0.0, 10.8, 0.0),
    );
    // Twin towering intake towers rising from the dark lake on both flanks
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

    // Stalled abandoned train carriage on flooded siding
    spawn_train_carriage(seg, a, siding_side, 0.0, false);

    // Mangled track fragments and wheel bogies in floodwater
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

    match variant {
        0 => {
            spawn_culvert_drain(seg, a, -siding_side, 10.0);
            spawn_electrical_box(seg, a, -siding_side, -4.0);
        }
        1 => {
            // Second carriage on the other siding
            spawn_train_carriage(seg, a, -siding_side, 12.0, false);
        }
        _ => {
            spawn_ladder(seg, a, siding_side, -14.0);
            spawn_pump_station(seg, a, -siding_side, 8.0);
        }
    }

    if landmark == FloodedMetroLandmark::SunkenTransitWreckage {
        spawn_landmark_sunken_train_wreck(seg, a);
    }
}

fn spawn_landmark_sunken_train_wreck(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    // The Sunken Express Derailment Formation: heavily tilted into water basin
    spawn_train_carriage(seg, a, -1.0, -4.0, true);
    spawn_train_carriage(seg, a, 1.0, 10.0, false);

    // Floating debris and dislodged bogies around the wreckage
    spawn_mesh(
        seg,
        &a.mesh_train_bogie,
        &a.mat_rusted_steel,
        Transform::from_xyz(-6.2, 0.35, 6.0).with_rotation(Quat::from_rotation_z(0.35)),
    );
    spawn_mesh(
        seg,
        &a.mesh_floating_debris,
        &a.mat_submerged_wood,
        Transform::from_xyz(6.8, 0.05, -6.0),
    );
}

fn spawn_train_carriage(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    side: f32,
    z: f32,
    tilted: bool,
) {
    let x = side * 8.2;
    let inward = -side;
    let rot = if tilted {
        Quat::from_rotation_z(side * 0.22) * Quat::from_rotation_x(0.08)
    } else {
        Quat::IDENTITY
    };
    let y = if tilted { 1.6 } else { 2.1 };

    spawn_mesh(
        seg,
        &a.mesh_train_body,
        &a.mat_train_body,
        Transform::from_xyz(x, y, z).with_rotation(rot),
    );
    spawn_mesh(
        seg,
        &a.mesh_train_roof,
        &a.mat_rusted_steel,
        Transform::from_xyz(x, y + 1.45, z).with_rotation(rot),
    );

    // Window cutouts with murky transit glass
    for win_z in [-4.0, 0.0, 4.0] {
        spawn_mesh(
            seg,
            &a.mesh_train_window,
            &a.mat_train_window,
            Transform::from_xyz(x + inward * 1.68, y + 0.25, z + win_z).with_rotation(rot),
        );
    }

    // Broken sliding doors jammed ajar
    for door_z in [-5.8, 5.8] {
        spawn_mesh(
            seg,
            &a.mesh_train_door,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + inward * 1.69, y - 0.40, z + door_z).with_rotation(rot),
        );
    }
}

// -------------------------------------------------------------
// CHAPTER 5 — DEEP-LINE APPROACH (7650m–8000m)
// -------------------------------------------------------------

fn spawn_deep_line_approach_segment(
    seg: &mut ChildBuilder,
    a: &FloodedMetroAssets,
    variant: u8,
    landmark: FloodedMetroLandmark,
) {
    spawn_tunnel_shell(seg, a, 9.0, 8.6, false);
    // Double reinforced concrete ribs for fortified restricted approach
    spawn_concrete_arch_ribs(seg, a, 8.6, &[-14.0, -7.0, 7.0, 14.0]);
    spawn_ceiling_infrastructure(seg, a, variant, false);
    spawn_pipe_runs(seg, a, variant);
    spawn_water_elements(seg, a, -1.0, false);

    // Sealed blast hatches with pressure locking lugs
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

    // High-security warning signage and emergency red beacons
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
            // Strange ECHO diagnostic relay pods embedded in alcoves
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

    if landmark == FloodedMetroLandmark::DeepLineBulkheadPortal {
        spawn_landmark_deep_bulkhead_portal(seg, a);
    }
}

fn spawn_landmark_deep_bulkhead_portal(seg: &mut ChildBuilder, a: &FloodedMetroAssets) {
    // Massive Reinforced Blast Bulkhead Vault Portal Framing the Deep Line
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

    // Circular ECHO waveguide containment collar in the upper portal arch
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
        // Each chapter must contain exactly one hero landmark chunk
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

        // Ordinary segments must return FloodedMetroLandmark::None
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
}
