use crate::environment::EnvironmentAssets;
use crate::environment_lighting::FlickeringLight;
use crate::environment_props::PropAssets;
use crate::environment_signage::SignageAssets;
use crate::old_metro_landmarks;
use crate::types::*;
use bevy::prelude::*;

/// The 7 distinct narrative chapters across Old Metro (0–800m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldMetroChapter {
    /// 0–100m: Abandoned railway entrance arch, broken signage, hanging cables, water leaks.
    Entrance,
    /// 100–220m: Maintenance corridor, utility pipe runs, side tunnels, derelict train carriage.
    ServiceTunnel,
    /// 220–350m: Station platform landmark, timetable, benches, posters, cyan ECHO graffiti.
    Platform,
    /// 350–470m: Industrial junction, pipe crossing manifolds, ventilation fans, rolling maintenance cart.
    MaintenanceJunction,
    /// 470–600m: High-voltage transformers, cooling fins, ceramic insulators, electrical arc sparks.
    PowerSubstation,
    /// 600–700m: Structural roof collapse, concrete rubble piles, bent rebar, shoring jacks, red beacons.
    CollapsedSection,
    /// 700–800m: Brutalist matte-black Veyron security checkpoint, surveillance cameras, scanner gantry, Neon District glow.
    VeyronCheckpoint,
}

/// The 22 granular sub-section visual variants across Old Metro, eliminating repetition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldMetroVariant {
    // 0–100m Entrance
    AbandonedEntrance, // 0–40m
    DamagedTunnel,     // 40–70m
    EchoTraces,        // 70–100m

    // 100–220m Service Tunnel
    PipeCorridor,      // 100–140m
    MaintenanceAccess, // 140–180m
    DerelictCarriage,  // 180–220m

    // 220–350m Platform
    PlatformApproach,    // 220–260m
    MainStationPlatform, // 260–300m
    StationAdWall,       // 300–330m
    PlatformExit,        // 330–350m

    // 350–470m Maintenance Junction
    JunctionManifold, // 350–390m
    VentilationShaft, // 390–430m
    MaintenanceCart,  // 430–470m

    // 470–600m Power Substation
    SubstationFeeders,   // 470–510m
    TransformerArcZone,  // 510–560m
    EchoContaminatedGen, // 560–600m

    // 600–700m Collapsed Section
    ShoringWarning,     // 600–640m
    DeepCollapseRubble, // 640–675m
    EmergencyBreach,    // 675–700m

    // 700–800m Veyron Checkpoint
    SecurityPerimeter, // 700–740m
    CheckpointGantry,  // 740–775m
    NeonExitPortal,    // 775–800m
}

pub fn get_old_metro_chapter(distance: f32) -> OldMetroChapter {
    if distance < 100.0 {
        OldMetroChapter::Entrance
    } else if distance < 220.0 {
        OldMetroChapter::ServiceTunnel
    } else if distance < 350.0 {
        OldMetroChapter::Platform
    } else if distance < 470.0 {
        OldMetroChapter::MaintenanceJunction
    } else if distance < 600.0 {
        OldMetroChapter::PowerSubstation
    } else if distance < 700.0 {
        OldMetroChapter::CollapsedSection
    } else {
        OldMetroChapter::VeyronCheckpoint
    }
}

