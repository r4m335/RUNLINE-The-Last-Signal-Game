use crate::environment::EnvironmentAssets;
use crate::types::*;
use crate::zones::normalized_zone_progress;
use bevy::prelude::*;

/// The five authored visual chapters across Zone 3 (4,000m–6,000m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrialChapter {
    IndustrialEntry,
    FreightYard,
    ProcessingPlant,
    VeyronHeavyWorks,
    IndustrialTransition,
}

/// Three deterministic variants per chapter keep rolling chunks from repeating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrialVariant {
    EntryPipeGallery,
    EntryServiceBlock,
    EntryWarningLine,
    FreightContainerStack,
    FreightCraneBay,
    FreightLoadingPlatform,
    ProcessingPressureRow,
    ProcessingSteamManifold,
    ProcessingCatwalk,
    HeavyWorksConveyor,
    HeavyWorksVeyronGate,
    HeavyWorksCoolingArray,
    TransitionLeakingMain,
    TransitionDamagedWorks,
    TransitionFloodApproach,
}

/// A small number of authored landmarks give Zone 3 memorable spatial beats
/// without turning every rolling chunk into the same set piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrialLandmark {
    None,
    IndustrialEntryGate,
    FreightGantry,
    ProcessingTower,
    VeyronHeavyWorks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndustrialTrackProfile {
    pub lane_markings: bool,
    pub service_conduits: bool,
    pub distressed: bool,
}

#[derive(Resource)]
pub struct IndustrialSectorAssets {
    pub mesh_factory_wall: Handle<Mesh>,
    pub mesh_pipe: Handle<Mesh>,
    pub mesh_large_pipe: Handle<Mesh>,
    pub mesh_container: Handle<Mesh>,
    pub mesh_container_rib: Handle<Mesh>,
    pub mesh_crane_column: Handle<Mesh>,
    pub mesh_crane_beam: Handle<Mesh>,
    pub mesh_crane_hook: Handle<Mesh>,
    pub mesh_processing_tower: Handle<Mesh>,
    pub mesh_pressure_vessel: Handle<Mesh>,
    pub mesh_catwalk: Handle<Mesh>,
    pub mesh_conveyor: Handle<Mesh>,
    pub mesh_cooling_tower: Handle<Mesh>,
    pub mesh_stack: Handle<Mesh>,
    pub mesh_sign: Handle<Mesh>,
    pub mesh_warning_light: Handle<Mesh>,
    pub mesh_steam: Handle<Mesh>,
    pub mesh_water_leak: Handle<Mesh>,
    // Reusable Visual Pass 2 modules. These are created once and instanced
    // across authored chunks so detail does not create per-frame allocations.
    pub mesh_facade_panel: Handle<Mesh>,
    pub mesh_structural_column: Handle<Mesh>,
    pub mesh_reinforcement_beam: Handle<Mesh>,
    pub mesh_service_door: Handle<Mesh>,
    pub mesh_loading_bay: Handle<Mesh>,
    pub mesh_vent_panel: Handle<Mesh>,
    pub mesh_platform: Handle<Mesh>,
    pub mesh_railing: Handle<Mesh>,
    pub mesh_ladder: Handle<Mesh>,
    pub mesh_pipe_junction: Handle<Mesh>,
    pub mesh_valve: Handle<Mesh>,
    pub mesh_pressure_gauge: Handle<Mesh>,
    pub mesh_machine: Handle<Mesh>,
    pub mesh_transformer: Handle<Mesh>,
    pub mesh_storage_tank: Handle<Mesh>,
    pub mesh_duct: Handle<Mesh>,
    pub mesh_cooling_unit: Handle<Mesh>,
    pub mesh_conveyor_frame: Handle<Mesh>,
    pub mesh_conveyor_roller: Handle<Mesh>,
    pub mesh_pallet: Handle<Mesh>,
    pub mesh_freight_platform: Handle<Mesh>,
    pub mesh_security_gate: Handle<Mesh>,
    // Deliberate architectural assemblies. These keep complete buildings
    // volumetric without stretching the thin legacy facade mesh into boxes.
    pub mesh_arch_front_wall: Handle<Mesh>,
    pub mesh_arch_side_wall: Handle<Mesh>,
    pub mesh_arch_roof: Handle<Mesh>,
    pub mesh_arch_foundation: Handle<Mesh>,
    pub mesh_arch_frame: Handle<Mesh>,
    pub mesh_arch_window_band: Handle<Mesh>,
    pub mesh_arch_entrance_recess: Handle<Mesh>,
    // Zone 3 heavy-road track modules.
    pub mesh_track_plate: Handle<Mesh>,
    pub mesh_track_seam: Handle<Mesh>,
    pub mesh_track_lane_mark: Handle<Mesh>,
    pub mesh_track_edge: Handle<Mesh>,
    pub mesh_track_conduit: Handle<Mesh>,
    pub mesh_track_grate: Handle<Mesh>,
    pub mat_factory: Handle<StandardMaterial>,
    pub mat_steel: Handle<StandardMaterial>,
    pub mat_rusted_steel: Handle<StandardMaterial>,
    pub mat_pipe: Handle<StandardMaterial>,
    pub mat_hazard: Handle<StandardMaterial>,
    pub mat_amber: Handle<StandardMaterial>,
    pub mat_veyron: Handle<StandardMaterial>,
    pub mat_cooling: Handle<StandardMaterial>,
    pub mat_steam: Handle<StandardMaterial>,
    pub mat_water_leak: Handle<StandardMaterial>,
    pub mat_facade: Handle<StandardMaterial>,
    pub mat_dark_steel: Handle<StandardMaterial>,
    pub mat_track_steel: Handle<StandardMaterial>,
    pub mat_track_seam: Handle<StandardMaterial>,
    pub mat_track_hazard: Handle<StandardMaterial>,
    pub mat_track_conduit: Handle<StandardMaterial>,
    pub mat_track_rust: Handle<StandardMaterial>,
}

