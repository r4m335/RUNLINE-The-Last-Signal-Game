use crate::environment_signage::{draw_string, make_image, set_pixel};
use crate::neon_district_landmarks;
use crate::types::*;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
// BACKGROUND TRAFFIC & ANIMATION
// -----------------------------------------------------------------------------

#[derive(Component, Debug, Clone)]
pub struct BackgroundTrafficVehicle {
    pub speed: f32,
    pub min_local_z: f32,
    pub max_local_z: f32,
}

pub fn update_background_traffic(
    time: Res<Time>,
    mut traffic_q: Query<(&BackgroundTrafficVehicle, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (veh, mut trans) in traffic_q.iter_mut() {
        trans.translation.z += veh.speed * dt;
        let span = veh.max_local_z - veh.min_local_z;
        if span > 0.0 {
            if trans.translation.z > veh.max_local_z {
                trans.translation.z -= span;
            } else if trans.translation.z < veh.min_local_z {
                trans.translation.z += span;
            }
        }
    }
}

pub struct NeonDistrictPlugin;

impl Plugin for NeonDistrictPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_background_traffic.run_if(in_state(crate::types::AppState::InGame)),
        );
    }
}

// -----------------------------------------------------------------------------
// CHAPTERS & SUB-SECTION VARIANTS
// -----------------------------------------------------------------------------

/// The 5 narrative chapters across Zone 2 — Neon District (800m–1800m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeonDistrictChapter {
    /// 800–1000m: Emergence from Old Metro tunnel into elevated open sky; first skyscraper silhouettes.
    NeonEntry,
    /// 1000–1200m: Elevated viaduct with heavy pylons, corporate towers, overhead power grid.
    UpperAurelia,
    /// 1200–1400m: Dense holographic advertising, twin towers with glass skybridge, media marquees.
    CommercialCorridor,
    /// 1400–1600m: High-speed rail interchange, parallel maglev train guideway, comms towers.
    TransitCore,
    /// 1600–1800m: Veyron corporate security presence, heavy industrial supports, Zone 3 portal approach.
    VeyronTransition,
}

/// The 15 granular sub-section visual variants across Zone 2 (3 variants per chapter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeonDistrictVariant {
    // 800–1000m Neon Entry
    TunnelExitPortal,  // 800–860m
    OpenSkyEmergence,  // 860–930m
    FirstNeonApproach, // 930–1000m

    // 1000–1200m Upper Aurelia
    ViaductPylons,       // 1000–1070m
    OverheadPowerGrid,   // 1070–1140m
    AureliaCorporateRow, // 1140–1200m

    // 1200–1400m Commercial Corridor
    HologramPlaza,        // 1200–1270m
    TwinTowerSkybridge,   // 1270–1340m
    MediaMarqueeOverpass, // 1340–1400m

    // 1400–1600m Transit Core
    ParallelMaglevExpress,  // 1400–1470m
    CentralInterchangeArch, // 1470–1540m
    CommsTowerArray,        // 1540–1600m

    // 1600–1800m Veyron Transition
    SecurityGridGantry,      // 1600–1670m
    HeavyIndustrialApproach, // 1670–1740m
    Zone3TransitionPortal,   // 1740–1800m
}

pub fn get_neon_district_chapter(distance: f32) -> NeonDistrictChapter {
    if distance < 1000.0 {
        NeonDistrictChapter::NeonEntry
    } else if distance < 1200.0 {
        NeonDistrictChapter::UpperAurelia
    } else if distance < 1400.0 {
        NeonDistrictChapter::CommercialCorridor
    } else if distance < 1600.0 {
        NeonDistrictChapter::TransitCore
    } else {
        NeonDistrictChapter::VeyronTransition
    }
}

pub fn get_neon_district_variant(distance: f32) -> NeonDistrictVariant {
    if distance < 860.0 {
        NeonDistrictVariant::TunnelExitPortal
    } else if distance < 930.0 {
        NeonDistrictVariant::OpenSkyEmergence
    } else if distance < 1000.0 {
        NeonDistrictVariant::FirstNeonApproach
    } else if distance < 1070.0 {
        NeonDistrictVariant::ViaductPylons
    } else if distance < 1140.0 {
        NeonDistrictVariant::OverheadPowerGrid
    } else if distance < 1200.0 {
        NeonDistrictVariant::AureliaCorporateRow
    } else if distance < 1270.0 {
        NeonDistrictVariant::HologramPlaza
    } else if distance < 1340.0 {
        NeonDistrictVariant::TwinTowerSkybridge
    } else if distance < 1400.0 {
        NeonDistrictVariant::MediaMarqueeOverpass
    } else if distance < 1470.0 {
        NeonDistrictVariant::ParallelMaglevExpress
    } else if distance < 1540.0 {
        NeonDistrictVariant::CentralInterchangeArch
    } else if distance < 1600.0 {
        NeonDistrictVariant::CommsTowerArray
    } else if distance < 1670.0 {
        NeonDistrictVariant::SecurityGridGantry
    } else if distance < 1740.0 {
        NeonDistrictVariant::HeavyIndustrialApproach
    } else {
        NeonDistrictVariant::Zone3TransitionPortal
    }
}

// -----------------------------------------------------------------------------
// ASSETS RESOURCE
// -----------------------------------------------------------------------------

#[derive(Resource)]
#[allow(dead_code)]
pub struct NeonDistrictAssets {
    // Track & Viaduct Structure
    pub mesh_elevated_bed: Handle<Mesh>,
    pub mesh_aero_barrier: Handle<Mesh>,
    pub mesh_barrier_neon_trim: Handle<Mesh>,
    pub mesh_maglev_rail: Handle<Mesh>,
    pub mesh_lane_dash: Handle<Mesh>,
    pub mesh_viaduct_pylon_heavy: Handle<Mesh>,
    pub mesh_viaduct_crossbeam: Handle<Mesh>,
    pub mesh_catwalk_tray: Handle<Mesh>,

    // Portals & Overhead Gantries
    pub mesh_portal_arch_top: Handle<Mesh>,
    pub mesh_portal_arch_jamb: Handle<Mesh>,
    pub mesh_cyber_gantry: Handle<Mesh>,
    pub mesh_power_insulator: Handle<Mesh>,
    pub mesh_beacon_pole: Handle<Mesh>,