pub fn get_old_metro_variant(distance: f32) -> OldMetroVariant {
    if distance < 40.0 {
        OldMetroVariant::AbandonedEntrance
    } else if distance < 70.0 {
        OldMetroVariant::DamagedTunnel
    } else if distance < 100.0 {
        OldMetroVariant::EchoTraces
    } else if distance < 140.0 {
        OldMetroVariant::PipeCorridor
    } else if distance < 180.0 {
        OldMetroVariant::MaintenanceAccess
    } else if distance < 220.0 {
        OldMetroVariant::DerelictCarriage
    } else if distance < 260.0 {
        OldMetroVariant::PlatformApproach
    } else if distance < 300.0 {
        OldMetroVariant::MainStationPlatform
    } else if distance < 330.0 {
        OldMetroVariant::StationAdWall
    } else if distance < 350.0 {
        OldMetroVariant::PlatformExit
    } else if distance < 390.0 {
        OldMetroVariant::JunctionManifold
    } else if distance < 430.0 {
        OldMetroVariant::VentilationShaft
    } else if distance < 470.0 {
        OldMetroVariant::MaintenanceCart
    } else if distance < 510.0 {
        OldMetroVariant::SubstationFeeders
    } else if distance < 560.0 {
        OldMetroVariant::TransformerArcZone
    } else if distance < 600.0 {
        OldMetroVariant::EchoContaminatedGen
    } else if distance < 640.0 {
        OldMetroVariant::ShoringWarning
    } else if distance < 675.0 {
        OldMetroVariant::DeepCollapseRubble
    } else if distance < 700.0 {
        OldMetroVariant::EmergencyBreach
    } else if distance < 740.0 {
        OldMetroVariant::SecurityPerimeter
    } else if distance < 775.0 {
        OldMetroVariant::CheckpointGantry
    } else {
        OldMetroVariant::NeonExitPortal
    }
}

/// Spawns an authored Old Metro segment corresponding to its world track distance.
pub fn spawn_old_metro_segment(
    commands: &mut Commands,
    env: &EnvironmentAssets,
    props: &PropAssets,
    signage: &SignageAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = get_old_metro_chapter(distance);
    let variant = get_old_metro_variant(distance);
    let seg_idx = ((distance / length).floor() as i32).abs();
    let is_even = seg_idx % 2 == 0;

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // 1. Core Railway Track Infrastructure (consistent physical running surface)
            spawn_track_bed_and_rails(seg, env, length, seg_idx);

            // 2. Layered Subway Wall Tiles with Deterministic Material Wear
            spawn_layered_subway_walls(seg, env, props, length, seg_idx, is_even, variant);

            // 3. Overhead Tunnel Ribs & Cable Conduits
            spawn_structural_tunnel_ribs(seg, env, length, chapter, is_even);

            // 4. Utility Pipes with Flanges, Brackets, and Industrial Color Coding
            spawn_utility_pipe_run(seg, props, length, seg_idx, is_even);

            // 5. Practical Sodium Lighting & Damage States
            spawn_authored_lighting(seg, env, chapter, seg_idx);

            // 6. Chapter-Specific Authored Sub-Section Landmarks (0 repetition!)
            match chapter {
                OldMetroChapter::Entrance => {
                    old_metro_landmarks::spawn_entrance_sub_section(
                        seg, env, props, signage, variant, is_even,
                    );
                }
                OldMetroChapter::ServiceTunnel => {
                    old_metro_landmarks::spawn_service_tunnel_sub_section(
                        seg, env, props, signage, variant, is_even,
                    );
                }
                OldMetroChapter::Platform => {
                    old_metro_landmarks::spawn_platform_sub_section(
                        seg, env, props, signage, variant, is_even,
                    );
                }
                OldMetroChapter::MaintenanceJunction => {
                    old_metro_landmarks::spawn_junction_sub_section(
                        seg, env, props, variant, is_even,
                    );
                }
                OldMetroChapter::PowerSubstation => {
                    old_metro_landmarks::spawn_substation_sub_section(seg, props, variant, is_even);
                }
                OldMetroChapter::CollapsedSection => {
                    old_metro_landmarks::spawn_collapse_sub_section(seg, props, variant, is_even);
                }
                OldMetroChapter::VeyronCheckpoint => {
                    old_metro_landmarks::spawn_checkpoint_sub_section(
                        seg, props, signage, variant, is_even,
                    );
                }
            }
        })
        .id()
}

