use bevy::prelude::*;
use crate::types::*;
use crate::environment::{EnvironmentAssets, EnvironmentFan, EnvironmentalSign};
use crate::environment_props::PropAssets;
use crate::environment_lighting::{
    FlickeringLight, EmergencyBeacon, SteamVent, ElectricalArc, RotatingCameraMount,
};

/// The 7 distinct narrative chapters across Old Metro (0–800m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldMetroChapter {
    /// 0–100m: Cracked concrete tunnel portal, damaged overhead sign, hanging cables, steam leaks.
    Entrance,
    /// 100–220m: Maintenance corridor, utility pipes with colored bands, side tunnel, derelict train carriage.
    ServiceTunnel,
    /// 220–350m: Grand station platform landmark, benches, timetable, clock, posters, cyan ECHO graffiti.
    Platform,
    /// 350–470m: Industrial junction, pipe network, spinning ventilation fan, rolling maintenance cart.
    MaintenanceJunction,
    /// 470–600m: Heavy electrical transformers, ceramic insulators, circuit breakers, arcing electrical sparks.
    PowerSubstation,
    /// 600–700m: Structural roof collapse, concrete rubble piles, bent rebar, shoring jacks, red emergency beacons.
    CollapsedSection,
    /// 700–800m: Clean brutalist matte-black Veyron security checkpoint, surveillance cameras, scanner gantry, Neon District glow.
    VeyronCheckpoint,
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

