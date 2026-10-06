use bevy::prelude::*;
use crate::types::*;
use crate::zones::get_zone_for_distance;

#[derive(Component)]
pub struct EnvironmentFan;

#[derive(Component)]
pub struct EnvironmentalSign;

#[derive(Resource)]
pub struct EnvironmentAssets {
    // Track & Ground Meshes
    pub mesh_bed: Handle<Mesh>,
    pub mesh_drain_trough: Handle<Mesh>,
    pub mesh_drain_grate: Handle<Mesh>,
    pub mesh_curb_shoulder: Handle<Mesh>,
    pub mesh_conduit_tray: Handle<Mesh>,
    pub mesh_cable_bundle: Handle<Mesh>,
    pub mesh_rail_base: Handle<Mesh>,
    pub mesh_rail_crown: Handle<Mesh>,
    pub mesh_sleeper: Handle<Mesh>,
    pub mesh_sleeper_heavy: Handle<Mesh>,
    pub mesh_sleeper_plate: Handle<Mesh>,
    pub mesh_rail_joint: Handle<Mesh>,
    pub mesh_lane_stripe: Handle<Mesh>,
    pub mesh_echo_conduit: Handle<Mesh>,

    // Architectural & Structural Meshes
    pub mesh_arch_top: Handle<Mesh>,
    pub mesh_arch_pillar: Handle<Mesh>,
    pub mesh_catenary_gantry: Handle<Mesh>,
    pub mesh_catenary_wire: Handle<Mesh>,
    pub mesh_lantern_cage: Handle<Mesh>,
    pub mesh_lantern_bulb: Handle<Mesh>,

    // Station & Wall Niches
    pub mesh_platform_slab: Handle<Mesh>,
    pub mesh_platform_curb: Handle<Mesh>,
    pub mesh_station_wall: Handle<Mesh>,
    pub mesh_vent_housing: Handle<Mesh>,
    pub mesh_vent_fan: Handle<Mesh>,
    pub mesh_station_sign: Handle<Mesh>,
    pub mesh_ladder_rail: Handle<Mesh>,
    pub mesh_ladder_rung: Handle<Mesh>,

