use crate::zones::get_zone_for_distance;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugStartPreset {
    pub label: &'static str,
    pub distance: f32,
}

pub const DEBUG_START_PRESETS: [DebugStartPreset; 20] = [
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
    DebugStartPreset {
        label: "ZONE 5 // SKY RAIL 8000M",
        distance: 8000.0,
    },
    DebugStartPreset {
        label: "ZONE 5 // UPPER AURELIA 8300M",
        distance: 8300.0,
    },
    DebugStartPreset {
        label: "ZONE 5 // STORM CORRIDOR 8700M",
        distance: 8700.0,
    },
    DebugStartPreset {
        label: "ZONE 5 // BROKEN SKYWAY 9200M",
        distance: 9200.0,
    },
    DebugStartPreset {
        label: "ZONE 5 // ECHO APPROACH 9650M",
        distance: 9650.0,
    },
    DebugStartPreset {
        label: "ZONE 6 // FORBIDDEN 10000M",
        distance: 10000.0,
    },
    DebugStartPreset {
        label: "ZONE 7 // ECHO CORE 12000M",
        distance: 12000.0,
    },
];

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugStartConfig {
    pub selected_preset: usize,
    pub autostart: bool,
}

impl Default for DebugStartConfig {
    fn default() -> Self {
        Self::from_env_or_args()
    }
}

pub fn preset_for_zone_id(zone_id: usize) -> usize {
    match zone_id {
        1 => 1,
        2 => 4,
        3 => 7,
        4 => 12,
        5 => 13,
        6 => 18,
        7 => 19,
        _ => 0,
    }
}

pub fn preset_for_distance(target: f32) -> usize {
    let mut best_idx = 0;
    let mut best_diff = f32::MAX;
    for (idx, preset) in DEBUG_START_PRESETS.iter().enumerate() {
        if idx == 0 && target > 0.0 {
            continue;
        }
        let diff = (preset.distance - target).abs();
        if diff < best_diff {
            best_diff = diff;
            best_idx = idx;
        }
    }
    best_idx
}

pub fn preset_for_zone_str(s: &str) -> Option<usize> {
    let clean = s.trim().to_lowercase();
    match clean.as_str() {
        "1" | "zone1" | "zone-1" | "metro" | "oldmetro" => Some(1),
        "2" | "zone2" | "zone-2" | "neon" | "neondistrict" => Some(4),
        "3" | "zone3" | "zone-3" | "industrial" => Some(7),
        "4" | "zone4" | "zone-4" | "flooded" | "floodedmetro" => Some(12),
        "5" | "zone5" | "zone-5" | "sky" | "skyrail" | "sky-rail" => Some(13),
        "6" | "zone6" | "zone-6" | "forbidden" => Some(18),
        "7" | "zone7" | "zone-7" | "echo" | "echocore" => Some(19),
        _ => {
            if let Ok(num) = clean.parse::<usize>() {
                if (1..=7).contains(&num) {
                    return Some(preset_for_zone_id(num));
                }
            }
            None
        }
    }
}