/// Spawns an authored Old Metro segment corresponding to its world track distance.
pub fn spawn_old_metro_segment(
    commands: &mut Commands,
    env: &EnvironmentAssets,
    props: &PropAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = get_old_metro_chapter(distance);
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
            spawn_track_bed_and_rails(seg, env, length);

            // 2. Layered Subway Wall Tiles with Deterministic Material Wear
            spawn_layered_subway_walls(seg, env, props, length, seg_idx, is_even);

            // 3. Overhead Tunnel Ribs & Cable Conduits
            spawn_structural_tunnel_ribs(seg, env, length, chapter, is_even);

            // 4. Utility Pipes with Flanges, Brackets, and Industrial Color Coding
            spawn_utility_pipe_run(seg, props, length, seg_idx, is_even);

            // 5. Practical Sodium Lighting & Damage States
            spawn_authored_lighting(seg, env, chapter, seg_idx);

            // 6. Chapter-Specific Authored Narrative Landmarks
            match chapter {
                OldMetroChapter::Entrance => {
                    spawn_entrance_chapter(seg, env, props, distance, is_even);
                }
                OldMetroChapter::ServiceTunnel => {
                    spawn_service_tunnel_chapter(seg, env, props, distance, is_even);
                }
                OldMetroChapter::Platform => {
                    spawn_platform_chapter(seg, env, props, distance, is_even);
                }
                OldMetroChapter::MaintenanceJunction => {
                    spawn_junction_chapter(seg, env, props, distance, is_even);
                }
                OldMetroChapter::PowerSubstation => {
                    spawn_substation_chapter(seg, props, distance, is_even);
                }
                OldMetroChapter::CollapsedSection => {
                    spawn_collapse_chapter(seg, props, distance, is_even);
                }
                OldMetroChapter::VeyronCheckpoint => {
                    spawn_checkpoint_chapter(seg, props, distance, is_even);
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
        }
    }

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

        // Deterministic tile variant selection: 70% normal, 15% dirty, 10% cracked, 5% broken
        let hash_left = ((seg_idx * 31 + p * 17).abs() % 100) as u32;
        let (mesh_l, mat_l) = select_tile_variant(props, hash_left);

        seg.spawn(PbrBundle {
            mesh: mesh_l,
            material: mat_l,
            transform: Transform::from_xyz(wall_x_left + 0.15, 2.3, p_z),
            ..default()
        });

        let hash_right = ((seg_idx * 43 + p * 23 + 11).abs() % 100) as u32;
        let (mesh_r, mat_r) = select_tile_variant(props, hash_right);

        seg.spawn(PbrBundle {
            mesh: mesh_r,
            material: mat_r,
            transform: Transform::from_xyz(wall_x_right - 0.15, 2.3, p_z),
            ..default()
        });
    }

    // Cable conduit trays along wall lower perimeter
    let tray_x = if is_even { wall_x_left + 0.3 } else { wall_x_right - 0.3 };
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
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    if hash < 70 {
        (props.mesh_tile_panel_normal.clone(), props.mat_tile_normal.clone())
    } else if hash < 85 {
        (props.mesh_tile_panel_dirty.clone(), props.mat_tile_dirty.clone())
    } else if hash < 95 {
        (props.mesh_tile_panel_cracked.clone(), props.mat_tile_cracked.clone())
    } else {
        (props.mesh_tile_panel_broken.clone(), props.mat_tile_broken.clone())
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
        // Collapsed section replaces normal arches with twisted framing in its own builder
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
        transform: Transform::from_xyz(-1.2, 4.4, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_catenary_wire.clone(),
        material: env.mat_catenary_wire.clone(),
        transform: Transform::from_xyz(1.2, 4.4, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Cable bundles along ceiling
    seg.spawn(PbrBundle {
        mesh: env.mesh_cable_bundle.clone(),
        material: env.mat_cables.clone(),
        transform: Transform::from_xyz(if is_even { -2.8 } else { 2.8 }, 5.0, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
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
    if chapter == OldMetroChapter::CollapsedSection || chapter == OldMetroChapter::VeyronCheckpoint {
        // These chapters author their own specialized lighting
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
        // ON: full warm amber
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
        // DIM: ~35% intensity
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
        // FLICKER: unsteady, failing bulb
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
    // else: DEAD (no point light, dark cold bulb)
}

// -----------------------------------------------------------------------------
// CHAPTER 1: 0–100m ABANDONED ENTRANCE
// -----------------------------------------------------------------------------
fn spawn_entrance_chapter(
    seg: &mut ChildBuilder,
    _env: &EnvironmentAssets,
    props: &PropAssets,
    distance: f32,
    _is_even: bool,
) {
    // Massive overhead entrance sign at beginning of the run (0–50m)
    if distance < 60.0 {
        seg.spawn(PbrBundle {
            mesh: props.mesh_entrance_sign.clone(),
            material: props.mat_station_sign_amber.clone(),
            transform: Transform::from_xyz(-1.7, 4.6, 5.0),
            ..default()
        });

        // Left half of sign: steady dim amber
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(1.0, 0.70, 0.20),
                intensity: 45_000.0,
                range: 14.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(-2.5, 4.4, 5.2),
            ..default()
        });

        // Right half of sign: damaged & flickering
        seg.spawn(PbrBundle {
            mesh: props.mesh_station_nameplate.clone(),
            material: props.mat_station_sign_flicker.clone(),
            transform: Transform::from_xyz(1.7, 4.6, 5.05),
            ..default()
        });
        seg.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.85, 0.55, 0.15),
                    intensity: 55_000.0,
                    range: 15.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(2.0, 4.4, 5.2),
                ..default()
            },
            FlickeringLight {
                base_intensity: 55_000.0,
                flicker_speed: 18.5,
                min_mult: 0.05,
                seed: 12.34,
            },
        ));
    }

    // Intermittent ceiling water/steam drip
    seg.spawn((
        PbrBundle {
            mesh: props.mesh_steam_plume.clone(),
            material: props.mat_steam.clone(),
            transform: Transform::from_xyz(-2.2, 4.2, -8.0).with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
            ..default()
        },
        SteamVent {
            timer: 0.0,
            cycle: 3.5,
        },
    ));
}