    // Materials
    pub mat_concrete_bed: Handle<StandardMaterial>,
    pub mat_drainage: Handle<StandardMaterial>,
    pub mat_grate: Handle<StandardMaterial>,
    pub mat_ballast_curb: Handle<StandardMaterial>,
    pub mat_hazard_stripe: Handle<StandardMaterial>,
    pub mat_cables: Handle<StandardMaterial>,
    pub mat_rail_base: Handle<StandardMaterial>,
    pub mat_rail_crown: Handle<StandardMaterial>,
    pub mat_sleeper: Handle<StandardMaterial>,
    pub mat_sleeper_stained: Handle<StandardMaterial>,
    pub mat_sleeper_cracked: Handle<StandardMaterial>,
    pub mat_sleeper_steel: Handle<StandardMaterial>,
    pub mat_lane_stripe: Handle<StandardMaterial>,
    pub mat_echo_conduit: Handle<StandardMaterial>,
    pub mat_rail_joint: Handle<StandardMaterial>,
    pub mat_steel_truss: Handle<StandardMaterial>,
    pub mat_catenary_wire: Handle<StandardMaterial>,
    pub mat_lantern_cage: Handle<StandardMaterial>,
    pub mat_amber_lantern_glow: Handle<StandardMaterial>,
    pub mat_platform_concrete: Handle<StandardMaterial>,
    pub mat_station_wall: Handle<StandardMaterial>,
    pub mat_vent_housing: Handle<StandardMaterial>,
    pub mat_vent_blades: Handle<StandardMaterial>,
    pub mat_station_sign_glow: Handle<StandardMaterial>,
    pub mat_rusted_iron: Handle<StandardMaterial>,
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_environment_assets)
            .add_systems(
                Update,
                (
                    animate_environment_fans,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn init_environment_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let length = 40.0;

    let env_assets = EnvironmentAssets {
        // Track & Ground Meshes
        mesh_bed: meshes.add(Cuboid::new(8.8, 0.45, length)),
        mesh_drain_trough: meshes.add(Cuboid::new(0.65, 0.22, length)),
        mesh_drain_grate: meshes.add(Cuboid::new(0.60, 0.04, 0.85)),
        mesh_curb_shoulder: meshes.add(Cuboid::new(0.6, 0.25, length)),
        mesh_conduit_tray: meshes.add(Cuboid::new(0.32, 0.20, length)),
        mesh_cable_bundle: meshes.add(Cylinder::new(0.06, length)),
        mesh_rail_base: meshes.add(Cuboid::new(0.12, 0.06, length)),
        mesh_rail_crown: meshes.add(Cuboid::new(0.09, 0.07, length)),
        mesh_sleeper: meshes.add(Cuboid::new(8.2, 0.12, 0.38)),
        mesh_sleeper_heavy: meshes.add(Cuboid::new(8.2, 0.15, 0.44)),
        mesh_sleeper_plate: meshes.add(Cuboid::new(0.24, 0.03, 0.28)),
        mesh_rail_joint: meshes.add(Cuboid::new(0.04, 0.08, 0.60)),
        mesh_lane_stripe: meshes.add(Cuboid::new(0.08, 0.015, 1.4)),
        mesh_echo_conduit: meshes.add(Cuboid::new(0.06, 0.02, length)),

        // Architectural & Structural Meshes
        mesh_arch_top: meshes.add(Cuboid::new(9.8, 0.55, 0.9)),
        mesh_arch_pillar: meshes.add(Cuboid::new(0.7, 5.2, 0.9)),
        mesh_catenary_gantry: meshes.add(Cuboid::new(8.8, 0.20, 0.25)),
        mesh_catenary_wire: meshes.add(Cylinder::new(0.025, length)),
        mesh_lantern_cage: meshes.add(Cuboid::new(0.4, 0.55, 0.4)),
        mesh_lantern_bulb: meshes.add(Sphere::new(0.18)),

        // Station & Wall Niches
        mesh_platform_slab: meshes.add(Cuboid::new(2.6, 1.3, length)),
        mesh_platform_curb: meshes.add(Cuboid::new(0.22, 0.06, length)),
        mesh_station_wall: meshes.add(Cuboid::new(0.35, 5.2, length)),
        mesh_vent_housing: meshes.add(Cuboid::new(1.6, 1.6, 0.4)),
        mesh_vent_fan: meshes.add(Cuboid::new(1.25, 0.18, 0.06)),
        mesh_station_sign: meshes.add(Cuboid::new(2.4, 0.7, 0.12)),
        mesh_ladder_rail: meshes.add(Cuboid::new(0.06, 1.3, 0.06)),
        mesh_ladder_rung: meshes.add(Cuboid::new(0.42, 0.04, 0.04)),

        // Materials
        mat_concrete_bed: materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.09, 0.11),
            perceptual_roughness: 0.82,
            metallic: 0.10,
            ..default()
        }),
        mat_drainage: materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.04, 0.05),
            perceptual_roughness: 0.95,
            metallic: 0.05,
            ..default()
        }),
        mat_grate: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.20),
            perceptual_roughness: 0.40,
            metallic: 0.85,
            ..default()
        }),
        mat_ballast_curb: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.11, 0.12),
            perceptual_roughness: 0.90,
            metallic: 0.15,
            ..default()
        }),
        mat_hazard_stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.70, 0.05),
            emissive: LinearRgba::new(0.6, 0.4, 0.0, 1.0),
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_cables: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.09),
            perceptual_roughness: 0.60,
            metallic: 0.20,
            ..default()
        }),
        mat_rail_base: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.16, 0.18),
            perceptual_roughness: 0.50,
            metallic: 0.80,
            ..default()
        }),
        mat_rail_crown: materials.add(StandardMaterial {
            base_color: Color::srgb(0.78, 0.82, 0.88),
            emissive: LinearRgba::new(0.05, 0.35, 0.45, 1.0),
            perceptual_roughness: 0.10,
            metallic: 0.98,
            ..default()
        }),
        mat_sleeper: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.13, 0.14),
            perceptual_roughness: 0.75,
            metallic: 0.25,
            ..default()
        }),
        mat_sleeper_stained: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.07, 0.08),
            perceptual_roughness: 0.92,
            metallic: 0.10,
            ..default()
        }),
        mat_sleeper_cracked: materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.19, 0.18),
            perceptual_roughness: 0.85,
            metallic: 0.20,
            ..default()
        }),
        mat_sleeper_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.24, 0.27),
            perceptual_roughness: 0.35,
            metallic: 0.90,
            ..default()
        }),
        mat_lane_stripe: materials.add(StandardMaterial {
            base_color: Color::srgba(0.85, 0.80, 0.65, 0.70),
            emissive: LinearRgba::new(0.4, 0.35, 0.2, 1.0),
            perceptual_roughness: 0.50,
            metallic: 0.10,
            ..default()
        }),
        mat_echo_conduit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.8, 1.0),
            emissive: LinearRgba::new(0.2, 2.2, 3.2, 1.0),
            perceptual_roughness: 0.15,
            metallic: 0.50,
            ..default()
        }),
        mat_rail_joint: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.18, 0.16),
            perceptual_roughness: 0.60,
            metallic: 0.75,
            ..default()
        }),
        mat_steel_truss: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.16, 0.15),
            perceptual_roughness: 0.45,
            metallic: 0.85,
            ..default()
        }),
        mat_catenary_wire: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.35, 0.40),
            emissive: LinearRgba::new(0.1, 0.1, 0.2, 1.0),
            perceptual_roughness: 0.20,
            metallic: 0.90,
            ..default()
        }),
        mat_lantern_cage: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.14, 0.13),
            perceptual_roughness: 0.50,
            metallic: 0.90,
            ..default()
        }),
        mat_amber_lantern_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.7, 0.2),
            emissive: LinearRgba::new(3.5, 1.8, 0.3, 1.0),
            ..default()
        }),
        mat_platform_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.13, 0.15),
            perceptual_roughness: 0.75,
            metallic: 0.15,
            ..default()
        }),
        mat_station_wall: materials.add(StandardMaterial {
            base_color: Color::srgb(0.11, 0.11, 0.13),
            perceptual_roughness: 0.80,
            metallic: 0.20,
            ..default()
        }),
        mat_vent_housing: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.14, 0.16),
            perceptual_roughness: 0.50,
            metallic: 0.75,
            ..default()
        }),
        mat_vent_blades: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.25, 0.28),
            perceptual_roughness: 0.35,
            metallic: 0.85,
            ..default()
        }),
        mat_station_sign_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.85, 0.7),
            emissive: LinearRgba::new(1.8, 1.4, 0.6, 1.0),
            ..default()
        }),
        mat_rusted_iron: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.14, 0.10),
            perceptual_roughness: 0.70,
            metallic: 0.60,
            ..default()
        }),
    };

    commands.insert_resource(env_assets);
    crate::environment_props::init_prop_assets(&mut commands, &mut meshes, &mut materials);
    crate::environment_signage::init_signage_assets(&mut commands, &mut images, &mut materials);
}