pub fn init_industrial_sector_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let assets = IndustrialSectorAssets {
        mesh_factory_wall: meshes.add(Cuboid::new(0.55, 6.5, 38.0)),
        mesh_pipe: meshes.add(Cylinder::new(0.14, 36.0)),
        mesh_large_pipe: meshes.add(Cylinder::new(0.34, 36.0)),
        mesh_container: meshes.add(Cuboid::new(2.7, 1.9, 6.2)),
        mesh_container_rib: meshes.add(Cuboid::new(0.08, 2.0, 6.35)),
        mesh_crane_column: meshes.add(Cuboid::new(0.42, 5.8, 0.42)),
        mesh_crane_beam: meshes.add(Cuboid::new(8.8, 0.32, 0.38)),
        mesh_crane_hook: meshes.add(Cuboid::new(0.16, 1.5, 0.16)),
        mesh_processing_tower: meshes.add(Cylinder::new(1.35, 8.5)),
        mesh_pressure_vessel: meshes.add(Cylinder::new(1.05, 3.8)),
        mesh_catwalk: meshes.add(Cuboid::new(4.4, 0.14, 0.65)),
        mesh_conveyor: meshes.add(Cuboid::new(3.8, 0.25, 18.0)),
        mesh_cooling_tower: meshes.add(Cylinder::new(2.5, 8.0)),
        mesh_stack: meshes.add(Cylinder::new(0.62, 10.0)),
        mesh_sign: meshes.add(Cuboid::new(2.2, 0.72, 0.10)),
        mesh_warning_light: meshes.add(Sphere::new(0.13)),
        mesh_steam: meshes.add(Sphere::new(0.48)),
        mesh_water_leak: meshes.add(Cuboid::new(0.16, 1.8, 0.16)),
        mesh_facade_panel: meshes.add(Cuboid::new(0.18, 3.1, 7.2)),
        mesh_structural_column: meshes.add(Cuboid::new(0.28, 7.0, 0.28)),
        mesh_reinforcement_beam: meshes.add(Cuboid::new(0.22, 0.24, 7.0)),
        mesh_service_door: meshes.add(Cuboid::new(0.10, 1.8, 1.4)),
        mesh_loading_bay: meshes.add(Cuboid::new(0.18, 2.4, 3.8)),
        mesh_vent_panel: meshes.add(Cuboid::new(0.12, 1.2, 1.6)),
        mesh_platform: meshes.add(Cuboid::new(3.4, 0.14, 6.0)),
        mesh_railing: meshes.add(Cuboid::new(0.10, 0.75, 6.0)),
        mesh_ladder: meshes.add(Cuboid::new(0.08, 2.2, 0.08)),
        mesh_pipe_junction: meshes.add(Sphere::new(0.24)),
        mesh_valve: meshes.add(Cylinder::new(0.42, 0.14)),
        mesh_pressure_gauge: meshes.add(Cylinder::new(0.22, 0.08)),
        mesh_machine: meshes.add(Cuboid::new(1.8, 1.5, 2.3)),
        mesh_transformer: meshes.add(Cuboid::new(1.4, 1.7, 1.5)),
        mesh_storage_tank: meshes.add(Cylinder::new(1.6, 3.2)),
        mesh_duct: meshes.add(Cylinder::new(0.7, 6.0)),
        mesh_cooling_unit: meshes.add(Cuboid::new(2.4, 1.1, 3.0)),
        mesh_conveyor_frame: meshes.add(Cuboid::new(4.2, 0.9, 0.18)),
        mesh_conveyor_roller: meshes.add(Cylinder::new(0.10, 3.4)),
        mesh_pallet: meshes.add(Cuboid::new(1.8, 0.25, 1.3)),
        mesh_freight_platform: meshes.add(Cuboid::new(4.0, 0.35, 6.0)),
        mesh_security_gate: meshes.add(Cuboid::new(0.42, 4.8, 0.42)),
        mesh_arch_front_wall: meshes.add(Cuboid::new(0.34, 6.5, 24.0)),
        mesh_arch_side_wall: meshes.add(Cuboid::new(10.0, 6.5, 0.34)),
        mesh_arch_roof: meshes.add(Cuboid::new(10.6, 0.38, 24.8)),
        mesh_arch_foundation: meshes.add(Cuboid::new(10.3, 0.45, 24.2)),
        mesh_arch_frame: meshes.add(Cuboid::new(0.38, 7.0, 0.38)),
        mesh_arch_window_band: meshes.add(Cuboid::new(0.14, 1.0, 5.0)),
        mesh_arch_entrance_recess: meshes.add(Cuboid::new(0.24, 3.1, 4.0)),
        mesh_track_plate: meshes.add(Cuboid::new(2.05, 0.07, 7.5)),
        mesh_track_seam: meshes.add(Cuboid::new(2.05, 0.025, 0.08)),
        mesh_track_lane_mark: meshes.add(Cuboid::new(0.08, 0.025, 2.0)),
        mesh_track_edge: meshes.add(Cuboid::new(0.34, 0.10, 7.5)),
        mesh_track_conduit: meshes.add(Cuboid::new(0.20, 0.18, 7.5)),
        mesh_track_grate: meshes.add(Cuboid::new(0.52, 0.035, 1.2)),
        mat_factory: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.13, 0.14),
            metallic: 0.65,
            perceptual_roughness: 0.72,
            ..default()
        }),
        mat_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.27, 0.28),
            metallic: 0.90,
            perceptual_roughness: 0.34,
            ..default()
        }),
        mat_rusted_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.13, 0.08),
            metallic: 0.65,
            perceptual_roughness: 0.78,
            ..default()
        }),
        mat_pipe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.20, 0.19),
            metallic: 0.85,
            perceptual_roughness: 0.46,
            ..default()
        }),
        mat_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.48, 0.04),
            emissive: LinearRgba::new(0.45, 0.16, 0.01, 1.0),
            metallic: 0.35,
            perceptual_roughness: 0.44,
            ..default()
        }),
        mat_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.55, 0.08),
            emissive: LinearRgba::new(2.8, 0.72, 0.04, 1.0),
            ..default()
        }),
        mat_veyron: materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.18, 0.22),
            emissive: LinearRgba::new(0.02, 0.42, 0.50, 1.0),
            metallic: 0.78,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_cooling: materials.add(StandardMaterial {
            base_color: Color::srgb(0.17, 0.20, 0.20),
            emissive: LinearRgba::new(0.05, 0.18, 0.16, 1.0),
            metallic: 0.72,
            perceptual_roughness: 0.62,
            ..default()
        }),
        mat_steam: materials.add(StandardMaterial {
            base_color: Color::srgba(0.72, 0.78, 0.76, 0.42),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_water_leak: materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.34, 0.40),
            emissive: LinearRgba::new(0.01, 0.18, 0.22, 1.0),
            metallic: 0.25,
            perceptual_roughness: 0.18,
            ..default()
        }),
        mat_facade: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.17, 0.18),
            metallic: 0.72,
            perceptual_roughness: 0.58,
            ..default()
        }),
        mat_dark_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.08, 0.09),
            metallic: 0.88,
            perceptual_roughness: 0.48,
            ..default()
        }),
        mat_track_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.22, 0.23),
            metallic: 0.92,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_track_seam: materials.add(StandardMaterial {
            base_color: Color::srgb(0.045, 0.05, 0.055),
            metallic: 0.72,
            perceptual_roughness: 0.64,
            ..default()
        }),
        mat_track_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.52, 0.06),
            emissive: LinearRgba::new(0.30, 0.12, 0.01, 1.0),
            metallic: 0.40,
            perceptual_roughness: 0.46,
            ..default()
        }),
        mat_track_conduit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.035, 0.16, 0.18),
            emissive: LinearRgba::new(0.01, 0.12, 0.14, 1.0),
            metallic: 0.76,
            perceptual_roughness: 0.42,
            ..default()
        }),
        mat_track_rust: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.12, 0.06),
            metallic: 0.58,
            perceptual_roughness: 0.82,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

pub fn get_industrial_sector_chapter(distance: f32) -> IndustrialChapter {
    let progress = normalized_zone_progress(3, distance);
    if progress < 0.20 {
        IndustrialChapter::IndustrialEntry
    } else if progress < 0.45 {
        IndustrialChapter::FreightYard
    } else if progress < 0.70 {
        IndustrialChapter::ProcessingPlant
    } else if progress < 0.90 {
        IndustrialChapter::VeyronHeavyWorks
    } else {
        IndustrialChapter::IndustrialTransition
    }
}

pub fn get_industrial_sector_variant(distance: f32) -> IndustrialVariant {
    let progress = normalized_zone_progress(3, distance);
    let (start, end, variants) = match get_industrial_sector_chapter(distance) {
        IndustrialChapter::IndustrialEntry => (
            0.0,
            0.20,
            [
                IndustrialVariant::EntryPipeGallery,
                IndustrialVariant::EntryServiceBlock,
                IndustrialVariant::EntryWarningLine,
            ],
        ),
        IndustrialChapter::FreightYard => (
            0.20,
            0.45,
            [
                IndustrialVariant::FreightContainerStack,
                IndustrialVariant::FreightCraneBay,
                IndustrialVariant::FreightLoadingPlatform,
            ],
        ),
        IndustrialChapter::ProcessingPlant => (
            0.45,
            0.70,
            [
                IndustrialVariant::ProcessingPressureRow,
                IndustrialVariant::ProcessingSteamManifold,
                IndustrialVariant::ProcessingCatwalk,
            ],
        ),
        IndustrialChapter::VeyronHeavyWorks => (
            0.70,
            0.90,
            [
                IndustrialVariant::HeavyWorksConveyor,
                IndustrialVariant::HeavyWorksVeyronGate,
                IndustrialVariant::HeavyWorksCoolingArray,
            ],
        ),
        IndustrialChapter::IndustrialTransition => (
            0.90,
            1.0,
            [
                IndustrialVariant::TransitionLeakingMain,
                IndustrialVariant::TransitionDamagedWorks,
                IndustrialVariant::TransitionFloodApproach,
            ],
        ),
    };
    let local = ((progress - start) / (end - start)).clamp(0.0, 0.9999);
    variants[(local * 3.0) as usize]
}

/// Returns the authored landmark for a chunk distance. Windows are expressed
/// as normalized Zone 3 progress so they stay tied to the chapter layout
/// rather than duplicating the global 2,000m zone boundaries.
pub fn get_industrial_sector_landmark(distance: f32) -> IndustrialLandmark {
    let progress = normalized_zone_progress(3, distance);
    let chapter = get_industrial_sector_chapter(distance);
    match chapter {
        IndustrialChapter::IndustrialEntry if (0.085..0.105).contains(&progress) => {
            IndustrialLandmark::IndustrialEntryGate
        }
        IndustrialChapter::FreightYard if (0.325..0.345).contains(&progress) => {
            IndustrialLandmark::FreightGantry
        }
        IndustrialChapter::ProcessingPlant if (0.565..0.585).contains(&progress) => {
            IndustrialLandmark::ProcessingTower
        }
        IndustrialChapter::VeyronHeavyWorks if (0.785..0.805).contains(&progress) => {
            IndustrialLandmark::VeyronHeavyWorks
        }
        _ => IndustrialLandmark::None,
    }
}

pub fn spawn_industrial_sector_segment(
    commands: &mut Commands,
    env: &EnvironmentAssets,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let variant = get_industrial_sector_variant(distance);
    let even = ((z_center.abs() / length).floor() as i32) % 2 == 0;

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            spawn_industrial_track(seg, env, industrial, length, distance);
            spawn_near_industrial_features(seg, industrial, variant, even);
            spawn_mid_industrial_features(seg, industrial, variant, even);
            spawn_far_industrial_features(seg, industrial, variant, even);
            spawn_industrial_signage(seg, industrial, signage, variant, even);
            spawn_industrial_architecture(seg, industrial, variant, even, length);
            spawn_industrial_background_detail(seg, industrial, props, signage, variant, even);
            spawn_industrial_landmark(
                seg,
                industrial,
                get_industrial_sector_landmark(distance),
                even,
            );
        })
        .id()
}