// -----------------------------------------------------------------------------
// CHAPTER 2: 100–220m SERVICE TUNNEL & DERELICT CARRIAGE
// -----------------------------------------------------------------------------
fn spawn_service_tunnel_chapter(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    distance: f32,
    is_even: bool,
) {
    // Electrical cabinet with amber fault light
    let cab_x = if is_even { -5.3 } else { 5.3 };
    seg.spawn(PbrBundle {
        mesh: props.mesh_breaker_cabinet.clone(),
        material: props.mat_transformer_metal.clone(),
        transform: Transform::from_xyz(cab_x, 1.4, 4.0),
        ..default()
    });

    // Blinking amber cabinet fault indicator
    seg.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(1.0, 0.65, 0.05),
                intensity: 12_000.0,
                range: 6.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(cab_x + if is_even { 0.25 } else { -0.25 }, 1.8, 4.0),
            ..default()
        },
        FlickeringLight {
            base_intensity: 12_000.0,
            flicker_speed: 8.0,
            min_mult: 0.0,
            seed: 88.1,
        },
    ));

    // Active steam vent hiss along wall pipe
    seg.spawn((
        PbrBundle {
            mesh: props.mesh_steam_plume.clone(),
            material: props.mat_steam.clone(),
            transform: Transform::from_xyz(cab_x + if is_even { 0.4 } else { -0.4 }, 0.8, -10.0),
            ..default()
        },
        SteamVent {
            timer: if is_even { 0.5 } else { 1.8 },
            cycle: 4.2,
        },
    ));

    // Branching side maintenance tunnel opening in background (at 120-160m)
    if (120.0..160.0).contains(&distance) {
        let side_tunnel_x = -7.5;
        // Archway framing into darkness
        seg.spawn(PbrBundle {
            mesh: env.mesh_arch_top.clone(),
            material: env.mat_steel_truss.clone(),
            transform: Transform::from_xyz(side_tunnel_x, 3.6, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        // Distant red service light down side tunnel
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.9, 0.15, 0.15),
                intensity: 24_000.0,
                range: 12.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(side_tunnel_x - 4.0, 2.2, 0.0),
            ..default()
        });
    }

    // MAJOR LANDMARK: Derelict Abandoned Metro Carriage on siding track (180–220m)
    if (180.0..220.0).contains(&distance) {
        let train_x = 6.4;
        // Main carriage chassis
        seg.spawn(PbrBundle {
            mesh: props.mesh_derelict_carriage.clone(),
            material: props.mat_derelict_train.clone(),
            transform: Transform::from_xyz(train_x, 1.8, 0.0),
            ..default()
        });

        // Dark windows along side facing track
        for wz in [-4.5, -1.5, 4.5] {
            seg.spawn(PbrBundle {
                mesh: props.mesh_carriage_window.clone(),
                material: props.mat_train_window_dark.clone(),
                transform: Transform::from_xyz(train_x - 1.42, 2.0, wz),
                ..default()
            });
        }

        // ONE FAINT LIT INTERIOR WINDOW (Visual mystery)
        seg.spawn(PbrBundle {
            mesh: props.mesh_carriage_window.clone(),
            material: props.mat_train_window_lit.clone(),
            transform: Transform::from_xyz(train_x - 1.42, 2.0, 1.5),
            ..default()
        });
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.95, 0.75, 0.35),
                intensity: 18_000.0,
                range: 8.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(train_x - 1.0, 2.0, 1.5),
            ..default()
        });

        // Half-open passenger door
        seg.spawn(PbrBundle {
            mesh: props.mesh_carriage_door.clone(),
            material: props.mat_derelict_train.clone(),
            transform: Transform::from_xyz(train_x - 1.44, 1.5, -1.8),
            ..default()
        });
    }
}

