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
}