fn spawn_industrial_track(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    industrial: &IndustrialSectorAssets,
    length: f32,
    distance: f32,
) {
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
    for x in [-4.2, 4.2] {
        seg.spawn(PbrBundle {
            mesh: env.mesh_curb_shoulder.clone(),
            material: env.mat_ballast_curb.clone(),
            transform: Transform::from_xyz(x, 0.12, 0.0),
            ..default()
        });
    }
    for i in -(length as i32) / 4..=(length as i32) / 4 {
        let z = i as f32 * 4.0;
        seg.spawn(PbrBundle {
            mesh: env.mesh_sleeper_heavy.clone(),
            material: env.mat_sleeper_steel.clone(),
            transform: Transform::from_xyz(0.0, 0.01, z),
            ..default()
        });
    }
    for lane_idx in -1..=1 {
        let lane_x = Lane::from_index(lane_idx).x_pos();
        for rail_x in [lane_x - 0.48, lane_x + 0.48] {
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
    }

    let track_profile = get_industrial_track_profile(distance);
    let plate_material = if track_profile.distressed {
        industrial.mat_track_rust.clone()
    } else {
        industrial.mat_track_steel.clone()
    };
    for lane_idx in -1..=1 {
        let lane_x = Lane::from_index(lane_idx).x_pos();
        for z in [-15.0, -7.0, 1.0, 9.0, 17.0] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_track_plate.clone(),
                material: plate_material.clone(),
                transform: Transform::from_xyz(lane_x, 0.075, z),
                ..default()
            });
        }
        for z in [-11.25, -3.25, 4.75, 12.75, 19.75] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_track_seam.clone(),
                material: industrial.mat_track_seam.clone(),
                transform: Transform::from_xyz(lane_x, 0.118, z),
                ..default()
            });
        }
    }

    if track_profile.lane_markings {
        for x in [-1.2, 1.2] {
            for z in [-13.0, -5.0, 3.0, 11.0, 19.0] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_track_lane_mark.clone(),
                    material: industrial.mat_track_hazard.clone(),
                    transform: Transform::from_xyz(x, 0.13, z),
                    ..default()
                });
            }
        }
    }

    for x in [-3.72, 3.72] {
        for z in [-15.0, -7.0, 1.0, 9.0, 17.0] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_track_edge.clone(),
                material: if track_profile.distressed {
                    industrial.mat_track_rust.clone()
                } else {
                    industrial.mat_track_steel.clone()
                },
                transform: Transform::from_xyz(x, 0.15, z),
                ..default()
            });
        }
    }

    if track_profile.service_conduits {
        for x in [-3.42, 3.42] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_track_conduit.clone(),
                material: industrial.mat_track_conduit.clone(),
                transform: Transform::from_xyz(x, 0.24, 0.0),
                ..default()
            });
            for z in [-14.0, -2.0, 10.0] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_track_grate.clone(),
                    material: industrial.mat_track_seam.clone(),
                    transform: Transform::from_xyz(x, 0.34, z),
                    ..default()
                });
            }
        }
    }
}

pub fn get_industrial_track_profile(distance: f32) -> IndustrialTrackProfile {
    match get_industrial_sector_chapter(distance) {
        IndustrialChapter::IndustrialEntry => IndustrialTrackProfile {
            lane_markings: false,
            service_conduits: false,
            distressed: false,
        },
        IndustrialChapter::FreightYard => IndustrialTrackProfile {
            lane_markings: true,
            service_conduits: false,
            distressed: false,
        },
        IndustrialChapter::ProcessingPlant => IndustrialTrackProfile {
            lane_markings: false,
            service_conduits: true,
            distressed: false,
        },
        IndustrialChapter::VeyronHeavyWorks => IndustrialTrackProfile {
            lane_markings: true,
            service_conduits: true,
            distressed: false,
        },
        IndustrialChapter::IndustrialTransition => IndustrialTrackProfile {
            lane_markings: true,
            service_conduits: true,
            distressed: true,
        },
    }
}

fn spawn_near_industrial_features(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    variant: IndustrialVariant,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };
    let opposite = -side;
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_factory_wall.clone(),
        material: industrial.mat_factory.clone(),
        transform: Transform::from_xyz(side * 8.0, 3.1, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_large_pipe.clone(),
        material: industrial.mat_pipe.clone(),
        transform: Transform::from_xyz(opposite * 5.0, 2.5, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_pipe.clone(),
        material: industrial.mat_rusted_steel.clone(),
        transform: Transform::from_xyz(opposite * 5.35, 1.35, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: if matches!(variant, IndustrialVariant::HeavyWorksVeyronGate) {
            industrial.mat_veyron.clone()
        } else {
            industrial.mat_hazard.clone()
        },
        transform: Transform::from_xyz(side * 5.8, 2.7, 7.5),
        ..default()
    });
    for &x in &[side * 5.2, side * 5.8] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_warning_light.clone(),
            material: industrial.mat_amber.clone(),
            transform: Transform::from_xyz(x, 1.55, -5.0),
            ..default()
        });
    }
    spawn_facade_modules(seg, industrial, side, variant);
    spawn_pipe_rack_detail(seg, industrial, opposite, variant);
}

fn spawn_mid_industrial_features(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    variant: IndustrialVariant,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };
    match variant {
        IndustrialVariant::FreightContainerStack
        | IndustrialVariant::FreightCraneBay
        | IndustrialVariant::FreightLoadingPlatform => {
            for (x, y, z) in [(side * 6.2, 1.0, -7.0), (side * 6.2, 2.95, 5.0)] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_container.clone(),
                    material: industrial.mat_rusted_steel.clone(),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                });
                for rib_z in [-2.2, 0.0, 2.2] {
                    seg.spawn(PbrBundle {
                        mesh: industrial.mesh_container_rib.clone(),
                        material: industrial.mat_hazard.clone(),
                        transform: Transform::from_xyz(x, y, z + rib_z),
                        ..default()
                    });
                }
            }
            if !matches!(variant, IndustrialVariant::FreightContainerStack) {
                spawn_crane(seg, industrial, -side * 7.0);
            }
        }
        IndustrialVariant::ProcessingPressureRow
        | IndustrialVariant::ProcessingSteamManifold
        | IndustrialVariant::ProcessingCatwalk => {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_processing_tower.clone(),
                material: industrial.mat_steel.clone(),
                transform: Transform::from_xyz(side * 7.0, 4.25, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_pressure_vessel.clone(),
                material: industrial.mat_rusted_steel.clone(),
                transform: Transform::from_xyz(-side * 6.5, 2.25, -5.0)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_large_pipe.clone(),
                material: industrial.mat_pipe.clone(),
                transform: Transform::from_xyz(0.0, 3.8, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_catwalk.clone(),
                material: industrial.mat_steel.clone(),
                transform: Transform::from_xyz(side * 5.0, 3.0, 5.0),
                ..default()
            });
            if !matches!(variant, IndustrialVariant::ProcessingPressureRow) {
                spawn_steam_vent(seg, industrial, -side * 5.5, 1.3, -5.0);
            }
        }
        IndustrialVariant::HeavyWorksConveyor
        | IndustrialVariant::HeavyWorksVeyronGate
        | IndustrialVariant::HeavyWorksCoolingArray => {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_conveyor.clone(),
                material: industrial.mat_steel.clone(),
                transform: Transform::from_xyz(side * 6.0, 2.5, 0.0)
                    .with_rotation(Quat::from_rotation_x(0.08)),
                ..default()
            });
            spawn_crane(seg, industrial, -side * 7.0);
            if matches!(variant, IndustrialVariant::HeavyWorksVeyronGate) {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_sign.clone(),
                    material: industrial.mat_veyron.clone(),
                    transform: Transform::from_xyz(0.0, 4.2, -8.0),
                    ..default()
                });
            }
        }
        IndustrialVariant::TransitionLeakingMain
        | IndustrialVariant::TransitionDamagedWorks
        | IndustrialVariant::TransitionFloodApproach => {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_factory_wall.clone(),
                material: industrial.mat_factory.clone(),
                transform: Transform::from_xyz(-side * 7.8, 2.8, 0.0)
                    .with_rotation(Quat::from_rotation_z(-0.035 * side)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_large_pipe.clone(),
                material: industrial.mat_rusted_steel.clone(),
                transform: Transform::from_xyz(side * 5.3, 2.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                    .with_rotation(Quat::from_rotation_z(0.10 * side)),
                ..default()
            });
            if matches!(variant, IndustrialVariant::TransitionLeakingMain) {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_water_leak.clone(),
                    material: industrial.mat_water_leak.clone(),
                    transform: Transform::from_xyz(side * 4.9, 0.8, -5.0),
                    ..default()
                });
            }
        }
        IndustrialVariant::EntryPipeGallery
        | IndustrialVariant::EntryServiceBlock
        | IndustrialVariant::EntryWarningLine => {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_pressure_vessel.clone(),
                material: industrial.mat_steel.clone(),
                transform: Transform::from_xyz(-side * 6.0, 2.2, -4.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_pipe.clone(),
                material: industrial.mat_pipe.clone(),
                transform: Transform::from_xyz(side * 6.0, 3.0, 5.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        }
    }
    spawn_mid_industrial_detail(seg, industrial, side, variant);
}

fn spawn_far_industrial_features(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    variant: IndustrialVariant,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };
    for x in [side * 14.0, -side * 16.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_stack.clone(),
            material: industrial.mat_factory.clone(),
            transform: Transform::from_xyz(x, 5.0, 2.0),
            ..default()
        });
    }
    if matches!(
        variant,
        IndustrialVariant::ProcessingSteamManifold
            | IndustrialVariant::ProcessingCatwalk
            | IndustrialVariant::HeavyWorksCoolingArray
            | IndustrialVariant::TransitionFloodApproach
    ) {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_cooling_tower.clone(),
            material: industrial.mat_cooling.clone(),
            transform: Transform::from_xyz(-side * 13.0, 4.0, -6.0),
            ..default()
        });
    }
    if matches!(
        variant,
        IndustrialVariant::ProcessingSteamManifold | IndustrialVariant::TransitionLeakingMain
    ) {
        spawn_steam_vent(seg, industrial, side * 12.0, 5.5, 4.0);
    }
    spawn_far_industrial_detail(seg, industrial, side, variant);
}

fn spawn_crane(seg: &mut ChildBuilder, industrial: &IndustrialSectorAssets, x: f32) {
    for z in [-7.0, 7.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_crane_column.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(x, 2.9, z),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_crane_beam.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(x, 5.7, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_crane_hook.clone(),
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(x, 4.6, 0.0),
        ..default()
    });
}

fn spawn_steam_vent(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    x: f32,
    y: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_steam.clone(),
        material: industrial.mat_steam.clone(),
        transform: Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.7, 1.4, 0.7)),
        ..default()
    });
}