// -----------------------------------------------------------------------------
// CHAPTER 3: 220–350m OLD METRO PLATFORM LANDMARK
// -----------------------------------------------------------------------------
fn spawn_platform_chapter(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    _distance: f32,
    is_even: bool,
) {
    let plat_x = if is_even { 6.3 } else { -6.3 };
    let curb_x = if is_even { 5.0 } else { -5.0 };
    let wall_x = if is_even { 7.8 } else { -7.8 };

    // Elevated concrete passenger platform slab
    seg.spawn(PbrBundle {
        mesh: env.mesh_platform_slab.clone(),
        material: env.mat_platform_concrete.clone(),
        transform: Transform::from_xyz(plat_x, 0.65, 0.0),
        ..default()
    });

    // Yellow hazard warning stripe curb
    seg.spawn(PbrBundle {
        mesh: env.mesh_platform_curb.clone(),
        material: env.mat_hazard_stripe.clone(),
        transform: Transform::from_xyz(curb_x, 1.32, 0.0),
        ..default()
    });

    // STATION IDENTITY PLACARD: SECTOR 04 OLD METRO // PLATFORM 07
    seg.spawn((
        PbrBundle {
            mesh: props.mesh_station_nameplate.clone(),
            material: props.mat_station_sign_amber.clone(),
            transform: Transform::from_xyz(wall_x - if is_even { 0.2 } else { -0.2 }, 3.2, 6.0),
            ..default()
        },
        EnvironmentalSign,
    ));

    // Directional Sign: AURELIA CENTRAL -> SURFACE <- INDUSTRIAL DISTRICT
    seg.spawn(PbrBundle {
        mesh: props.mesh_directional_sign.clone(),
        material: props.mat_station_sign_amber.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.2 } else { -0.2 }, 2.8, -8.0),
        ..default()
    });

    // Broken wooden station benches
    seg.spawn(PbrBundle {
        mesh: props.mesh_bench_seat.clone(),
        material: props.mat_bench_wood.clone(),
        transform: Transform::from_xyz(plat_x, 1.65, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_bench_legs.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(plat_x, 1.45, -0.8),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_bench_legs.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(plat_x, 1.45, 0.8),
        ..default()
    });

    // Station timetable board & round station clock
    seg.spawn(PbrBundle {
        mesh: props.mesh_timetable_board.clone(),
        material: props.mat_poster_veyron.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.22 } else { -0.22 }, 2.5, -2.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_station_clock.clone(),
        material: env.mat_steel_truss.clone(),
        transform: Transform::from_xyz(curb_x + if is_even { 0.5 } else { -0.5 }, 3.5, 0.0),
        ..default()
    });

    // Ticket vending machine & emergency phone callbox
    seg.spawn(PbrBundle {
        mesh: props.mesh_ticket_machine.clone(),
        material: props.mat_transformer_metal.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.5 } else { -0.5 }, 2.2, 12.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_emergency_phone.clone(),
        material: props.mat_pipe_red.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.22 } else { -0.22 }, 2.2, 8.5),
        ..default()
    });

    // Trash bin & platform ladder
    seg.spawn(PbrBundle {
        mesh: props.mesh_trash_bin.clone(),
        material: env.mat_rusted_iron.clone(),
        transform: Transform::from_xyz(plat_x + 0.3, 1.65, 4.0),
        ..default()
    });

    // Corporate Advertisement Poster: "VEYRON DYNAMICS: BUILD TOMORROW. TODAY."
    seg.spawn(PbrBundle {
        mesh: props.mesh_poster_board.clone(),
        material: props.mat_poster_veyron.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.22 } else { -0.22 }, 2.8, -12.0),
        ..default()
    });

    // Cyan Resistance Graffiti: "ECHO LIVES" (Subtle atmospheric glow)
    seg.spawn(PbrBundle {
        mesh: props.mesh_graffiti_panel.clone(),
        material: props.mat_graffiti_cyan.clone(),
        transform: Transform::from_xyz(wall_x - if is_even { 0.23 } else { -0.23 }, 2.0, 2.0),
        ..default()
    });
    seg.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(0.05, 0.85, 0.95),
            intensity: 8_000.0,
            range: 5.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(wall_x - if is_even { 0.5 } else { -0.5 }, 2.0, 2.0),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 4: 350–470m MAINTENANCE JUNCTION & ROLLING CART