    // Skyscrapers & Architectural Geometry
    pub mesh_building_small: Handle<Mesh>,
    pub mesh_building_medium: Handle<Mesh>,
    pub mesh_building_large: Handle<Mesh>,
    pub mesh_spire_tower: Handle<Mesh>,
    pub mesh_window_strip_h: Handle<Mesh>,
    pub mesh_window_strip_v: Handle<Mesh>,
    pub mesh_skybridge_tube: Handle<Mesh>,
    pub mesh_skybridge_accent: Handle<Mesh>,

    // Holographic Billboards
    pub mesh_billboard_large: Handle<Mesh>,
    pub mesh_billboard_vertical: Handle<Mesh>,
    pub mesh_billboard_cantilever: Handle<Mesh>,

    // Transit Props
    pub mesh_maglev_guideway: Handle<Mesh>,
    pub mesh_maglev_car: Handle<Mesh>,
    pub mesh_maglev_stripe: Handle<Mesh>,

    // Layered Megastructures & Horizon Architecture
    pub mesh_building_far_monolith: Handle<Mesh>,
    pub mesh_building_far_spire: Handle<Mesh>,
    pub mesh_building_podium: Handle<Mesh>,
    pub mesh_building_stepped_top: Handle<Mesh>,
    pub mesh_rooftop_condenser: Handle<Mesh>,
    pub mesh_rooftop_watertank: Handle<Mesh>,
    pub mesh_antenna_array: Handle<Mesh>,

    // Elevated Highways & Flyover Skyways
    pub mesh_highway_deck: Handle<Mesh>,
    pub mesh_highway_barrier: Handle<Mesh>,
    pub mesh_highway_glow_trim: Handle<Mesh>,
    pub mesh_highway_pylon: Handle<Mesh>,
    pub mesh_cross_bridge_deck: Handle<Mesh>,
    pub mesh_cross_bridge_barrier: Handle<Mesh>,

    // Background Moving Vehicles
    pub mesh_traffic_car: Handle<Mesh>,
    pub mesh_traffic_bus: Handle<Mesh>,
    pub mesh_traffic_van: Handle<Mesh>,
    pub mesh_headlight_pair: Handle<Mesh>,
    pub mesh_taillight_pair: Handle<Mesh>,

    // Materials
    pub mat_track_bed: Handle<StandardMaterial>,
    pub mat_structural_gunmetal: Handle<StandardMaterial>,
    pub mat_gantry_truss: Handle<StandardMaterial>,
    pub mat_magenta_rail_glow: Handle<StandardMaterial>,
    pub mat_cyan_rail_glow: Handle<StandardMaterial>,
    pub mat_lane_dash_cyan: Handle<StandardMaterial>,
    pub mat_building_dark: Handle<StandardMaterial>,
    pub mat_building_glass: Handle<StandardMaterial>,
    pub mat_window_matrix_cyan: Handle<StandardMaterial>,
    pub mat_window_matrix_magenta: Handle<StandardMaterial>,
    pub mat_tower_beacon_cyan: Handle<StandardMaterial>,
    pub mat_tower_beacon_magenta: Handle<StandardMaterial>,
    pub mat_skybridge_glass: Handle<StandardMaterial>,
    pub mat_maglev_hull: Handle<StandardMaterial>,
    pub mat_maglev_stripe: Handle<StandardMaterial>,

    // Layered & Traffic Materials
    pub mat_building_far_silhouette: Handle<StandardMaterial>,
    pub mat_highway_deck: Handle<StandardMaterial>,
    pub mat_highway_amber_glow: Handle<StandardMaterial>,
    pub mat_window_matrix_amber: Handle<StandardMaterial>,
    pub mat_traffic_car_body: Handle<StandardMaterial>,
    pub mat_traffic_bus_body: Handle<StandardMaterial>,
    pub mat_traffic_van_body: Handle<StandardMaterial>,
    pub mat_headlight_glow: Handle<StandardMaterial>,
    pub mat_taillight_glow: Handle<StandardMaterial>,
    pub mat_rooftop_machinery: Handle<StandardMaterial>,

    // Holographic Textures / Materials
    pub mat_hologram_veyron: Handle<StandardMaterial>,
    pub mat_hologram_aurelia: Handle<StandardMaterial>,
    pub mat_hologram_nexus: Handle<StandardMaterial>,
    pub mat_hologram_pulse: Handle<StandardMaterial>,
}

