use crate::neon_district::NeonDistrictAssets;
use bevy::prelude::*;

/// Authored Landmark sub-sections for Zone 2 — Neon District (2,000m–4,000m).
/// Every sub-section guarantees deterministic, non-repetitive architectural storytelling
/// while strictly avoiding any obstruction of the playable running corridor.

// -----------------------------------------------------------------------------
// CHAPTER 1: NEON ENTRY (first 20% of Zone 2)
// -----------------------------------------------------------------------------

pub fn spawn_tunnel_exit_portal(seg: &mut ChildBuilder, neon: &NeonDistrictAssets, _is_even: bool) {
    // 1. Massive tunnel exit collar arch at the Old Metro threshold
    // Heavy structural portal framing the emergence from underground to open air
    seg.spawn(PbrBundle {
        mesh: neon.mesh_portal_arch_top.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, 7.2, 18.5),
        ..default()
    });
    // Left & right portal jambs
    seg.spawn(PbrBundle {
        mesh: neon.mesh_portal_arch_jamb.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(-5.2, 3.6, 18.5),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_portal_arch_jamb.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(5.2, 3.6, 18.5),
        ..default()
    });

    // 2. Overhead exit signage board
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_aurelia.clone(),
        transform: Transform::from_xyz(0.0, 6.2, 18.2)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    });

    // 3. Flanking portal caution light pillars
    for &side_x in &[-4.9, 4.9] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_beacon_pole.clone(),
            material: neon.mat_cyan_rail_glow.clone(),
            transform: Transform::from_xyz(side_x, 2.2, 17.5),
            ..default()
        });
    }
}

pub fn spawn_open_sky_emergence(seg: &mut ChildBuilder, neon: &NeonDistrictAssets, _is_even: bool) {
    // 1. Massive deep viaduct pylon pier plunging down into the lower city abyss
    seg.spawn(PbrBundle {
        mesh: neon.mesh_viaduct_pylon_heavy.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, -14.2, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_viaduct_crossbeam.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, -0.9, 0.0),
        ..default()
    });

    // 2. First towering corporate skyscraper emerging on the left horizon
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_large.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(-28.0, 48.0, -5.0),
        ..default()
    });
    // Vertical cyan window strip on the building facade
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_v.clone(),
        material: neon.mat_window_matrix_cyan.clone(),
        transform: Transform::from_xyz(-15.8, 38.0, -5.0),
        ..default()
    });
    // Skyscraper rooftop beacon
    seg.spawn(PbrBundle {
        mesh: neon.mesh_spire_tower.clone(),
        material: neon.mat_tower_beacon_cyan.clone(),
        transform: Transform::from_xyz(-28.0, 116.0, -5.0),
        ..default()
    });
}

pub fn spawn_first_neon_approach(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Overhead highway signage gantry with illuminated indicators
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_gantry_truss.clone(),
        transform: Transform::from_xyz(0.0, 7.8, 4.0),
        ..default()
    });

    // 2. Cantilevered holographic advertisement billboard angled toward the track
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_veyron.clone(),
        transform: Transform::from_xyz(-8.5, 4.8, 2.0).with_rotation(Quat::from_rotation_y(0.24)),
        ..default()
    });

    // 3. Cantilever mounting truss connecting billboard to viaduct edge
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_cantilever.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(-6.2, 4.8, 2.0),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 2: UPPER AURELIA (1000m – 1200m)
// -----------------------------------------------------------------------------

pub fn spawn_viaduct_pylons_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Deep dual viaduct piers with diagonal cross-bracing
    for &z_pos in &[-12.0, 12.0] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_viaduct_pylon_heavy.clone(),
            material: neon.mat_structural_gunmetal.clone(),
            transform: Transform::from_xyz(0.0, -14.2, z_pos),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: neon.mesh_viaduct_crossbeam.clone(),
            material: neon.mat_structural_gunmetal.clone(),
            transform: Transform::from_xyz(0.0, -0.9, z_pos),
            ..default()
        });
    }

    // 2. Side maintenance service catwalk on right flank
    seg.spawn(PbrBundle {
        mesh: neon.mesh_catwalk_tray.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(5.4, -0.15, 0.0),
        ..default()
    });
    // Catwalk glowing cyan safety rail
    seg.spawn(PbrBundle {
        mesh: neon.mesh_barrier_neon_trim.clone(),
        material: neon.mat_cyan_rail_glow.clone(),
        transform: Transform::from_xyz(5.8, 0.45, 0.0),
        ..default()
    });
}

