use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct ZoneConfig {
    pub id: usize,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub start_distance: f32,
    pub end_distance: f32,
    pub ambient_color: Color,
    pub directional_color: Color,
    pub track_base_color: Color,
    pub rail_emissive: LinearRgba,
    pub arch_color: Color,
    pub accent_glow: LinearRgba,
    pub speed_modifier: f32,
    pub obstacle_density: f32,
}

pub const BOSS_MILESTONE_4_DISTANCE: f32 = 3000.0;
pub const BOSS_MILESTONE_7_DISTANCE: f32 = 8500.0;

pub const ZONES: [ZoneConfig; 7] = [
    // Zone 1: Old Metro (0 - 800m)
    ZoneConfig {
        id: 1,
        name: "ZONE 1 — OLD METRO",
        subtitle: "Abandoned Sector 04 // Warning: Veyron patrols inbound",
        start_distance: 0.0,
        end_distance: 800.0,
        ambient_color: Color::srgb(0.04, 0.05, 0.07),
        directional_color: Color::srgb(0.30, 0.35, 0.42),
        track_base_color: Color::srgb(0.09, 0.09, 0.11),
        rail_emissive: LinearRgba::new(0.0, 0.70, 0.95, 1.0),
        arch_color: Color::srgb(0.18, 0.16, 0.14),
        accent_glow: LinearRgba::new(1.0, 0.55, 0.1, 1.0), // Warm amber incandescent work lamps
        speed_modifier: 1.0,
        obstacle_density: 0.22,
    },
    // Zone 2: Neon District (800 - 1800m)
    ZoneConfig {
        id: 2,
        name: "ZONE 2 — NEON DISTRICT",
        subtitle: "Upper Aurelia Railway // Holographic Traffic Active",
        start_distance: 800.0,
        end_distance: 1800.0,
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
        start_distance: 1800.0,
        end_distance: 3000.0,
        ambient_color: Color::srgb(0.16, 0.08, 0.04),
        directional_color: Color::srgb(0.9, 0.45, 0.15),
        track_base_color: Color::srgb(0.15, 0.10, 0.08),
        rail_emissive: LinearRgba::new(1.0, 0.4, 0.0, 1.0), // Molten Orange
        arch_color: Color::srgb(0.22, 0.16, 0.12),
        accent_glow: LinearRgba::new(1.0, 0.8, 0.1, 1.0), // Sparks & fire
        speed_modifier: 1.30,
        obstacle_density: 0.30,
    },
    // Zone 4: Flooded Metro (3000 - 4800m)
    ZoneConfig {
        id: 4,
        name: "ZONE 4 — FLOODED METRO",
        subtitle: "Subterranean Reservoir // Submerged Line B",
        start_distance: 3000.0,
        end_distance: 4800.0,
        ambient_color: Color::srgb(0.03, 0.12, 0.16),
        directional_color: Color::srgb(0.1, 0.6, 0.7),
        track_base_color: Color::srgb(0.05, 0.15, 0.18),
        rail_emissive: LinearRgba::new(0.0, 0.8, 0.7, 1.0), // Turquoise bioluminescence
        arch_color: Color::srgb(0.08, 0.18, 0.20),
        accent_glow: LinearRgba::new(0.2, 0.9, 0.6, 1.0), // Water shimmer
        speed_modifier: 1.45,
        obstacle_density: 0.33,
    },
    // Zone 5: Sky Rail (4800 - 6500m)
    ZoneConfig {
        id: 5,
        name: "ZONE 5 — SKY RAIL",
        subtitle: "Altitude: 850m Above Aurelia // Severe Turbulence",
        start_distance: 4800.0,
        end_distance: 6500.0,
        ambient_color: Color::srgb(0.08, 0.12, 0.25),
        directional_color: Color::srgb(0.4, 0.6, 0.9),
        track_base_color: Color::srgb(0.12, 0.16, 0.26),
        rail_emissive: LinearRgba::new(0.3, 0.7, 1.0, 1.0), // Cloud Lightning Blue
        arch_color: Color::srgb(0.18, 0.22, 0.35),
        accent_glow: LinearRgba::new(0.7, 0.8, 1.0, 1.0), // Storm glow
        speed_modifier: 1.60,
        obstacle_density: 0.36,
    },
    // Zone 6: Forbidden Line (6500 - 8500m)
    ZoneConfig {
        id: 6,
        name: "ZONE 6 — THE FORBIDDEN LINE",
        subtitle: "Incident Ground Zero // Reality Distortion Detected",
        start_distance: 6500.0,
        end_distance: 8500.0,
        ambient_color: Color::srgb(0.18, 0.02, 0.04),
        directional_color: Color::srgb(0.9, 0.1, 0.2),
        track_base_color: Color::srgb(0.12, 0.02, 0.04),
        rail_emissive: LinearRgba::new(1.0, 0.05, 0.1, 1.0), // Emergency Blood Crimson
        arch_color: Color::srgb(0.25, 0.05, 0.08),
        accent_glow: LinearRgba::new(1.0, 0.1, 0.2, 1.0), // Glitch warnings
        speed_modifier: 1.75,
        obstacle_density: 0.40,
    },
    // Zone 7: ECHO Core (8500m+)
    ZoneConfig {
        id: 7,
        name: "ZONE 7 — THE ECHO CORE",
        subtitle: "Deep Crystal Chamber // AI Resonance Maximum",
        start_distance: 8500.0,
        end_distance: f32::INFINITY,
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
    for zone in ZONES.iter() {
        if distance >= zone.start_distance && distance < zone.end_distance {
            return *zone;
        }
    }
    ZONES[ZONES.len() - 1]
}

#[allow(dead_code)]
pub fn get_zone_by_id(id: usize) -> Option<ZoneConfig> {
    ZONES.iter().find(|z| z.id == id).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_boundaries_are_contiguous_and_unbroken() {
        assert_eq!(ZONES[0].start_distance, 0.0, "Zone 1 must begin at 0.0m");
        for i in 0..ZONES.len() - 1 {
            assert_eq!(
                ZONES[i].end_distance,
                ZONES[i + 1].start_distance,
                "Zone {} end ({:.0}m) must equal Zone {} start ({:.0}m)",
                ZONES[i].id,
                ZONES[i].end_distance,
                ZONES[i + 1].id,
                ZONES[i + 1].start_distance
            );
        }
        assert!(
            ZONES.last().unwrap().end_distance.is_infinite(),
            "Final zone (Zone 7) must have infinite end distance"
        );
    }

    #[test]
    fn test_boss_milestones_align_with_zones() {
        assert_eq!(
            BOSS_MILESTONE_4_DISTANCE, ZONES[3].start_distance,
            "Milestone 4 boss must trigger at Zone 4 start"
        );
        assert_eq!(
            BOSS_MILESTONE_7_DISTANCE, ZONES[6].start_distance,
            "Milestone 7 boss must trigger at Zone 7 start"
        );
    }

    #[test]
    fn test_zone_lookup_accuracy() {
        assert_eq!(get_zone_for_distance(0.0).id, 1);
        assert_eq!(get_zone_for_distance(799.0).id, 1);
        assert_eq!(get_zone_for_distance(800.0).id, 2);
        assert_eq!(get_zone_for_distance(1800.0).id, 3);
        assert_eq!(get_zone_for_distance(3000.0).id, 4);
        assert_eq!(get_zone_for_distance(4800.0).id, 5);
        assert_eq!(get_zone_for_distance(6500.0).id, 6);
        assert_eq!(get_zone_for_distance(8500.0).id, 7);
        assert_eq!(get_zone_for_distance(25000.0).id, 7);
    }
}