fn spawn_industrial_signage(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    signage: Option<&crate::environment_signage::SignageAssets>,
    variant: IndustrialVariant,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };
    let primary_material = match (signage, get_industrial_sector_chapter_from_variant(variant)) {
        (Some(signage), IndustrialChapter::IndustrialEntry)
        | (Some(signage), IndustrialChapter::VeyronHeavyWorks) => {
            signage.mat_industrial_corporate.clone()
        }
        (Some(signage), IndustrialChapter::FreightYard) => signage.mat_industrial_freight.clone(),
        (Some(signage), IndustrialChapter::ProcessingPlant) => {
            signage.mat_industrial_processing.clone()
        }
        (Some(signage), IndustrialChapter::IndustrialTransition) => {
            signage.mat_industrial_safety.clone()
        }
        (None, IndustrialChapter::IndustrialEntry)
        | (None, IndustrialChapter::FreightYard)
        | (None, IndustrialChapter::ProcessingPlant)
        | (None, IndustrialChapter::VeyronHeavyWorks)
        | (None, IndustrialChapter::IndustrialTransition) => industrial.mat_hazard.clone(),
    };
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: primary_material,
        transform: Transform::from_xyz(side * 8.70, 5.15, -14.5)
            .with_scale(Vec3::new(1.25, 1.0, 1.0)),
        ..default()
    });

    if matches!(
        variant,
        IndustrialVariant::FreightCraneBay
            | IndustrialVariant::ProcessingSteamManifold
            | IndustrialVariant::HeavyWorksVeyronGate
            | IndustrialVariant::TransitionDamagedWorks
            | IndustrialVariant::TransitionFloodApproach
    ) {
        let safety_material = signage
            .map(|assets| assets.mat_industrial_safety.clone())
            .unwrap_or_else(|| industrial.mat_hazard.clone());
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_sign.clone(),
            material: safety_material,
            transform: Transform::from_xyz(-side * 5.75, 2.85, -8.5)
                .with_scale(Vec3::new(0.72, 0.72, 1.0)),
            ..default()
        });
    }
}

fn get_industrial_sector_chapter_from_variant(variant: IndustrialVariant) -> IndustrialChapter {
    match variant {
        IndustrialVariant::EntryPipeGallery
        | IndustrialVariant::EntryServiceBlock
        | IndustrialVariant::EntryWarningLine => IndustrialChapter::IndustrialEntry,
        IndustrialVariant::FreightContainerStack
        | IndustrialVariant::FreightCraneBay
        | IndustrialVariant::FreightLoadingPlatform => IndustrialChapter::FreightYard,
        IndustrialVariant::ProcessingPressureRow
        | IndustrialVariant::ProcessingSteamManifold
        | IndustrialVariant::ProcessingCatwalk => IndustrialChapter::ProcessingPlant,
        IndustrialVariant::HeavyWorksConveyor
        | IndustrialVariant::HeavyWorksVeyronGate
        | IndustrialVariant::HeavyWorksCoolingArray => IndustrialChapter::VeyronHeavyWorks,
        IndustrialVariant::TransitionLeakingMain
        | IndustrialVariant::TransitionDamagedWorks
        | IndustrialVariant::TransitionFloodApproach => IndustrialChapter::IndustrialTransition,
    }
}

/// Builds a small repeated facade module instead of treating a factory as one
/// undifferentiated wall. The modules stay outside the playable lanes while
/// giving each 40m chunk visible columns, panels, service access, and vents.
fn spawn_facade_modules(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    variant: IndustrialVariant,
) {
    for z in [-12.0, -4.0, 4.0, 12.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_facade_panel.clone(),
            material: industrial.mat_facade.clone(),
            transform: Transform::from_xyz(side * 8.34, 3.35, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_structural_column.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(side * 8.48, 3.45, z - 3.25),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_reinforcement_beam.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(side * 8.48, 1.75, z),
            ..default()
        });
    }

    let detail_mesh = match variant {
        IndustrialVariant::FreightContainerStack
        | IndustrialVariant::FreightCraneBay
        | IndustrialVariant::FreightLoadingPlatform => industrial.mesh_loading_bay.clone(),
        _ => industrial.mesh_service_door.clone(),
    };
    seg.spawn(PbrBundle {
        mesh: detail_mesh,
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(side * 8.57, 1.4, 0.0),
        ..default()
    });
    for z in [-8.0, 8.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_vent_panel.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(side * 8.57, 4.4, z),
            ..default()
        });
    }
}

/// A compact pipe rack gives the industrial silhouettes a believable service
/// route. Valves and gauges are fixed geometry, not animated entities.
fn spawn_pipe_rack_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    variant: IndustrialVariant,
) {
    for (y, material) in [
        (2.35, industrial.mat_pipe.clone()),
        (3.05, industrial.mat_steel.clone()),
    ] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_large_pipe.clone(),
            material,
            transform: Transform::from_xyz(side * 5.2, y, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
    for z in [-13.0, 13.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_structural_column.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(side * 5.25, 3.05, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pipe_junction.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(side * 5.2, 2.35, z),
            ..default()
        });
    }
    if !matches!(variant, IndustrialVariant::EntryServiceBlock) {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_valve.clone(),
            material: industrial.mat_hazard.clone(),
            transform: Transform::from_xyz(side * 5.2, 2.35, -6.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pressure_gauge.clone(),
            material: industrial.mat_amber.clone(),
            transform: Transform::from_xyz(side * 5.2, 3.05, 6.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
}

fn spawn_mid_industrial_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    variant: IndustrialVariant,
) {
    match variant {
        IndustrialVariant::EntryPipeGallery
        | IndustrialVariant::EntryServiceBlock
        | IndustrialVariant::EntryWarningLine => {
            spawn_service_machinery(seg, industrial, side, -6.0);
        }
        IndustrialVariant::FreightContainerStack
        | IndustrialVariant::FreightCraneBay
        | IndustrialVariant::FreightLoadingPlatform => {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_freight_platform.clone(),
                material: industrial.mat_dark_steel.clone(),
                transform: Transform::from_xyz(-side * 6.0, 0.55, 0.0),
                ..default()
            });
            for z in [-7.0, 0.0, 7.0] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_pallet.clone(),
                    material: industrial.mat_rusted_steel.clone(),
                    transform: Transform::from_xyz(-side * 7.1, 0.75, z),
                    ..default()
                });
            }
            spawn_service_machinery(seg, industrial, side, 8.0);
        }
        IndustrialVariant::ProcessingPressureRow
        | IndustrialVariant::ProcessingSteamManifold
        | IndustrialVariant::ProcessingCatwalk => {
            for z in [-8.0, 8.0] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_storage_tank.clone(),
                    material: industrial.mat_steel.clone(),
                    transform: Transform::from_xyz(-side * 8.0, 2.0, z),
                    ..default()
                });
            }
            spawn_catwalk_detail(seg, industrial, side, 0.0);
        }
        IndustrialVariant::HeavyWorksConveyor
        | IndustrialVariant::HeavyWorksVeyronGate
        | IndustrialVariant::HeavyWorksCoolingArray => {
            spawn_conveyor_detail(seg, industrial, side);
            spawn_catwalk_detail(seg, industrial, side, -2.0);
            for z in [-8.0, 8.0] {
                seg.spawn(PbrBundle {
                    mesh: industrial.mesh_cooling_unit.clone(),
                    material: industrial.mat_cooling.clone(),
                    transform: Transform::from_xyz(-side * 8.2, 1.0, z),
                    ..default()
                });
            }
            if matches!(variant, IndustrialVariant::HeavyWorksVeyronGate) {
                spawn_veyron_security_detail(seg, industrial, side);
            }
        }
        IndustrialVariant::TransitionLeakingMain
        | IndustrialVariant::TransitionDamagedWorks
        | IndustrialVariant::TransitionFloodApproach => {
            spawn_service_machinery(seg, industrial, side, -7.0);
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_transformer.clone(),
                material: industrial.mat_rusted_steel.clone(),
                transform: Transform::from_xyz(-side * 7.0, 1.1, 8.0),
                ..default()
            });
        }
    }
}

fn spawn_service_machinery(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_machine.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(side * 6.8, 1.0, z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_transformer.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * 7.6, 0.95, z + 3.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_warning_light.clone(),
        material: industrial.mat_amber.clone(),
        transform: Transform::from_xyz(side * 7.7, 2.0, z + 3.0),
        ..default()
    });
}

fn spawn_catwalk_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_platform.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * 6.4, 4.0, z),
        ..default()
    });
    for x in [side * 5.0, side * 7.8] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_railing.clone(),
            material: industrial.mat_hazard.clone(),
            transform: Transform::from_xyz(x, 4.45, z),
            ..default()
        });
    }
    for x in [side * 5.0, side * 7.8] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_ladder.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(x, 2.85, z + 2.2),
            ..default()
        });
    }
}

fn spawn_conveyor_detail(seg: &mut ChildBuilder, industrial: &IndustrialSectorAssets, side: f32) {
    for z in [-6.0, 0.0, 6.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_conveyor_frame.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(side * 6.4, 2.15, z)
                .with_rotation(Quat::from_rotation_x(0.08)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_conveyor_roller.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(side * 6.4, 2.38, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_machine.clone(),
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(side * 6.4, 1.1, 8.0),
        ..default()
    });
}

fn spawn_veyron_security_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
) {
    for z in [-9.0, 9.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_veyron.clone(),
            transform: Transform::from_xyz(side * 9.5, 2.4, z),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: industrial.mat_veyron.clone(),
        transform: Transform::from_xyz(side * 9.5, 5.0, 0.0),
        ..default()
    });
}