// -----------------------------------------------------------------------------
fn spawn_junction_chapter(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    _distance: f32,
    is_even: bool,
) {
    let side_x = if is_even { -5.8 } else { 5.8 };

    // Industrial ventilation shaft with spinning blades
    seg.spawn(PbrBundle {
        mesh: env.mesh_vent_housing.clone(),
        material: env.mat_vent_housing.clone(),
        transform: Transform::from_xyz(side_x, 2.8, -6.0),
        ..default()
    });
    seg.spawn((
        PbrBundle {
            mesh: env.mesh_vent_fan.clone(),
            material: env.mat_vent_blades.clone(),
            transform: Transform::from_xyz(side_x + if is_even { 0.15 } else { -0.15 }, 2.8, -6.0),
            ..default()
        },
        EnvironmentFan,
    ));

    // Overhead pipe crossing network with valves
    seg.spawn(PbrBundle {
        mesh: props.mesh_pipe_medium.clone(),
        material: props.mat_pipe_red.clone(),
        transform: Transform::from_xyz(0.0, 4.6, 2.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // MAJOR LANDMARK: Abandoned Maintenance Cart on track shoulder
    let cart_x = if is_even { 4.6 } else { -4.6 };
    // Cart flatbed chassis
    seg.spawn(PbrBundle {
        mesh: props.mesh_maintenance_cart.clone(),
        material: env.mat_rusted_iron.clone(),
        transform: Transform::from_xyz(cart_x, 0.35, 4.0),
        ..default()
    });
    // 4 flanged rail wheels
    for wz in [2.8, 5.2] {
        seg.spawn(PbrBundle {
            mesh: props.mesh_cart_wheel.clone(),
            material: env.mat_steel_truss.clone(),
            transform: Transform::from_xyz(cart_x - 0.7, 0.24, wz).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: props.mesh_cart_wheel.clone(),
            material: env.mat_steel_truss.clone(),
            transform: Transform::from_xyz(cart_x + 0.7, 0.24, wz).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }
    // Industrial metal toolbox on cart
    seg.spawn(PbrBundle {
        mesh: props.mesh_toolbox.clone(),
        material: props.mat_pipe_red.clone(),
        transform: Transform::from_xyz(cart_x - 0.3, 0.65, 3.8),
        ..default()
    });
    // Cable spool next to cart
    seg.spawn(PbrBundle {
        mesh: props.mesh_cable_spool.clone(),
        material: props.mat_bench_wood.clone(),
        transform: Transform::from_xyz(cart_x + 0.3, 0.65, 4.6).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 5: 470–600m POWER SUBSTATION & ELECTRICAL ARCS
// -----------------------------------------------------------------------------
fn spawn_substation_chapter(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    _distance: f32,
    is_even: bool,
) {
    let sub_x = if is_even { -5.4 } else { 5.4 };

    // Massive electrical transformer block
    seg.spawn(PbrBundle {
        mesh: props.mesh_transformer_body.clone(),
        material: props.mat_transformer_metal.clone(),
        transform: Transform::from_xyz(sub_x, 1.4, 0.0),
        ..default()
    });

    // Ribbed cooling fins on transformer sides
    for fz in [-0.6, 0.0, 0.6] {
        seg.spawn(PbrBundle {
            mesh: props.mesh_transformer_fin.clone(),
            material: props.mat_transformer_metal.clone(),
            transform: Transform::from_xyz(sub_x + if is_even { 0.85 } else { -0.85 }, 1.4, fz),
            ..default()
        });
    }

    // Ceramic high-voltage insulator bushings on top of transformer
    for iz in [-0.5, 0.5] {
        seg.spawn(PbrBundle {
            mesh: props.mesh_insulator_bushing.clone(),
            material: props.mat_ceramic_insulator.clone(),
            transform: Transform::from_xyz(sub_x, 2.8, iz),
            ..default()
        });
    }

    // High-voltage circuit breaker cabinets
    seg.spawn(PbrBundle {
        mesh: props.mesh_breaker_cabinet.clone(),
        material: props.mat_transformer_metal.clone(),
        transform: Transform::from_xyz(sub_x, 1.2, -6.0),
        ..default()
    });

    // DYNAMIC ELECTRICAL ARC SPARK (Cyan-white discharge flashing intermittently)
    seg.spawn((
        PbrBundle {
            mesh: props.mesh_electric_arc.clone(),
            material: props.mat_arc_spark.clone(),
            transform: Transform::from_xyz(sub_x + if is_even { 0.4 } else { -0.4 }, 2.9, 0.0).with_rotation(Quat::from_rotation_y(0.4)),
            visibility: Visibility::Hidden,
            ..default()
        },
        ElectricalArc {
            timer: if is_even { 0.0 } else { 1.2 },
            interval: 2.4,
        },
    ));

    // Cyan electrical discharge point light
    seg.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.4, 0.85, 1.0),
                intensity: 65_000.0,
                range: 16.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(sub_x, 2.9, 0.0),
            ..default()
        },
        FlickeringLight {
            base_intensity: 65_000.0,
            flicker_speed: 25.0,
            min_mult: 0.0,
            seed: 47.3,
        },
    ));

    // Early Cyan ECHO Contamination creeping along transformer base
    seg.spawn(PbrBundle {
        mesh: props.mesh_graffiti_panel.clone(),
        material: props.mat_graffiti_cyan.clone(),
        transform: Transform::from_xyz(sub_x + if is_even { 0.9 } else { -0.9 }, 0.2, 0.0).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 6: 600–700m COLLAPSED SECTION & EMERGENCY RED BEACONS
// -----------------------------------------------------------------------------
fn spawn_collapse_chapter(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    _distance: f32,
    is_even: bool,
) {
    // Heavy emergency tubular steel shore jacks propping up ceiling
    seg.spawn(PbrBundle {
        mesh: props.mesh_emergency_shore_jack.clone(),
        material: props.mat_pipe_yellow.clone(),
        transform: Transform::from_xyz(-4.6, 2.4, -4.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_emergency_shore_jack.clone(),
        material: props.mat_pipe_yellow.clone(),
        transform: Transform::from_xyz(4.6, 2.4, 6.0),
        ..default()
    });

    // Concrete rubble piles on shoulders
    let pile_x = if is_even { -4.5 } else { 4.5 };
    seg.spawn(PbrBundle {
        mesh: props.mesh_rubble_chunk_large.clone(),
        material: props.mat_rubble_concrete.clone(),
        transform: Transform::from_xyz(pile_x, 0.35, -2.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.2, 0.5, -0.3)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_rubble_chunk_med.clone(),
        material: props.mat_rubble_concrete.clone(),
        transform: Transform::from_xyz(pile_x + if is_even { 0.4 } else { -0.4 }, 0.25, 0.5).with_rotation(Quat::from_euler(EulerRot::XYZ, -0.3, 0.1, 0.4)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_rubble_chunk_small.clone(),
        material: props.mat_rubble_concrete.clone(),
        transform: Transform::from_xyz(pile_x - if is_even { 0.3 } else { -0.3 }, 0.15, -4.0),
        ..default()
    });

    // Bent rusty rebar protruding from ceiling / fracture zone
    seg.spawn(PbrBundle {
        mesh: props.mesh_bent_rebar.clone(),
        material: props.mat_bent_rebar.clone(),
        transform: Transform::from_xyz(if is_even { -3.8 } else { 3.8 }, 4.2, 2.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.6, 0.3, 0.8)),
        ..default()
    });

    // Intermittent steam leak hissing from ruptured pipe
    seg.spawn((
        PbrBundle {
            mesh: props.mesh_steam_plume.clone(),
            material: props.mat_steam.clone(),
            transform: Transform::from_xyz(pile_x, 0.4, -2.0),
            ..default()
        },
        SteamVent {
            timer: if is_even { 0.2 } else { 1.5 },
            cycle: 2.8,
        },
    ));

    // Shift to Tense Emergency Red Lighting (Pulsating Red Beacons)
    seg.spawn(PbrBundle {
        mesh: props.mesh_camera_lens.clone(),
        material: props.mat_emergency_red_light.clone(),
        transform: Transform::from_xyz(0.0, 4.3, 0.0),
        ..default()
    });
    seg.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(1.0, 0.15, 0.15),
                intensity: 85_000.0,
                range: 22.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 4.2, 0.0),
            ..default()
        },
        EmergencyBeacon {
            frequency: 1.8,
            phase: if is_even { 0.0 } else { std::f32::consts::PI },
            base_intensity: 85_000.0,
        },
    ));

    // Damaged flickering amber lantern in the background
    seg.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.9, 0.45, 0.1),
                intensity: 35_000.0,
                range: 14.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(if is_even { 3.5 } else { -3.5 }, 3.5, -12.0),
            ..default()
        },
        FlickeringLight {
            base_intensity: 35_000.0,
            flicker_speed: 22.0,
            min_mult: 0.05,
            seed: 61.9,
        },
    ));
}

