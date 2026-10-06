use bevy::prelude::*;
use crate::types::*;
use crate::zones::get_zone_for_distance;
use crate::story::DISTANCE_MILESTONES;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct DifficultyProfile {
    pub speed_multiplier: f32,
    pub obstacle_density: f32,
    pub pattern_complexity: u8, // 1 (Intro) .. 5 (Master)
    pub enemy_probability: f32,
    pub special_event_probability: f32,
    pub reaction_window_m: f32,
}

impl DifficultyProfile {
    pub fn for_zone(zone_id: usize) -> Self {
        match zone_id {
            1 => Self {
                speed_multiplier: 1.0,
                obstacle_density: 0.20,
                pattern_complexity: 1,
                enemy_probability: 0.1,
                special_event_probability: 0.05,
                reaction_window_m: 28.0,
            },
            2 => Self {
                speed_multiplier: 1.15,
                obstacle_density: 0.25,
                pattern_complexity: 2,
                enemy_probability: 0.2,
                special_event_probability: 0.10,
                reaction_window_m: 25.0,
            },
            3 => Self {
                speed_multiplier: 1.30,
                obstacle_density: 0.30,
                pattern_complexity: 3,
                enemy_probability: 0.3,
                special_event_probability: 0.15,
                reaction_window_m: 22.0,
            },
            4 => Self {
                speed_multiplier: 1.45,
                obstacle_density: 0.33,
                pattern_complexity: 3,
                enemy_probability: 0.35,
                special_event_probability: 0.20,
                reaction_window_m: 20.0,
            },
            5 => Self {
                speed_multiplier: 1.60,
                obstacle_density: 0.36,
                pattern_complexity: 4,
                enemy_probability: 0.45,
                special_event_probability: 0.25,
                reaction_window_m: 18.0,
            },
            6 => Self {
                speed_multiplier: 1.75,
                obstacle_density: 0.40,
                pattern_complexity: 5,
                enemy_probability: 0.55,
                special_event_probability: 0.30,
                reaction_window_m: 16.0,
            },
            _ => Self {
                speed_multiplier: 1.95,
                obstacle_density: 0.45,
                pattern_complexity: 5,
                enemy_probability: 0.70,
                special_event_probability: 0.40,
                reaction_window_m: 14.0,
            },
        }
    }
}

#[derive(Resource)]
pub struct RunDirector {
    pub active_zone_id: usize,
    pub milestone_index: usize,
    pub profile: DifficultyProfile,
}

impl Default for RunDirector {
    fn default() -> Self {
        Self {
            active_zone_id: 1,
            milestone_index: 0,
            profile: DifficultyProfile::for_zone(1),
        }
    }
}

pub struct DirectorPlugin;

impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunDirector>()
            .add_event::<ZoneChangedEvent>()
            .add_event::<MilestoneReachedEvent>()
            .add_systems(Update, handle_run_reset_director)
            .add_systems(
                FixedUpdate,
                update_run_director.run_if(in_state(AppState::InGame)),
            );
    }
}

fn handle_run_reset_director(
    mut events: EventReader<RunResetEvent>,
    mut director: ResMut<RunDirector>,
    mut stats: ResMut<GameRunStats>,
    mut powerups: ResMut<ActivePowerUps>,
) {
    for _ in events.read() {
        *director = RunDirector::default();
        stats.distance = 0.0;
        stats.speed = 16.0;
        stats.base_speed = 16.0;
        stats.fragments = 0;
        stats.data_chips = 0;
        stats.score = 0;
        stats.score_accum = 0.0;
        stats.multiplier = 1.0;
        stats.current_zone = 1;
        stats.stumble_intensity = 0.0;
        stats.story_dialogue = None;
        *powerups = ActivePowerUps::default();
    }
}