fn spawn_far_industrial_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    variant: IndustrialVariant,
) {
    for (x, z) in [(side * 18.0, -10.0), (-side * 19.0, 10.0)] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_facade_panel.clone(),
            material: industrial.mat_factory.clone(),
            transform: Transform::from_xyz(x, 5.2, z).with_scale(Vec3::new(1.0, 1.8, 1.8)),
            ..default()
        });
    }
    if matches!(
        variant,
        IndustrialVariant::FreightCraneBay
            | IndustrialVariant::FreightLoadingPlatform
            | IndustrialVariant::HeavyWorksConveyor
            | IndustrialVariant::HeavyWorksVeyronGate
    ) {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_duct.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(side * 15.0, 6.0, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
    if matches!(
        variant,
        IndustrialVariant::TransitionDamagedWorks | IndustrialVariant::TransitionFloodApproach
    ) {
        spawn_steam_vent(seg, industrial, -side * 17.0, 5.3, -8.0);
    }
}

#[derive(Clone, Copy)]
enum IndustrialBuildingStyle {
    Entry,
    Freight,
    Processing,
    HeavyWorks,
    Transition,
}

#[derive(Clone, Copy)]
struct CompleteBuildingSpec {
    side: f32,
    center_x: f32,
    z: f32,
    depth: f32,
    height: f32,
    length: f32,
    floors: u8,
    style: IndustrialBuildingStyle,
    loading_bays: bool,
    roof_units: u8,
    distressed: bool,
}

/// Composes full industrial buildings from the existing low-poly modules. The
/// shell is deliberately a real volume, rather than another thin facade, so
/// roofs, side walls, and overlapping service buildings read as one campus.
fn spawn_industrial_architecture(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    variant: IndustrialVariant,
    even: bool,
    segment_length: f32,
) {
    let side = if even { 1.0 } else { -1.0 };
    let building_length = (segment_length - 5.0).clamp(30.0, 36.0);

    match variant {
        IndustrialVariant::EntryPipeGallery => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 14.5,
                    z: 0.0,
                    depth: 10.5,
                    height: 8.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: false,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 14.0,
                    z: 0.0,
                    depth: 6.5,
                    height: 5.5,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 10.0, 19.0, 7.35, 0.0);
            spawn_utility_tower(seg, industrial, -side, 20.0, 8.0, 9.0);
        }
        IndustrialVariant::EntryServiceBlock => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 11.0,
                    height: 8.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 13.5,
                    z: 0.0,
                    depth: 7.0,
                    height: 5.0,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 10.0, 20.0, 6.95, -7.0);
        }
        IndustrialVariant::EntryWarningLine => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 14.0,
                    z: 0.0,
                    depth: 9.5,
                    height: 7.2,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 15.5,
                    z: 0.0,
                    depth: 8.0,
                    height: 6.0,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Entry,
                    loading_bays: true,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.0, 18.0, 6.6, 6.0);
        }
        IndustrialVariant::FreightContainerStack => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 11.5,
                    height: 8.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 15.5,
                    z: 0.0,
                    depth: 8.0,
                    height: 6.0,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.5, 20.0, 7.0, -5.0);
        }
        IndustrialVariant::FreightCraneBay => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.5,
                    z: 0.0,
                    depth: 12.0,
                    height: 8.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 14.0,
                    z: 0.0,
                    depth: 7.0,
                    height: 5.5,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.0, 21.5, 7.5, 6.0);
        }
        IndustrialVariant::FreightLoadingPlatform => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 14.5,
                    z: 0.0,
                    depth: 10.0,
                    height: 7.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 16.5,
                    z: 0.0,
                    depth: 9.0,
                    height: 6.5,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Freight,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, -side, 11.0, 21.0, 6.6, 2.0);
        }
        IndustrialVariant::ProcessingPressureRow => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 12.0,
                    height: 10.5,
                    length: building_length,
                    floors: 3,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: true,
                    roof_units: 3,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 8.0,
                    height: 7.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 9.0, 21.0, 8.8, -5.0);
            spawn_utility_tower(seg, industrial, -side, 20.0, 9.5, 8.0);
        }
        IndustrialVariant::ProcessingSteamManifold => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 16.0,
                    z: 0.0,
                    depth: 13.0,
                    height: 11.5,
                    length: building_length,
                    floors: 3,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: true,
                    roof_units: 3,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 14.0,
                    z: 0.0,
                    depth: 7.5,
                    height: 6.5,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: false,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 9.5, 22.0, 9.5, 6.0);
        }
        IndustrialVariant::ProcessingCatwalk => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 14.5,
                    z: 0.0,
                    depth: 10.5,
                    height: 9.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 16.0,
                    z: 0.0,
                    depth: 9.5,
                    height: 7.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Processing,
                    loading_bays: false,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.0, 20.5, 8.0, -4.0);
        }
        IndustrialVariant::HeavyWorksConveyor => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 16.0,
                    z: 0.0,
                    depth: 14.0,
                    height: 12.0,
                    length: building_length,
                    floors: 3,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: true,
                    roof_units: 3,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 9.0,
                    height: 8.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.5, 22.5, 10.0, -5.0);
            spawn_utility_tower(seg, industrial, side, 23.0, 11.0, 9.0);
        }
        IndustrialVariant::HeavyWorksVeyronGate => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.5,
                    z: 0.0,
                    depth: 13.0,
                    height: 11.0,
                    length: building_length,
                    floors: 3,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: true,
                    roof_units: 3,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 16.5,
                    z: 0.0,
                    depth: 10.0,
                    height: 7.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 9.0, 22.0, 9.5, 5.0);
        }
        IndustrialVariant::HeavyWorksCoolingArray => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 16.5,
                    z: 0.0,
                    depth: 15.0,
                    height: 12.5,
                    length: building_length,
                    floors: 3,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: false,
                    roof_units: 4,
                    distressed: false,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 14.5,
                    z: 0.0,
                    depth: 8.0,
                    height: 8.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::HeavyWorks,
                    loading_bays: false,
                    roof_units: 2,
                    distressed: false,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 10.0, 23.5, 10.5, 0.0);
        }
        IndustrialVariant::TransitionLeakingMain => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 12.0,
                    height: 10.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: true,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 14.0,
                    z: 0.0,
                    depth: 8.0,
                    height: 6.5,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: true,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, side, 9.0, 20.0, 8.0, -4.0);
        }
        IndustrialVariant::TransitionDamagedWorks => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 16.0,
                    z: 0.0,
                    depth: 13.0,
                    height: 10.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: true,
                    roof_units: 2,
                    distressed: true,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 9.0,
                    height: 7.0,
                    length: building_length,
                    floors: 1,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: true,
                },
            );
            spawn_side_service_bridge(seg, industrial, side, 9.5, 22.0, 7.4, 3.0);
        }
        IndustrialVariant::TransitionFloodApproach => {
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side,
                    center_x: 15.0,
                    z: 0.0,
                    depth: 11.0,
                    height: 9.0,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: false,
                    roof_units: 1,
                    distressed: true,
                },
            );
            spawn_complete_industrial_building(
                seg,
                industrial,
                CompleteBuildingSpec {
                    side: -side,
                    center_x: 16.5,
                    z: 0.0,
                    depth: 10.0,
                    height: 7.5,
                    length: building_length,
                    floors: 2,
                    style: IndustrialBuildingStyle::Transition,
                    loading_bays: true,
                    roof_units: 1,
                    distressed: true,
                },
            );
            spawn_side_pipe_bridge(seg, industrial, -side, 10.5, 22.0, 7.0, 6.0);
        }
    }
}