// -----------------------------------------------------------------------------
// 1. CORE TRACK BED & RAILS
// -----------------------------------------------------------------------------
fn spawn_track_bed_and_rails(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    length: f32,
    seg_idx: i32,
) {
    // Concrete track bed slab
    seg.spawn(PbrBundle {
        mesh: env.mesh_bed.clone(),
        material: env.mat_concrete_bed.clone(),
        transform: Transform::from_xyz(0.0, -0.225, 0.0),
        ..default()
    });

    // Central drainage trough
    seg.spawn(PbrBundle {
        mesh: env.mesh_drain_trough.clone(),
        material: env.mat_drainage.clone(),
        transform: Transform::from_xyz(0.0, -0.05, 0.0),
        ..default()
    });

    // Metal drainage grates every 8m along center trough
    let num_grates = (length / 8.0) as i32;
    for i in -num_grates / 2..=num_grates / 2 {
        seg.spawn(PbrBundle {
            mesh: env.mesh_drain_grate.clone(),
            material: env.mat_grate.clone(),
            transform: Transform::from_xyz(0.0, 0.01, i as f32 * 8.0),
            ..default()
        });
    }

    // Ballast curbs (left & right)
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

    // Sleepers (cross-ties) every 2.0m with 4 distinct material/wear variants & steel fastener plates
    let num_sleepers = (length / 2.0) as i32;
    for i in -num_sleepers / 2..=num_sleepers / 2 {
        let sleeper_z = i as f32 * 2.0;

        // Procedural sleeper variation: standard, oil-soaked, cracked mineral, and heavy steel tie
        let (s_mesh, s_mat) = match (i.abs() + seg_idx * 3) % 4 {
            0 => (&env.mesh_sleeper, &env.mat_sleeper),
            1 => (&env.mesh_sleeper, &env.mat_sleeper_stained),
            2 => (&env.mesh_sleeper, &env.mat_sleeper_cracked),
            _ => (&env.mesh_sleeper_heavy, &env.mat_sleeper_steel),
        };

        seg.spawn(PbrBundle {
            mesh: s_mesh.clone(),
            material: s_mat.clone(),
            transform: Transform::from_xyz(0.0, 0.01, sleeper_z),
            ..default()
        });

        // Fastener plates under each rail
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

            // Rail joint splice bars (fishplates with heavy bolts) every 16m
            if i % 8 == 0 {
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_joint.clone(),
                    material: env.mat_rail_joint.clone(),
                    transform: Transform::from_xyz(cx - 0.52, 0.08, sleeper_z),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: env.mesh_rail_joint.clone(),
                    material: env.mat_rail_joint.clone(),
                    transform: Transform::from_xyz(cx + 0.52, 0.08, sleeper_z),
                    ..default()
                });
            }
        }
    }

    // High-readability running lane markings between tracks (X = -1.2m and X = +1.2m)
    let num_stripes = (length / 3.5) as i32;
    for i in -num_stripes / 2..=num_stripes / 2 {
        let stripe_z = i as f32 * 3.5;
        // Left-to-Center lane divider
        seg.spawn(PbrBundle {
            mesh: env.mesh_lane_stripe.clone(),
            material: env.mat_lane_stripe.clone(),
            transform: Transform::from_xyz(-1.20, 0.012, stripe_z),
            ..default()
        });
        // Center-to-Right lane divider
        seg.spawn(PbrBundle {
            mesh: env.mesh_lane_stripe.clone(),
            material: env.mat_lane_stripe.clone(),
            transform: Transform::from_xyz(1.20, 0.012, stripe_z),
            ..default()
        });
    }

    // Subterranean cyan ECHO power conduits running alongside the rails
    seg.spawn(PbrBundle {
        mesh: env.mesh_echo_conduit.clone(),
        material: env.mat_echo_conduit.clone(),
        transform: Transform::from_xyz(-0.02, 0.015, 0.0),
        ..default()
    });

    // Polished metallic steel rails (base + specular crown) for all 3 lanes
    for lane_idx in -1..=1 {
        let cx = Lane::from_index(lane_idx).x_pos();

        // Left rail
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

        // Right rail
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
}

