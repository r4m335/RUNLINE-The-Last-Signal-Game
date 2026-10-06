use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct ZoneConfig {
    pub id: usize,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub distance_start: f32,
    pub ambient_color: Color,
    pub directional_color: Color,
    pub track_base_color: Color,
    pub rail_emissive: LinearRgba,
    pub arch_color: Color,
    pub accent_glow: LinearRgba,
    pub speed_modifier: f32,
    pub obstacle_density: f32,
}

pub const ZONES: [ZoneConfig; 7] = [
    // Zone 1: Old Metro (0 - 800m)
    ZoneConfig {
        id: 1,
        name: "ZONE 1 — OLD METRO",
        subtitle: "Abandoned Sector 04 // Warning: Veyron patrols inbound",
        distance_start: 0.0,
        ambient_color: Color::srgb(0.08, 0.10, 0.12),
        directional_color: Color::srgb(0.4, 0.45, 0.5),
        track_base_color: Color::srgb(0.12, 0.12, 0.14),
        rail_emissive: LinearRgba::new(0.0, 0.6, 0.8, 1.0),
        arch_color: Color::srgb(0.2, 0.18, 0.16),
        accent_glow: LinearRgba::new(0.9, 0.5, 0.1, 1.0), // Amber warning lights
        speed_modifier: 1.0,
        obstacle_density: 0.22,
    },
    // Zone 2: Neon District (800 - 1800m)
    ZoneConfig {
        id: 2,
        name: "ZONE 2 — NEON DISTRICT",
        subtitle: "Upper Aurelia Railway // Holographic Traffic Active",
        distance_start: 800.0,
        ambient_color: Color::srgb(0.12, 0.05, 0.18),
        directional_color: Color::srgb(0.6, 0.2, 0.8),
        track_base_color: Color::srgb(0.10, 0.08, 0.15),
        rail_emissive: LinearRgba::new(0.9, 0.1, 0.7, 1.0), // Vivid Neon Magenta
        arch_color: Color::srgb(0.15, 0.12, 0.25),
        accent_glow: LinearRgba::new(0.1, 0.9, 0.9, 1.0), // Cyan holograms
        speed_modifier: 1.15,
        obstacle_density: 0.26,
    },
    // Zone 3: Industrial Sector (1800 - 3000m)
    ZoneConfig {
        id: 3,
        name: "ZONE 3 — INDUSTRIAL SECTOR",
        subtitle: "Foundry Transit Grid // High Temperature & Machinery",
        distance_start: 1800.0,
        ambient_color: Color::srgb(0.16, 0.08, 0.04),
        directional_color: Color::srgb(0.9, 0.45, 0.15),
        track_base_color: Color::srgb(0.15, 0.10, 0.08),
        rail_emissive: LinearRgba::new(1.0, 0.4, 0.0, 1.0), // Molten Orange
        arch_color: Color::srgb(0.22, 0.16, 0.12),
        accent_glow: LinearRgba::new(1.0, 0.8, 0.1, 1.0), // Sparks & fire
        speed_modifier: 1.30,
        obstacle_density: 0.30,
    },
    // Zone 4: Flooded Metro (3000 - 4500m)
    ZoneConfig {
        id: 4,
        name: "ZONE 4 — FLOODED METRO",
        subtitle: "Subterranean Reservoir // Submerged Line B",
        distance_start: 3000.0,
        ambient_color: Color::srgb(0.03, 0.12, 0.16),
        directional_color: Color::srgb(0.1, 0.6, 0.7),
        track_base_color: Color::srgb(0.05, 0.15, 0.18),
        rail_emissive: LinearRgba::new(0.0, 0.8, 0.7, 1.0), // Turquoise bioluminescence
        arch_color: Color::srgb(0.08, 0.18, 0.20),
        accent_glow: LinearRgba::new(0.2, 0.9, 0.6, 1.0), // Water shimmer
        speed_modifier: 1.45,
        obstacle_density: 0.33,
    },
    // Zone 5: Sky Rail (4500 - 6500m)
    ZoneConfig {
        id: 5,
        name: "ZONE 5 — SKY RAIL",
        subtitle: "Altitude: 850m Above Aurelia // Severe Turbulence",
        distance_start: 4500.0,
        ambient_color: Color::srgb(0.08, 0.12, 0.25),
        directional_color: Color::srgb(0.4, 0.6, 0.9),
        track_base_color: Color::srgb(0.12, 0.16, 0.26),
        rail_emissive: LinearRgba::new(0.3, 0.7, 1.0, 1.0), // Cloud Lightning Blue
        arch_color: Color::srgb(0.18, 0.22, 0.35),
        accent_glow: LinearRgba::new(0.7, 0.8, 1.0, 1.0), // Storm glow
        speed_modifier: 1.60,
        obstacle_density: 0.36,
    },
    // Zone 6: Forbidden Line (6500 - 9000m)
    ZoneConfig {
        id: 6,
        name: "ZONE 6 — THE FORBIDDEN LINE",
        subtitle: "Incident Ground Zero // Reality Distortion Detected",
        distance_start: 6500.0,
        ambient_color: Color::srgb(0.18, 0.02, 0.04),
        directional_color: Color::srgb(0.9, 0.1, 0.2),
        track_base_color: Color::srgb(0.12, 0.02, 0.04),
        rail_emissive: LinearRgba::new(1.0, 0.05, 0.1, 1.0), // Emergency Blood Crimson
        arch_color: Color::srgb(0.25, 0.05, 0.08),
        accent_glow: LinearRgba::new(1.0, 0.1, 0.2, 1.0), // Glitch warnings
        speed_modifier: 1.75,
        obstacle_density: 0.40,
    },
    // Zone 7: ECHO Core (9000m+)
    ZoneConfig {
        id: 7,
        name: "ZONE 7 — THE ECHO CORE",
        subtitle: "Deep Crystal Chamber // AI Resonance Maximum",
        distance_start: 9000.0,
        ambient_color: Color::srgb(0.05, 0.20, 0.25),
        directional_color: Color::srgb(0.2, 0.8, 1.0),
        track_base_color: Color::srgb(0.08, 0.22, 0.28),
        rail_emissive: LinearRgba::new(0.1, 1.0, 0.9, 1.0), // Pure Radiant Cyan Crystal
        arch_color: Color::srgb(0.10, 0.30, 0.40),
        accent_glow: LinearRgba::new(0.4, 1.0, 1.0, 1.0), // Singularity brilliance
        speed_modifier: 1.95,
        obstacle_density: 0.44,
    },
];

pub fn get_zone_for_distance(distance: f32) -> ZoneConfig {
    let mut current = ZONES[0];
    for zone in ZONES.iter() {
        if distance >= zone.distance_start {
            current = *zone;
        } else {
            break;
        }
    }
    current
}