fn spawn_complete_industrial_building(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    spec: CompleteBuildingSpec,
) {
    let front_material = match spec.style {
        IndustrialBuildingStyle::Entry => industrial.mat_factory.clone(),
        IndustrialBuildingStyle::Freight => industrial.mat_facade.clone(),
        IndustrialBuildingStyle::Processing => industrial.mat_steel.clone(),
        IndustrialBuildingStyle::HeavyWorks => industrial.mat_dark_steel.clone(),
        IndustrialBuildingStyle::Transition => industrial.mat_rusted_steel.clone(),
    };
    let panel_material = match spec.style {
        IndustrialBuildingStyle::Entry => industrial.mat_veyron.clone(),
        IndustrialBuildingStyle::Freight => industrial.mat_rusted_steel.clone(),
        IndustrialBuildingStyle::Processing => industrial.mat_cooling.clone(),
        IndustrialBuildingStyle::HeavyWorks => industrial.mat_veyron.clone(),
        IndustrialBuildingStyle::Transition => industrial.mat_facade.clone(),
    };
    let roof_material = if spec.distressed {
        industrial.mat_rusted_steel.clone()
    } else {
        industrial.mat_dark_steel.clone()
    };
    let x = spec.side * spec.center_x;
    let near_x = spec.side * (spec.center_x - spec.depth * 0.5);
    let far_x = spec.side * (spec.center_x + spec.depth * 0.5);
    let wall_base = 0.45;
    let wall_center_y = wall_base + spec.height * 0.5;
    let floor_height = spec.height / spec.floors.max(1) as f32;

    // Foundation, front/rear walls, and end walls form an intentional volume.
    // Each mesh has a useful native proportion; only the authored building
    // footprint is scaled within a moderate range.
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_arch_foundation.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(x, 0.22, spec.z).with_scale(Vec3::new(
            spec.depth / 10.3,
            1.0,
            spec.length / 24.2,
        )),
        ..default()
    });
    for (wall_x, material) in [
        (near_x, front_material),
        (far_x, industrial.mat_facade.clone()),
    ] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_arch_front_wall.clone(),
            material,
            transform: Transform::from_xyz(wall_x, wall_center_y, spec.z).with_scale(Vec3::new(
                1.0,
                spec.height / 6.5,
                spec.length / 24.0,
            )),
            ..default()
        });
    }
    for z_end in [spec.z - spec.length * 0.5, spec.z + spec.length * 0.5] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_arch_side_wall.clone(),
            material: roof_material.clone(),
            transform: Transform::from_xyz(x, wall_center_y, z_end).with_scale(Vec3::new(
                spec.depth / 10.0,
                spec.height / 6.5,
                1.0,
            )),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_arch_roof.clone(),
        material: roof_material.clone(),
        transform: Transform::from_xyz(x, wall_base + spec.height + 0.2, spec.z)
            .with_scale(Vec3::new(spec.depth / 10.6, 1.0, spec.length / 24.8)),
        ..default()
    });

    for floor in 1..spec.floors {
        let y = wall_base + floor_height * floor as f32;
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_reinforcement_beam.clone(),
            material: roof_material.clone(),
            transform: Transform::from_xyz(near_x - spec.side * 0.24, y, spec.z)
                .with_scale(Vec3::new(1.0, 1.0, spec.length / 7.0)),
            ..default()
        });
    }
    for z_offset in [-0.38, 0.0, 0.38] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_arch_frame.clone(),
            material: roof_material.clone(),
            transform: Transform::from_xyz(
                near_x - spec.side * 0.24,
                wall_center_y,
                spec.z + spec.length * z_offset,
            )
            .with_scale(Vec3::new(1.0, spec.height / 7.0, 1.0)),
            ..default()
        });
    }

    for floor in 0..spec.floors {
        let y = wall_base + floor_height * (floor as f32 + 0.58);
        for z_offset in [-0.25, 0.0, 0.25] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_arch_window_band.clone(),
                material: panel_material.clone(),
                transform: Transform::from_xyz(
                    near_x - spec.side * 0.24,
                    y,
                    spec.z + spec.length * z_offset,
                )
                .with_scale(Vec3::new(
                    1.0,
                    (floor_height / 1.0 * 0.30).max(0.7),
                    1.0,
                )),
                ..default()
            });
        }
    }

    let entrance_z = spec.z - spec.length * 0.32;
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_arch_entrance_recess.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(near_x - spec.side * 0.22, 1.95, entrance_z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: if spec.loading_bays {
            industrial.mesh_loading_bay.clone()
        } else {
            industrial.mesh_service_door.clone()
        },
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(near_x - spec.side * 0.42, 1.35, entrance_z).with_scale(
            if spec.loading_bays {
                Vec3::new(1.0, 1.0, 1.15)
            } else {
                Vec3::new(1.0, 1.25, 1.4)
            },
        ),
        ..default()
    });
    if spec.loading_bays {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_loading_bay.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(
                near_x - spec.side * 0.42,
                1.3,
                spec.z + spec.length * 0.22,
            ),
            ..default()
        });
    }

    // Short roof trusses articulate the roofline and give the equipment a
    // visible structural relationship instead of leaving it floating.
    for z_offset in [-0.32, 0.0, 0.32] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_reinforcement_beam.clone(),
            material: roof_material.clone(),
            transform: Transform::from_xyz(
                x,
                wall_base + spec.height + 0.72,
                spec.z + spec.length * z_offset,
            )
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(1.0, 1.0, spec.depth / 7.0)),
            ..default()
        });
    }

    let roof_y = wall_base + spec.height + 1.0;
    for unit in 0..spec.roof_units {
        let centered = unit as f32 - (spec.roof_units.saturating_sub(1) as f32 * 0.5);
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_cooling_unit.clone(),
            material: industrial.mat_cooling.clone(),
            transform: Transform::from_xyz(
                x + spec.side * centered * 2.0,
                roof_y,
                spec.z + spec.length * 0.18,
            ),
            ..default()
        });
    }
    for z_offset in [-0.28, 0.28] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_stack.clone(),
            material: if spec.distressed {
                industrial.mat_rusted_steel.clone()
            } else {
                industrial.mat_pipe.clone()
            },
            transform: Transform::from_xyz(
                x + spec.side * spec.depth * 0.22,
                spec.height + 4.5,
                spec.z + spec.length * z_offset,
            )
            .with_scale(Vec3::splat(if spec.distressed { 0.82 } else { 1.0 })),
            ..default()
        });
    }
}

fn spawn_side_pipe_bridge(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    from_x: f32,
    to_x: f32,
    y: f32,
    z: f32,
) {
    let span = (to_x - from_x).abs();
    let center_x = side * (from_x + to_x) * 0.5;
    for offset in [-0.35, 0.35] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_duct.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(center_x, y + offset, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::new(1.0, span / 6.0, 1.0)),
            ..default()
        });
    }
    spawn_side_service_bridge(seg, industrial, side, from_x, to_x, y - 0.9, z);
}

fn spawn_side_service_bridge(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    from_x: f32,
    to_x: f32,
    y: f32,
    z: f32,
) {
    let span = (to_x - from_x).abs();
    let center_x = side * (from_x + to_x) * 0.5;
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_reinforcement_beam.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(center_x, y, z)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(1.0, 1.0, span / 7.0)),
        ..default()
    });
    for x in [from_x, to_x] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(side * x, y - 2.0, z),
            ..default()
        });
    }
}

fn spawn_utility_tower(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
    x: f32,
    y: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_stack.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * x, y, z).with_scale(Vec3::new(1.5, 1.0, 1.5)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_platform.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(side * x, y - 2.8, z),
        ..default()
    });
    for x_offset in [-1.2, 1.2] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(side * x + x_offset, y - 1.4, z),
            ..default()
        });
    }
}

/// Adds a restrained third layer of industrial storytelling to each authored
/// variant. These are static children of the rolling segment, use shared
/// handles, and intentionally stay on the service shoulders or above the
/// camera/player clearance envelope.
fn spawn_industrial_background_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    variant: IndustrialVariant,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };

    match variant {
        IndustrialVariant::EntryPipeGallery => {
            spawn_wall_control_box(seg, industrial, props, side, 1.45, -8.0);
            spawn_wall_camera(seg, industrial, props, side, 4.75, 6.0);
            spawn_wall_cable(seg, props, side, 3.0, -1.0);
            spawn_small_sign(seg, industrial, signage, side, 2.75, -8.0, |assets| {
                assets.mat_industrial_entry_detail.clone()
            });
            spawn_indicator(seg, industrial, props, side * 8.58, 2.1, -8.55, false);
        }
        IndustrialVariant::EntryServiceBlock => {
            spawn_wall_control_box(seg, industrial, props, side, 1.35, 5.5);
            spawn_wall_cable(seg, props, side, 2.7, 5.5);
            spawn_small_sign(seg, industrial, signage, side, 2.7, 5.5, |assets| {
                assets.mat_industrial_entry_detail.clone()
            });
            spawn_indicator(seg, industrial, props, side * 8.58, 2.05, 6.25, true);
        }
        IndustrialVariant::EntryWarningLine => {
            spawn_wall_control_box(seg, industrial, props, side, 1.55, -2.0);
            spawn_wall_camera(seg, industrial, props, side, 4.85, -6.0);
            spawn_wall_cable(seg, props, side, 3.15, -2.0);
            spawn_indicator(seg, industrial, props, side * 8.58, 2.1, -2.55, true);
        }
        IndustrialVariant::FreightContainerStack => {
            spawn_freight_ground_detail(seg, industrial, props, side, -7.0);
            spawn_small_sign(seg, industrial, signage, side, 2.45, -7.0, |assets| {
                assets.mat_industrial_loading_detail.clone()
            });
        }
        IndustrialVariant::FreightCraneBay => {
            spawn_freight_ground_detail(seg, industrial, props, side, 6.0);
            spawn_overhead_bridge(seg, industrial, props, 0.0, 6.45, 0.0);
            spawn_small_sign(seg, industrial, signage, side, 3.0, 6.0, |assets| {
                assets.mat_industrial_loading_detail.clone()
            });
        }
        IndustrialVariant::FreightLoadingPlatform => {
            spawn_freight_ground_detail(seg, industrial, props, side, 7.0);
            spawn_wall_control_box(seg, industrial, props, -side, 1.25, -6.0);
            spawn_overhead_bridge(seg, industrial, props, 0.0, 6.35, 8.0);
            spawn_small_sign(seg, industrial, signage, -side, 2.35, -6.0, |assets| {
                assets.mat_industrial_loading_detail.clone()
            });
        }
        IndustrialVariant::ProcessingPressureRow => {
            spawn_processing_control_detail(seg, industrial, props, signage, side, -8.0);
        }
        IndustrialVariant::ProcessingSteamManifold => {
            spawn_processing_control_detail(seg, industrial, props, signage, side, 8.0);
            spawn_overhead_pipe_bridge(seg, industrial, props, side, 6.55, -4.0);
        }
        IndustrialVariant::ProcessingCatwalk => {
            spawn_processing_control_detail(seg, industrial, props, signage, -side, -6.0);
            spawn_overhead_pipe_bridge(seg, industrial, props, -side, 6.35, 5.0);
        }
        IndustrialVariant::HeavyWorksConveyor => {
            spawn_heavy_works_detail(seg, industrial, props, signage, side, -8.0);
            spawn_overhead_bridge(seg, industrial, props, 0.0, 6.7, -5.0);
        }
        IndustrialVariant::HeavyWorksVeyronGate => {
            spawn_heavy_works_detail(seg, industrial, props, signage, side, 5.0);
            spawn_overhead_bridge(seg, industrial, props, 0.0, 6.85, 5.0);
        }
        IndustrialVariant::HeavyWorksCoolingArray => {
            spawn_heavy_works_detail(seg, industrial, props, signage, -side, -5.0);
            spawn_overhead_pipe_bridge(seg, industrial, props, side, 6.4, 0.0);
        }
        IndustrialVariant::TransitionLeakingMain => {
            spawn_transition_failure_detail(seg, industrial, props, signage, side, -7.0);
        }
        IndustrialVariant::TransitionDamagedWorks => {
            spawn_transition_failure_detail(seg, industrial, props, signage, -side, 6.0);
            spawn_overhead_bridge(seg, industrial, props, 0.0, 6.35, 0.0);
        }
        IndustrialVariant::TransitionFloodApproach => {
            spawn_transition_failure_detail(seg, industrial, props, signage, side, 5.0);
            spawn_wall_cable(seg, props, -side, 2.5, -7.0);
        }
    }
}

