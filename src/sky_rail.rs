use crate::environment::EnvironmentAssets;
use crate::types::{Despawnable, Lane, TrackSegmentMarker};
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
    pub mesh_sky_signal_mast: Handle<Mesh>,

    // Architectural & Atmospheric Prop Meshes
    pub mesh_sky_catwalk: Handle<Mesh>,
    pub mesh_relay_spire: Handle<Mesh>,
    pub mesh_cable_bundle_elevated: Handle<Mesh>,
    pub mesh_hazard_reflector: Handle<Mesh>,

    // Skyline Megastructure Meshes (Aurelia dark skyline)
    pub mesh_tower_colossus: Handle<Mesh>,
    pub mesh_tower_obelisk: Handle<Mesh>,
    pub mesh_tower_slab: Handle<Mesh>,
    pub mesh_tower_stepped_crown: Handle<Mesh>,
    pub mesh_tower_antenna_spire: Handle<Mesh>,
    pub mesh_tower_window_slit_v: Handle<Mesh>,
    pub mesh_tower_window_slit_h: Handle<Mesh>,
    pub mesh_sky_bridge_span: Handle<Mesh>,

    // Cloud Abyss & Atmospheric Depth Meshes (850m Altitude)
    pub mesh_cloud_deck_vast: Handle<Mesh>,
    pub mesh_cloud_strata_low: Handle<Mesh>,
    pub mesh_cloud_ceiling_storm: Handle<Mesh>,

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

    // Skyline & Cloud Abyss Materials (90% darkness, 10% light)
    pub mat_sky_tower_dark: Handle<StandardMaterial>,
    pub mat_sky_tower_distant: Handle<StandardMaterial>,
    pub mat_sky_window_dim: Handle<StandardMaterial>,
    pub mat_sky_window_amber_dim: Handle<StandardMaterial>,
    pub mat_sky_antenna_red: Handle<StandardMaterial>,
    pub mat_sky_cloud_abyss: Handle<StandardMaterial>,
    pub mat_sky_cloud_ceiling: Handle<StandardMaterial>,
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
        mesh_sky_signal_mast: meshes.add(Cuboid::new(0.26, 4.4, 0.26)),

        // Supporting props
        mesh_sky_catwalk: meshes.add(Cuboid::new(1.10, 0.12, 12.0)),
        mesh_relay_spire: meshes.add(Cylinder::new(0.18, 6.0)),
        mesh_cable_bundle_elevated: meshes.add(Cylinder::new(0.08, 40.0)),
        mesh_hazard_reflector: meshes.add(Cuboid::new(0.15, 0.25, 0.05)),

        // Skyline Megastructure Meshes (Low-poly brutalist silhouettes for integrated GPUs)
        mesh_tower_colossus: meshes.add(Cuboid::new(34.0, 190.0, 36.0)),
        mesh_tower_obelisk: meshes.add(Cuboid::new(18.0, 220.0, 18.0)),
        mesh_tower_slab: meshes.add(Cuboid::new(52.0, 150.0, 26.0)),
        mesh_tower_stepped_crown: meshes.add(Cuboid::new(22.0, 32.0, 24.0)),
        mesh_tower_antenna_spire: meshes.add(Cylinder::new(0.35, 38.0)),
        mesh_tower_window_slit_v: meshes.add(Cuboid::new(0.5, 36.0, 10.0)),
        mesh_tower_window_slit_h: meshes.add(Cuboid::new(18.0, 0.6, 0.5)),
        mesh_sky_bridge_span: meshes.add(Cuboid::new(76.0, 3.6, 4.6)),

        // Cloud Abyss & Atmospheric Depth Meshes
        mesh_cloud_deck_vast: meshes.add(Cuboid::new(380.0, 12.0, 40.0)),
        mesh_cloud_strata_low: meshes.add(Cuboid::new(440.0, 14.0, 40.0)),
        mesh_cloud_ceiling_storm: meshes.add(Cuboid::new(360.0, 10.0, 40.0)),

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

        // Skyline & Cloud Abyss Materials (90% darkness, 10% light)
        mat_sky_tower_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.045, 0.055, 0.075),
            metallic: 0.85,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_sky_tower_distant: materials.add(StandardMaterial {
            base_color: Color::srgb(0.025, 0.035, 0.05),
            metallic: 0.70,
            perceptual_roughness: 0.55,
            ..default()
        }),
        mat_sky_window_dim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.18, 0.26),
            emissive: LinearRgba::new(0.06, 0.28, 0.42, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_sky_window_amber_dim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.14, 0.04),
            emissive: LinearRgba::new(0.45, 0.22, 0.03, 1.0),
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_sky_antenna_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.06, 0.06),
            emissive: LinearRgba::new(1.85, 0.08, 0.05, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_cloud_abyss: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.028, 0.042),
            perceptual_roughness: 0.96,
            metallic: 0.0,
            ..default()
        }),
        mat_sky_cloud_ceiling: materials.add(StandardMaterial {
            base_color: Color::srgb(0.014, 0.02, 0.03),
            perceptual_roughness: 0.98,
            metallic: 0.0,
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

            // 4. Underside Support Pylons into 850m Abyss
            spawn_abyss_pylons(seg, sky);

            // 5. Cloud Abyss Ocean & Storm Cloud Ceiling (850m Altitude)
            spawn_cloud_abyss(seg, sky);

            // 6. Aurelia Megatower Skyline Silhouettes (Dark Cyberpunk Megastructures)
            spawn_skyline_megatowers(seg, sky, distance);

            // 7. Chapter-Specific Visual Dressing
            match chapter {
                SkyRailChapter::SkyRailEmergence => {
                    spawn_emergence_dressing(seg, sky, variant);
                }
                SkyRailChapter::UpperAurelia => {
                    spawn_upper_aurelia_dressing(seg, sky, variant);
                }
                SkyRailChapter::StormCorridor => {
                    spawn_storm_corridor_dressing(seg, sky, variant);
                }
                SkyRailChapter::BrokenSkyway => {
                    spawn_broken_skyway_dressing(seg, sky, variant);
                }
                SkyRailChapter::EchoApproach => {
                    spawn_echo_approach_dressing(seg, sky, variant);
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

            // Amber warning luminaire on top of each post
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_amber_glow.clone(),
                transform: Transform::from_xyz(x, 1.45, z),
                ..default()
            });
        }

        // Suspended high-voltage cable runs outside the barriers
        seg.spawn(PbrBundle {
            mesh: sky.mesh_cable_bundle_elevated.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(x + side * 0.35, 0.40, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
}

fn spawn_abyss_pylons(seg: &mut ChildBuilder, sky: &SkyRailAssets) {
    // Massive support pylon plunging 80m below track into the cloud deck (at center z = 0)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_pylon_massive.clone(),
        material: sky.mat_sky_pylon.clone(),
        transform: Transform::from_xyz(0.0, -42.0, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// CLOUD ABYSS & STORMY ATMOSPHERE (850M ALTITUDE)
// -------------------------------------------------------------

fn spawn_cloud_abyss(seg: &mut ChildBuilder, sky: &SkyRailAssets) {
    // 1. Primary Lower Cloud Deck (churning storm ocean beneath track at y = -42.0m)
    // Support pylons plunge through this deck, establishing the 850m altitude
    seg.spawn(PbrBundle {
        mesh: sky.mesh_cloud_deck_vast.clone(),
        material: sky.mat_sky_cloud_abyss.clone(),
        transform: Transform::from_xyz(0.0, -42.0, 0.0),
        ..default()
    });

    // 2. Secondary Deeper Strata Haze (y = -58.0m, swallowing invisible city below)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_cloud_strata_low.clone(),
        material: sky.mat_sky_cloud_abyss.clone(),
        transform: Transform::from_xyz(0.0, -58.0, 0.0),
        ..default()
    });

    // 3. Heavy Overhead Storm Cloud Ceiling (oppressive dark canopy at y = 56.0m)
    seg.spawn(PbrBundle {
        mesh: sky.mesh_cloud_ceiling_storm.clone(),
        material: sky.mat_sky_cloud_ceiling.clone(),
        transform: Transform::from_xyz(0.0, 56.0, 0.0),
        ..default()
    });
}

// -------------------------------------------------------------
// DARK CITY SKYLINE (AURELIA MEGATOWERS & MONOLITHS)
// -------------------------------------------------------------

fn spawn_skyline_megatowers(seg: &mut ChildBuilder, sky: &SkyRailAssets, distance: f32) {
    let seg_idx = (distance / 40.0).floor() as i32;
    let h1 = ((seg_idx.wrapping_mul(19) + 5).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(37) + 13).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(59) + 23).rem_euclid(100)) as f32 / 100.0;
    let h4 = ((seg_idx.wrapping_mul(83) + 31).rem_euclid(100)) as f32 / 100.0;

    // -------------------------------------------------------------
    // LAYER 1: LEFT FLANK MIDGROUND MEGATOWERS (|X| in [42.0, 68.0]m)
    // -------------------------------------------------------------
    let left_x = -44.0 - h1 * 22.0;
    let left_z = -12.0 + h2 * 24.0;
    let left_type = (seg_idx.rem_euclid(3)) as usize;

    match left_type {
        0 => {
            // Monolithic Slab rising from cloud abyss into upper storm sky
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_slab.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(left_x, 25.0, left_z),
                ..default()
            });
            // Horizontal dim window slits on inward face
            if h2 > 0.25 {
                let win_mat = if h1 > 0.5 {
                    sky.mat_sky_window_dim.clone()
                } else {
                    sky.mat_sky_window_amber_dim.clone()
                };
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_h.clone(),
                    material: win_mat,
                    transform: Transform::from_xyz(left_x + 25.0, 32.0, left_z),
                    ..default()
                });
            }
            // Rooftop pinnacle warning beacon
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_antenna_spire.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(left_x, 100.0 + 19.0, left_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(left_x, 100.0 + 38.0, left_z),
                ..default()
            });
        }
        1 => {
            // Colossus Block with stepped crown
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(left_x, 35.0, left_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_stepped_crown.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(left_x, 35.0 + 95.0 + 16.0, left_z),
                ..default()
            });
            // Vertical dim window slit
            if h2 > 0.20 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_v.clone(),
                    material: sky.mat_sky_window_dim.clone(),
                    transform: Transform::from_xyz(left_x + 16.0, 42.0, left_z),
                    ..default()
                });
            }
            // Pinnacle beacon atop crown
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(left_x, 35.0 + 95.0 + 32.0 + 1.0, left_z),
                ..default()
            });
        }
        _ => {
            // Tall needle Obelisk
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_obelisk.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(left_x, 45.0, left_z),
                ..default()
            });
            // Spire beacon
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_antenna_spire.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(left_x, 45.0 + 110.0 + 19.0, left_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(left_x, 45.0 + 110.0 + 38.0, left_z),
                ..default()
            });
        }
    }

    // -------------------------------------------------------------
    // LAYER 2: RIGHT FLANK MIDGROUND MEGATOWERS (|X| in [44.0, 72.0]m)
    // -------------------------------------------------------------
    let right_x = 46.0 + h3 * 22.0;
    let right_z = 10.0 - h4 * 24.0;
    let right_type = ((seg_idx + 1).rem_euclid(3)) as usize;

    match right_type {
        0 => {
            // Colossus Block with vertical slit
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_colossus.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(right_x, 30.0, right_z),
                ..default()
            });
            if h4 > 0.25 {
                let win_mat = if h3 > 0.5 {
                    sky.mat_sky_window_amber_dim.clone()
                } else {
                    sky.mat_sky_window_dim.clone()
                };
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_v.clone(),
                    material: win_mat,
                    transform: Transform::from_xyz(right_x - 16.0, 36.0, right_z),
                    ..default()
                });
            }
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_antenna_spire.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(right_x, 30.0 + 95.0 + 19.0, right_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(right_x, 30.0 + 95.0 + 38.0, right_z),
                ..default()
            });
        }
        1 => {
            // Obelisk Spire
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_obelisk.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(right_x, 50.0, right_z),
                ..default()
            });
            if h3 > 0.30 {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_tower_window_slit_v.clone(),
                    material: sky.mat_sky_window_dim.clone(),
                    transform: Transform::from_xyz(right_x - 8.5, 52.0, right_z),
                    ..default()
                });
            }
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(right_x, 50.0 + 110.0 + 1.0, right_z),
                ..default()
            });
        }
        _ => {
            // Monolithic Slab
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_slab.clone(),
                material: sky.mat_sky_tower_dark.clone(),
                transform: Transform::from_xyz(right_x, 22.0, right_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_tower_antenna_spire.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(right_x, 22.0 + 75.0 + 19.0, right_z),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: sky.mesh_hazard_reflector.clone(),
                material: sky.mat_sky_antenna_red.clone(),
                transform: Transform::from_xyz(right_x, 22.0 + 75.0 + 38.0, right_z),
                ..default()
            });
        }
    }

    // -------------------------------------------------------------
    // LAYER 3: FAR BACKGROUND MONOLITHS & CITADELS (|X| in [95.0, 155.0]m)
    // -------------------------------------------------------------
    let (far_x, far_z) = if seg_idx.rem_euclid(2) == 0 {
        (-105.0 - h2 * 45.0, -15.0 + h3 * 30.0)
    } else {
        (108.0 + h4 * 42.0, 12.0 - h1 * 28.0)
    };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_tower_slab.clone(),
        material: sky.mat_sky_tower_distant.clone(),
        transform: Transform::from_xyz(far_x, 40.0, far_z).with_scale(Vec3::new(1.8, 1.6, 1.8)),
        ..default()
    });
    // Far beacon in the storm
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_antenna_red.clone(),
        transform: Transform::from_xyz(far_x, 40.0 + 120.0, far_z)
            .with_scale(Vec3::new(2.5, 2.5, 2.5)),
        ..default()
    });

    // -------------------------------------------------------------
    // LAYER 4: TRANSVERSE DISTANT HIGH-ALTITUDE SKYBRIDGES
    // Spanning between towers at high elevation far outside player bounds
    // -------------------------------------------------------------
    if seg_idx.rem_euclid(3) == 1 {
        let bridge_side = if h1 > 0.5 { -1.0 } else { 1.0 };
        let bridge_center_x = bridge_side * (left_x.abs() + 28.0);
        let bridge_y = 38.0 + h2 * 18.0;
        let bridge_z = -6.0 + h3 * 12.0;
        seg.spawn(PbrBundle {
            mesh: sky.mesh_sky_bridge_span.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(bridge_center_x, bridge_y, bridge_z),
            ..default()
        });
    }
}