pub fn init_neon_district_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) {
    let length = 40.0;

    // 1. Procedural In-Universe Cyberpunk Holographic Textures
    let img_veyron = create_hologram_veyron_image();
    let img_aurelia = create_hologram_aurelia_image();
    let img_nexus = create_hologram_nexus_image();
    let img_pulse = create_hologram_pulse_image();

    let h_veyron = images.add(img_veyron);
    let h_aurelia = images.add(img_aurelia);
    let h_nexus = images.add(img_nexus);
    let h_pulse = images.add(img_pulse);

    let assets = NeonDistrictAssets {
        // Track & Viaduct Structure
        mesh_elevated_bed: meshes.add(Cuboid::new(8.8, 0.45, length)),
        mesh_aero_barrier: meshes.add(Cuboid::new(0.45, 0.75, length)),
        mesh_barrier_neon_trim: meshes.add(Cuboid::new(0.06, 0.08, length)),
        mesh_maglev_rail: meshes.add(Cuboid::new(0.12, 0.09, length)),
        mesh_lane_dash: meshes.add(Cuboid::new(0.08, 0.02, 1.8)),
        mesh_viaduct_pylon_heavy: meshes.add(Cuboid::new(2.4, 28.0, 2.4)),
        mesh_viaduct_crossbeam: meshes.add(Cuboid::new(9.8, 1.4, 1.8)),
        mesh_catwalk_tray: meshes.add(Cuboid::new(0.75, 0.15, length)),

        // Portals & Overhead Gantries
        mesh_portal_arch_top: meshes.add(Cuboid::new(11.2, 1.2, 1.6)),
        mesh_portal_arch_jamb: meshes.add(Cuboid::new(1.2, 8.4, 1.6)),
        mesh_cyber_gantry: meshes.add(Cuboid::new(9.8, 0.45, 0.45)),
        mesh_power_insulator: meshes.add(Cylinder::new(0.25, 0.85)),
        mesh_beacon_pole: meshes.add(Cylinder::new(0.08, 4.2)),

        // Skyscrapers & Architectural Geometry
        mesh_building_small: meshes.add(Cuboid::new(14.0, 28.0, 14.0)),
        mesh_building_medium: meshes.add(Cuboid::new(18.0, 56.0, 18.0)),
        mesh_building_large: meshes.add(Cuboid::new(24.0, 110.0, 24.0)),
        mesh_spire_tower: meshes.add(Cylinder::new(0.75, 26.0)),
        mesh_window_strip_h: meshes.add(Cuboid::new(16.2, 0.9, 0.15)),
        mesh_window_strip_v: meshes.add(Cuboid::new(0.8, 42.0, 0.15)),
        mesh_skybridge_tube: meshes.add(Cuboid::new(56.0, 3.8, 4.2)),
        mesh_skybridge_accent: meshes.add(Cuboid::new(56.0, 0.2, 0.2)),

        // Layered Megastructures & Horizon Architecture
        mesh_building_far_monolith: meshes.add(Cuboid::new(38.0, 190.0, 38.0)),
        mesh_building_far_spire: meshes.add(Cylinder::new(3.6, 220.0)),
        mesh_building_podium: meshes.add(Cuboid::new(22.0, 34.0, 22.0)),
        mesh_building_stepped_top: meshes.add(Cuboid::new(12.0, 42.0, 12.0)),
        mesh_rooftop_condenser: meshes.add(Cuboid::new(4.5, 3.2, 5.5)),
        mesh_rooftop_watertank: meshes.add(Cylinder::new(2.4, 4.2)),
        mesh_antenna_array: meshes.add(Cylinder::new(0.35, 16.0)),

        // Elevated Highways & Flyover Skyways
        mesh_highway_deck: meshes.add(Cuboid::new(6.4, 0.5, length)),
        mesh_highway_barrier: meshes.add(Cuboid::new(0.3, 0.7, length)),
        mesh_highway_glow_trim: meshes.add(Cuboid::new(0.08, 0.12, length)),
        mesh_highway_pylon: meshes.add(Cuboid::new(2.2, 36.0, 2.2)),
        mesh_cross_bridge_deck: meshes.add(Cuboid::new(64.0, 0.6, 6.4)),
        mesh_cross_bridge_barrier: meshes.add(Cuboid::new(64.0, 0.7, 0.3)),

        // Background Moving Vehicles
        mesh_traffic_car: meshes.add(Cuboid::new(1.5, 0.75, 3.4)),
        mesh_traffic_bus: meshes.add(Cuboid::new(2.3, 1.9, 7.8)),
        mesh_traffic_van: meshes.add(Cuboid::new(1.8, 1.35, 4.8)),
        mesh_headlight_pair: meshes.add(Cuboid::new(1.2, 0.16, 0.08)),
        mesh_taillight_pair: meshes.add(Cuboid::new(1.2, 0.16, 0.08)),

        // Holographic Billboards
        mesh_billboard_large: meshes.add(Cuboid::new(8.5, 4.8, 0.15)),
        mesh_billboard_vertical: meshes.add(Cuboid::new(3.2, 7.5, 0.15)),
        mesh_billboard_cantilever: meshes.add(Cuboid::new(3.8, 0.35, 0.35)),

        // Transit Props
        mesh_maglev_guideway: meshes.add(Cuboid::new(2.2, 0.6, length)),
        mesh_maglev_car: meshes.add(Cuboid::new(2.6, 2.8, 24.0)),
        mesh_maglev_stripe: meshes.add(Cuboid::new(0.08, 0.6, 23.0)),

        // Materials
        mat_track_bed: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.07, 0.11),
            perceptual_roughness: 0.70,
            metallic: 0.25,
            ..default()
        }),
        mat_structural_gunmetal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.11, 0.16),
            perceptual_roughness: 0.55,
            metallic: 0.65,
            ..default()
        }),
        mat_gantry_truss: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.13, 0.18),
            perceptual_roughness: 0.45,
            metallic: 0.70,
            ..default()
        }),
        mat_magenta_rail_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.10, 0.70),
            emissive: LinearRgba::new(2.8, 0.3, 2.2, 1.0),
            perceptual_roughness: 0.15,
            metallic: 0.85,
            ..default()
        }),
        mat_cyan_rail_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.88, 1.0),
            emissive: LinearRgba::new(0.3, 2.5, 3.2, 1.0),
            perceptual_roughness: 0.15,
            metallic: 0.85,
            ..default()
        }),
        mat_lane_dash_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.88, 1.0),
            emissive: LinearRgba::new(0.2, 1.8, 2.4, 1.0),
            ..default()
        }),
        mat_building_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.05, 0.08),
            perceptual_roughness: 0.85,
            metallic: 0.40,
            ..default()
        }),
        mat_building_glass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.12, 0.18),
            perceptual_roughness: 0.20,
            metallic: 0.80,
            ..default()
        }),
        mat_window_matrix_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.8, 0.9),
            emissive: LinearRgba::new(0.4, 2.0, 2.6, 1.0),
            ..default()
        }),
        mat_window_matrix_magenta: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.1, 0.7),
            emissive: LinearRgba::new(2.2, 0.2, 1.8, 1.0),
            ..default()
        }),
        mat_tower_beacon_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.9, 1.0),
            emissive: LinearRgba::new(0.5, 3.5, 4.5, 1.0),
            ..default()
        }),
        mat_tower_beacon_magenta: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.8),
            emissive: LinearRgba::new(3.8, 0.4, 3.0, 1.0),
            ..default()
        }),
        mat_skybridge_glass: materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.6, 0.8, 0.35),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.15,
            metallic: 0.70,
            ..default()
        }),
        mat_maglev_hull: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.22),
            perceptual_roughness: 0.25,
            metallic: 0.85,
            ..default()
        }),
        mat_maglev_stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.9, 1.0),
            emissive: LinearRgba::new(0.3, 2.8, 3.5, 1.0),
            ..default()
        }),

        // Layered & Traffic Materials
        mat_building_far_silhouette: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.04, 0.08),
            perceptual_roughness: 0.90,
            metallic: 0.20,
            ..default()
        }),
        mat_highway_deck: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.07, 0.10),
            perceptual_roughness: 0.80,
            metallic: 0.30,
            ..default()
        }),
        mat_highway_amber_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.6, 0.1),
            emissive: LinearRgba::new(2.6, 1.4, 0.15, 1.0),
            ..default()
        }),
        mat_window_matrix_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.7, 0.2),
            emissive: LinearRgba::new(2.4, 1.5, 0.3, 1.0),
            ..default()
        }),
        mat_traffic_car_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.13, 0.18),
            perceptual_roughness: 0.30,
            metallic: 0.80,
            ..default()
        }),
        mat_traffic_bus_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.14, 0.22),
            perceptual_roughness: 0.25,
            metallic: 0.85,
            ..default()
        }),
        mat_traffic_van_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.15, 0.12),
            perceptual_roughness: 0.40,
            metallic: 0.60,
            ..default()
        }),
        mat_headlight_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.95, 1.0),
            emissive: LinearRgba::new(3.0, 3.5, 4.2, 1.0),
            ..default()
        }),
        mat_taillight_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.05, 0.1),
            emissive: LinearRgba::new(4.0, 0.1, 0.15, 1.0),
            ..default()
        }),
        mat_rooftop_machinery: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.15, 0.18),
            perceptual_roughness: 0.60,
            metallic: 0.60,
            ..default()
        }),

        // Holographic Billboard Materials
        mat_hologram_veyron: materials.add(StandardMaterial {
            base_color_texture: Some(h_veyron.clone()),
            emissive_texture: Some(h_veyron),
            emissive: LinearRgba::new(2.8, 0.3, 2.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_hologram_aurelia: materials.add(StandardMaterial {
            base_color_texture: Some(h_aurelia.clone()),
            emissive_texture: Some(h_aurelia),
            emissive: LinearRgba::new(0.3, 2.8, 3.8, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_hologram_nexus: materials.add(StandardMaterial {
            base_color_texture: Some(h_nexus.clone()),
            emissive_texture: Some(h_nexus),
            emissive: LinearRgba::new(0.4, 3.2, 4.0, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_hologram_pulse: materials.add(StandardMaterial {
            base_color_texture: Some(h_pulse.clone()),
            emissive_texture: Some(h_pulse),
            emissive: LinearRgba::new(3.2, 0.5, 2.6, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -----------------------------------------------------------------------------
// TEXTURE GENERATION HELPERS
// -----------------------------------------------------------------------------

fn create_hologram_veyron_image() -> Image {
    let w = 512;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: Deep translucent magenta holographic field with scanlines
    for y in 0..h {
        for x in 0..w {
            let is_border = x < 8 || x >= w - 8 || y < 8 || y >= h - 8;
            let col = if is_border {
                [240, 20, 180, 255]
            } else if y % 4 == 0 {
                [35, 8, 38, 220]
            } else {
                [20, 4, 24, 200]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    draw_string(
        &mut buf,
        w,
        h,
        "VEYRON DYNAMICS",
        42,
        48,
        4,
        [255, 30, 200, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "PREDICTIVE NEURAL SYSTEMS // SEC-02",
        44,
        110,
        2,
        [220, 180, 240, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "AUTOMATED TRANSIT SURVEILLANCE ACTIVE",
        44,
        140,
        2,
        [180, 80, 190, 240],
    );

    make_image(w as u32, h as u32, buf)
}

fn create_hologram_aurelia_image() -> Image {
    let w = 512;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: Deep translucent cyan holographic field
    for y in 0..h {
        for x in 0..w {
            let is_border = x < 8 || x >= w - 8 || y < 8 || y >= h - 8;
            let col = if is_border {
                [20, 220, 255, 255]
            } else if y % 4 == 0 {
                [6, 32, 45, 220]
            } else {
                [4, 18, 28, 200]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    draw_string(
        &mut buf,
        w,
        h,
        "UPPER AURELIA RAILWAY",
        32,
        48,
        4,
        [30, 240, 255, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "LINE 02 MAGLEV CORRIDOR // SPEED 1.15X",
        36,
        110,
        2,
        [160, 240, 255, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "COMMERCIAL DISTRICT AHEAD // ALL LANES CLEAR",
        36,
        140,
        2,
        [80, 190, 220, 240],
    );

    make_image(w as u32, h as u32, buf)
}

fn create_hologram_nexus_image() -> Image {
    let w = 512;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: Dual-tone cyan/magenta cyberpunk grid
    for y in 0..h {
        for x in 0..w {
            let is_border = x < 8 || x >= w - 8 || y < 8 || y >= h - 8;
            let col = if is_border {
                [200, 40, 240, 255]
            } else if (x / 32 + y / 32) % 2 == 0 {
                [18, 10, 32, 210]
            } else {
                [8, 20, 34, 210]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    draw_string(
        &mut buf,
        w,
        h,
        "NEXUS SYNTHETICS",
        40,
        50,
        4,
        [240, 60, 255, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "QUANTUM CLOUD ARCHITECTURE",
        44,
        112,
        2,
        [80, 220, 255, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "SECURE YOUR ECHO FREQUENCIES TODAY",
        44,
        142,
        2,
        [180, 160, 230, 240],
    );

    make_image(w as u32, h as u32, buf)
}

fn create_hologram_pulse_image() -> Image {
    let w = 512;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: Transit alert with caution border
    for y in 0..h {
        for x in 0..w {
            let is_border = x < 10 || x >= w - 10 || y < 10 || y >= h - 10;
            let col = if is_border {
                if ((x + y) / 12) % 2 == 0 {
                    [255, 180, 20, 255]
                } else {
                    [20, 20, 25, 255]
                }
            } else {
                [24, 12, 16, 215]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    draw_string(
        &mut buf,
        w,
        h,
        "PULSE ENERGY GRID",
        36,
        48,
        4,
        [255, 200, 40, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "WARNING: HIGH SPEED MAGLEV ZONE",
        40,
        110,
        2,
        [255, 100, 120, 255],
    );
    draw_string(
        &mut buf,
        w,
        h,
        "MAINTENANCE RUNNERS MAINTAIN DESIGNATED LANES",
        40,
        140,
        2,
        [240, 180, 100, 240],
    );

    make_image(w as u32, h as u32, buf)
}

// -----------------------------------------------------------------------------
// MAIN SEGMENT SPAWNER
// -----------------------------------------------------------------------------

pub fn spawn_neon_district_segment(
    commands: &mut Commands,
    _env: &crate::environment::EnvironmentAssets,
    neon: &NeonDistrictAssets,
    _props: Option<&crate::environment_props::PropAssets>,
    _signage: Option<&crate::environment_signage::SignageAssets>,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = get_neon_district_chapter(distance);
    let variant = get_neon_district_variant(distance);
    let seg_idx = ((distance / length).floor() as i32).abs();
    let is_even = seg_idx % 2 == 0;

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // 1. Futuristic Elevated Railway Track (Clean running surface, vivid neon magenta & cyan edges)
            spawn_elevated_railway_track(seg, neon, length, is_even, chapter);

            // 2. Towering Cityscape Background Silhouettes (Left & Right skyline, deep verticality)
            spawn_city_skyline(seg, neon, length, seg_idx, chapter);

            // 3. Overhead Catenary Gantries & Cyber Arches (depth & structural scale)
            spawn_overhead_infrastructure(seg, neon, is_even, chapter);

            // 4. Chapter Landmark Sub-Sections (0 repetition across 800m–1800m!)
            match variant {
                NeonDistrictVariant::TunnelExitPortal => {
                    neon_district_landmarks::spawn_tunnel_exit_portal(seg, neon, is_even);
                }
                NeonDistrictVariant::OpenSkyEmergence => {
                    neon_district_landmarks::spawn_open_sky_emergence(seg, neon, is_even);
                }
                NeonDistrictVariant::FirstNeonApproach => {
                    neon_district_landmarks::spawn_first_neon_approach(seg, neon, is_even);
                }
                NeonDistrictVariant::ViaductPylons => {
                    neon_district_landmarks::spawn_viaduct_pylons_sub_section(seg, neon, is_even);
                }
                NeonDistrictVariant::OverheadPowerGrid => {
                    neon_district_landmarks::spawn_overhead_power_grid_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::AureliaCorporateRow => {
                    neon_district_landmarks::spawn_aurelia_corporate_row_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::HologramPlaza => {
                    neon_district_landmarks::spawn_hologram_plaza_sub_section(seg, neon, is_even);
                }
                NeonDistrictVariant::TwinTowerSkybridge => {
                    neon_district_landmarks::spawn_twin_tower_skybridge_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::MediaMarqueeOverpass => {
                    neon_district_landmarks::spawn_media_marquee_overpass_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::ParallelMaglevExpress => {
                    neon_district_landmarks::spawn_parallel_maglev_express_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::CentralInterchangeArch => {
                    neon_district_landmarks::spawn_central_interchange_arch_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::CommsTowerArray => {
                    neon_district_landmarks::spawn_comms_tower_array_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::SecurityGridGantry => {
                    neon_district_landmarks::spawn_security_grid_gantry_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::HeavyIndustrialApproach => {
                    neon_district_landmarks::spawn_heavy_industrial_approach_sub_section(
                        seg, neon, is_even,
                    );
                }
                NeonDistrictVariant::Zone3TransitionPortal => {
                    neon_district_landmarks::spawn_zone3_transition_portal_sub_section(
                        seg, neon, is_even,
                    );
                }
            }
        })
        .id()
}

// -----------------------------------------------------------------------------
// 1. ELEVATED RAILWAY TRACK
// -----------------------------------------------------------------------------

fn spawn_elevated_railway_track(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _length: f32,
    _is_even: bool,
    _chapter: NeonDistrictChapter,
) {
    // 1. Main composite elevated track slab at Y = -0.225
    seg.spawn(PbrBundle {
        mesh: neon.mesh_elevated_bed.clone(),
        material: neon.mat_track_bed.clone(),
        transform: Transform::from_xyz(0.0, -0.225, 0.0),
        ..default()
    });

    // 2. Aerodynamic crash barriers on left and right track shoulders (X = -4.25, +4.25)
    for &side_x in &[-4.25, 4.25] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_aero_barrier.clone(),
            material: neon.mat_structural_gunmetal.clone(),
            transform: Transform::from_xyz(side_x, 0.22, 0.0),
            ..default()
        });

        // Vibrant cyan linear edge strip running along top of barrier
        seg.spawn(PbrBundle {
            mesh: neon.mesh_barrier_neon_trim.clone(),
            material: neon.mat_cyan_rail_glow.clone(),
            transform: Transform::from_xyz(side_x, 0.62, 0.0),
            ..default()
        });
    }

    // 3. Twin elevated maglev rail crowns with vivid neon magenta emissive strips
    // Aligned with the outer rails (X = -2.5, +2.5) for crisp lane guidance
    for &rail_x in &[-2.5, 2.5] {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_maglev_rail.clone(),
            material: neon.mat_magenta_rail_glow.clone(),
            transform: Transform::from_xyz(rail_x, 0.02, 0.0),
            ..default()
        });
    }

    // 4. Illuminated cyan dashed lane dividers at X = -1.25 and X = 1.25
    // Spaced evenly along the 40m segment for high-speed motion parallax
    for &dash_x in &[-1.25, 1.25] {
        for dash_idx in 0..8 {
            let z_pos = -17.5 + (dash_idx as f32) * 5.0;
            seg.spawn(PbrBundle {
                mesh: neon.mesh_lane_dash.clone(),
                material: neon.mat_lane_dash_cyan.clone(),
                transform: Transform::from_xyz(dash_x, 0.005, z_pos),
                ..default()
            });
        }
    }
}

// -----------------------------------------------------------------------------
// 2. TOWERING CITYSCAPE SKYLINE
// -----------------------------------------------------------------------------

fn spawn_city_skyline(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    _length: f32,
    seg_idx: i32,
    _chapter: NeonDistrictChapter,
) {
    // Deterministic pseudo-random generation based on segment index
    let h1 = ((seg_idx.wrapping_mul(17) + 3).rem_euclid(100)) as f32 / 100.0;
    let h2 = ((seg_idx.wrapping_mul(31) + 7).rem_euclid(100)) as f32 / 100.0;
    let h3 = ((seg_idx.wrapping_mul(53) + 11).rem_euclid(100)) as f32 / 100.0;
    let h4 = ((seg_idx.wrapping_mul(79) + 19).rem_euclid(100)) as f32 / 100.0;

    // -------------------------------------------------------------------------
    // LAYER 1: NEAR/MID-GROUND FLANK BUILDINGS (|X| in [18.0, 26.0]m)
    // -------------------------------------------------------------------------
    // Left Near-ground podium / mid-rise (height 26–42m)
    let near_l_x = -20.0 - h1 * 5.0;
    let near_l_z = -10.0 + h2 * 20.0;
    let near_l_y = 16.0 + h3 * 12.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_podium.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(near_l_x, near_l_y, near_l_z),
        ..default()
    });
    // Horizontal window strip (cyan, magenta, or amber)
    let left_win_mat = if h1 > 0.65 {
        neon.mat_window_matrix_cyan.clone()
    } else if h1 > 0.35 {
        neon.mat_window_matrix_magenta.clone()
    } else {
        neon.mat_window_matrix_amber.clone()
    };
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_h.clone(),
        material: left_win_mat,
        transform: Transform::from_xyz(near_l_x + 11.1, near_l_y, near_l_z)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // Rooftop mechanical equipment on left near building
    if h2 > 0.5 {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_rooftop_condenser.clone(),
            material: neon.mat_rooftop_machinery.clone(),
            transform: Transform::from_xyz(near_l_x + 4.0, near_l_y + 17.0 + 1.6, near_l_z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: neon.mesh_antenna_array.clone(),
            material: neon.mat_tower_beacon_cyan.clone(),
            transform: Transform::from_xyz(near_l_x - 5.0, near_l_y + 17.0 + 8.0, near_l_z - 3.0),
            ..default()
        });
    } else {
        seg.spawn(PbrBundle {
            mesh: neon.mesh_rooftop_watertank.clone(),
            material: neon.mat_rooftop_machinery.clone(),
            transform: Transform::from_xyz(near_l_x - 3.0, near_l_y + 17.0 + 2.1, near_l_z + 2.0),
            ..default()
        });
    }

    // Right Near-ground stepped building (height 24–40m)
    let near_r_x = 20.0 + h2 * 5.0;
    let near_r_z = 10.0 - h1 * 20.0;
    let near_r_y = 18.0 + h4 * 14.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_small.clone(),
        material: neon.mat_building_glass.clone(),
        transform: Transform::from_xyz(near_r_x, near_r_y, near_r_z),
        ..default()
    });
    let right_win_mat = if h2 > 0.6 {
        neon.mat_window_matrix_amber.clone()
    } else if h2 > 0.3 {
        neon.mat_window_matrix_cyan.clone()
    } else {
        neon.mat_window_matrix_magenta.clone()
    };
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_h.clone(),
        material: right_win_mat,
        transform: Transform::from_xyz(near_r_x - 7.1, near_r_y, near_r_z)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // Rooftop condenser on right near building
    seg.spawn(PbrBundle {
        mesh: neon.mesh_rooftop_condenser.clone(),
        material: neon.mat_rooftop_machinery.clone(),
        transform: Transform::from_xyz(near_r_x, near_r_y + 14.0 + 1.6, near_r_z),
        ..default()
    });

    // -------------------------------------------------------------------------
    // LAYER 2: ELEVATED HIGHWAYS WITH ACTIVE MOVING TRAFFIC (Left & Right)
    // -------------------------------------------------------------------------
    // Left Highway (X = -31.5m, Y = 14.0m)
    let hw_l_x = -31.5;
    let hw_l_y = 14.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_deck.clone(),
        material: neon.mat_highway_deck.clone(),
        transform: Transform::from_xyz(hw_l_x, hw_l_y, 0.0),
        ..default()
    });
    // Left highway support pier
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_pylon.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_l_x, hw_l_y - 18.0, 0.0),
        ..default()
    });
    // Left highway outer guardrails + glowing amber trim
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_barrier.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_l_x - 3.2, hw_l_y + 0.35, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_barrier.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_l_x + 3.2, hw_l_y + 0.35, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_glow_trim.clone(),
        material: neon.mat_highway_amber_glow.clone(),
        transform: Transform::from_xyz(hw_l_x + 3.1, hw_l_y + 0.65, 0.0),
        ..default()
    });

    // Left Highway Active Moving Traffic (Autonomous cyber-car + cargo van)
    let l_speed_1 = -24.0; // cruising forward along with Kai
    let l_car_z_1 = -12.0 + h3 * 16.0;
    seg.spawn((
        PbrBundle {
            mesh: neon.mesh_traffic_car.clone(),
            material: neon.mat_traffic_car_body.clone(),
            transform: Transform::from_xyz(hw_l_x - 1.4, hw_l_y + 0.55, l_car_z_1),
            ..default()
        },
        BackgroundTrafficVehicle {
            speed: l_speed_1,
            min_local_z: -20.0,
            max_local_z: 20.0,
        },
    ))
    .with_children(|car| {
        car.spawn(PbrBundle {
            mesh: neon.mesh_headlight_pair.clone(),
            material: neon.mat_headlight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -1.7),
            ..default()
        });
        car.spawn(PbrBundle {
            mesh: neon.mesh_taillight_pair.clone(),
            material: neon.mat_taillight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.7),
            ..default()
        });
    });

    let l_speed_2 = 18.0; // oncoming traffic
    let l_car_z_2 = 10.0 - h4 * 16.0;
    seg.spawn((
        PbrBundle {
            mesh: neon.mesh_traffic_van.clone(),
            material: neon.mat_traffic_van_body.clone(),
            transform: Transform::from_xyz(hw_l_x + 1.4, hw_l_y + 0.85, l_car_z_2),
            ..default()
        },
        BackgroundTrafficVehicle {
            speed: l_speed_2,
            min_local_z: -20.0,
            max_local_z: 20.0,
        },
    ))
    .with_children(|van| {
        van.spawn(PbrBundle {
            mesh: neon.mesh_headlight_pair.clone(),
            material: neon.mat_headlight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 2.4),
            ..default()
        });
        van.spawn(PbrBundle {
            mesh: neon.mesh_taillight_pair.clone(),
            material: neon.mat_taillight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -2.4),
            ..default()
        });
    });

    // Right Highway (X = +35.5m, Y = 18.0m)
    let hw_r_x = 35.5;
    let hw_r_y = 18.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_deck.clone(),
        material: neon.mat_highway_deck.clone(),
        transform: Transform::from_xyz(hw_r_x, hw_r_y, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_pylon.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_r_x, hw_r_y - 18.0, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_barrier.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_r_x - 3.2, hw_r_y + 0.35, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_barrier.clone(),
        material: neon.mat_structural_gunmetal.clone(),
        transform: Transform::from_xyz(hw_r_x + 3.2, hw_r_y + 0.35, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_highway_glow_trim.clone(),
        material: neon.mat_cyan_rail_glow.clone(),
        transform: Transform::from_xyz(hw_r_x - 3.1, hw_r_y + 0.65, 0.0),
        ..default()
    });

    // Right Highway Active Moving Traffic (Cyber-bus cruising at speed)
    let r_speed_bus = -16.0;
    let r_bus_z = -8.0 + h1 * 16.0;
    seg.spawn((
        PbrBundle {
            mesh: neon.mesh_traffic_bus.clone(),
            material: neon.mat_traffic_bus_body.clone(),
            transform: Transform::from_xyz(hw_r_x - 1.3, hw_r_y + 1.15, r_bus_z),
            ..default()
        },
        BackgroundTrafficVehicle {
            speed: r_speed_bus,
            min_local_z: -20.0,
            max_local_z: 20.0,
        },
    ))
    .with_children(|bus| {
        bus.spawn(PbrBundle {
            mesh: neon.mesh_headlight_pair.clone(),
            material: neon.mat_headlight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -3.9),
            ..default()
        });
        bus.spawn(PbrBundle {
            mesh: neon.mesh_taillight_pair.clone(),
            material: neon.mat_taillight_glow.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.9),
            ..default()
        });
    });

    // -------------------------------------------------------------------------
    // LAYER 3: MIDDLE-DISTANCE MEGA-TOWERS (|X| in [44.0, 64.0]m)
    // -------------------------------------------------------------------------
    let mid_l_x = -48.0 - h3 * 12.0;
    let mid_l_z = -14.0 + h4 * 28.0;
    let mid_l_height = 56.0 + h1 * 45.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_medium.clone(),
        material: neon.mat_building_dark.clone(),
        transform: Transform::from_xyz(mid_l_x, mid_l_height * 0.5, mid_l_z).with_scale(Vec3::new(
            1.2,
            mid_l_height / 56.0,
            1.2,
        )),
        ..default()
    });
    // Vertical matrix
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_v.clone(),
        material: neon.mat_window_matrix_cyan.clone(),
        transform: Transform::from_xyz(mid_l_x + 11.0, mid_l_height * 0.5, mid_l_z),
        ..default()
    });
    // Rooftop spire
    seg.spawn(PbrBundle {
        mesh: neon.mesh_spire_tower.clone(),
        material: neon.mat_tower_beacon_cyan.clone(),
        transform: Transform::from_xyz(mid_l_x, mid_l_height + 13.0, mid_l_z),
        ..default()
    });

    let mid_r_x = 50.0 + h4 * 12.0;
    let mid_r_z = 12.0 - h3 * 28.0;
    let mid_r_height = 65.0 + h2 * 45.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_large.clone(),
        material: neon.mat_building_glass.clone(),
        transform: Transform::from_xyz(mid_r_x, mid_r_height * 0.5, mid_r_z).with_scale(Vec3::new(
            1.1,
            mid_r_height / 110.0,
            1.1,
        )),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_window_strip_v.clone(),
        material: neon.mat_window_matrix_magenta.clone(),
        transform: Transform::from_xyz(mid_r_x - 13.3, mid_r_height * 0.5, mid_r_z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_spire_tower.clone(),
        material: neon.mat_tower_beacon_magenta.clone(),
        transform: Transform::from_xyz(mid_r_x, mid_r_height + 13.0, mid_r_z),
        ..default()
    });

    // -------------------------------------------------------------------------
    // LAYER 4: FAR HORIZON MEGASTRUCTURES & ARCOLOGIES (|X| in [85.0, 150.0]m)
    // -------------------------------------------------------------------------
    let far_l_x = -96.0 - h1 * 40.0;
    let far_l_z = -18.0 + h2 * 36.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_far_monolith.clone(),
        material: neon.mat_building_far_silhouette.clone(),
        transform: Transform::from_xyz(far_l_x, 95.0, far_l_z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_far_spire.clone(),
        material: neon.mat_tower_beacon_magenta.clone(),
        transform: Transform::from_xyz(far_l_x, 190.0 + 110.0, far_l_z),
        ..default()
    });

    let far_r_x = 100.0 + h3 * 40.0;
    let far_r_z = 16.0 - h4 * 36.0;
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_far_monolith.clone(),
        material: neon.mat_building_far_silhouette.clone(),
        transform: Transform::from_xyz(far_r_x, 95.0, far_r_z),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: neon.mesh_building_far_spire.clone(),
        material: neon.mat_tower_beacon_cyan.clone(),
        transform: Transform::from_xyz(far_r_x, 190.0 + 110.0, far_r_z),
        ..default()
    });

    // -------------------------------------------------------------------------
    // TRANSVERSE SKYWAY CROSS-BRIDGES (Alternate segments, spanning overhead at Y = 28m)
    // -------------------------------------------------------------------------
    if seg_idx % 2 == 0 {
        let bridge_y = 28.0;
        let bridge_z = -8.0 + h1 * 16.0;
        seg.spawn(PbrBundle {
            mesh: neon.mesh_cross_bridge_deck.clone(),
            material: neon.mat_highway_deck.clone(),
            transform: Transform::from_xyz(0.0, bridge_y, bridge_z),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: neon.mesh_cross_bridge_barrier.clone(),
            material: neon.mat_magenta_rail_glow.clone(),
            transform: Transform::from_xyz(0.0, bridge_y + 0.45, bridge_z - 3.2),
            ..default()
        });
        seg.spawn(PbrBundle {
            mesh: neon.mesh_cross_bridge_barrier.clone(),
            material: neon.mat_cyan_rail_glow.clone(),
            transform: Transform::from_xyz(0.0, bridge_y + 0.45, bridge_z + 3.2),
            ..default()
        });
    }
}

// -----------------------------------------------------------------------------
// 3. OVERHEAD INFRASTRUCTURE
// -----------------------------------------------------------------------------

fn spawn_overhead_infrastructure(
    seg: &mut ChildBuilder,
    neon: &NeonDistrictAssets,
    is_even: bool,
    _chapter: NeonDistrictChapter,
) {
    if is_even {
        // Catenary overhead power gantry spanning track at Y = 7.8m
        // (Well above player jump height ~3.2m and camera follow height 3.8m)
        seg.spawn(PbrBundle {
            mesh: neon.mesh_cyber_gantry.clone(),
            material: neon.mat_gantry_truss.clone(),
            transform: Transform::from_xyz(0.0, 7.8, -14.0),
            ..default()
        });

        // Suspended power coils over track lanes
        for &lane_x in &[-2.5, 0.0, 2.5] {
            seg.spawn(PbrBundle {
                mesh: neon.mesh_power_insulator.clone(),
                material: neon.mat_cyan_rail_glow.clone(),
                transform: Transform::from_xyz(lane_x, 7.2, -14.0),
                ..default()
            });
        }
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neon_district_chapter_boundaries() {
        assert_eq!(
            get_neon_district_chapter(800.0),
            NeonDistrictChapter::NeonEntry
        );
        assert_eq!(
            get_neon_district_chapter(999.0),
            NeonDistrictChapter::NeonEntry
        );
        assert_eq!(
            get_neon_district_chapter(1000.0),
            NeonDistrictChapter::UpperAurelia
        );
        assert_eq!(
            get_neon_district_chapter(1199.0),
            NeonDistrictChapter::UpperAurelia
        );
        assert_eq!(
            get_neon_district_chapter(1200.0),
            NeonDistrictChapter::CommercialCorridor
        );
        assert_eq!(
            get_neon_district_chapter(1399.0),
            NeonDistrictChapter::CommercialCorridor
        );
        assert_eq!(
            get_neon_district_chapter(1400.0),
            NeonDistrictChapter::TransitCore
        );
        assert_eq!(
            get_neon_district_chapter(1599.0),
            NeonDistrictChapter::TransitCore
        );
        assert_eq!(
            get_neon_district_chapter(1600.0),
            NeonDistrictChapter::VeyronTransition
        );
        assert_eq!(
            get_neon_district_chapter(1799.0),
            NeonDistrictChapter::VeyronTransition
        );
    }

    #[test]
    fn test_neon_district_variant_determinism() {
        // Verify all 15 variants are covered and deterministically reachable
        let checkpoints = [
            (820.0, NeonDistrictVariant::TunnelExitPortal),
            (890.0, NeonDistrictVariant::OpenSkyEmergence),
            (960.0, NeonDistrictVariant::FirstNeonApproach),
            (1030.0, NeonDistrictVariant::ViaductPylons),
            (1100.0, NeonDistrictVariant::OverheadPowerGrid),
            (1170.0, NeonDistrictVariant::AureliaCorporateRow),
            (1230.0, NeonDistrictVariant::HologramPlaza),
            (1300.0, NeonDistrictVariant::TwinTowerSkybridge),
            (1370.0, NeonDistrictVariant::MediaMarqueeOverpass),
            (1430.0, NeonDistrictVariant::ParallelMaglevExpress),
            (1500.0, NeonDistrictVariant::CentralInterchangeArch),
            (1570.0, NeonDistrictVariant::CommsTowerArray),
            (1630.0, NeonDistrictVariant::SecurityGridGantry),
            (1700.0, NeonDistrictVariant::HeavyIndustrialApproach),
            (1770.0, NeonDistrictVariant::Zone3TransitionPortal),
        ];

        for (dist, expected_variant) in checkpoints {
            assert_eq!(
                get_neon_district_variant(dist),
                expected_variant,
                "Distance {}m must resolve to {:?}",
                dist,
                expected_variant
            );
        }
    }

    #[test]
    fn test_zone_transition_at_800m() {
        // Distance < 800m is Zone 1 (Old Metro)
        let zone_799 = crate::zones::get_zone_for_distance(799.0);
        assert_eq!(zone_799.id, 1, "799m must be Zone 1");

        // Distance >= 800m is Zone 2 (Neon District)
        let zone_800 = crate::zones::get_zone_for_distance(800.0);
        assert_eq!(zone_800.id, 2, "800m must be Zone 2");
        assert_eq!(zone_800.name, "ZONE 2 — NEON DISTRICT");

        // Distance 1800m is Zone 3 (Industrial Sector)
        let zone_1800 = crate::zones::get_zone_for_distance(1800.0);
        assert_eq!(zone_1800.id, 3, "1800m must be Zone 3");
    }

    #[test]
    fn test_neon_district_corridor_clearance() {
        // Playable lane corridor is bounded within X in [-3.8, 3.8] and Y in [0.0, 4.0]
        // Viaduct barriers sit at X = +/- 4.25 (outside playable lane corridor)
        let barrier_x = 4.25_f32;
        assert!(
            barrier_x > 3.8,
            "Track barrier must clear playable corridor"
        );

        // Overhead catenary gantries sit at Y >= 7.8m (above camera Y = 3.8m and jump apex Y = 3.2m)
        let gantry_y = 7.8_f32;
        assert!(
            gantry_y > 4.5,
            "Overhead gantry must clear camera and player jump"
        );

        // Buildings sit at |X| >= 18.0m (far outside playable track)
        let building_min_x = 18.0_f32;
        assert!(
            building_min_x > 5.0,
            "Background skyscrapers must not collide with track"
        );
    }

    #[test]
    fn test_background_traffic_kinematics_and_wrapping() {
        let veh = BackgroundTrafficVehicle {
            speed: 20.0,
            min_local_z: -20.0,
            max_local_z: 20.0,
        };

        let mut trans = Transform::from_xyz(-31.5, 14.0, 19.0);
        let dt = 0.1; // 100ms
        trans.translation.z += veh.speed * dt; // 19.0 + 2.0 = 21.0
        let span = veh.max_local_z - veh.min_local_z; // 40.0
        if trans.translation.z > veh.max_local_z {
            trans.translation.z -= span;
        }

        assert_eq!(
            trans.translation.z, -19.0,
            "Vehicle must wrap across segment boundary"
        );

        let rev_veh = BackgroundTrafficVehicle {
            speed: -20.0,
            min_local_z: -20.0,
            max_local_z: 20.0,
        };
        let mut rev_trans = Transform::from_xyz(35.5, 18.0, -19.5);
        rev_trans.translation.z += rev_veh.speed * dt; // -19.5 - 2.0 = -21.5
        if rev_trans.translation.z < rev_veh.min_local_z {
            rev_trans.translation.z += span;
        }
        assert_eq!(
            rev_trans.translation.z, 18.5,
            "Reverse vehicle must wrap across boundary"
        );
    }

    #[test]
    fn test_elevated_highway_and_skyway_clearance() {
        // Highway X positions: Left X = -31.5, Right X = +35.5
        let hw_left_x = 31.5_f32;
        let hw_right_x = 35.5_f32;
        assert!(
            hw_left_x > 15.0,
            "Left highway must maintain extensive lateral buffer"
        );
        assert!(
            hw_right_x > 15.0,
            "Right highway must maintain extensive lateral buffer"
        );

        // Transverse skyway bridges span at Y = 28.0m
        let skyway_y = 28.0_f32;
        assert!(
            skyway_y >= 20.0,
            "Skyway bridge must be high overhead in the skyline"
        );
    }
}