fn animate_environment_fans(
    time: Res<Time>,
    mut fan_q: Query<&mut Transform, With<EnvironmentFan>>,
) {
    let dt = time.delta_seconds();
    for mut trans in fan_q.iter_mut() {
        trans.rotate_z(2.8 * dt);
    }
}

pub fn spawn_modular_environment_slice(
    commands: &mut Commands,
    env: &EnvironmentAssets,
    props: Option<&crate::environment_props::PropAssets>,
    signage: Option<&crate::environment_signage::SignageAssets>,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let segment_dist = (-z_center).max(0.0);
    if segment_dist < 800.0 {
        if let (Some(props_ref), Some(signage_ref)) = (props, signage) {
            return crate::old_metro::spawn_old_metro_segment(
                commands,
                env,
                props_ref,
                signage_ref,
                z_center,
                length,
                segment_dist,
            );
        }
    }

    let _zone = get_zone_for_distance(distance);
    let is_even_segment = ((z_center.abs() / length).floor() as i32) % 2 == 0;

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // -------------------------------------------------------------
            // 1. TRACK BED & DRAINAGE INFRASTRUCTURE
            // -------------------------------------------------------------
            // Main sub-ballast slab
            seg.spawn(PbrBundle {
                mesh: env.mesh_bed.clone(),
                material: env.mat_concrete_bed.clone(),
                transform: Transform::from_xyz(0.0, -0.225, 0.0),
                ..default()
            });

            // Recessed central drainage trough
            seg.spawn(PbrBundle {
                mesh: env.mesh_drain_trough.clone(),
                material: env.mat_drainage.clone(),
                transform: Transform::from_xyz(0.0, -0.05, 0.0),
                ..default()
            });

            // Metal drainage grates spaced every 8m along the center trough
            let num_grates = (length / 8.0) as i32;
            for i in -num_grates / 2..=num_grates / 2 {
                seg.spawn(PbrBundle {
                    mesh: env.mesh_drain_grate.clone(),
                    material: env.mat_grate.clone(),
                    transform: Transform::from_xyz(0.0, 0.01, i as f32 * 8.0),
                    ..default()
                });
            }

            // Ballast curb shoulders (Left & Right)
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

            // Utility cable conduit trays and high-voltage cable bundles flanking track
            seg.spawn(PbrBundle {
                mesh: env.mesh_conduit_tray.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(-4.5, 0.22, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_cable_bundle.clone(),
                material: env.mat_cables.clone(),
                transform: Transform::from_xyz(-4.5, 0.28, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_conduit_tray.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(4.5, 0.22, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_cable_bundle.clone(),
                material: env.mat_cables.clone(),
                transform: Transform::from_xyz(4.5, 0.28, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // -------------------------------------------------------------
            // 2. METALLIC STEEL RAILS & FASTENED SLEEPERS
            // -------------------------------------------------------------
            // Sleepers (cross-ties) every 2.0m with steel fastener plates
            let num_sleepers = (length / 2.0) as i32;
            for i in -num_sleepers / 2..=num_sleepers / 2 {
                let sleeper_z = i as f32 * 2.0;
                seg.spawn(PbrBundle {
                    mesh: env.mesh_sleeper.clone(),
                    material: env.mat_sleeper.clone(),
                    transform: Transform::from_xyz(0.0, 0.01, sleeper_z),
                    ..default()
                });

                // Steel tie fastener plates at rail positions
                for lane_idx in -1..=1 {
                    let cx = Lane::from_index(lane_idx).x_pos();
                    seg.spawn(PbrBundle {
                        mesh: env.mesh_sleeper_plate.clone(),
                        material: env.mat_grate.clone(),
                        transform: Transform::from_xyz(cx - 0.48, 0.07, sleeper_z),
                        ..default()
                    });
                    seg.spawn(PbrBundle {
                        mesh: env.mesh_sleeper_plate.clone(),
                        material: env.mat_grate.clone(),
                        transform: Transform::from_xyz(cx + 0.48, 0.07, sleeper_z),
                        ..default()
                    });
                }
            }

            // 6 Polished Metallic Steel Rails (Base + Specular Crown)
            for lane_idx in -1..=1 {
                let cx = Lane::from_index(lane_idx).x_pos();

                // Left Rail of Lane
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_base.clone(),
                    material: env.mat_rail_base.clone(),
                    transform: Transform::from_xyz(cx - 0.48, 0.04, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_crown.clone(),
                    material: env.mat_rail_crown.clone(),
                    transform: Transform::from_xyz(cx - 0.48, 0.09, 0.0),
                    ..default()
                });

                // Right Rail of Lane
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_base.clone(),
                    material: env.mat_rail_base.clone(),
                    transform: Transform::from_xyz(cx + 0.48, 0.04, 0.0),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_crown.clone(),
                    material: env.mat_rail_crown.clone(),
                    transform: Transform::from_xyz(cx + 0.48, 0.09, 0.0),
                    ..default()
                });
            }

            // -------------------------------------------------------------
            // 3. VAULTED GOTHIC INDUSTRIAL ARCH RIBS & CATENARY GANTRY
            // -------------------------------------------------------------
            // Heavy arch rib at center of segment
            seg.spawn(PbrBundle {
                mesh: env.mesh_arch_top.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(0.0, 5.2, 0.0),
                ..default()
            });

            // Vertical support columns
            seg.spawn(PbrBundle {
                mesh: env.mesh_arch_pillar.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(-4.9, 2.6, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_arch_pillar.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(4.9, 2.6, 0.0),
                ..default()
            });

            // Overhead catenary gantry & contact wires
            seg.spawn(PbrBundle {
                mesh: env.mesh_catenary_gantry.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(0.0, 4.6, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_catenary_wire.clone(),
                material: env.mat_catenary_wire.clone(),
                transform: Transform::from_xyz(-1.2, 4.4, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_catenary_wire.clone(),
                material: env.mat_catenary_wire.clone(),
                transform: Transform::from_xyz(1.2, 4.4, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // Caged industrial amber lantern suspended from gantry
            seg.spawn(PbrBundle {
                mesh: env.mesh_lantern_cage.clone(),
                material: env.mat_lantern_cage.clone(),
                transform: Transform::from_xyz(0.0, 4.3, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_lantern_bulb.clone(),
                material: env.mat_amber_lantern_glow.clone(),
                transform: Transform::from_xyz(0.0, 4.25, 0.0),
                ..default()
            });

            // REAL POINT LIGHT: casting warm incandescent sodium pools onto the track
            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 0.65, 0.25),
                    intensity: 95_000.0,
                    range: 22.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 4.1, 0.0),
                ..default()
            });

            // -------------------------------------------------------------
            // 4. ABANDONED STATION PLATFORM & INDUSTRIAL NICHES
            // -------------------------------------------------------------
            // Spawn elevated station platform on alternating side for asymmetrical depth
            let platform_x = if is_even_segment { 6.3 } else { -6.3 };
            let wall_x = if is_even_segment { 7.8 } else { -7.8 };
            let curb_edge_x = if is_even_segment { 5.0 } else { -5.0 };

            // Concrete platform slab
            seg.spawn(PbrBundle {
                mesh: env.mesh_platform_slab.clone(),
                material: env.mat_platform_concrete.clone(),
                transform: Transform::from_xyz(platform_x, 0.65, 0.0),
                ..default()
            });

            // Yellow caution tile line along platform edge with high-vis hazard stripe
            seg.spawn(PbrBundle {
                mesh: env.mesh_platform_curb.clone(),
                material: env.mat_hazard_stripe.clone(),
                transform: Transform::from_xyz(curb_edge_x, 1.32, 0.0),
                ..default()
            });

            // Station subway tiled wall
            seg.spawn(PbrBundle {
                mesh: env.mesh_station_wall.clone(),
                material: env.mat_station_wall.clone(),
                transform: Transform::from_xyz(wall_x, 2.6, 0.0),
                ..default()
            });

            // Industrial ventilation fan shaft in station wall
            seg.spawn(PbrBundle {
                mesh: env.mesh_vent_housing.clone(),
                material: env.mat_vent_housing.clone(),
                transform: Transform::from_xyz(wall_x - if is_even_segment { 0.15 } else { -0.15 }, 2.8, -6.0),
                ..default()
            });
            seg.spawn((
                PbrBundle {
                    mesh: env.mesh_vent_fan.clone(),
                    material: env.mat_vent_blades.clone(),
                    transform: Transform::from_xyz(wall_x - if is_even_segment { 0.18 } else { -0.18 }, 2.8, -6.0),
                    ..default()
                },
                EnvironmentFan,
            ));

            // Overhead station placard: SECTOR 04 // SUB-LEVEL 7
            seg.spawn((
                PbrBundle {
                    mesh: env.mesh_station_sign.clone(),
                    material: env.mat_station_sign_glow.clone(),
                    transform: Transform::from_xyz(curb_edge_x + if is_even_segment { 0.6 } else { -0.6 }, 3.4, 4.0),
                    ..default()
                },
                EnvironmentalSign,
            ));

            // Rusted maintenance ladder mounted on platform face
            seg.spawn(PbrBundle {
                mesh: env.mesh_ladder_rail.clone(),
                material: env.mat_rusted_iron.clone(),
                transform: Transform::from_xyz(curb_edge_x, 0.65, -12.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_ladder_rail.clone(),
                material: env.mat_rusted_iron.clone(),
                transform: Transform::from_xyz(curb_edge_x, 0.65, -12.4),
                ..default()
            });
            for r in 0..4 {
                seg.spawn(PbrBundle {
                    mesh: env.mesh_ladder_rung.clone(),
                    material: env.mat_rusted_iron.clone(),
                    transform: Transform::from_xyz(curb_edge_x, 0.25 + r as f32 * 0.28, -12.2),
                    ..default()
                });
            }
        })
        .id()
}
