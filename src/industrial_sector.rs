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
            spawn_industrial_track(seg, env, length);
            spawn_near_industrial_features(seg, industrial, variant, even);
            spawn_mid_industrial_features(seg, industrial, variant, even);
            spawn_far_industrial_features(seg, industrial, variant, even);
            spawn_industrial_landmark(
                seg,
                industrial,
                get_industrial_sector_landmark(distance),
                even,
            );
        })
        .id()
}

fn spawn_industrial_track(seg: &mut ChildBuilder, env: &EnvironmentAssets, length: f32) {
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
}
