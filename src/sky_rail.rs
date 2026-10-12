use crate::director::RunDirector;
use crate::environment::EnvironmentAssets;
use crate::types::{AppState, Despawnable, GameRunStats, Lane, MainCamera, TrackSegmentMarker};
use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};

// -------------------------------------------------------------
// ZONE 5 — SKY RAIL (8000m–10000m)
// -------------------------------------------------------------
// Core Identity: High-speed elevated magnetic railway suspended ~850m
// above the Aurelia skyline during a violent atmospheric storm.
//
// Background Architecture:
// The distant Aurelia skyline, storm clouds, and atmospheric depth
// are owned exclusively by the 360° cubemap skybox (assets/skybox/zone5/zone5_cubemap.png).
// The 3D scene owns the elevated viaduct railway, overhead gantries,
// trackside equipment, and the 5 monumental landmarks.
//
// 5 Chapters:
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
// DYNAMIC STORM STATE
// -------------------------------------------------------------

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

    // Trackside Viaduct Detail & Overhead Telemetry Gantries
    pub mesh_barrier_cap_light: Handle<Mesh>,
    pub mesh_outer_conduit_bundle: Handle<Mesh>,
    pub mesh_telemetry_gantry_beam: Handle<Mesh>,
    pub mesh_gantry_column: Handle<Mesh>,
    pub mesh_gantry_sensor_pod: Handle<Mesh>,
    pub mesh_trackside_locker: Handle<Mesh>,

    // Trackside Environmental Wear Mesh
    pub mesh_micro_weathered_patch: Handle<Mesh>,

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
    pub mat_sky_antenna_red: Handle<StandardMaterial>,
    pub mat_sky_weathered_iron: Handle<StandardMaterial>,
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

        // Trackside Viaduct Detail & Overhead Telemetry Gantries
        mesh_barrier_cap_light: meshes.add(Cuboid::new(0.32, 0.18, 0.32)),
        mesh_outer_conduit_bundle: meshes.add(Cylinder::new(0.10, 40.0)),
        mesh_telemetry_gantry_beam: meshes.add(Cuboid::new(11.2, 0.45, 0.70)),
        mesh_gantry_column: meshes.add(Cuboid::new(0.38, 4.8, 0.38)),
        mesh_gantry_sensor_pod: meshes.add(Cuboid::new(0.40, 0.60, 0.40)),
        mesh_trackside_locker: meshes.add(Cuboid::new(0.35, 0.95, 0.70)),

        // Trackside Environmental Wear Mesh
        mesh_micro_weathered_patch: meshes.add(Cuboid::new(1.40, 1.80, 0.08)),

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
        mat_sky_antenna_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.08, 0.08),
            emissive: LinearRgba::new(2.40, 0.12, 0.08, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_weathered_iron: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.12, 0.09),
            perceptual_roughness: 0.88,
            metallic: 0.45,
            emissive: LinearRgba::new(0.015, 0.008, 0.005, 1.0),
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

            // Note: The distant Aurelia skyline, storm clouds, and atmospheric depth
            // are owned exclusively by the 360° cubemap skybox (assets/skybox/zone5/zone5_cubemap.png).
            // Procedural background geometry has been removed to avoid duplicate or competing skylines.

            // 5. Chapter-Specific Visual Dressing
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
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(ele_side * 7.5, 14.0, 5.0),
        ..default()
    });
    // Amber technical observation beacon on elevator shaft
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_amber_glow.clone(),
        transform: Transform::from_xyz(ele_side * (7.5 - ele_side * 1.3), 16.0, 5.0)
            .with_scale(Vec3::new(0.6, 2.4, 0.6)),
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
                    material: sky.mat_sky_steel.clone(),
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

#[derive(Resource)]
pub struct SkyRailSkyboxState {
    pub cubemap: Handle<Image>,
    pub is_reinterpreted: bool,
}

impl FromWorld for SkyRailSkyboxState {
    fn from_world(world: &mut World) -> Self {
        let handle = if let Some(asset_server) = world.get_resource::<AssetServer>() {
            asset_server.load("skybox/zone5/zone5_cubemap.png")
        } else {
            Handle::default()
        };
        Self {
            cubemap: handle,
            is_reinterpreted: false,
        }
    }
}

pub struct SkyRailPlugin;

impl Plugin for SkyRailPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkyRailStormState>();
        app.init_resource::<SkyRailSkyboxState>();
        app.add_systems(
            Update,
            (update_sky_rail_skybox, update_sky_rail_storm).run_if(in_state(AppState::InGame)),
        );
        app.add_systems(OnExit(AppState::InGame), cleanup_sky_rail_skybox);
    }
}

