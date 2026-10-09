use crate::environment::{EnvironmentAssets, EnvironmentFan, EnvironmentalSign};
use crate::environment_lighting::{
    ElectricalArc, EmergencyBeacon, FlickeringLight, RotatingCameraMount, SteamVent,
};
use crate::environment_props::PropAssets;
use crate::environment_signage::SignageAssets;
use crate::old_metro::OldMetroVariant;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
// 1. ENTRANCE SUB-SECTIONS (0–100m)
// -----------------------------------------------------------------------------
pub fn spawn_entrance_sub_section(
    seg: &mut ChildBuilder,
    _env: &EnvironmentAssets,
    props: &PropAssets,
    signage: &SignageAssets,
    variant: OldMetroVariant,
    _is_even: bool,
) {
    match variant {
        OldMetroVariant::AbandonedEntrance => {
            // [0–40m] Abandoned Entrance: Portal Arch + Graphic Sign + Half-flicker Lighting
            seg.spawn(PbrBundle {
                mesh: props.mesh_entrance_sign.clone(),
                material: signage.mat_entrance_sign.clone(),
                transform: Transform::from_xyz(0.0, 4.6, 5.0),
                ..default()
            });

            // Left half of sign: steady warm amber
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
        OldMetroVariant::DamagedTunnel => {
            // [40–70m] Damaged Tunnel: Hanging cable loops + dripping leaks
            seg.spawn((
                PbrBundle {
                    mesh: props.mesh_steam_plume.clone(),
                    material: props.mat_steam.clone(),
                    transform: Transform::from_xyz(-2.2, 4.2, -8.0)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
                    ..default()
                },
                SteamVent {
                    timer: 0.0,
                    cycle: 3.5,
                },
            ));

            // Loose sagging cable bundle
            seg.spawn(PbrBundle {
                mesh: props.mesh_pipe_small.clone(),
                material: props.mat_pipe_dark.clone(),
                transform: Transform::from_xyz(-1.8, 4.1, 0.0)
                    .with_rotation(Quat::from_rotation_z(0.12)),
                ..default()
            });
        }
        OldMetroVariant::EchoTraces => {
            // [70–100m] First ECHO Traces: Cyan residue + cracked tiles
            seg.spawn(PbrBundle {
                mesh: props.mesh_graffiti_panel.clone(),
                material: signage.mat_graffiti_echo.clone(),
                transform: Transform::from_xyz(-5.3, 2.0, 2.0),
                ..default()
            });
            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.05, 0.85, 0.95),
                    intensity: 14_000.0,
                    range: 7.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(-4.8, 2.0, 2.0),
                ..default()
            });
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 2. SERVICE TUNNEL SUB-SECTIONS (100–220m)
// -----------------------------------------------------------------------------
pub fn spawn_service_tunnel_sub_section(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    _signage: &SignageAssets,
    variant: OldMetroVariant,
    is_even: bool,
) {
    let side_x = if is_even { -5.4 } else { 5.4 };

    match variant {
        OldMetroVariant::PipeCorridor => {
            // [100–140m] Pipe Corridor: Dense industrial utility pipes with brackets
            for y in [2.6, 3.1, 3.6] {
                seg.spawn(PbrBundle {
                    mesh: props.mesh_pipe_medium.clone(),
                    material: props.mat_pipe_dark.clone(),
                    transform: Transform::from_xyz(side_x, y, 0.0)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            }
        }
        OldMetroVariant::MaintenanceAccess => {
            // [140–180m] Maintenance Access: Breaker cabinet + side tunnel branch
            seg.spawn(PbrBundle {
                mesh: props.mesh_breaker_cabinet.clone(),
                material: props.mat_transformer_metal.clone(),
                transform: Transform::from_xyz(side_x, 1.4, 4.0),
                ..default()
            });

            // Blinking amber cabinet fault indicator
            seg.spawn((
                PointLightBundle {
                    point_light: PointLight {
                        color: Color::srgb(1.0, 0.65, 0.05),
                        intensity: 14_000.0,
                        range: 6.0,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        side_x + if is_even { 0.25 } else { -0.25 },
                        1.8,
                        4.0,
                    ),
                    ..default()
                },
                FlickeringLight {
                    base_intensity: 14_000.0,
                    flicker_speed: 8.0,
                    min_mult: 0.0,
                    seed: 88.1,
                },
            ));

            // Steam vent hiss
            seg.spawn((
                PbrBundle {
                    mesh: props.mesh_steam_plume.clone(),
                    material: props.mat_steam.clone(),
                    transform: Transform::from_xyz(
                        side_x + if is_even { 0.4 } else { -0.4 },
                        0.8,
                        -10.0,
                    ),
                    ..default()
                },
                SteamVent {
                    timer: if is_even { 0.5 } else { 1.8 },
                    cycle: 4.2,
                },
            ));

            // Branching side tunnel opening into darkness
            let side_tunnel_x = -7.5;
            seg.spawn(PbrBundle {
                mesh: env.mesh_arch_top.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(side_tunnel_x, 3.6, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.9, 0.15, 0.15),
                    intensity: 22_000.0,
                    range: 12.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(side_tunnel_x - 4.0, 2.2, 0.0),
                ..default()
            });
        }
        OldMetroVariant::DerelictCarriage => {
            // [180–220m] Derelict Train Landmark (Appears ONLY in this section!)
            let train_x = 6.4;
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
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 3. PLATFORM SUB-SECTIONS (220–350m)
// -----------------------------------------------------------------------------
pub fn spawn_platform_sub_section(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    signage: &SignageAssets,
    variant: OldMetroVariant,
    is_even: bool,
) {
    let plat_x = if is_even { 6.3 } else { -6.3 };
    let curb_x = if is_even { 5.0 } else { -5.0 };
    let wall_x = if is_even { 7.8 } else { -7.8 };

    // Elevated concrete platform slab
    seg.spawn(PbrBundle {
        mesh: env.mesh_platform_slab.clone(),
        material: env.mat_platform_concrete.clone(),
        transform: Transform::from_xyz(plat_x, 0.65, 0.0),
        ..default()
    });

    // Yellow hazard warning stripe curb using procedural hazard stripes
    seg.spawn(PbrBundle {
        mesh: env.mesh_platform_curb.clone(),
        material: signage.mat_hazard_stripes.clone(),
        transform: Transform::from_xyz(curb_x, 1.32, 0.0),
        ..default()
    });

    match variant {
        OldMetroVariant::PlatformApproach => {
            // [220–260m] Platform Approach: Station nameplate placard + turnstile remnants
            seg.spawn((
                PbrBundle {
                    mesh: props.mesh_station_nameplate.clone(),
                    material: signage.mat_station_placard.clone(),
                    transform: Transform::from_xyz(
                        wall_x - if is_even { 0.2 } else { -0.2 },
                        3.2,
                        4.0,
                    ),
                    ..default()
                },
                EnvironmentalSign,
            ));
            seg.spawn(PbrBundle {
                mesh: props.mesh_ticket_machine.clone(),
                material: props.mat_transformer_metal.clone(),
                transform: Transform::from_xyz(
                    wall_x - if is_even { 0.5 } else { -0.5 },
                    2.2,
                    -6.0,
                ),
                ..default()
            });
        }
        OldMetroVariant::MainStationPlatform => {
            // [260–300m] Main Station Platform: Timetable + Station Clock + Emergency Phone
            seg.spawn(PbrBundle {
                mesh: props.mesh_timetable_board.clone(),
                material: signage.mat_station_placard.clone(),
                transform: Transform::from_xyz(
                    wall_x - if is_even { 0.22 } else { -0.22 },
                    2.5,
                    -2.0,
                ),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_station_clock.clone(),
                material: env.mat_steel_truss.clone(),
                transform: Transform::from_xyz(curb_x + if is_even { 0.5 } else { -0.5 }, 3.5, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_emergency_phone.clone(),
                material: props.mat_pipe_red.clone(),
                transform: Transform::from_xyz(
                    wall_x - if is_even { 0.22 } else { -0.22 },
                    2.2,
                    8.5,
                ),
                ..default()
            });
        }
        OldMetroVariant::StationAdWall => {
            // [300–330m] Station Ad Wall: Benches + Veyron Poster + ECHO LIVES Graffiti
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

            // REAL VEYRON PROPAGANDA POSTER WITH GRAPHIC ARTWORK
            seg.spawn(PbrBundle {
                mesh: props.mesh_poster_board.clone(),
                material: signage.mat_poster_veyron.clone(),
                transform: Transform::from_xyz(
                    wall_x - if is_even { 0.22 } else { -0.22 },
                    2.8,
                    -8.0,
                ),
                ..default()
            });

            // REAL CYAN ECHO GRAFFITI WITH STENCIL ARTWORK
            seg.spawn(PbrBundle {
                mesh: props.mesh_graffiti_panel.clone(),
                material: signage.mat_graffiti_echo.clone(),
                transform: Transform::from_xyz(
                    wall_x - if is_even { 0.23 } else { -0.23 },
                    2.2,
                    8.0,
                ),
                ..default()
            });
            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.05, 0.85, 0.95),
                    intensity: 14_000.0,
                    range: 6.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(wall_x - if is_even { 0.5 } else { -0.5 }, 2.2, 8.0),
                ..default()
            });

            seg.spawn(PbrBundle {
                mesh: props.mesh_trash_bin.clone(),
                material: env.mat_rusted_iron.clone(),
                transform: Transform::from_xyz(plat_x + 0.3, 1.65, 4.0),
                ..default()
            });
        }
        OldMetroVariant::PlatformExit => {
            // [330–350m] Platform Exit: Directional signage leading out
            seg.spawn(PbrBundle {
                mesh: props.mesh_directional_sign.clone(),
                material: signage.mat_station_placard.clone(),
                transform: Transform::from_xyz(wall_x - if is_even { 0.2 } else { -0.2 }, 2.8, 0.0),
                ..default()
            });
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 4. MAINTENANCE JUNCTION SUB-SECTIONS (350–470m)
// -----------------------------------------------------------------------------
pub fn spawn_junction_sub_section(
    seg: &mut ChildBuilder,
    env: &EnvironmentAssets,
    props: &PropAssets,
    variant: OldMetroVariant,
    is_even: bool,
) {
    let side_x = if is_even { -5.8 } else { 5.8 };

    match variant {
        OldMetroVariant::JunctionManifold => {
            // [350–390m] Junction Manifold: Overhead pipe crossover network
            seg.spawn(PbrBundle {
                mesh: props.mesh_pipe_medium.clone(),
                material: props.mat_pipe_red.clone(),
                transform: Transform::from_xyz(0.0, 4.6, 2.0)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_pipe_large.clone(),
                material: props.mat_pipe_yellow.clone(),
                transform: Transform::from_xyz(0.0, 4.9, -6.0)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        }
        OldMetroVariant::VentilationShaft => {
            // [390–430m] Ventilation Shaft: Industrial fan in wall niche
            seg.spawn(PbrBundle {
                mesh: env.mesh_vent_housing.clone(),
                material: env.mat_vent_housing.clone(),
                transform: Transform::from_xyz(side_x, 2.8, -4.0),
                ..default()
            });
            seg.spawn((
                PbrBundle {
                    mesh: env.mesh_vent_fan.clone(),
                    material: env.mat_vent_blades.clone(),
                    transform: Transform::from_xyz(
                        side_x + if is_even { 0.15 } else { -0.15 },
                        2.8,
                        -4.0,
                    ),
                    ..default()
                },
                EnvironmentFan,
            ));
        }
        OldMetroVariant::MaintenanceCart => {
            // [430–470m] Maintenance Cart Landmark (Appears ONLY in this section!)
            let cart_x = if is_even { 4.6 } else { -4.6 };
            seg.spawn(PbrBundle {
                mesh: props.mesh_maintenance_cart.clone(),
                material: env.mat_rusted_iron.clone(),
                transform: Transform::from_xyz(cart_x, 0.35, 4.0),
                ..default()
            });
            for wz in [2.8, 5.2] {
                seg.spawn(PbrBundle {
                    mesh: props.mesh_cart_wheel.clone(),
                    material: env.mat_steel_truss.clone(),
                    transform: Transform::from_xyz(cart_x - 0.7, 0.24, wz)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
                seg.spawn(PbrBundle {
                    mesh: props.mesh_cart_wheel.clone(),
                    material: env.mat_steel_truss.clone(),
                    transform: Transform::from_xyz(cart_x + 0.7, 0.24, wz)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            }
            seg.spawn(PbrBundle {
                mesh: props.mesh_toolbox.clone(),
                material: props.mat_pipe_red.clone(),
                transform: Transform::from_xyz(cart_x - 0.3, 0.65, 3.8),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_cable_spool.clone(),
                material: props.mat_bench_wood.clone(),
                transform: Transform::from_xyz(cart_x + 0.3, 0.65, 4.6)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 5. POWER SUBSTATION SUB-SECTIONS (470–600m)
// -----------------------------------------------------------------------------
pub fn spawn_substation_sub_section(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    variant: OldMetroVariant,
    is_even: bool,
) {
    let sub_x = if is_even { -5.4 } else { 5.4 };

    match variant {
        OldMetroVariant::SubstationFeeders => {
            // [470–510m] Substation Feeders: Breaker racks and conduits
            seg.spawn(PbrBundle {
                mesh: props.mesh_breaker_cabinet.clone(),
                material: props.mat_transformer_metal.clone(),
                transform: Transform::from_xyz(sub_x, 1.2, -4.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_breaker_cabinet.clone(),
                material: props.mat_transformer_metal.clone(),
                transform: Transform::from_xyz(sub_x, 1.2, 4.0),
                ..default()
            });
        }
        OldMetroVariant::TransformerArcZone => {
            // [510–560m] Transformer Arc Zone (Sparks & Heavy Voltage ONLY here!)
            seg.spawn(PbrBundle {
                mesh: props.mesh_transformer_body.clone(),
                material: props.mat_transformer_metal.clone(),
                transform: Transform::from_xyz(sub_x, 1.4, 0.0),
                ..default()
            });
            for fz in [-0.6, 0.0, 0.6] {
                seg.spawn(PbrBundle {
                    mesh: props.mesh_transformer_fin.clone(),
                    material: props.mat_transformer_metal.clone(),
                    transform: Transform::from_xyz(
                        sub_x + if is_even { 0.85 } else { -0.85 },
                        1.4,
                        fz,
                    ),
                    ..default()
                });
            }
            for iz in [-0.5, 0.5] {
                seg.spawn(PbrBundle {
                    mesh: props.mesh_insulator_bushing.clone(),
                    material: props.mat_ceramic_insulator.clone(),
                    transform: Transform::from_xyz(sub_x, 2.8, iz),
                    ..default()
                });
            }

            // Active Electrical Arc Discharge
            seg.spawn((
                PbrBundle {
                    mesh: props.mesh_electric_arc.clone(),
                    material: props.mat_arc_spark.clone(),
                    transform: Transform::from_xyz(
                        sub_x + if is_even { 0.4 } else { -0.4 },
                        2.9,
                        0.0,
                    )
                    .with_rotation(Quat::from_rotation_y(0.4)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ElectricalArc {
                    timer: if is_even { 0.0 } else { 1.2 },
                    interval: 2.4,
                },
            ));

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
        }
        OldMetroVariant::EchoContaminatedGen => {
            // [560–600m] Echo Contaminated Generator: Cyan crystals spreading along base
            seg.spawn(PbrBundle {
                mesh: props.mesh_graffiti_panel.clone(),
                material: props.mat_graffiti_cyan.clone(),
                transform: Transform::from_xyz(sub_x + if is_even { 0.9 } else { -0.9 }, 0.2, 0.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.05, 0.95, 1.0),
                    intensity: 32_000.0,
                    range: 12.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(sub_x, 0.5, 0.0),
                ..default()
            });
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 6. COLLAPSED SECTION SUB-SECTIONS (600–700m)
// -----------------------------------------------------------------------------
pub fn spawn_collapse_sub_section(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    variant: OldMetroVariant,
    is_even: bool,
) {
    match variant {
        OldMetroVariant::ShoringWarning => {
            // [600–640m] Shoring Warning: Emergency yellow tubular shore jacks
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
        }
        OldMetroVariant::DeepCollapseRubble => {
            // [640–675m] Deep Collapse Rubble (Dramatic destruction ONLY here!)
            let pile_x = if is_even { -4.5 } else { 4.5 };
            seg.spawn(PbrBundle {
                mesh: props.mesh_rubble_chunk_large.clone(),
                material: props.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(pile_x, 0.35, -2.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.2,
                    0.5,
                    -0.3,
                )),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_rubble_chunk_med.clone(),
                material: props.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(
                    pile_x + if is_even { 0.4 } else { -0.4 },
                    0.25,
                    0.5,
                )
                .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.3, 0.1, 0.4)),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_rubble_chunk_small.clone(),
                material: props.mat_rubble_concrete.clone(),
                transform: Transform::from_xyz(
                    pile_x - if is_even { 0.3 } else { -0.3 },
                    0.15,
                    -4.0,
                ),
                ..default()
            });

            // Bent rusty rebar
            seg.spawn(PbrBundle {
                mesh: props.mesh_bent_rebar.clone(),
                material: props.mat_bent_rebar.clone(),
                transform: Transform::from_xyz(if is_even { -3.8 } else { 3.8 }, 4.2, 2.0)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.6, 0.3, 0.8)),
                ..default()
            });

            // Intermittent steam leak
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

            // Pulsating Red Emergency Beacon
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
        }
        OldMetroVariant::EmergencyBreach => {
            // [675–700m] Emergency Breach: Broken framing and exposed structure
            seg.spawn(PbrBundle {
                mesh: props.mesh_bent_rebar.clone(),
                material: props.mat_bent_rebar.clone(),
                transform: Transform::from_xyz(0.0, 4.6, 0.0)
                    .with_rotation(Quat::from_rotation_z(0.35)),
                ..default()
            });
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------
// 7. VEYRON CHECKPOINT SUB-SECTIONS (final 12.5% of Zone 1)
// -----------------------------------------------------------------------------
pub fn spawn_checkpoint_sub_section(
    seg: &mut ChildBuilder,
    props: &PropAssets,
    signage: &SignageAssets,
    variant: OldMetroVariant,
    _is_even: bool,
) {
    match variant {
        OldMetroVariant::SecurityPerimeter => {
            // [700–740m] Security Perimeter: Sweeping cameras + Warning placard + Barriers
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

            // REAL VEYRON SECURITY WARNING PLACARD
            seg.spawn(PbrBundle {
                mesh: props.mesh_security_placard.clone(),
                material: signage.mat_security_placard.clone(),
                transform: Transform::from_xyz(0.0, 3.8, 2.0),
                ..default()
            });

            // Security barriers with hazard stripes
            seg.spawn(PbrBundle {
                mesh: props.mesh_security_barrier.clone(),
                material: signage.mat_hazard_stripes.clone(),
                transform: Transform::from_xyz(-3.8, 0.55, 3.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: props.mesh_security_barrier.clone(),
                material: signage.mat_hazard_stripes.clone(),
                transform: Transform::from_xyz(3.8, 0.55, 3.0),
                ..default()
            });

            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 0.08, 0.12),
                    intensity: 65_000.0,
                    range: 16.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                ..default()
            });
        }
        OldMetroVariant::CheckpointGantry => {
            // [740–775m] Checkpoint Gantry (Appears ONLY in this section!)
            seg.spawn(PbrBundle {
                mesh: props.mesh_checkpoint_gantry.clone(),
                material: props.mat_security_black.clone(),
                transform: Transform::from_xyz(0.0, 4.8, 0.0),
                ..default()
            });
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

            seg.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 0.08, 0.12),
                    intensity: 85_000.0,
                    range: 20.0,
                    shadows_enabled: false,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 4.2, 0.0),
                ..default()
            });
        }
        OldMetroVariant::NeonExitPortal => {
            // Final checkpoint subsection: Cyan & Magenta city lights bleed into the tunnel ahead.
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
        _ => {}
    }
}