fn spawn_wall_control_box(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    side: f32,
    y: f32,
    z: f32,
) {
    let mesh = props
        .map(|assets| assets.mesh_breaker_cabinet.clone())
        .unwrap_or_else(|| industrial.mesh_transformer.clone());
    seg.spawn(PbrBundle {
        mesh,
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(side * 8.62, y, z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_reinforcement_beam.clone(),
        material: industrial.mat_rusted_steel.clone(),
        transform: Transform::from_xyz(side * 8.54, y + 0.05, z)
            .with_scale(Vec3::new(0.35, 0.65, 0.12)),
        ..default()
    });
}

fn spawn_wall_camera(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    side: f32,
    y: f32,
    z: f32,
) {
    let Some(props) = props else {
        return;
    };
    seg.spawn(PbrBundle {
        mesh: props.mesh_camera_mount.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(side * 8.55, y, z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_camera_housing.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * 8.72, y - 0.18, z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_camera_lens.clone(),
        material: props.mat_security_red_led.clone(),
        transform: Transform::from_xyz(side * 8.94, y - 0.18, z),
        ..default()
    });
}

fn spawn_wall_cable(
    seg: &mut ChildBuilder,
    props: Option<&crate::environment_props::PropAssets>,
    side: f32,
    y: f32,
    z: f32,
) {
    let Some(props) = props else {
        return;
    };
    seg.spawn(PbrBundle {
        mesh: props.mesh_pipe_small.clone(),
        material: props.mat_pipe_dark.clone(),
        transform: Transform::from_xyz(side * 8.58, y, z).with_scale(Vec3::new(1.0, 0.12, 1.0)),
        ..default()
    });
}

fn spawn_small_sign<F>(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    signage: Option<&crate::environment_signage::SignageAssets>,
    side: f32,
    y: f32,
    z: f32,
    material: F,
) where
    F: FnOnce(&crate::environment_signage::SignageAssets) -> Handle<StandardMaterial>,
{
    let Some(signage) = signage else {
        return;
    };
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: material(signage),
        transform: Transform::from_xyz(side * 8.76, y, z).with_scale(Vec3::new(0.62, 0.42, 0.80)),
        ..default()
    });
}

fn spawn_indicator(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    x: f32,
    y: f32,
    z: f32,
    red: bool,
) {
    let material = if red {
        props
            .map(|assets| assets.mat_security_red_led.clone())
            .unwrap_or_else(|| industrial.mat_amber.clone())
    } else {
        props
            .map(|assets| assets.mat_graffiti_cyan.clone())
            .unwrap_or_else(|| industrial.mat_amber.clone())
    };
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_warning_light.clone(),
        material,
        transform: Transform::from_xyz(x, y, z).with_scale(Vec3::splat(0.7)),
        ..default()
    });
}

fn spawn_freight_ground_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    side: f32,
    z: f32,
) {
    if let Some(props) = props {
        seg.spawn(PbrBundle {
            mesh: props.mesh_cable_spool.clone(),
            material: props.mat_pipe_dark.clone(),
            transform: Transform::from_xyz(-side * 6.8, 0.72, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: props.mesh_toolbox.clone(),
            material: industrial.mat_hazard.clone(),
            transform: Transform::from_xyz(-side * 7.45, 0.42, z + 1.25),
            ..default()
        });
    }
    spawn_indicator(seg, industrial, props, -side * 6.1, 1.5, z - 1.2, false);
}

fn spawn_processing_control_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    side: f32,
    z: f32,
) {
    spawn_wall_control_box(seg, industrial, props, side, 1.35, z);
    spawn_indicator(seg, industrial, props, side * 8.58, 2.0, z - 0.65, false);
    spawn_small_sign(seg, industrial, signage, side, 2.6, z, |assets| {
        assets.mat_industrial_coolant_detail.clone()
    });
    for (pipe_z, pipe_y) in [(z - 2.0, 2.45), (z + 2.0, 3.15)] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pipe_junction.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(side * 5.2, pipe_y, pipe_z),
            ..default()
        });
        if let Some(props) = props {
            seg.spawn(PbrBundle {
                mesh: props.mesh_pipe_flange.clone(),
                material: props.mat_pipe_flange.clone(),
                transform: Transform::from_xyz(side * 5.2, pipe_y, pipe_z)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        }
    }
}

fn spawn_heavy_works_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    side: f32,
    z: f32,
) {
    spawn_wall_control_box(seg, industrial, props, side, 1.45, z);
    spawn_wall_camera(seg, industrial, props, side, 4.8, z + 4.0);
    spawn_indicator(seg, industrial, props, side * 8.58, 2.1, z - 0.7, true);
    spawn_small_sign(seg, industrial, signage, side, 3.0, z, |assets| {
        assets.mat_industrial_heavy_detail.clone()
    });
    if let Some(props) = props {
        seg.spawn(PbrBundle {
            mesh: props.mesh_transformer_body.clone(),
            material: props.mat_transformer_metal.clone(),
            transform: Transform::from_xyz(-side * 7.0, 1.25, z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: props.mesh_breaker_cabinet.clone(),
            material: props.mat_security_black.clone(),
            transform: Transform::from_xyz(-side * 7.8, 1.0, z + 2.2),
            ..default()
        });
    }
}

fn spawn_overhead_bridge(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    x: f32,
    y: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_reinforcement_beam.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(x, y, z)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(1.0, 1.0, 1.25)),
        ..default()
    });
    for support_x in [-4.0, 4.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(support_x, y * 0.58, z),
            ..default()
        });
    }
    if let Some(props) = props {
        seg.spawn(PbrBundle {
            mesh: props.mesh_pipe_small.clone(),
            material: props.mat_pipe_dark.clone(),
            transform: Transform::from_xyz(0.0, y - 0.42, z)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::new(1.0, 0.08, 1.0)),
            ..default()
        });
    }
    spawn_indicator(seg, industrial, props, x, y - 0.38, z - 0.3, true);
}

fn spawn_overhead_pipe_bridge(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    side: f32,
    y: f32,
    z: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_duct.clone(),
        material: industrial.mat_pipe.clone(),
        transform: Transform::from_xyz(0.0, y, z)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(1.0, 1.25, 1.0)),
        ..default()
    });
    for x in [side * 4.8, side * 7.6] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pipe_junction.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        });
    }
    spawn_indicator(seg, industrial, props, side * 5.0, y - 0.35, z, false);
}

fn spawn_transition_failure_detail(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    side: f32,
    z: f32,
) {
    spawn_wall_control_box(seg, industrial, props, side, 1.25, z);
    spawn_indicator(seg, industrial, props, side * 8.58, 2.0, z, true);
    spawn_small_sign(seg, industrial, signage, side, 2.65, z, |assets| {
        assets.mat_industrial_failure_detail.clone()
    });
    if let Some(props) = props {
        for (x, y, debris_z) in [(-side * 6.4, 0.32, z - 1.8), (-side * 7.1, 0.25, z + 1.5)] {
            seg.spawn(PbrBundle {
                mesh: props.mesh_rubble_chunk_med.clone(),
                material: props.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(x, y, debris_z)
                    .with_rotation(Quat::from_rotation_y(0.3 * side)),
                ..default()
            });
        }
        if z < -6.0 {
            seg.spawn(PbrBundle {
                mesh: props.mesh_electric_arc.clone(),
                material: props.mat_arc_spark.clone(),
                transform: Transform::from_xyz(side * 6.4, 1.65, z + 1.2)
                    .with_rotation(Quat::from_rotation_x(0.35)),
                ..default()
            });
        }
    }
}

fn spawn_industrial_landmark(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    landmark: IndustrialLandmark,
    even: bool,
) {
    let side = if even { 1.0 } else { -1.0 };
    match landmark {
        IndustrialLandmark::None => {}
        IndustrialLandmark::IndustrialEntryGate => spawn_entry_gate_landmark(seg, industrial),
        IndustrialLandmark::FreightGantry => spawn_freight_gantry_landmark(seg, industrial, side),
        IndustrialLandmark::ProcessingTower => {
            spawn_processing_tower_landmark(seg, industrial, side)
        }
        IndustrialLandmark::VeyronHeavyWorks => {
            spawn_veyron_heavy_works_landmark(seg, industrial, side)
        }
    }
}

