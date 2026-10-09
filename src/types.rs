use crate::zones::ZoneConfig;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
    StoryLog,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Left = -1,
    Center = 0,
    Right = 1,
}

impl Lane {
    pub const SPACING: f32 = 2.4;

    #[inline(always)]
    pub fn x_pos(&self) -> f32 {
        match self {
            Lane::Left => -Self::SPACING,
            Lane::Center => 0.0,
            Lane::Right => Self::SPACING,
        }
    }

    #[inline(always)]
    pub fn move_left(&self) -> Self {
        match self {
            Lane::Left => Lane::Left,
            Lane::Center => Lane::Left,
            Lane::Right => Lane::Center,
        }
    }

    #[inline(always)]
    pub fn move_right(&self) -> Self {
        match self {
            Lane::Left => Lane::Center,
            Lane::Center => Lane::Right,
            Lane::Right => Lane::Right,
        }
    }

    #[inline(always)]
    pub fn from_index(idx: i32) -> Self {
        match idx {
            -1 => Lane::Left,
            1 => Lane::Right,
            _ => Lane::Center,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CharacterType {
    #[default]
    Kai, // Courier: Quick recovery & agile
    Mira, // Engineer: Energy Dash ability
    Jax,  // Street Racer: Higher base speed & fragment gain
    Nyx,  // Hacker: Drone Magnet boost
    Arin, // Ex-Security: Shield durability +
}

impl CharacterType {
    pub fn name(&self) -> &'static str {
        match self {
            CharacterType::Kai => "Kai Renn",
            CharacterType::Mira => "Mira Vasquez",
            CharacterType::Jax => "Jax Chen",
            CharacterType::Nyx => "Nyx Holloway",
            CharacterType::Arin => "Arin Cole",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            CharacterType::Kai => "The Courier",
            CharacterType::Mira => "Ex-Veyron Engineer",
            CharacterType::Jax => "Underground Racer",
            CharacterType::Nyx => "Data Infiltrator",
            CharacterType::Arin => "Rogue Operative",
        }
    }

    pub fn ability_desc(&self) -> &'static str {
        match self {
            CharacterType::Kai => "Balanced agility & swift lane transitions",
            CharacterType::Mira => "Energy Dash: Burst forward through obstacles",
            CharacterType::Jax => "+25% Speed & double ECHO multiplier",
            CharacterType::Nyx => "Enhanced ECHO magnet radius & duration",
            CharacterType::Arin => "Reinforced ECHO shields absorb 2 impacts",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum QualityTier {
    Low,
    Medium,
    #[default]
    High,
}

impl QualityTier {
    pub fn next(&self) -> Self {
        match self {
            QualityTier::Low => QualityTier::Medium,
            QualityTier::Medium => QualityTier::High,
            QualityTier::High => QualityTier::Low,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            QualityTier::Low => "LOW (Max FPS)",
            QualityTier::Medium => "MEDIUM (Balanced)",
            QualityTier::High => "HIGH (Full Bloom & Lights)",
        }
    }
}

#[derive(Resource)]
pub struct QualitySettings {
    pub tier: QualityTier,
    pub show_perf_overlay: bool,
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self {
            tier: QualityTier::High,
            show_perf_overlay: true,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub lane: Lane,
    pub target_x: f32,
    pub y_velocity: f32,
    pub is_grounded: bool,
    pub is_sliding: bool,
    pub slide_timer: f32,
    pub character: CharacterType,
    pub has_double_jumped: bool,
    pub invulnerable_timer: f32,
    pub dash_cooldown: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            lane: Lane::Center,
            target_x: 0.0,
            y_velocity: 0.0,
            is_grounded: true,
            is_sliding: false,
            slide_timer: 0.0,
            character: CharacterType::Kai,
            has_double_jumped: false,
            invulnerable_timer: 0.0,
            dash_cooldown: 0.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct ActivePowerUps {
    pub shield: bool,
    pub shield_hits: u32,
    pub shield_absorb_flash_timer: f32,
    pub overdrive_timer: f32,
    pub magnet_timer: f32,
    pub time_break_timer: f32,
    pub double_jump_timer: f32,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct BossBattleState {
    pub is_active: bool,
    pub boss_name: &'static str,
    pub time_remaining: f32,
    pub boss_health: f32,
    pub victory_banner_timer: f32,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerMovementHistory {
    pub recent_switches: Vec<(f32, Lane)>, // (timestamp, target_lane)
    pub last_switch_time: f32,
    pub switch_count_last_second: u32,
    pub current_lateral_velocity: f32,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct ThreatAlertState {
    pub scout_grappling: bool,
    pub hunter_telegraph_lane: Option<Lane>,
    pub heavy_warning_lane: Option<Lane>,
}

#[derive(Resource)]
pub struct GameRunStats {
    pub distance: f32,
    pub speed: f32,
    pub base_speed: f32,
    pub fragments: u32,
    pub data_chips: u32,
    pub score: u32,
    pub score_accum: f32,
    pub multiplier: f32,
    pub high_score: u32,
    pub best_distance: f32,
    pub current_zone: usize,
    pub stumble_intensity: f32,
    pub selected_character: CharacterType,
    pub story_dialogue: Option<StoryMessage>,
}

impl Default for GameRunStats {
    fn default() -> Self {
        Self {
            distance: 0.0,
            speed: 16.0,
            base_speed: 16.0,
            fragments: 0,
            data_chips: 0,
            score: 0,
            score_accum: 0.0,
            multiplier: 1.0,
            high_score: 0,
            best_distance: 0.0,
            current_zone: 1,
            stumble_intensity: 0.0,
            selected_character: CharacterType::Kai,
            story_dialogue: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StoryMessage {
    pub title: String,
    pub sender: String,
    pub body: String,
    pub timer: f32,
}

// -------------------------------------------------------------
// FOCUSED ECS COMPONENTS & MARKERS
// -------------------------------------------------------------
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Despawnable {
    pub z_center: f32,
}

#[derive(Component)]
pub struct TrackSegmentMarker;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObstacleType {
    LowBarrier,      // Vault over (Jump)
    HighHangingWire, // Duck under (Slide)
    TallPillar,      // Lane switch block
    StaticTrain,     // Large obstacle, climbable roof
    MovingTrain { speed: f32 },
}

#[derive(Component)]
pub struct ActiveObstacle {
    pub lane: Lane,
    pub obstacle_type: ObstacleType,
    pub size: Vec3,
}

#[derive(Component)]
pub struct MovingObstacle {
    pub speed: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum CollectibleType {
    EchoFragment { value: u32 },
    DataChip { index: usize },
    EchoShield,
    Overdrive,
    Magnet,
    TimeBreak,
    DoubleJump,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct CollectibleItem {
    pub item_type: CollectibleType,
    pub lane: Lane,
    pub initial_y: f32,
    pub rot_speed: f32,
}

#[derive(Resource, Clone, Default)]
pub struct PowerUpModelAssets {
    pub shield_scene: Handle<Scene>,
    pub magnet_scene: Handle<Scene>,
    pub overdrive_scene: Handle<Scene>,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct ChaserDrone {
    pub distance_behind: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PoolType {
    LowBarrier,
    HighHangingWire,
    TallPillar,
    StaticTrain,
    MovingTrain,
    EchoFragment,
    DataChip,
    PowerUp,
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct PooledItem {
    pub pool_type: PoolType,
    pub is_active: bool,
}

// -------------------------------------------------------------
// EVENTS
// -------------------------------------------------------------
#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct ZoneChangedEvent {
    pub from_zone: usize,
    pub to_zone: usize,
    pub config: ZoneConfig,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct MilestoneReachedEvent {
    pub milestone_idx: usize,
    pub distance: f32,
    pub title: &'static str,
    pub sender: &'static str,
    pub message: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Scout,      // Level 1: Predictable pursuit follower
    Hunter,     // Level 2: Predictive interceptor (reads lateral player velocity)
    Heavy,      // Level 3: Tactical route blocker (suppression zone ahead)
    EchoHunter, // Elite: Environmental anomaly manipulator
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct ActiveEnemy {
    pub enemy_type: EnemyType,
    pub target_lane: Lane,
    pub current_lane: Lane,
    pub lane_switch_timer: f32,
    pub behavior_timer: f32,
    pub distance_from_player: f32, // Positive = trailing behind, Negative = ahead on track
    pub is_attacking: bool,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct BossStartedEvent {
    pub boss_name: &'static str,
    pub health: f32,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct BossDefeatedEvent {
    pub boss_name: &'static str,
    pub reward_fragments: u32,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct EnemySpawnedEvent {
    pub enemy_type: EnemyType,
    pub lane: Lane,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct PowerUpCollectedEvent {
    pub powerup_type: CollectibleType,
    pub duration: f32,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct PlayerStumbledEvent {
    pub severity: f32,
    pub reason: &'static str,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct TrainSpawnedEvent {
    pub lane: Lane,
    pub is_moving: bool,
    pub speed: f32,
}

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct StoryUnlockedEvent {
    pub log_index: usize,
    pub title: &'static str,
}

#[derive(Event, Debug, Clone, Default)]
pub struct RunResetEvent;

#[derive(Event, Debug, Clone)]
#[allow(dead_code)]
pub struct RunEndedEvent {
    pub distance: f32,
    pub score: u32,
    pub fragments: u32,
    pub zone: usize,
}

#[derive(Event, Debug)]
pub enum SoundEffect {
    Jump,
    Slide,
    LaneSwitch,
    FragmentPickup,
    DataChipPickup,
    PowerupPickup,
    ShieldBreak,
    Stumble,
    Crash,
    Dash,
    ZoneTransition,
}