// -------------------------------------------------------------
// CHAPTER-SPECIFIC DRESSING
// -------------------------------------------------------------

fn spawn_emergence_dressing(seg: &mut ChildBuilder, sky: &SkyRailAssets, variant: u8) {
    // Chapter 1: Transitional emergence infrastructure from underground facility
    let signal_side = if variant == 1 { -1.0 } else { 1.0 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_sky_signal_mast.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(signal_side * 4.4, 2.2, -6.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: sky.mesh_hazard_reflector.clone(),
        material: sky.mat_sky_amber_glow.clone(),
        transform: Transform::from_xyz(signal_side * 4.4, 4.3, -6.0),
        ..default()
    });
}

fn spawn_upper_aurelia_dressing(seg: &mut ChildBuilder, sky: &SkyRailAssets, variant: u8) {
    // Chapter 2: High skyline maintenance catwalks and antennas
    let cat_side = if variant == 0 { -1.0 } else { 1.0 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_sky_catwalk.clone(),
        material: sky.mat_sky_grate.clone(),
        transform: Transform::from_xyz(cat_side * 5.2, 0.35, 4.0),
        ..default()
    });
}

fn spawn_storm_corridor_dressing(seg: &mut ChildBuilder, sky: &SkyRailAssets, _variant: u8) {
    // Chapter 3: Reinforced storm spires & wind sensors
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_relay_spire.clone(),
            material: sky.mat_sky_storm_composite.clone(),
            transform: Transform::from_xyz(side * 4.65, 3.2, 10.0),
            ..default()
        });
    }
}