pub fn preset_for_positional_zone(arg: &str) -> Option<usize> {
    let lower = arg.to_lowercase();
    match lower.as_str() {
        "1" | "zone1" | "zone-1" => Some(1),
        "2" | "zone2" | "zone-2" => Some(4),
        "3" | "zone3" | "zone-3" => Some(7),
        "4" | "zone4" | "zone-4" => Some(12),
        "5" | "zone5" | "zone-5" | "skyrail" => Some(13),
        "6" | "zone6" | "zone-6" => Some(18),
        "7" | "zone7" | "zone-7" => Some(19),
        _ => None,
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

    pub fn from_env_or_args() -> Self {
        Self::from_args_and_env(std::env::args(), |var| std::env::var(var).ok())
    }

    pub fn from_args_and_env<I, F>(args: I, env_lookup: F) -> Self
    where
        I: IntoIterator<Item = String>,
        F: Fn(&str) -> Option<String>,
    {
        let mut selected_preset = 0;
        let mut autostart = false;
        let mut explicit_selection = false;

        // Check environment variables first
        if let Some(zone_str) = env_lookup("ZONE")
            .or_else(|| env_lookup("RUNLINE_ZONE"))
            .or_else(|| env_lookup("START_ZONE"))
        {
            if let Some(preset_idx) = preset_for_zone_str(&zone_str) {
                selected_preset = preset_idx;
                autostart = true;
                explicit_selection = true;
            }
        }

        if let Some(dist_str) = env_lookup("DISTANCE").or_else(|| env_lookup("RUNLINE_DISTANCE")) {
            if let Ok(dist) = dist_str.parse::<f32>() {
                selected_preset = preset_for_distance(dist);
                autostart = true;
                explicit_selection = true;
            }
        }

        if let Some(preset_str) = env_lookup("PRESET").or_else(|| env_lookup("RUNLINE_PRESET")) {
            if let Ok(idx) = preset_str.parse::<usize>() {
                if idx < DEBUG_START_PRESETS.len() {
                    selected_preset = idx;
                    autostart = true;
                    explicit_selection = true;
                }
            }
        }

        // Parse CLI arguments (taking precedence over env vars)
        let args_vec: Vec<String> = args.into_iter().collect();
        let mut i = 1; // skip binary path
        while i < args_vec.len() {
            let arg = &args_vec[i];
            let lower = arg.to_lowercase();

            if lower == "--zone" || lower == "-z" {
                if i + 1 < args_vec.len() {
                    i += 1;
                    if let Some(preset_idx) = preset_for_zone_str(&args_vec[i]) {
                        selected_preset = preset_idx;
                        autostart = true;
                        explicit_selection = true;
                    }
                }
            } else if lower.starts_with("--zone=") {
                let val = &arg[7..];
                if let Some(preset_idx) = preset_for_zone_str(val) {
                    selected_preset = preset_idx;
                    autostart = true;
                    explicit_selection = true;
                }
            } else if lower.starts_with("-z=") {
                let val = &arg[3..];
                if let Some(preset_idx) = preset_for_zone_str(val) {
                    selected_preset = preset_idx;
                    autostart = true;
                    explicit_selection = true;
                }
            } else if lower == "--distance" || lower == "-d" || lower == "--dist" {
                if i + 1 < args_vec.len() {
                    i += 1;
                    if let Ok(dist) = args_vec[i].parse::<f32>() {
                        selected_preset = preset_for_distance(dist);
                        autostart = true;
                        explicit_selection = true;
                    }
                }
            } else if lower.starts_with("--distance=") {
                let val = &arg[11..];
                if let Ok(dist) = val.parse::<f32>() {
                    selected_preset = preset_for_distance(dist);
                    autostart = true;
                    explicit_selection = true;
                }
            } else if lower == "--preset" || lower == "-p" {
                if i + 1 < args_vec.len() {
                    i += 1;
                    if let Ok(idx) = args_vec[i].parse::<usize>() {
                        if idx < DEBUG_START_PRESETS.len() {
                            selected_preset = idx;
                            autostart = true;
                            explicit_selection = true;
                        }
                    }
                }
            } else if lower == "--skyrail" || lower == "--sky-rail" || lower == "--sky" {
                selected_preset = preset_for_zone_id(5);
                autostart = true;
                explicit_selection = true;
            } else if lower == "--flooded" || lower == "--flooded-metro" {
                selected_preset = preset_for_zone_id(4);
                autostart = true;
                explicit_selection = true;
            } else if lower == "--industrial" {
                selected_preset = preset_for_zone_id(3);
                autostart = true;
                explicit_selection = true;
            } else if lower == "--neon" {
                selected_preset = preset_for_zone_id(2);
                autostart = true;
                explicit_selection = true;
            } else if lower == "--metro" {
                selected_preset = preset_for_zone_id(1);
                autostart = true;
                explicit_selection = true;
            } else if lower == "--autostart"
                || lower == "--play"
                || lower == "--instant"
                || lower == "--direct"
            {
                autostart = true;
            } else if lower == "--menu" {
                autostart = false;
            } else if let Some(preset_idx) = preset_for_positional_zone(&lower) {
                selected_preset = preset_idx;
                autostart = true;
                explicit_selection = true;
            }

            i += 1;
        }

        if let Some(auto_env) = env_lookup("AUTOSTART").or_else(|| env_lookup("RUNLINE_AUTOSTART"))
        {
            if auto_env == "1" || auto_env.eq_ignore_ascii_case("true") {
                autostart = true;
            } else if auto_env == "0" || auto_env.eq_ignore_ascii_case("false") {
                autostart = false;
            }
        }

        Self {
            selected_preset,
            autostart: explicit_selection && autostart,
        }
    }
}

pub struct DebugStartPlugin;

impl Plugin for DebugStartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugStartConfig>()
            .add_systems(Update, handle_cli_autostart);
    }
}