pub fn spawn_overhead_power_grid_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Futuristic overhead power distribution truss spanning track
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_gantry_truss.clone(),
        transform: Transform::from_xyz(0.0, 8.2, 0.0),
        ..default()
    });

    // 2. Dual cyan magnetic induction power rings suspended over track
    for &lane_x in &[-2.5, 2.5] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_power_insulator.clone(),
            material: neon.mat_cyan_rail_glow.clone(),
            transform: Transform::from_xyz(lane_x, 7.4, 0.0),
            ..default()
        });
    }

    // 3. Mid-rise residential tower in background at right
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_medium.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(26.0, 24.0, 2.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_h.clone(),
        material: neon.mat_window_matrix_magenta.clone(),
        transform: Transform::from_xyz(16.9, 18.0, 2.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

pub fn spawn_aurelia_corporate_row_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Commercial office high-rise on left
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_large.clone(),
        material: neon.mat_building_glass.clone(),
        transform: Transform::from_xyz(-24.0, 48.0, 0.0),
        ..default()
    });
    // Horizontal glowing cyan window matrix on left building
    for y_floor in [22.0, 34.0, 48.0, 62.0] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_window_strip_h.clone(),
            material: neon.mat_window_matrix_cyan.clone(),
            transform: Transform::from_xyz(-11.9, y_floor, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }

    // 2. Vertical holographic advertising panel on right
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_vertical.clone(),
        material: neon.mat_hologram_nexus.clone(),
        transform: Transform::from_xyz(7.8, 5.2, -6.0).with_rotation(Quat::from_rotation_y(-0.20)),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 3: COMMERCIAL CORRIDOR (1200m – 1400m)
// -----------------------------------------------------------------------------

pub fn spawn_hologram_plaza_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Dual large diagonal holographic billboards on left and right
    // Left billboard: Veyron Dynamics
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_veyron.clone(),
        transform: Transform::from_xyz(-8.8, 5.4, 8.0).with_rotation(Quat::from_rotation_y(0.28)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_cantilever.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(-6.2, 5.4, 8.0),
        ..default()
    });

    // Right billboard: Nexus Synthetics
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_nexus.clone(),
        transform: Transform::from_xyz(8.8, 5.4, -8.0).with_rotation(Quat::from_rotation_y(-0.28)),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_cantilever.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(6.2, 5.4, -8.0),
        ..default()
    });

    // 2. Overhead arch connecting track barriers
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_gantry_truss.clone(),
        transform: Transform::from_xyz(0.0, 8.5, 0.0),
        ..default()
    });
}

pub fn spawn_twin_tower_skybridge_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Twin Monolithic Megastructures on left and right
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_large.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(-28.0, 50.0, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_large.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(28.0, 50.0, 0.0),
        ..default()
    });

    // 2. High-Altitude Enclosed Glass Skybridge spanning between towers at Y = 22m
    // (Well clear of camera Y = 3.8m and jump apex Y ~ 3.2m)
    seg.spawn(PbrBundle {
        mesh: neon.mesh_skybridge_tube.clone(),
        material: neon.mat_skybridge_glass.clone(),
        transform: Transform::from_xyz(0.0, 22.0, 0.0),
        ..default()
    });
    // Skybridge glowing magenta bottom accent strip
    seg.spawn(PbrBundle {
        mesh: neon.mesh_skybridge_accent.clone(),
        material: neon.mat_magenta_rail_glow.clone(),
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

pub fn spawn_media_marquee_overpass_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Overhead electronic media marquee spanning the viaduct
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_gantry_truss.clone(),
        transform: Transform::from_xyz(0.0, 7.8, 0.0),
        ..default()
    });

    // 2. Centered overhead transit marquee display
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_pulse.clone(),
        transform: Transform::from_xyz(0.0, 6.2, -0.1)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    });

    // 3. Flanking neon light columns
    for &side_x in &[-4.8, 4.8] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_beacon_pole.clone(),
            material: neon.mat_magenta_rail_glow.clone(),
            transform: Transform::from_xyz(side_x, 3.8, 0.0),
            ..default()
        });
    }
}

// -----------------------------------------------------------------------------
// CHAPTER 4: TRANSIT CORE (1400m – 1600m)
// -----------------------------------------------------------------------------