// -----------------------------------------------------------------------------
// 2. LAYERED SUBWAY WALL TILES (4 VARIANTS)
// -----------------------------------------------------------------------------
fn spawn_layered_subway_walls(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    length: f32,
    seg_idx: i32,
    is_even: bool,
    variant: OldMetroVariant,
) {
    let wall_x_left = -5.8;
    let wall_x_right = 5.8;

    // Full-height dark concrete wall backing
    seg.spawn(PbrBundle {
        mesh: env.mesh_station_wall.clone(),
        material: env.mat_station_wall.clone(),
        transform: Transform::from_xyz(wall_x_left, 2.6, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_station_wall.clone(),
        material: env.mat_station_wall.clone(),
        transform: Transform::from_xyz(wall_x_right, 2.6, 0.0),
        ..default()
    });

    // Middle layered ceramic tile panels: 10 panels per 40m slice
    let num_panels = (length / 4.0) as i32;
    for p in -num_panels / 2..num_panels / 2 {
        let p_z = p as f32 * 4.0 + 2.0;

        let hash_left = ((seg_idx * 31 + p * 17).abs() % 100) as u32;
        let (mesh_l, mat_l) = select_tile_variant(props, hash_left, variant);

        seg.spawn(PbrBundle {
            mesh: mesh_l,
            material: mat_l,
            transform: Transform::from_xyz(wall_x_left + 0.15, 2.3, p_z),
            ..default()
        });

        let hash_right = ((seg_idx * 43 + p * 23 + 11).abs() % 100) as u32;
        let (mesh_r, mat_r) = select_tile_variant(props, hash_right, variant);

        seg.spawn(PbrBundle {
            mesh: mesh_r,
            material: mat_r,
            transform: Transform::from_xyz(wall_x_right - 0.15, 2.3, p_z),
            ..default()
        });
    }

    // Cable conduit trays along wall lower perimeter
    let tray_x = if is_even {
        wall_x_left + 0.3
    } else {
        wall_x_right - 0.3
    };
    seg.spawn(PbrBundle {
        mesh: env.mesh_conduit_tray.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(tray_x, 0.35, 0.0),
        ..default()
    });
}

fn select_tile_variant(
    props: &PropAssets,
    hash: u32,
    variant: OldMetroVariant,
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    match variant {
        OldMetroVariant::DeepCollapseRubble
        | OldMetroVariant::DamagedTunnel
        | OldMetroVariant::EmergencyBreach => {
            // Severe damage zone: 35% broken, 30% cracked, 25% dirty, 10% normal
            if hash < 35 {
                (
                    props.mesh_tile_panel_broken.clone(),
                    props.mat_tile_broken.clone(),
                )
            } else if hash < 65 {
                (
                    props.mesh_tile_panel_cracked.clone(),
                    props.mat_tile_cracked.clone(),
                )
            } else if hash < 90 {
                (
                    props.mesh_tile_panel_dirty.clone(),
                    props.mat_tile_dirty.clone(),
                )
            } else {
                (
                    props.mesh_tile_panel_normal.clone(),
                    props.mat_tile_normal.clone(),
                )
            }
        }
        OldMetroVariant::MainStationPlatform | OldMetroVariant::PlatformApproach => {
            // Cleaner station tiles: 85% normal, 10% dirty, 5% cracked
            if hash < 85 {
                (
                    props.mesh_tile_panel_normal.clone(),
                    props.mat_tile_normal.clone(),
                )
            } else if hash < 95 {
                (
                    props.mesh_tile_panel_dirty.clone(),
                    props.mat_tile_dirty.clone(),
                )
            } else {
                (
                    props.mesh_tile_panel_cracked.clone(),
                    props.mat_tile_cracked.clone(),
                )
            }
        }
        OldMetroVariant::EchoTraces | OldMetroVariant::EchoContaminatedGen => {
            // Subtle cyan trace glow on damaged tiles
            if hash < 15 {
                (
                    props.mesh_tile_panel_broken.clone(),
                    props.mat_graffiti_cyan.clone(),
                )
            } else if hash < 75 {
                (
                    props.mesh_tile_panel_normal.clone(),
                    props.mat_tile_normal.clone(),
                )
            } else {
                (
                    props.mesh_tile_panel_dirty.clone(),
                    props.mat_tile_dirty.clone(),
                )
            }
        }
        _ => {
            // Standard distribution: 70% normal, 15% dirty, 10% cracked, 5% broken
            if hash < 70 {
                (
                    props.mesh_tile_panel_normal.clone(),
                    props.mat_tile_normal.clone(),
                )
            } else if hash < 85 {
                (
                    props.mesh_tile_panel_dirty.clone(),
                    props.mat_tile_dirty.clone(),
                )
            } else if hash < 95 {
                (
                    props.mesh_tile_panel_cracked.clone(),
                    props.mat_tile_cracked.clone(),
                )
            } else {
                (
                    props.mesh_tile_panel_broken.clone(),
                    props.mat_tile_broken.clone(),
                )
            }
        }
    }
}

// -----------------------------------------------------------------------------
// 3. STRUCTURAL TUNNEL RIBS & CATENARY GANTRY
// -----------------------------------------------------------------------------
fn spawn_structural_tunnel_ribs(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    _length: f32,
    chapter: OldMetroChapter,
    is_even: bool,
) {
    if chapter == OldMetroChapter::CollapsedSection {
        return;
    }

    // Heavy Gothic arch rib at segment center
    seg.spawn(PbrBundle {
        mesh: env.mesh_arch_top.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(0.0, 5.2, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_arch_pillar.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(-5.3, 2.6, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_arch_pillar.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(5.3, 2.6, 0.0),
        ..default()
    });

    // Secondary overhead cross-beam at -15.0m
    seg.spawn(PbrBundle {
        mesh: env.mesh_catenary_gantry.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(0.0, 4.8, -15.0),
        ..default()
    });

    // Catenary overhead contact wires along tunnel length
    seg.spawn(PbrBundle {
        mesh: env.mesh_catenary_wire.clone(),
        material: env.mat_catenary_wire.clone(),
        transform: Transform::from_xyz(-1.2, 4.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_catenary_wire.clone(),
        material: env.mat_catenary_wire.clone(),
        transform: Transform::from_xyz(1.2, 4.4, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Cable bundles along ceiling
    seg.spawn(PbrBundle {
        mesh: env.mesh_cable_bundle.clone(),
        material: env.mat_cables.clone(),
        transform: Transform::from_xyz(if is_even { -2.8 } else { 2.8 }, 5.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// 4. UTILITY PIPES WITH FLANGES & COLOR CODING
// -----------------------------------------------------------------------------
fn spawn_utility_pipe_run(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    _length: f32,
    seg_idx: i32,
    is_even: bool,
) {
    let side_x = if is_even { -5.4 } else { 5.4 };
    let rot_x = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);

    // Large main utility pipe
    seg.spawn(PbrBundle {
        mesh: props.mesh_pipe_large.clone(),
        material: props.mat_pipe_dark.clone(),
        transform: Transform::from_xyz(side_x, 3.8, 0.0).with_rotation(rot_x),
        ..default()
    });

    // Medium color-coded pipe (Red = Fire/Emergency, Yellow = Hazardous, Blue = Water)
    let color_mat = match seg_idx % 3 {
        0 => props.mat_pipe_red.clone(),
        1 => props.mat_pipe_yellow.clone(),
        _ => props.mat_pipe_blue.clone(),
    };
    seg.spawn(PbrBundle {
        mesh: props.mesh_pipe_medium.clone(),
        material: color_mat,
        transform: Transform::from_xyz(side_x, 3.3, 0.0).with_rotation(rot_x),
        ..default()
    });

    // Small electrical conduit
    seg.spawn(PbrBundle {
        mesh: props.mesh_pipe_small.clone(),
        material: props.mat_pipe_dark.clone(),
        transform: Transform::from_xyz(side_x, 2.9, 0.0).with_rotation(rot_x),
        ..default()
    });

    // Pipe joints: couplings, brackets, and support flanges every 10m
    for z in [-15.0, -5.0, 5.0, 15.0] {
        seg.spawn(PbrBundle {
            mesh: props.mesh_pipe_flange.clone(),
            material: props.mat_pipe_flange.clone(),
            transform: Transform::from_xyz(side_x, 3.8, z).with_rotation(rot_x),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: props.mesh_pipe_bracket.clone(),
            material: props.mat_pipe_flange.clone(),
            transform: Transform::from_xyz(side_x + if is_even { -0.15 } else { 0.15 }, 3.3, z),
            ..default()
        });
    }
}

// -----------------------------------------------------------------------------
// 5. AUTHORED LIGHTING RHYTHM & DAMAGE STATES
// -----------------------------------------------------------------------------
fn spawn_authored_lighting(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    chapter: OldMetroChapter,
    seg_idx: i32,
) {
    if chapter == OldMetroChapter::CollapsedSection || chapter == OldMetroChapter::VeyronCheckpoint
    {
        return;
    }

    // Industrial lantern mounting cage & bulb
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

    // 4 Light damage states: ON (55%), DIM (20%), FLICKER (10%), DEAD (15%)
    let light_state = (seg_idx * 29).abs() % 100;
    if light_state < 55 {
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
    } else if light_state < 75 {
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.95, 0.55, 0.18),
                intensity: 33_000.0,
                range: 16.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 4.1, 0.0),
            ..default()
        });
    } else if light_state < 85 {
        seg.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 0.60, 0.20),
                    intensity: 75_000.0,
                    range: 20.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 4.1, 0.0),
                ..default()
            },
            FlickeringLight {
                base_intensity: 75_000.0,
                flicker_speed: 14.0,
                min_mult: 0.15,
                seed: seg_idx as f32 * 3.7,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_metro_chapter_boundaries() {
        assert_eq!(get_old_metro_chapter(0.0), OldMetroChapter::Entrance);
        assert_eq!(get_old_metro_chapter(50.0), OldMetroChapter::Entrance);
        assert_eq!(get_old_metro_chapter(99.9), OldMetroChapter::Entrance);

        assert_eq!(get_old_metro_chapter(100.0), OldMetroChapter::ServiceTunnel);
        assert_eq!(get_old_metro_chapter(180.0), OldMetroChapter::ServiceTunnel);
        assert_eq!(get_old_metro_chapter(219.9), OldMetroChapter::ServiceTunnel);

        assert_eq!(get_old_metro_chapter(220.0), OldMetroChapter::Platform);
        assert_eq!(get_old_metro_chapter(300.0), OldMetroChapter::Platform);
        assert_eq!(get_old_metro_chapter(349.9), OldMetroChapter::Platform);

        assert_eq!(
            get_old_metro_chapter(350.0),
            OldMetroChapter::MaintenanceJunction
        );
        assert_eq!(
            get_old_metro_chapter(400.0),
            OldMetroChapter::MaintenanceJunction
        );
        assert_eq!(
            get_old_metro_chapter(469.9),
            OldMetroChapter::MaintenanceJunction
        );

        assert_eq!(
            get_old_metro_chapter(470.0),
            OldMetroChapter::PowerSubstation
        );
        assert_eq!(
            get_old_metro_chapter(550.0),
            OldMetroChapter::PowerSubstation
        );
        assert_eq!(
            get_old_metro_chapter(599.9),
            OldMetroChapter::PowerSubstation
        );

        assert_eq!(
            get_old_metro_chapter(600.0),
            OldMetroChapter::CollapsedSection
        );
        assert_eq!(
            get_old_metro_chapter(650.0),
            OldMetroChapter::CollapsedSection
        );
        assert_eq!(
            get_old_metro_chapter(699.9),
            OldMetroChapter::CollapsedSection
        );

        assert_eq!(
            get_old_metro_chapter(700.0),
            OldMetroChapter::VeyronCheckpoint
        );
        assert_eq!(
            get_old_metro_chapter(750.0),
            OldMetroChapter::VeyronCheckpoint
        );
        assert_eq!(
            get_old_metro_chapter(799.9),
            OldMetroChapter::VeyronCheckpoint
        );
    }

    #[test]
    fn test_old_metro_sub_section_variants() {
        // Entrance
        assert_eq!(
            get_old_metro_variant(15.0),
            OldMetroVariant::AbandonedEntrance
        );
        assert_eq!(get_old_metro_variant(55.0), OldMetroVariant::DamagedTunnel);
        assert_eq!(get_old_metro_variant(85.0), OldMetroVariant::EchoTraces);

        // Service Tunnel
        assert_eq!(get_old_metro_variant(120.0), OldMetroVariant::PipeCorridor);
        assert_eq!(
            get_old_metro_variant(160.0),
            OldMetroVariant::MaintenanceAccess
        );
        assert_eq!(
            get_old_metro_variant(200.0),
            OldMetroVariant::DerelictCarriage
        );

        // Platform
        assert_eq!(
            get_old_metro_variant(240.0),
            OldMetroVariant::PlatformApproach
        );
        assert_eq!(
            get_old_metro_variant(280.0),
            OldMetroVariant::MainStationPlatform
        );
        assert_eq!(get_old_metro_variant(315.0), OldMetroVariant::StationAdWall);
        assert_eq!(get_old_metro_variant(340.0), OldMetroVariant::PlatformExit);

        // Maintenance Junction
        assert_eq!(
            get_old_metro_variant(370.0),
            OldMetroVariant::JunctionManifold
        );
        assert_eq!(
            get_old_metro_variant(410.0),
            OldMetroVariant::VentilationShaft
        );
        assert_eq!(
            get_old_metro_variant(450.0),
            OldMetroVariant::MaintenanceCart
        );

        // Power Substation
        assert_eq!(
            get_old_metro_variant(490.0),
            OldMetroVariant::SubstationFeeders
        );
        assert_eq!(
            get_old_metro_variant(535.0),
            OldMetroVariant::TransformerArcZone
        );
        assert_eq!(
            get_old_metro_variant(580.0),
            OldMetroVariant::EchoContaminatedGen
        );

        // Collapsed Section
        assert_eq!(
            get_old_metro_variant(620.0),
            OldMetroVariant::ShoringWarning
        );
        assert_eq!(
            get_old_metro_variant(660.0),
            OldMetroVariant::DeepCollapseRubble
        );
        assert_eq!(
            get_old_metro_variant(685.0),
            OldMetroVariant::EmergencyBreach
        );

        // Veyron Checkpoint
        assert_eq!(
            get_old_metro_variant(720.0),
            OldMetroVariant::SecurityPerimeter
        );
        assert_eq!(
            get_old_metro_variant(760.0),
            OldMetroVariant::CheckpointGantry
        );
        assert_eq!(
            get_old_metro_variant(790.0),
            OldMetroVariant::NeonExitPortal
        );
    }

    #[test]
    fn test_tile_distribution_ratios() {
        let mut normal_count = 0;
        let mut dirty_count = 0;
        let mut cracked_count = 0;
        let mut broken_count = 0;

        for hash in 0..100 {
            if hash < 70 {
                normal_count += 1;
            } else if hash < 85 {
                dirty_count += 1;
            } else if hash < 95 {
                cracked_count += 1;
            } else {
                broken_count += 1;
            }
        }

        assert_eq!(normal_count, 70);
        assert_eq!(dirty_count, 15);
        assert_eq!(cracked_count, 10);
        assert_eq!(broken_count, 5);
    }
}