pub fn handle_cli_autostart(
    mut executed: Local<bool>,
    config: Res<DebugStartConfig>,
    mut stats: ResMut<crate::types::GameRunStats>,
    mut powerups: ResMut<crate::types::ActivePowerUps>,
    mut run_reset_events: EventWriter<crate::types::RunResetEvent>,
    mut next_state: ResMut<NextState<crate::types::AppState>>,
) {
    if *executed {
        return;
    }
    *executed = true;

    if config.autostart {
        println!(
            "\n>>> [RUNLINE CLI] Auto-starting directly into {} ({:.0}M)\n",
            config.preset().label,
            config.distance()
        );
        run_reset_events.send(crate::types::RunResetEvent);
        stats.distance = config.distance();
        stats.fragments = 0;
        stats.data_chips = 0;
        stats.score = 0;
        stats.score_accum = 0.0;
        stats.current_zone = config.zone_id();
        stats.stumble_intensity = 0.0;
        stats.story_dialogue = None;
        *powerups = crate::types::ActivePowerUps::default();
        next_state.set(crate::types::AppState::InGame);
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
                5850.0, 6000.0, 8000.0, 8300.0, 8700.0, 9200.0, 9650.0, 10000.0, 12000.0
            ]
        );
    }

    #[test]
    fn zone_four_preset_resolves_to_flooded_metro() {
        let preset = &DEBUG_START_PRESETS[12];
        assert_eq!(preset.distance, 6000.0);
        assert_eq!(get_zone_for_distance(preset.distance).id, 4);
    }

    #[test]
    fn zone_five_presets_resolve_to_sky_rail() {
        for preset in &DEBUG_START_PRESETS[13..18] {
            assert_eq!(get_zone_for_distance(preset.distance).id, 5);
        }
    }

    #[test]
    fn zone_six_and_seven_presets_resolve_correctly() {
        assert_eq!(
            get_zone_for_distance(DEBUG_START_PRESETS[18].distance).id,
            6
        );
        assert_eq!(
            get_zone_for_distance(DEBUG_START_PRESETS[19].distance).id,
            7
        );
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
            autostart: false,
        };
        assert_eq!(config.distance(), 0.0);
        assert_eq!(config.zone_id(), 1);
    }

    #[test]
    fn cli_args_zone_five_selection() {
        let cfg = DebugStartConfig::from_args_and_env(
            vec!["runline".into(), "--zone".into(), "5".into()],
            |_| None,
        );
        assert_eq!(cfg.selected_preset, 13);
        assert_eq!(cfg.distance(), 8000.0);
        assert_eq!(cfg.zone_id(), 5);
        assert!(cfg.autostart);
    }

    #[test]
    fn cli_args_positional_zone_selection() {
        let cfg = DebugStartConfig::from_args_and_env(vec!["runline".into(), "5".into()], |_| None);
        assert_eq!(cfg.selected_preset, 13);
        assert_eq!(cfg.distance(), 8000.0);
        assert!(cfg.autostart);
    }

    #[test]
    fn cli_args_distance_selection() {
        let cfg = DebugStartConfig::from_args_and_env(
            vec!["runline".into(), "-d".into(), "8700".into()],
            |_| None,
        );
        assert_eq!(cfg.selected_preset, 15);
        assert_eq!(cfg.distance(), 8700.0);
        assert!(cfg.autostart);
    }

    #[test]
    fn env_var_zone_selection() {
        let cfg = DebugStartConfig::from_args_and_env(vec!["runline".into()], |var| {
            if var == "ZONE" {
                Some("5".into())
            } else {
                None
            }
        });
        assert_eq!(cfg.selected_preset, 13);
        assert_eq!(cfg.distance(), 8000.0);
        assert!(cfg.autostart);
    }

    #[test]
    fn cli_args_menu_flag_disables_autostart() {
        let cfg = DebugStartConfig::from_args_and_env(
            vec![
                "runline".into(),
                "--zone".into(),
                "5".into(),
                "--menu".into(),
            ],
            |_| None,
        );
        assert_eq!(cfg.selected_preset, 13);
        assert!(!cfg.autostart);
    }

    #[test]
    fn cargo_test_args_do_not_trigger_autostart() {
        let cfg = DebugStartConfig::from_args_and_env(
            vec![
                "target/debug/deps/runline".into(),
                "--nocapture".into(),
                "test_name".into(),
            ],
            |_| None,
        );
        assert_eq!(cfg.selected_preset, 0);
        assert!(!cfg.autostart);
    }
}