fn spawn_broken_skyway_dressing(seg: &mut ChildBuilder, sky: &SkyRailAssets, variant: u8) {
    // Chapter 4: Damaged storm barriers and maintenance emergency signals
    let mast_x = if variant == 2 { -4.4 } else { 4.4 };
    seg.spawn(PbrBundle {
        mesh: sky.mesh_sky_signal_mast.clone(),
        material: sky.mat_sky_steel.clone(),
        transform: Transform::from_xyz(mast_x, 2.0, -10.0)
            .with_rotation(Quat::from_rotation_z(0.18)),
        ..default()
    });
}

fn spawn_echo_approach_dressing(seg: &mut ChildBuilder, sky: &SkyRailAssets, _variant: u8) {
    // Chapter 5: Clean, geometric ECHO conduit arrays and cyan telemetry nodes
    for side in [-1.0, 1.0] {
        seg.spawn(PbrBundle {
            mesh: sky.mesh_relay_spire.clone(),
            material: sky.mat_sky_steel.clone(),
            transform: Transform::from_xyz(side * 4.55, 3.0, 0.0),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: sky.mesh_hazard_reflector.clone(),
            material: sky.mat_sky_cyan_glow.clone(),
            transform: Transform::from_xyz(side * 4.55, 6.0, 0.0),
            ..default()
        });
    }
}