// -----------------------------------------------------------------------------
// CHAPTER 7: 700–800m VEYRON CHECKPOINT & TRANSITION TO NEON DISTRICT
// -----------------------------------------------------------------------------
fn spawn_checkpoint_chapter(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    distance: f32,
    _is_even: bool,
) {
    // Heavy Brutalist Matte-Black Checkpoint Gantry
    seg.spawn(PbrBundle {
        mesh: props.mesh_checkpoint_gantry.clone(),
        material: props.mat_security_black.clone(),
        transform: Transform::from_xyz(0.0, 4.8, 0.0),
        ..default()
    });

    // Scanner towers flanking track
    seg.spawn(PbrBundle {
        mesh: props.mesh_checkpoint_scanner.clone(),
        material: props.mat_security_black.clone(),
        transform: Transform::from_xyz(-4.8, 2.4, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_checkpoint_scanner.clone(),
        material: props.mat_security_black.clone(),
        transform: Transform::from_xyz(4.8, 2.4, 0.0),
        ..default()
    });

    // Security barriers flanking the rails
    seg.spawn(PbrBundle {
        mesh: props.mesh_security_barrier.clone(),
        material: props.mat_security_black.clone(),
        transform: Transform::from_xyz(-3.8, 0.55, 3.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: props.mesh_security_barrier.clone(),
        material: props.mat_security_black.clone(),
        transform: Transform::from_xyz(3.8, 0.55, 3.0),
        ..default()
    });

    // Security warning placards: VEYRON DYNAMICS // AUTHORIZED PERSONNEL ONLY
    seg.spawn(PbrBundle {
        mesh: props.mesh_security_placard.clone(),
        material: props.mat_security_red_led.clone(),
        transform: Transform::from_xyz(0.0, 4.2, 0.85),
        ..default()
    });

    // Surveillance cameras with sweeping red LED optic eye
    seg.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(-4.6, 4.2, 1.2)),
        RotatingCameraMount {
            sweep_speed: 1.2,
            max_angle: 0.65,
        },
    ))
    .with_children(|cam| {
        cam.spawn(PbrBundle {
            mesh: props.mesh_camera_mount.clone(),
            material: props.mat_security_black.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
        cam.spawn(PbrBundle {
            mesh: props.mesh_camera_housing.clone(),
            material: props.mat_security_black.clone(),
            transform: Transform::from_xyz(0.2, -0.1, 0.0),
            ..default()
        });
        cam.spawn(PbrBundle {
            mesh: props.mesh_camera_lens.clone(),
            material: props.mat_security_red_led.clone(),
            transform: Transform::from_xyz(0.35, -0.1, 0.2),
            ..default()
        });
    });

    // Red Security Spot Light
    seg.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(1.0, 0.08, 0.12),
            intensity: 75_000.0,
            range: 18.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    });

    // AT 750–800m: NEON DISTRICT TRANSITION GLOW BLEEDING THROUGH EXIT ARCH!
    if distance >= 740.0 {
        // Cyan & Magenta atmospheric city lights ahead
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(0.05, 0.95, 1.0),
                intensity: 120_000.0,
                range: 35.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(-3.0, 3.5, -18.0),
            ..default()
        });
        seg.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::srgb(1.0, 0.15, 0.85),
                intensity: 110_000.0,
                range: 35.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(3.0, 3.5, -18.0),
            ..default()
        });
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

        assert_eq!(get_old_metro_chapter(350.0), OldMetroChapter::MaintenanceJunction);
        assert_eq!(get_old_metro_chapter(400.0), OldMetroChapter::MaintenanceJunction);
        assert_eq!(get_old_metro_chapter(469.9), OldMetroChapter::MaintenanceJunction);

        assert_eq!(get_old_metro_chapter(470.0), OldMetroChapter::PowerSubstation);
        assert_eq!(get_old_metro_chapter(550.0), OldMetroChapter::PowerSubstation);
        assert_eq!(get_old_metro_chapter(599.9), OldMetroChapter::PowerSubstation);

        assert_eq!(get_old_metro_chapter(600.0), OldMetroChapter::CollapsedSection);
        assert_eq!(get_old_metro_chapter(650.0), OldMetroChapter::CollapsedSection);
        assert_eq!(get_old_metro_chapter(699.9), OldMetroChapter::CollapsedSection);

        assert_eq!(get_old_metro_chapter(700.0), OldMetroChapter::VeyronCheckpoint);
        assert_eq!(get_old_metro_chapter(750.0), OldMetroChapter::VeyronCheckpoint);
        assert_eq!(get_old_metro_chapter(799.9), OldMetroChapter::VeyronCheckpoint);
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
