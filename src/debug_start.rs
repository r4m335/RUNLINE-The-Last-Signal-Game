use crate::zones::get_zone_for_distance;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugStartPreset {
    pub label: &'static str,
    pub distance: f32,
}

pub const DEBUG_START_PRESETS: [DebugStartPreset; 13] = [
    DebugStartPreset {
        label: "NORMAL RUN",
        distance: 0.0,
    },
    DebugStartPreset {
        label: "ZONE 1 // 0M",
        distance: 0.0,
    },
    DebugStartPreset {
        label: "ZONE 1 // 1000M",
        distance: 1000.0,
    },
    DebugStartPreset {
        label: "ZONE 1 // 1800M",
        distance: 1800.0,
    },
    DebugStartPreset {
        label: "ZONE 2 // 2000M",
        distance: 2000.0,
    },
    DebugStartPreset {
        label: "ZONE 2 // 3000M",
        distance: 3000.0,
    },
    DebugStartPreset {
        label: "ZONE 2 // 3800M",
        distance: 3800.0,
    },
    DebugStartPreset {
        label: "ZONE 3 // ENTRY 4000M",
        distance: 4000.0,
    },
    DebugStartPreset {
        label: "ZONE 3 // FREIGHT 4500M",
        distance: 4500.0,
    },
    DebugStartPreset {
        label: "ZONE 3 // PROCESSING 5100M",
        distance: 5100.0,
    },
    DebugStartPreset {
        label: "ZONE 3 // HEAVY WORKS 5500M",
        distance: 5500.0,
    },
    DebugStartPreset {
        label: "ZONE 3 // TRANSITION 5850M",
        distance: 5850.0,
    },
    DebugStartPreset {
        label: "ZONE 4 // 6000M",
        distance: 6000.0,
    },
];

#[derive(bevy::prelude::Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugStartConfig {
    pub selected_preset: usize,
}

impl Default for DebugStartConfig {
    fn default() -> Self {
        Self { selected_preset: 0 }
    }
}

impl DebugStartConfig {
    pub fn preset(self) -> DebugStartPreset {
        DEBUG_START_PRESETS
            .get(self.selected_preset)
            .copied()
            .unwrap_or(DEBUG_START_PRESETS[0])
    }

    pub fn distance(self) -> f32 {
        self.preset().distance
    }

    pub fn zone_id(self) -> usize {
        get_zone_for_distance(self.distance()).id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zones::get_zone_for_distance;

    #[test]
    fn presets_cover_requested_zone_test_points() {
        let distances: Vec<f32> = DEBUG_START_PRESETS
            .iter()
            .map(|preset| preset.distance)
            .collect();
        assert_eq!(
            distances,
            vec![
                0.0, 0.0, 1000.0, 1800.0, 2000.0, 3000.0, 3800.0, 4000.0, 4500.0, 5100.0, 5500.0,
                5850.0, 6000.0
            ]
        );
    }

    #[test]
    fn zone_four_preset_resolves_to_flooded_metro() {
        let preset = DEBUG_START_PRESETS.last().unwrap();
        assert_eq!(preset.distance, 6000.0);
        assert_eq!(get_zone_for_distance(preset.distance).id, 4);
    }

    #[test]
    fn zone_three_presets_resolve_to_industrial_sector() {
        for preset in &DEBUG_START_PRESETS[7..12] {
            assert_eq!(get_zone_for_distance(preset.distance).id, 3);
        }
    }

    #[test]
    fn invalid_selection_falls_back_to_normal_run() {
        let config = DebugStartConfig {
            selected_preset: usize::MAX,
        };
        assert_eq!(config.distance(), 0.0);
        assert_eq!(config.zone_id(), 1);
    }
}