fn spawn_sky_rail_landmark(seg: &mut ChildBuilder, sky: &SkyRailAssets, landmark: SkyRailLandmark) {
    match landmark {
        SkyRailLandmark::SkyRailGate => {
            // Tunnel exit portal framing stanchions at margins (|x| = 5.2m, leaving corridor open)
            for side in [-1.0, 1.0] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_sky_signal_mast.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 5.2, 3.5, 0.0)
                        .with_scale(Vec3::new(1.8, 2.0, 1.8)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_amber_glow.clone(),
                    transform: Transform::from_xyz(side * 5.2, 7.5, 0.0),
                    ..default()
                });
            }
        }
        SkyRailLandmark::AureliaCrownStation => {
            // Suspended station framing masts at |x| = 5.6m
            for side in [-1.0, 1.0] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_sky_catwalk.clone(),
                    material: sky.mat_sky_grate.clone(),
                    transform: Transform::from_xyz(side * 5.5, 1.2, 0.0),
                    ..default()
                });
            }
        }
        SkyRailLandmark::StormSpineBridge => {
            // High suspension cable anchorage masts at margins
            for side in [-1.0, 1.0] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_relay_spire.clone(),
                    material: sky.mat_sky_storm_composite.clone(),
                    transform: Transform::from_xyz(side * 5.4, 4.5, 0.0)
                        .with_scale(Vec3::new(2.0, 1.8, 2.0)),
                    ..default()
                });
            }
        }
        SkyRailLandmark::CollapsedSkywaySpan => {
            // Damaged maintenance cat-tracks outside the running corridor
            seg.spawn(PbrBundle {
                mesh: sky.mesh_sky_catwalk.clone(),
                material: sky.mat_sky_steel.clone(),
                transform: Transform::from_xyz(-5.6, 0.6, 0.0)
                    .with_rotation(Quat::from_rotation_z(0.35)),
                ..default()
            });
        }
        SkyRailLandmark::EchoRelayArray => {
            // Twin colossal cyan beacons towering at outer margins
            for side in [-1.0, 1.0] {
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_relay_spire.clone(),
                    material: sky.mat_sky_steel.clone(),
                    transform: Transform::from_xyz(side * 5.8, 5.0, 0.0)
                        .with_scale(Vec3::new(2.5, 2.2, 2.5)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: sky.mesh_hazard_reflector.clone(),
                    material: sky.mat_sky_cyan_glow.clone(),
                    transform: Transform::from_xyz(side * 5.8, 11.5, 0.0)
                        .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                    ..default()
                });
            }
        }
        SkyRailLandmark::None => {}
    }
}

// -------------------------------------------------------------
// PLUGIN DEFINITION
// -------------------------------------------------------------

pub struct SkyRailPlugin;

impl Plugin for SkyRailPlugin {
    fn build(&self, _app: &mut App) {
        // Systems will be registered as needed for dynamic storm effects
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
}
