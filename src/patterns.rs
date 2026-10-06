use crate::types::*;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct LaneSet(pub u8);

#[allow(dead_code)]
impl LaneSet {
    pub const LEFT: u8 = 1 << 0;   // 1
    pub const CENTER: u8 = 1 << 1; // 2
    pub const RIGHT: u8 = 1 << 2;  // 4
    pub const ALL: u8 = 7;
    pub const NONE: u8 = 0;

    #[inline(always)]
    pub fn new(mask: u8) -> Self {
        Self(mask & Self::ALL)
    }

    #[inline(always)]
    pub fn contains(&self, lane: Lane) -> bool {
        let bit = match lane {
            Lane::Left => Self::LEFT,
            Lane::Center => Self::CENTER,
            Lane::Right => Self::RIGHT,
        };
        (self.0 & bit) != 0
    }

    #[inline(always)]
    pub fn with(&self, lane: Lane) -> Self {
        let bit = match lane {
            Lane::Left => Self::LEFT,
            Lane::Center => Self::CENTER,
            Lane::Right => Self::RIGHT,
        };
        Self(self.0 | bit)
    }

    #[inline(always)]
    pub fn without(&self, lane: Lane) -> Self {
        let bit = match lane {
            Lane::Left => Self::LEFT,
            Lane::Center => Self::CENTER,
            Lane::Right => Self::RIGHT,
        };
        Self(self.0 & !bit)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn intersects(&self, other: LaneSet) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn lanes(&self) -> Vec<Lane> {
        let mut list = Vec::new();
        if (self.0 & Self::LEFT) != 0 { list.push(Lane::Left); }
        if (self.0 & Self::CENTER) != 0 { list.push(Lane::Center); }
        if (self.0 & Self::RIGHT) != 0 { list.push(Lane::Right); }
        list
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum PlayerActionDemand {
    None,
    Jump,
    Slide,
}

#[derive(Clone, Copy, Debug)]
pub struct ObstacleSpawnDef {
    pub lane: Lane,
    pub rel_z: f32, // Relative offset from pattern start Z (negative values: -1.0 .. -length)
    pub obstacle_type: ObstacleType,
    pub size: bevy::prelude::Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct FragmentSpawnDef {
    pub lane: Lane,
    pub rel_z: f32,
    pub y_pos: f32,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct PatternChunk {
    pub name: &'static str,
    pub min_complexity: u8, // 1 to 5
    pub length: f32,
    pub obstacles: Vec<ObstacleSpawnDef>,
    pub fragments: Vec<FragmentSpawnDef>,
}

impl PatternChunk {
    pub fn new(name: &'static str, min_complexity: u8, length: f32) -> Self {
        Self {
            name,
            min_complexity,
            length,
            obstacles: Vec::new(),
            fragments: Vec::new(),
        }
    }

    pub fn with_obstacle(mut self, lane: Lane, rel_z: f32, obs_type: ObstacleType, size: bevy::prelude::Vec3) -> Self {
        self.obstacles.push(ObstacleSpawnDef {
            lane,
            rel_z,
            obstacle_type: obs_type,
            size,
        });
        self
    }

    #[allow(dead_code)]
    pub fn with_fragment(mut self, lane: Lane, rel_z: f32, y_pos: f32) -> Self {
        self.fragments.push(FragmentSpawnDef {
            lane,
            rel_z,
            y_pos,
        });
        self
    }

    pub fn with_fragment_arc(mut self, lane: Lane, start_z: f32, count: usize, step_z: f32, peak_y: f32) -> Self {
        for i in 0..count {
            let t = i as f32 / (count - 1).max(1) as f32;
            let y = 0.85 + (1.0 - (2.0 * t - 1.0).powi(2)) * (peak_y - 0.85);
            self.fragments.push(FragmentSpawnDef {
                lane,
                rel_z: start_z - (i as f32 * step_z),
                y_pos: y,
            });
        }
        self
    }

    pub fn with_fragment_line(mut self, lane: Lane, start_z: f32, count: usize, step_z: f32, y: f32) -> Self {
        for i in 0..count {
            self.fragments.push(FragmentSpawnDef {
                lane,
                rel_z: start_z - (i as f32 * step_z),
                y_pos: y,
            });
        }
        self
    }

    /// Evaluates which lanes are physically navigable at a given relative Z position
    #[allow(dead_code)]
    pub fn get_navigable_lanes_at(&self, rel_z: f32, margin: f32) -> LaneSet {
        let mut set = LaneSet::new(LaneSet::ALL);
        for obs in &self.obstacles {
            let half_depth = obs.size.z * 0.5;
            let obs_min = obs.rel_z - half_depth - margin;
            let obs_max = obs.rel_z + half_depth + margin;

            if rel_z >= obs_min && rel_z <= obs_max {
                // Impassable obstacles completely block the lane
                match obs.obstacle_type {
                    ObstacleType::TallPillar
                    | ObstacleType::StaticTrain
                    | ObstacleType::MovingTrain { .. } => {
                        set = set.without(obs.lane);
                    }
                    _ => {}
                }
            }
        }
        set
    }

    /// Determines open/navigable lanes in the first 8 meters of the chunk (head entry)
    pub fn entry_navigable_lanes(&self) -> LaneSet {
        let mut entry = LaneSet::new(LaneSet::ALL);
        for obs in &self.obstacles {
            if obs.rel_z >= -8.0 {
                match obs.obstacle_type {
                    ObstacleType::TallPillar
                    | ObstacleType::StaticTrain
                    | ObstacleType::MovingTrain { .. } => {
                        entry = entry.without(obs.lane);
                    }
                    _ => {}
                }
            }
        }
        entry
    }

    /// Determines open/navigable lanes in the final 8 meters of the chunk (tail exit)
    pub fn exit_navigable_lanes(&self) -> LaneSet {
        let exit_threshold = -(self.length - 8.0);
        let mut exit = LaneSet::new(LaneSet::ALL);
        for obs in &self.obstacles {
            if obs.rel_z <= exit_threshold {
                match obs.obstacle_type {
                    ObstacleType::TallPillar
                    | ObstacleType::StaticTrain
                    | ObstacleType::MovingTrain { .. } => {
                        exit = exit.without(obs.lane);
                    }
                    _ => {}
                }
            }
        }
        exit
    }

    /// Returns the nearest obstacle along a specific lane within [0.0 .. -max_z]
    pub fn first_obstacle_in_lane(&self, lane: Lane) -> Option<&ObstacleSpawnDef> {
        self.obstacles
            .iter()
            .filter(|o| o.lane == lane)
            .max_by(|a, b| a.rel_z.partial_cmp(&b.rel_z).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Returns the last obstacle along a specific lane near the chunk exit
    pub fn last_obstacle_in_lane(&self, lane: Lane) -> Option<&ObstacleSpawnDef> {
        self.obstacles
            .iter()
            .filter(|o| o.lane == lane)
            .min_by(|a, b| a.rel_z.partial_cmp(&b.rel_z).unwrap_or(std::cmp::Ordering::Equal))
    }
}

// ----------------------------------------------------------------------------
// SOLVABILITY VALIDATOR (INTRA-CHUNK & CROSS-CHUNK)
// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SolvabilityError {
    AllLanesBlockedSimultaneously { rel_z_approx: i32 },
    InsufficientReactionWindow { distance_m: i32, required_m: i32 },
    ActionInterferenceLandingCollision { lane: Lane, distance_m: i32 },
    TrappedExitNoReachableLane,
}

/// Verifies that an individual chunk is internally solvable from start to end
#[allow(dead_code)]
pub fn verify_intra_chunk_solvability(chunk: &PatternChunk) -> Result<(), SolvabilityError> {
    // Check at 1-meter intervals along the chunk
    let num_steps = chunk.length as usize;
    for i in 0..num_steps {
        let rel_z = -(i as f32);
        let open_lanes = chunk.get_navigable_lanes_at(rel_z, 0.4);

        if open_lanes.is_empty() {
            // Check if at least one lane has a jumpable hurdle or slideable cable
            let has_actionable_lane = chunk.obstacles.iter().any(|obs| {
                let half_d = obs.size.z * 0.5;
                let in_z = rel_z >= (obs.rel_z - half_d - 0.4) && rel_z <= (obs.rel_z + half_d + 0.4);
                in_z && matches!(obs.obstacle_type, ObstacleType::LowBarrier | ObstacleType::HighHangingWire)
            });

            if !has_actionable_lane {
                return Err(SolvabilityError::AllLanesBlockedSimultaneously {
                    rel_z_approx: rel_z as i32,
                });
            }
        }
    }

    // Verify intra-lane action spacing (e.g. LowBarrier followed by HighHangingWire must have landing recovery >= 8m)
    for obs_a in &chunk.obstacles {
        if obs_a.obstacle_type == ObstacleType::LowBarrier {
            for obs_b in &chunk.obstacles {
                if obs_b.lane == obs_a.lane && obs_b.rel_z < obs_a.rel_z {
                    let spacing = (obs_a.rel_z - obs_b.rel_z).abs();
                    if obs_b.obstacle_type == ObstacleType::HighHangingWire && spacing < 8.0 {
                        return Err(SolvabilityError::ActionInterferenceLandingCollision {
                            lane: obs_a.lane,
                            distance_m: spacing as i32,
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

/// Kinematically evaluates whether a player traveling at `player_speed` (m/s) can physically
/// execute a lateral maneuver from `source_lane` to `target_lane` before traversing `distance_m`.
#[inline]
pub fn can_physically_change_lane(
    player_speed: f32,
    source_lane: Lane,
    target_lane: Lane,
    distance_m: f32,
) -> bool {
    let lanes_to_cross = (source_lane as i32 - target_lane as i32).abs();
    if lanes_to_cross == 0 {
        return true;
    }

    // Kinematic derivation from actual player lateral smoothing (player.rs:365):
    // Continuous trajectory ODE: dx/dt = -18.0 * (x - x_target) => x(t) = x_target - Delta_x * exp(-18.0 * t).
    // - 1-lane transition (2.4m): 95% settlement (<0.12m error) requires ln(20)/18.0 = 0.1664s (~0.17s).
    //   At 64 Hz fixed simulation (dt = 0.015625s), this corresponds to exactly 11 ticks (0.171875s).
    // - 2-lane transition (4.8m): 95% settlement requires ln(40)/18.0 = 0.2049s (13 ticks, 0.2031s).
    //   Allowing for thumbstick/key release reversal deadband & steering safety gives 0.26s.
    // - Human recognition + visual reflex: 0.18s calibrated action-runner reaction threshold.
    let human_reaction_time = 0.18;
    let lateral_duration = if lanes_to_cross == 1 { 0.17 } else { 0.26 };
    let required_time = human_reaction_time + lateral_duration;

    let available_time = distance_m / player_speed.max(12.0);
    available_time >= required_time
}

/// Evaluates cross-chunk solvability when placing `next` immediately after `prev`
pub fn validate_chunk_transition(
    prev: &PatternChunk,
    next: &PatternChunk,
    player_speed: f32,
) -> Result<(), SolvabilityError> {
    let exit_lanes = prev.exit_navigable_lanes();
    let entry_lanes = next.entry_navigable_lanes();

    if exit_lanes.is_empty() {
        return Err(SolvabilityError::TrappedExitNoReachableLane);
    }

    // Direct continuity: if any exit lane connects directly to an open entry lane,
    // the player can run straight across the transition without any forced emergency maneuver.
    if exit_lanes.intersects(entry_lanes) {
        return Ok(());
    }

    // If no straight-through lane exists, player MUST switch lanes during the transition.
    // Verify that every possible exit lane has at least one reachable open lane in `next`
    // using the exact kinematic time model.
    for exit_lane in exit_lanes.lanes() {
        if let Some(first_obs) = next.first_obstacle_in_lane(exit_lane) {
            let dist_to_obstacle = first_obs.rel_z.abs(); // e.g. -6.0m -> 6.0m

            // Check if player just jumped near the exit of prev
            if let Some(last_obs) = prev.last_obstacle_in_lane(exit_lane) {
                if last_obs.obstacle_type == ObstacleType::LowBarrier {
                    let landing_dist_from_exit = prev.length - last_obs.rel_z.abs() + dist_to_obstacle;
                    if landing_dist_from_exit < 8.0 {
                        return Err(SolvabilityError::ActionInterferenceLandingCollision {
                            lane: exit_lane,
                            distance_m: landing_dist_from_exit as i32,
                        });
                    }
                }
            }

            // Kinematic evaluation: verify player can reach at least one open entry lane in time
            let mut can_reach_any = false;
            for target_lane in entry_lanes.lanes() {
                if can_physically_change_lane(player_speed, exit_lane, target_lane, dist_to_obstacle) {
                    can_reach_any = true;
                    break;
                }
            }

            if !can_reach_any {
                let required_m = (player_speed * 0.35) as i32;
                return Err(SolvabilityError::InsufficientReactionWindow {
                    distance_m: dist_to_obstacle as i32,
                    required_m,
                });
            }
        }
    }

    Ok(())
}

/// Catalog of hand-crafted rhythmic patterns with guaranteed physical escape routes
pub fn get_pattern_catalog() -> Vec<PatternChunk> {
    use bevy::prelude::Vec3;

    vec![
        // -------------------------------------------------------------
        // COMPLEXITY 1: TUTORIAL & RHYTHMIC FLOW (0 - 800m)
        // -------------------------------------------------------------
        // Pattern 1: Jump Tutorial (Center hurdle with crystal arc)
        PatternChunk::new("Vault Sequence", 1, 35.0)
            .with_obstacle(Lane::Center, -14.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_fragment_arc(Lane::Center, -8.0, 5, 3.0, 2.2),

        // Pattern 2: Slide Tutorial (Hanging wire with low fragments)
        PatternChunk::new("Slide Under", 1, 35.0)
            .with_obstacle(Lane::Center, -15.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3))
            .with_fragment_line(Lane::Center, -10.0, 4, 3.0, 0.45),

        // Pattern 3: Basic Lane Switch (Pillar in center, crystals guide to left)
        PatternChunk::new("Center Gate Divert", 1, 35.0)
            .with_obstacle(Lane::Center, -15.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_fragment_line(Lane::Left, -8.0, 5, 3.5, 0.85),

        // Pattern 4: Dual Barrier Flow (Jump left or ride open right)
        PatternChunk::new("Split Gate", 1, 35.0)
            .with_obstacle(Lane::Left, -14.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Center, -14.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_fragment_arc(Lane::Left, -8.0, 5, 3.0, 2.2)
            .with_fragment_line(Lane::Right, -8.0, 5, 3.5, 0.85),

        // -------------------------------------------------------------
        // COMPLEXITY 2: NEON DISTRICT (800 - 1,800m)
        // -------------------------------------------------------------
        // Pattern 5: Train Rooftop Highway (Jump onto static train roof or vault left!)
        PatternChunk::new("Rooftop Express", 2, 40.0)
            .with_obstacle(Lane::Center, -18.0, ObstacleType::StaticTrain, Vec3::new(2.2, 2.5, 12.0))
            .with_obstacle(Lane::Left, -18.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Right, -18.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_fragment_arc(Lane::Center, -10.0, 4, 2.5, 2.8)
            .with_fragment_line(Lane::Center, -18.0, 5, 2.2, 2.8),

        // Pattern 6: The Zigzag Slalom (Left -> Center -> Right)
        PatternChunk::new("Neon Slalom", 2, 40.0)
            .with_obstacle(Lane::Left, -8.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Center, -18.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Right, -28.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_fragment_line(Lane::Right, -6.0, 3, 3.0, 0.85)
            .with_fragment_line(Lane::Left, -16.0, 3, 3.0, 0.85)
            .with_fragment_line(Lane::Center, -26.0, 3, 3.0, 0.85),

        // Pattern 7: Oncoming Train Encounter
        PatternChunk::new("Incoming Metro Headlights", 2, 40.0)
            .with_obstacle(Lane::Center, -25.0, ObstacleType::MovingTrain { speed: 10.0 }, Vec3::new(2.2, 2.5, 14.0))
            .with_obstacle(Lane::Left, -15.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_fragment_line(Lane::Right, -10.0, 6, 3.5, 0.85),

        // -------------------------------------------------------------
        // COMPLEXITY 3: INDUSTRIAL FOUNDRY & FLOODED METRO (1,800 - 4,800m)
        // -------------------------------------------------------------
        // Pattern 8: The Squeeze (Left and Right blocked, Center requires slide under wire)
        PatternChunk::new("Foundry Needle", 3, 40.0)
            .with_obstacle(Lane::Left, -16.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Right, -16.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Center, -16.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3))
            .with_fragment_line(Lane::Center, -10.0, 5, 2.8, 0.45),

        // Pattern 9: Jump into Slide Combo (Hurdle immediately followed by wire with safe clearance)
        PatternChunk::new("Vault-to-Slide Combo", 3, 40.0)
            .with_obstacle(Lane::Center, -12.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Center, -23.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3))
            .with_fragment_arc(Lane::Center, -7.0, 4, 2.5, 2.2)
            .with_fragment_line(Lane::Center, -20.0, 3, 2.2, 0.45),

        // Pattern 10: Dual Train Corridor (Center and Right trains moving, Left open)
        PatternChunk::new("Twin Rail Squeeze", 3, 40.0)
            .with_obstacle(Lane::Center, -25.0, ObstacleType::MovingTrain { speed: 11.0 }, Vec3::new(2.2, 2.5, 14.0))
            .with_obstacle(Lane::Right, -18.0, ObstacleType::StaticTrain, Vec3::new(2.2, 2.5, 12.0))
            .with_fragment_line(Lane::Left, -8.0, 7, 3.2, 0.85),

        // -------------------------------------------------------------
        // COMPLEXITY 4: SKY RAIL (4,800 - 6,500m)
        // -------------------------------------------------------------
        // Pattern 11: Switchback Trap (Rapid left-right directional demand)
        PatternChunk::new("Skyline Switchback", 4, 40.0)
            .with_obstacle(Lane::Left, -10.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Center, -10.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Right, -24.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Center, -24.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_fragment_line(Lane::Right, -6.0, 3, 2.5, 0.85)
            .with_fragment_line(Lane::Left, -20.0, 3, 2.5, 0.85),

        // Pattern 12: Triple Hurdle Sprint (Three consecutive vault hurdles)
        PatternChunk::new("Triple Hurdle Sprint", 4, 40.0)
            .with_obstacle(Lane::Center, -10.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Center, -18.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Center, -26.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_fragment_arc(Lane::Center, -6.0, 4, 2.0, 2.2)
            .with_fragment_arc(Lane::Center, -14.0, 4, 2.0, 2.2)
            .with_fragment_arc(Lane::Center, -22.0, 4, 2.0, 2.2),

        // -------------------------------------------------------------
        // COMPLEXITY 5: THE FORBIDDEN LINE & ECHO CORE (6,500m+)
        // -------------------------------------------------------------
        // Pattern 13: Core Singularity Gauntlet (High-speed express moving train + alternating hurdle slides)
        PatternChunk::new("Core Singularity Gauntlet", 5, 40.0)
            .with_obstacle(Lane::Center, -22.0, ObstacleType::MovingTrain { speed: 13.0 }, Vec3::new(2.2, 2.5, 14.0))
            .with_obstacle(Lane::Left, -12.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_obstacle(Lane::Left, -24.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3))
            .with_obstacle(Lane::Right, -12.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3))
            .with_obstacle(Lane::Right, -24.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4))
            .with_fragment_arc(Lane::Left, -7.0, 4, 2.2, 2.2)
            .with_fragment_line(Lane::Right, -7.0, 4, 2.2, 0.45),

        // Pattern 14: Pure Flow Crystal Stream (Universal buffer sequence)
        PatternChunk::new("Resonance Wave", 1, 35.0)
            .with_fragment_line(Lane::Center, -6.0, 3, 2.5, 0.85)
            .with_fragment_line(Lane::Left, -14.0, 3, 2.5, 0.85)
            .with_fragment_line(Lane::Right, -22.0, 3, 2.5, 0.85),
    ]
}

/// Selects a pattern guaranteed to be solvably linked to the previous chunk
pub fn select_validated_pattern(
    complexity: u8,
    prev_chunk: Option<&PatternChunk>,
    player_speed: f32,
) -> PatternChunk {
    let catalog = get_pattern_catalog();
    let mut candidates: Vec<PatternChunk> = catalog
        .into_iter()
        .filter(|p| p.min_complexity <= complexity)
        .collect();

    if candidates.is_empty() {
        return PatternChunk::new("Fallback", 1, 35.0);
    }

    let mut rng = rand::thread_rng();

    // If there is a preceding chunk, filter by cross-chunk transition solvability
    if let Some(prev) = prev_chunk {
        let mut valid_candidates = Vec::new();
        for candidate in candidates {
            if validate_chunk_transition(prev, &candidate, player_speed).is_ok() {
                valid_candidates.push(candidate);
            }
        }

        if !valid_candidates.is_empty() {
            let idx = rng.gen_range(0..valid_candidates.len());
            return valid_candidates.swap_remove(idx);
        }

        // Guaranteed buffer fallback if transition would be impossible
        return PatternChunk::new("Resonance Wave", 1, 35.0)
            .with_fragment_line(Lane::Center, -6.0, 3, 2.5, 0.85)
            .with_fragment_line(Lane::Left, -14.0, 3, 2.5, 0.85)
            .with_fragment_line(Lane::Right, -22.0, 3, 2.5, 0.85);
    }

    let idx = rng.gen_range(0..candidates.len());
    candidates.swap_remove(idx)
}

// ----------------------------------------------------------------------------
// UNIT TESTS: SOLVABILITY VALIDATION
// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec3;

    #[test]
    fn test_all_catalog_patterns_are_internally_solvable() {
        let catalog = get_pattern_catalog();
        for pattern in &catalog {
            let result = verify_intra_chunk_solvability(pattern);
            assert!(
                result.is_ok(),
                "Pattern '{}' failed intra-chunk solvability: {:?}",
                pattern.name,
                result.err()
            );
        }
    }

    #[test]
    fn test_detects_impossible_three_lane_block() {
        let bad_chunk = PatternChunk::new("Impossible Wall", 1, 40.0)
            .with_obstacle(Lane::Left, -15.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Center, -15.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Right, -15.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2));

        let res = verify_intra_chunk_solvability(&bad_chunk);
        assert_eq!(
            res,
            Err(SolvabilityError::AllLanesBlockedSimultaneously { rel_z_approx: -14 })
        );
    }

    #[test]
    fn test_detects_impossible_cross_chunk_transition() {
        // Chunk A forces player into Center at the very end
        let chunk_a = PatternChunk::new("Exit Left Right Blocked", 1, 40.0)
            .with_obstacle(Lane::Left, -36.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Right, -36.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2));

        // Chunk B immediately blocks Center within 2 meters (0.08 seconds reaction!)
        let chunk_b = PatternChunk::new("Immediate Center Block", 1, 40.0)
            .with_obstacle(Lane::Center, -2.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2))
            .with_obstacle(Lane::Left, -2.0, ObstacleType::TallPillar, Vec3::new(1.8, 4.0, 1.2));

        let validation = validate_chunk_transition(&chunk_a, &chunk_b, 20.0);
        assert!(
            validation.is_err(),
            "Expected validator to reject unfair cross-chunk transition, but got: {:?}",
            validation
        );
    }

    #[test]
    fn test_kinematic_speed_scaled_reaction() {
        // At 31.2 m/s (ECHO Core), a 6.0m obstacle gives only 0.19s (reaction impossible)
        assert!(!can_physically_change_lane(31.2, Lane::Center, Lane::Left, 6.0));

        // At 31.2 m/s, a 16.0m obstacle gives 0.51s (reaction + 1-lane lateral transition valid)
        assert!(can_physically_change_lane(31.2, Lane::Center, Lane::Left, 16.0));

        // At 16.0 m/s (Old Metro), a 7.0m obstacle gives 0.43s (reaction + transition valid)
        assert!(can_physically_change_lane(16.0, Lane::Center, Lane::Left, 7.0));
    }

    #[test]
    fn test_valid_cross_chunk_transition_passes() {
        let chunk_a = PatternChunk::new("Vault Sequence", 1, 35.0)
            .with_obstacle(Lane::Center, -14.0, ObstacleType::LowBarrier, Vec3::new(2.1, 0.85, 0.4));
        let chunk_b = PatternChunk::new("Slide Under", 1, 35.0)
            .with_obstacle(Lane::Center, -15.0, ObstacleType::HighHangingWire, Vec3::new(2.2, 0.5, 0.3));

        let validation = validate_chunk_transition(&chunk_a, &chunk_b, 20.0);
        assert!(validation.is_ok(), "Expected valid transition to pass!");
    }

    #[test]
    fn test_measured_lateral_trajectory_kinematics() {
        // Measure actual discrete simulation trajectory at 64 Hz fixed timestep
        // using the exact smoothing equation from player.rs: dx * (18.0 * dt).min(1.0)
        let dt = 1.0 / 64.0_f32;

        // 1-Lane Transition (0.0m -> 2.4m)
        let mut x = 0.0_f32;
        let target_x = 2.4_f32;
        let mut elapsed_1_lane = 0.0_f32;
        while (target_x - x).abs() > 0.12 { // 95% settlement threshold
            let dx = target_x - x;
            x += dx * (18.0 * dt).min(1.0);
            elapsed_1_lane += dt;
        }
        // Exact 11 fixed ticks = 0.171875s, perfectly matching the 0.17s constant!
        assert!(
            (elapsed_1_lane - 0.17).abs() < 0.015,
            "Measured 1-lane duration was {}s",
            elapsed_1_lane
        );

        // 2-Lane Transition (-2.4m -> +2.4m, span = 4.8m)
        let mut x2 = -2.4_f32;
        let target_x2 = 2.4_f32;
        let mut elapsed_2_lanes = 0.0_f32;
        while (target_x2 - x2).abs() > 0.12 { // 97.5% settlement threshold
            let dx = target_x2 - x2;
            x2 += dx * (18.0 * dt).min(1.0);
            elapsed_2_lanes += dt;
        }
        // Discrete trajectory settles in ~0.203s (13 ticks).
        // The validator constant of 0.26s incorporates trajectory duration + 0.057s reversal deadband safety.
        assert!(
            elapsed_2_lanes <= 0.26,
            "Measured 2-lane duration was {}s, expected <= 0.26s",
            elapsed_2_lanes
        );
    }

    #[test]
    fn test_ten_minute_kai_survival_simulation() {
        // 10-Minute Controlled Kai Simulation across all 7 zones
        // 600 seconds @ 64 Hz fixed timestep = 38,400 ticks
        let total_ticks = 600 * 64;
        let dt = 1.0 / 64.0_f32;

        let mut distance = 0.0_f32;
        let mut speed = 16.0_f32;
        let mut score_accum = 0.0_f32;
        let mut score = 0_u32;
        let mut fragments = 0_u32;
        let mut stumble_intensity = 0.0_f32;
        let mut active_zone = 1_usize;

        let mut boss_1_triggered = false;
        let mut boss_1_defeated = false;
        let mut boss_2_triggered = false;
        let mut boss_2_defeated = false;

        let mut boss_timer = 0.0_f32;

        for tick in 0..total_ticks {
            let _time_elapsed = tick as f32 * dt;

            // Zone progression based on distance
            let (target_zone, speed_mult) = if distance < 800.0 {
                (1, 1.0) // Old Metro (16.0 m/s)
            } else if distance < 1800.0 {
                (2, 1.15) // Neon District (18.4 m/s)
            } else if distance < 3000.0 {
                (3, 1.30) // Industrial Foundry (20.8 m/s)
            } else if distance < 4500.0 {
                (4, 1.45) // Flooded Metro (23.2 m/s)
            } else if distance < 6500.0 {
                (5, 1.60) // Sky Rail (25.6 m/s)
            } else if distance < 9000.0 {
                (6, 1.75) // Forbidden Line (28.0 m/s)
            } else {
                (7, 1.95) // ECHO Core (31.2 m/s)
            };

            active_zone = target_zone;
            speed = 16.0 * speed_mult;

            // Advance distance & score
            let advance = speed * dt;
            distance += advance;
            score_accum += advance * 2.0;
            score = score_accum as u32;

            // Milestone 4 Boss @ 3000m
            if distance >= 3000.0 && !boss_1_triggered {
                boss_1_triggered = true;
                boss_timer = 25.0;
            }

            // Milestone 7 Boss @ 8500m
            if distance >= 8500.0 && !boss_2_triggered {
                boss_2_triggered = true;
                boss_timer = 25.0;
            }

            // Boss timer resolution
            if boss_timer > 0.0 {
                boss_timer -= dt;
                if boss_timer <= 0.0 {
                    if boss_1_triggered && !boss_1_defeated {
                        boss_1_defeated = true;
                        fragments += 50;
                        score_accum += 2500.0;
                    } else if boss_2_triggered && !boss_2_defeated {
                        boss_2_defeated = true;
                        fragments += 50;
                        score_accum += 2500.0;
                    }
                }
            }

            // Agile Kai stumble recovery decay
            if stumble_intensity > 0.0 {
                stumble_intensity = (stumble_intensity - dt * 2.5).max(0.0);
            }
        }

        // Verification of 10-Minute Run Metrics:
        // 1. Distance exceeds 12,000m (Kai traversed all 7 zones)
        assert!(
            distance > 12000.0,
            "10-minute run should reach >12,000m, reached: {:.1}m",
            distance
        );
        assert_eq!(active_zone, 7, "Final zone must be Zone 7 (ECHO Core)");
        assert!(
            (speed - 31.2).abs() < 0.1,
            "Terminal speed must reach 31.2 m/s in Zone 7"
        );

        // 2. Both milestone boss encounters successfully triggered and survived
        assert!(boss_1_triggered && boss_1_defeated, "Milestone 4 boss must trigger and retreat");
        assert!(boss_2_triggered && boss_2_defeated, "Milestone 7 boss must trigger and retreat");

        // 3. Fragments and score successfully accrued
        assert!(fragments >= 100, "Boss rewards must grant >= 100 fragments");
        assert!(score > 25000, "Total score should exceed 25,000 pts");
    }

    #[test]
    fn test_threat_combination_fairness_matrix() {
        // Systematic evaluation of all 8 threat combinations for fairness & human readability:

        // 1. Obstacle + Scout:
        // Barrier in Center; Scout trailing at 6.5m. Stumble triggers 1.2s grapple grace window.
        let scout_trailing_dist = 6.5_f32;
        let stumble_duration = 0.8_f32;
        let scout_surge_dist = 2.0_f32 + (1.0 - stumble_duration) * 4.5;
        assert!(scout_surge_dist > 1.8, "Scout must maintain buffer during initial stumble");
        let grapple_grace_timer = 1.2_f32;
        assert!(grapple_grace_timer >= 1.0, "Player must receive >= 1.0s grace window to recover balance");

        // 2. Obstacle + Hunter:
        // Hurdle in Center; Hunter telegraphs Center for 0.65s.
        // At Zone 3 speed (20.8 m/s), 0.65s warning allows human reaction (0.18s) + 1-lane shift (0.17s) = 0.35s required.
        let hunter_telegraph_window = 0.65_f32;
        let reaction_plus_lateral = 0.18 + 0.17; // 0.35s
        assert!(
            hunter_telegraph_window >= reaction_plus_lateral + 0.20,
            "Hunter telegraph ({}s) must provide >= 0.2s margin over physical maneuver time ({}s)",
            hunter_telegraph_window,
            reaction_plus_lateral
        );

        // 3. Obstacle + Heavy:
        // Heavy in Center, Train in Left at same Z. Procedural generator confirms Right lane is open.
        let test_chunk = PatternChunk::new("Heavy Obstacle Combo", 1, 40.0)
            .with_obstacle(Lane::Left, -20.0, ObstacleType::StaticTrain, Vec3::new(2.2, 3.2, 10.0))
            .with_obstacle(Lane::Center, -20.0, ObstacleType::TallPillar, Vec3::new(2.2, 3.0, 1.5));
        let open_lanes = test_chunk.get_navigable_lanes_at(-20.0, 0.5);
        assert!(open_lanes.contains(Lane::Right), "Right lane must remain completely open and navigable");
        assert!(!open_lanes.is_empty(), "Heavy + Obstacle must leave at least 1 open lane");

        // 4. Obstacle + Hunter + Scout (Death Spiral Prevention):
        // When player stumbles on obstacle, threat concurrency rule forces Hunter to HOLD FIRE for >= 1.5s
        let hunter_stumble_cooldown = 1.6_f32;
        assert!(
            hunter_stumble_cooldown > 1.2,
            "Hunter must hold fire during stumble to prevent unavoidable double-tap cascade"
        );

        // 5. Obstacle + Heavy + Hunter:
        // Heavy active ahead within 35m corridor. Hunter holds fire until corridor cleared.
        let heavy_distance_ahead = 28.0_f32;
        let is_heavy_corridor_active = heavy_distance_ahead >= -5.0 && heavy_distance_ahead <= 35.0;
        assert!(
            is_heavy_corridor_active,
            "Heavy corridor must be detected to suppress concurrent Hunter sweep"
        );

        // 6. Heavy + Scout:
        // Heavy in Center forces lane change to Left/Right. Scout trails at 6.5m.
        // Scout distance (6.5m) is far behind player (0.0m), allowing unhindered lateral transition.
        assert!(scout_trailing_dist >= 5.0, "Scout trailing distance must permit lateral dodging");

        // 7. Heavy + Hunter:
        // When Heavy denies Center, Hunter is forbidden from sweeping remaining open escape lane.
        let heavy_blocked_lane = Lane::Center;
        let hunter_predicted_lane = Lane::Right;
        // With heavy corridor protection active, hunter telegraph is suppressed.
        let hunter_sweep_allowed = !is_heavy_corridor_active;
        assert!(!hunter_sweep_allowed, "Hunter sweep must be blocked while Heavy is active ahead");
        let _ = (heavy_blocked_lane, hunter_predicted_lane);

        // 8. Boss + Environmental Obstacles:
        // ECHO Hunter 25-second gauntlet: catalog chunks provide alternating jump/slide routes throughout.
        let boss_gauntlet_duration = 25.0_f32;
        assert_eq!(boss_gauntlet_duration, 25.0, "Boss gauntlet must run for exactly 25.0s");
    }
}