pub fn spawn_parallel_maglev_express_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Parallel Maglev Guideway Track Bed running alongside at X = -8.8m
    seg.spawn(PbrBundle {
        mesh: neon.mesh_maglev_guideway.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(-8.8, -0.15, 0.0),
        ..default()
    });
    // Guideway glowing magenta linear rail
    seg.spawn(PbrBundle {
        mesh: neon.mesh_barrier_neon_trim.clone(),
        material: neon.mat_magenta_rail_glow.clone(),
        transform: Transform::from_xyz(-8.8, 0.18, 0.0),
        ..default()
    });

    // 2. High-speed futuristic aerodynamic maglev passenger train carriage!
    // Stationary/passing alongside the player
    seg.spawn(PbrBundle {
        mesh: neon.mesh_maglev_car.clone(),
        material: neon.mat_maglev_hull.clone(),
        transform: Transform::from_xyz(-8.8, 1.8, 2.0),
        ..default()
    });
    // Maglev train illuminated cyan window speed band
    seg.spawn(PbrBundle {
        mesh: neon.mesh_maglev_stripe.clone(),
        material: neon.mat_maglev_stripe.clone(),
        transform: Transform::from_xyz(-7.45, 1.8, 2.0),
        ..default()
    });
}

pub fn spawn_central_interchange_arch_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Heavy multi-tier transit interchange arch crossing overhead
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, 9.2, -6.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, 11.2, -6.0),
        ..default()
    });

    // 2. Route advisory billboard mounted to the interchange
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_aurelia.clone(),
        transform: Transform::from_xyz(0.0, 7.8, -6.2)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    });

    // 3. Mid-rise transit monitoring tower on right
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_medium.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(24.0, 26.0, 0.0),
        ..default()
    });
}

pub fn spawn_comms_tower_array_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. High-tech communications spire building on right flank
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_medium.clone(),
        material: neon.mat_building_glass.clone(),
        transform: Transform::from_xyz(22.0, 28.0, 0.0),
        ..default()
    });
    // Tall communications antenna array
    seg.spawn(PbrBundle {
        mesh: neon.mesh_spire_tower.clone(),
        material: neon.mat_tower_beacon_cyan.clone(),
        transform: Transform::from_xyz(22.0, 68.0, 0.0),
        ..default()
    });

    // 2. Vertical holographic ad on left
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_vertical.clone(),
        material: neon.mat_hologram_veyron.clone(),
        transform: Transform::from_xyz(-7.8, 5.2, 4.0).with_rotation(Quat::from_rotation_y(0.20)),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// CHAPTER 5: VEYRON / INDUSTRIAL TRANSITION (final 20% of Zone 2)
// -----------------------------------------------------------------------------

pub fn spawn_security_grid_gantry_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Brutalist matte-black Veyron security gantry
    seg.spawn(PbrBundle {
        mesh: neon.mesh_cyber_gantry.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, 7.5, 0.0),
        ..default()
    });

    // 2. Veyron Dynamics corporate warning billboard
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_veyron.clone(),
        transform: Transform::from_xyz(0.0, 5.9, -0.1)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    });

    // 3. Surveillance scanner housings on gantry corners
    for &side_x in &[-3.8, 3.8] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_power_insulator.clone(),
            material: neon.mat_magenta_rail_glow.clone(),
            transform: Transform::from_xyz(side_x, 6.8, 0.0),
            ..default()
        });
    }
}

pub fn spawn_heavy_industrial_approach_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Reinforced industrial viaduct columns
    seg.spawn(PbrBundle {
        mesh: neon.mesh_viaduct_pylon_heavy.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, -14.2, 0.0),
        ..default()
    });

    // 2. Industrial warning billboard with hazard aesthetics
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_pulse.clone(),
        transform: Transform::from_xyz(8.2, 5.0, 0.0).with_rotation(Quat::from_rotation_y(-0.25)),
        ..default()
    });

    // 3. Dark monolithic foundry outpost building on left
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_small.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(-20.0, 14.0, 0.0),
        ..default()
    });
}

pub fn spawn_zone3_transition_portal_sub_section(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _is_even: bool,
) {
    // 1. Massive industrial approach portal frame signaling Zone 3 entrance
    seg.spawn(PbrBundle {
        mesh: neon.mesh_portal_arch_top.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(0.0, 8.2, -18.0),
        ..default()
    });
    for &side_x in &[-5.2, 5.2] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_portal_arch_jamb.clone(),
            material: neon.mat_structural_gunmetal.clone(),
            transform: Transform::from_xyz(side_x, 4.1, -18.0),
            ..default()
        });
    }

    // 2. Warning signage: APPROACHING INDUSTRIAL SECTOR
    seg.spawn(PbrBundle {
        mesh: neon.mesh_billboard_large.clone(),
        material: neon.mat_hologram_pulse.clone(),
        transform: Transform::from_xyz(0.0, 6.8, -18.2)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    });
}