fn spawn_entry_gate_landmark(seg: &mut ChildBuilder, industrial: &IndustrialSectorAssets) {
    for x in [-6.8, 6.8] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_veyron.clone(),
            transform: Transform::from_xyz(x, 2.5, 0.0).with_scale(Vec3::new(1.4, 1.0, 1.4)),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_crane_beam.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(0.0, 5.05, 0.0).with_scale(Vec3::new(1.35, 1.0, 1.0)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: industrial.mat_veyron.clone(),
        transform: Transform::from_xyz(0.0, 4.45, -0.08).with_scale(Vec3::new(1.35, 1.0, 1.0)),
        ..default()
    });
    for x in [-5.6, 5.6] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_warning_light.clone(),
            material: industrial.mat_amber.clone(),
            transform: Transform::from_xyz(x, 4.3, -0.28),
            ..default()
        });
    }
    for x in [-5.0, 5.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_large_pipe.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(x, 2.2, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
}

fn spawn_freight_gantry_landmark(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
) {
    for z in [-11.0, 11.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_crane_column.clone(),
            material: industrial.mat_steel.clone(),
            transform: Transform::from_xyz(side * 7.2, 3.3, z)
                .with_scale(Vec3::new(1.35, 1.3, 1.35)),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_crane_beam.clone(),
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(side * 7.2, 6.2, 0.0).with_scale(Vec3::new(1.35, 1.2, 1.2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_machine.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(side * 3.0, 5.7, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_crane_hook.clone(),
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(side * 3.0, 4.85, 0.0).with_scale(Vec3::new(1.2, 1.6, 1.2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: industrial.mat_hazard.clone(),
        transform: Transform::from_xyz(side * 7.2, 4.65, -11.25),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_freight_platform.clone(),
        material: industrial.mat_dark_steel.clone(),
        transform: Transform::from_xyz(-side * 6.4, 0.7, 0.0),
        ..default()
    });
}

fn spawn_processing_tower_landmark(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
) {
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_processing_tower.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * 9.0, 5.2, 0.0)
            .with_scale(Vec3::new(1.25, 1.35, 1.25)),
        ..default()
    });
    for y in [2.0, 4.0, 6.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_platform.clone(),
            material: industrial.mat_dark_steel.clone(),
            transform: Transform::from_xyz(side * 8.2, y, 0.0),
            ..default()
        });
        for x in [side * 6.8, side * 9.7] {
            seg.spawn(PbrBundle {
                mesh: industrial.mesh_railing.clone(),
                material: industrial.mat_hazard.clone(),
                transform: Transform::from_xyz(x, y + 0.45, 0.0),
                ..default()
            });
        }
    }
    for z in [-8.0, 8.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pressure_vessel.clone(),
            material: industrial.mat_rusted_steel.clone(),
            transform: Transform::from_xyz(-side * 8.0, 2.4, z)
                .with_scale(Vec3::new(1.15, 1.15, 1.15)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_pipe_junction.clone(),
            material: industrial.mat_pipe.clone(),
            transform: Transform::from_xyz(-side * 8.0, 4.35, z),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_large_pipe.clone(),
        material: industrial.mat_pipe.clone(),
        transform: Transform::from_xyz(0.0, 4.2, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_steam.clone(),
        material: industrial.mat_steam.clone(),
        transform: Transform::from_xyz(side * 9.0, 9.8, 0.0).with_scale(Vec3::new(1.2, 1.7, 1.2)),
        ..default()
    });
}

fn spawn_veyron_heavy_works_landmark(
    seg: &mut ChildBuilder,
    industrial: &IndustrialSectorAssets,
    side: f32,
) {
    for (x, z) in [(side * 10.5, 0.0), (-side * 13.0, 0.0)] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_factory_wall.clone(),
            material: industrial.mat_factory.clone(),
            transform: Transform::from_xyz(x, 5.0, z).with_scale(Vec3::new(1.0, 1.8, 0.9)),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_conveyor.clone(),
        material: industrial.mat_steel.clone(),
        transform: Transform::from_xyz(side * 7.6, 6.75, 0.0).with_scale(Vec3::new(1.25, 1.0, 1.0)),
        ..default()
    });
    for z in [-9.0, 9.0] {
        seg.spawn(PbrBundle {
            mesh: industrial.mesh_security_gate.clone(),
            material: industrial.mat_veyron.clone(),
            transform: Transform::from_xyz(side * 7.6, 3.2, z)
                .with_scale(Vec3::new(1.5, 1.35, 1.5)),
            ..default()
        });
    }
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_duct.clone(),
        material: industrial.mat_pipe.clone(),
        transform: Transform::from_xyz(0.0, 7.4, 0.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(1.0, 1.5, 1.0)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_processing_tower.clone(),
        material: industrial.mat_cooling.clone(),
        transform: Transform::from_xyz(-side * 8.5, 5.2, 0.0).with_scale(Vec3::new(1.4, 1.3, 1.4)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_sign.clone(),
        material: industrial.mat_veyron.clone(),
        transform: Transform::from_xyz(side * 10.5, 7.0, -0.12)
            .with_scale(Vec3::new(1.6, 1.1, 1.0)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: industrial.mesh_warning_light.clone(),
        material: industrial.mat_amber.clone(),
        transform: Transform::from_xyz(side * 7.6, 5.8, 0.0),
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zones::get_zone_for_distance;

    #[test]
    fn test_industrial_chapter_boundaries() {
        assert_eq!(get_zone_for_distance(3999.99).id, 2);
        assert_eq!(
            get_industrial_sector_chapter(4000.0),
            IndustrialChapter::IndustrialEntry
        );
        assert_eq!(
            get_industrial_sector_chapter(4399.99),
            IndustrialChapter::IndustrialEntry
        );
        assert_eq!(
            get_industrial_sector_chapter(4400.0),
            IndustrialChapter::FreightYard
        );
        assert_eq!(
            get_industrial_sector_chapter(4899.99),
            IndustrialChapter::FreightYard
        );
        assert_eq!(
            get_industrial_sector_chapter(4900.0),
            IndustrialChapter::ProcessingPlant
        );
        assert_eq!(
            get_industrial_sector_chapter(5399.99),
            IndustrialChapter::ProcessingPlant
        );
        assert_eq!(
            get_industrial_sector_chapter(5400.0),
            IndustrialChapter::VeyronHeavyWorks
        );
        assert_eq!(
            get_industrial_sector_chapter(5799.99),
            IndustrialChapter::VeyronHeavyWorks
        );
        assert_eq!(
            get_industrial_sector_chapter(5800.0),
            IndustrialChapter::IndustrialTransition
        );
        assert_eq!(
            get_industrial_sector_chapter(5999.99),
            IndustrialChapter::IndustrialTransition
        );
        assert_eq!(get_zone_for_distance(6000.0).id, 4);
    }

    #[test]
    fn test_each_industrial_chapter_has_three_deterministic_variants() {
        let checkpoints = [
            (4000.0, IndustrialVariant::EntryPipeGallery),
            (4135.0, IndustrialVariant::EntryServiceBlock),
            (4350.0, IndustrialVariant::EntryWarningLine),
            (4400.0, IndustrialVariant::FreightContainerStack),
            (4580.0, IndustrialVariant::FreightCraneBay),
            (4750.0, IndustrialVariant::FreightLoadingPlatform),
            (4900.0, IndustrialVariant::ProcessingPressureRow),
            (5100.0, IndustrialVariant::ProcessingSteamManifold),
            (5260.0, IndustrialVariant::ProcessingCatwalk),
            (5400.0, IndustrialVariant::HeavyWorksConveyor),
            (5600.0, IndustrialVariant::HeavyWorksVeyronGate),
            (5730.0, IndustrialVariant::HeavyWorksCoolingArray),
            (5800.0, IndustrialVariant::TransitionLeakingMain),
            (5900.0, IndustrialVariant::TransitionDamagedWorks),
            (5970.0, IndustrialVariant::TransitionFloodApproach),
        ];

        for (distance, expected) in checkpoints {
            assert_eq!(get_industrial_sector_variant(distance), expected);
            assert_eq!(
                get_industrial_sector_variant(distance),
                get_industrial_sector_variant(distance),
                "Industrial variant must be deterministic at {distance}m"
            );
        }
    }

    #[test]
    fn test_signature_landmarks_are_deterministic_and_sparse() {
        let checkpoints = [
            (4180.0, IndustrialLandmark::IndustrialEntryGate),
            (4660.0, IndustrialLandmark::FreightGantry),
            (5140.0, IndustrialLandmark::ProcessingTower),
            (5580.0, IndustrialLandmark::VeyronHeavyWorks),
        ];

        for (distance, expected) in checkpoints {
            assert_eq!(get_industrial_sector_landmark(distance), expected);
            assert_eq!(
                get_industrial_sector_landmark(distance),
                get_industrial_sector_landmark(distance),
                "Landmark selection must be deterministic at {distance}m"
            );
        }

        for distance in [4000.0, 4400.0, 4900.0, 5400.0, 5800.0, 5999.0] {
            assert_eq!(
                get_industrial_sector_landmark(distance),
                IndustrialLandmark::None,
                "Chapter boundary should not create an accidental landmark at {distance}m"
            );
        }
    }

    #[test]
    fn test_industrial_track_profile_follows_chapters_deterministically() {
        assert_eq!(
            get_industrial_track_profile(4200.0),
            IndustrialTrackProfile {
                lane_markings: false,
                service_conduits: false,
                distressed: false,
            }
        );
        assert_eq!(
            get_industrial_track_profile(4600.0),
            IndustrialTrackProfile {
                lane_markings: true,
                service_conduits: false,
                distressed: false,
            }
        );
        assert_eq!(
            get_industrial_track_profile(5000.0),
            IndustrialTrackProfile {
                lane_markings: false,
                service_conduits: true,
                distressed: false,
            }
        );
        assert_eq!(
            get_industrial_track_profile(5500.0),
            IndustrialTrackProfile {
                lane_markings: true,
                service_conduits: true,
                distressed: false,
            }
        );
        assert_eq!(
            get_industrial_track_profile(5900.0),
            IndustrialTrackProfile {
                lane_markings: true,
                service_conduits: true,
                distressed: true,
            }
        );
        assert_eq!(
            get_industrial_track_profile(5500.0),
            get_industrial_track_profile(5500.0)
        );
    }
}
