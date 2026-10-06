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
            .add_systems(OnEnter(AppState::InGame), reset_director)
            .add_systems(
                FixedUpdate,
                update_run_director.run_if(in_state(AppState::InGame)),
            );
    }
}

fn reset_director(mut director: ResMut<RunDirector>) {
    *director = RunDirector::default();
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