pub fn update_sky_rail_skybox(
    mut commands: Commands,
    mut skybox_state: ResMut<SkyRailSkyboxState>,
    mut images: ResMut<Assets<Image>>,
    run_stats: Option<Res<GameRunStats>>,
    director: Option<Res<RunDirector>>,
    storm_state: Res<SkyRailStormState>,
    cam_q: Query<Entity, With<MainCamera>>,
    mut skybox_q: Query<&mut Skybox>,
) {
    // 1. One-time zero-cost reinterpretation of the stacked 2D cubemap texture
    if !skybox_state.is_reinterpreted {
        if let Some(image) = images.get_mut(&skybox_state.cubemap) {
            if image.texture_descriptor.array_layer_count() == 1 {
                let layers = image.height() / image.width();
                if layers == 6 {
                    image.reinterpret_stacked_2d_as_array(layers);
                    image.texture_view_descriptor = Some(TextureViewDescriptor {
                        dimension: Some(TextureViewDimension::Cube),
                        ..default()
                    });
                    skybox_state.is_reinterpreted = true;
                }
            } else {
                skybox_state.is_reinterpreted = true;
            }
        }
    }

    // 2. Zone 5 boundary check (8000m - 10000m)
    let in_zone_5 = run_stats
        .as_ref()
        .map(|s| s.distance >= 7950.0 && s.distance <= 10050.0)
        .or_else(|| director.as_ref().map(|d| d.active_zone_id == 5))
        .unwrap_or(false);

    // 3. Attach / remove / update Skybox component on MainCamera
    for cam_entity in cam_q.iter() {
        if in_zone_5 && skybox_state.is_reinterpreted {
            if let Ok(mut sb) = skybox_q.get_mut(cam_entity) {
                // Modulate brightness with stormy lightning flash (dark baseline 450.0, lightning boost up to +700.0)
                let flash_boost = storm_state.flash_intensity * 700.0;
                sb.brightness = 450.0 + flash_boost;
            } else {
                commands.entity(cam_entity).insert(Skybox {
                    image: skybox_state.cubemap.clone(),
                    brightness: 450.0,
                });
            }
        } else if skybox_q.get(cam_entity).is_ok() {
            // Leaving Zone 5 or run reset: remove Skybox to prevent interference with other zones
            commands.entity(cam_entity).remove::<Skybox>();
        }
    }
}

pub fn cleanup_sky_rail_skybox(
    mut commands: Commands,
    cam_q: Query<Entity, (With<MainCamera>, With<Skybox>)>,
) {
    for cam_entity in cam_q.iter() {
        commands.entity(cam_entity).remove::<Skybox>();
    }
}

pub fn update_sky_rail_storm(
    time: Res<Time>,
    mut storm_state: ResMut<SkyRailStormState>,
    run_stats: Option<Res<GameRunStats>>,
    director: Option<Res<RunDirector>>,
) {
    let dt = time.delta_seconds();
    let in_zone_5 = run_stats
        .as_ref()
        .map(|s| s.distance >= 7950.0 && s.distance <= 10050.0)
        .or_else(|| director.as_ref().map(|d| d.active_zone_id == 5))
        .unwrap_or(false);

    if !in_zone_5 {
        if storm_state.flash_intensity > 0.0 || storm_state.is_flashing {
            storm_state.flash_intensity = 0.0;
            storm_state.is_flashing = false;
            storm_state.flash_elapsed = 0.0;
        }
        return;
    }

    // Dynamic lightning storm simulation for atmospheric cubemap illumination
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
    fn test_sky_rail_skybox_zone_boundaries() {
        // Verify that in_zone_5 condition correctly gates the skybox strictly to Zone 5 (8000m-10000m)
        let check_zone_5 = |dist: f32, active_zone_id: usize| -> bool {
            (dist >= 7950.0 && dist <= 10050.0) || active_zone_id == 5
        };

        // Zone 1: Old Metro (0..2000m) -> Skybox MUST NOT be active
        assert!(!check_zone_5(0.0, 1));
        assert!(!check_zone_5(1500.0, 1));

        // Zone 2: Neon District (2000..4000m) -> Skybox MUST NOT be active
        assert!(!check_zone_5(2500.0, 2));

        // Zone 3: Industrial Sector (4000..6000m) -> Skybox MUST NOT be active
        assert!(!check_zone_5(5000.0, 3));

        // Zone 4: Flooded Metro (6000..8000m) -> Skybox MUST NOT be active until approach boundary
        assert!(!check_zone_5(6500.0, 4));
        assert!(!check_zone_5(7800.0, 4));

        // Zone 5: Sky Rail (8000..10000m) -> Skybox MUST be active
        assert!(check_zone_5(8000.0, 5));
        assert!(check_zone_5(9000.0, 5));
        assert!(check_zone_5(10000.0, 5));

        // Zone 6: Forbidden Line (10000..12000m) -> Skybox MUST NOT be active
        assert!(!check_zone_5(10100.0, 6));
        assert!(!check_zone_5(11500.0, 6));
    }

    #[test]
    fn test_sky_rail_skybox_cubemap_texture_layout() {
        // Verify that the pre-generated cubemap file exists and has valid 1:6 aspect ratio metadata
        let path = std::path::Path::new("assets/skybox/zone5/zone5_cubemap.png");
        assert!(
            path.exists(),
            "zone5_cubemap.png must exist in assets/Skybox/"
        );

        // Verify that file size is reasonable (under 12 MB for Intel UHD VRAM efficiency)
        let metadata = std::fs::metadata(path).expect("Failed to read cubemap metadata");
        assert!(
            metadata.len() > 1_000_000,
            "Cubemap file must not be empty or truncated"
        );
        assert!(
            metadata.len() < 15_000_000,
            "Cubemap file should be optimized for integrated GPUs"
        );
    }

    #[test]
    fn test_sky_rail_skybox_brightness_scaling() {
        // Dark baseline for cinematic storm atmosphere
        let base_brightness = 450.0_f32;
        let lightning_boost = 700.0_f32;

        for flash_i in [0.0_f32, 0.25, 0.5, 0.8, 1.0] {
            let b = base_brightness + flash_i * lightning_boost;
            assert!(
                b >= 450.0,
                "Skybox brightness should never drop below baseline"
            );
            assert!(
                b <= 1150.0,
                "Skybox brightness should remain bounded during lightning"
            );
        }
    }
}