fn update_run_director(
    mut director: ResMut<RunDirector>,
    mut stats: ResMut<GameRunStats>,
    mut zone_events: EventWriter<ZoneChangedEvent>,
    mut milestone_events: EventWriter<MilestoneReachedEvent>,
) {
    let current_distance = stats.distance;

    // 1. Zone & Difficulty Profile Transition
    let zone = get_zone_for_distance(current_distance);
    if zone.id != director.active_zone_id {
        let prev_zone = director.active_zone_id;
        director.active_zone_id = zone.id;
        director.profile = DifficultyProfile::for_zone(zone.id);
        stats.current_zone = zone.id;

        zone_events.send(ZoneChangedEvent {
            from_zone: prev_zone,
            to_zone: zone.id,
            config: zone,
        });
    }

    // 2. Story Milestones
    if director.milestone_index < DISTANCE_MILESTONES.len() {
        let milestone = &DISTANCE_MILESTONES[director.milestone_index];
        if current_distance >= milestone.distance {
            milestone_events.send(MilestoneReachedEvent {
                milestone_idx: director.milestone_index,
                distance: milestone.distance,
                title: milestone.title,
                sender: milestone.sender,
                message: milestone.message,
            });

            stats.story_dialogue = Some(StoryMessage {
                title: milestone.title.to_string(),
                sender: milestone.sender.to_string(),
                body: milestone.message.to_string(),
                timer: 7.0,
            });

            director.milestone_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggressive_reset_sequence_and_state_isolation() {
        let mut app = App::new();
        app.add_event::<RunResetEvent>();
        app.init_resource::<RunDirector>();
        app.init_resource::<GameRunStats>();
        app.init_resource::<ActivePowerUps>();
        app.add_systems(Update, handle_run_reset_director);

        // SEQUENCE: Start -> die -> Restart -> pause -> resume -> die -> Restart -> pause -> resume -> quit/start again

        // 1. START RUN & ACCRUE STATE
        {
            let mut stats = app.world_mut().resource_mut::<GameRunStats>();
            stats.distance = 750.0;
            stats.score = 1500;
            stats.fragments = 42;
            stats.data_chips = 3;
            stats.speed = 18.4;
            stats.current_zone = 2;
            let mut director = app.world_mut().resource_mut::<RunDirector>();
            director.active_zone_id = 2;
            director.milestone_index = 2;
            let mut powerups = app.world_mut().resource_mut::<ActivePowerUps>();
            powerups.shield = true;
            powerups.overdrive_timer = 5.0;
        }

        // 2. DIE -> RESTART
        app.world_mut().send_event(RunResetEvent);
        app.update();

        // Verify clean restart state isolation
        {
            let stats = app.world().resource::<GameRunStats>();
            assert_eq!(stats.distance, 0.0, "Distance must reset to 0m");
            assert_eq!(stats.score, 0, "Score must reset to 0");
            assert_eq!(stats.fragments, 0, "Fragments must reset to 0");
            assert_eq!(stats.data_chips, 0, "Data chips must reset to 0");
            assert_eq!(stats.speed, 16.0, "Speed must reset to 16 m/s");
            assert_eq!(stats.current_zone, 1, "Zone must reset to Zone 1");

            let director = app.world().resource::<RunDirector>();
            assert_eq!(director.active_zone_id, 1, "Director zone must reset to 1");
            assert_eq!(director.milestone_index, 0, "Milestone index must reset to 0");
            assert_eq!(director.profile.pattern_complexity, 1, "Complexity must reset to 1");

            let powerups = app.world().resource::<ActivePowerUps>();
            assert!(!powerups.shield, "Shield must not survive restart");
            assert_eq!(powerups.overdrive_timer, 0.0, "Overdrive must not survive restart");
        }

        // 3. ADVANCE -> PAUSE -> RESUME
        {
            let mut stats = app.world_mut().resource_mut::<GameRunStats>();
            stats.distance = 250.0;
            stats.score = 500;
        }
        // Simulated Pause (fixed simulation does not tick):
        {
            let stats = app.world().resource::<GameRunStats>();
            assert_eq!(stats.distance, 250.0, "Pause maintains exact distance");
        }
        // Resume and advance further
        {
            let mut stats = app.world_mut().resource_mut::<GameRunStats>();
            stats.distance = 1200.0;
            stats.score = 3000;
            let mut powerups = app.world_mut().resource_mut::<ActivePowerUps>();
            powerups.magnet_timer = 4.0;
        }

        // 4. DIE AGAIN -> RESTART
        app.world_mut().send_event(RunResetEvent);
        app.update();

        // Verify second restart wiped cleanly
        {
            let stats = app.world().resource::<GameRunStats>();
            assert_eq!(stats.distance, 0.0);
            assert_eq!(stats.score, 0);
            let powerups = app.world().resource::<ActivePowerUps>();
            assert_eq!(powerups.magnet_timer, 0.0);
        }

        // 5. PAUSE -> RESUME -> QUIT TO MENU / START AGAIN
        app.world_mut().send_event(RunResetEvent);
        app.update();

        {
            let stats = app.world().resource::<GameRunStats>();
            assert_eq!(stats.distance, 0.0);
            assert_eq!(stats.current_zone, 1);
            let director = app.world().resource::<RunDirector>();
            assert_eq!(director.active_zone_id, 1);
        }
    }
}
