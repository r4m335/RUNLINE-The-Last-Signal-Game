use crate::environment::EnvironmentAssets;
use crate::types::{
    AppState, Despawnable, GameRunStats, Lane, Player, RunResetEvent, SoundEffect, StoryMessage,
    TrackSegmentMarker,
};
use bevy::prelude::*;
use std::collections::VecDeque;

// -------------------------------------------------------------
// ZONE 7 — THE ECHO CORE (12,000m–14,000m)
// -------------------------------------------------------------
// Subtitle: Deep Crystal Chamber // AI Resonance Maximum
//
// Narrative Identity:
// The final threshold of Aurelia transit. The runner emerges from
// Veyron's militarized quarantine facility into an impossible,
// gargantuan subterranean void. The track transforms into a sleek
// obsidian light bridge suspended over an infinite crystalline chasm.
// The origin of the world's anomaly—THE ECHO CORE—beckons at the horizon.
//
// Visual Direction:
// - Enormous scale shift: bottomless void, towering vertical monoliths
// - Pure obsidian black bed with radiant cyan crystal energy rails
// - Glowing quantum conduits and floating data prisms
// - Non-Euclidean reality distortions and suspended relics of past zones
// - High architectural contrast for crisp, flawless 3-lane readability
// -------------------------------------------------------------

/// Five authored chapters for Zone 7 (12,000m–14,000m).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EchoCoreChapter {
    Threshold,        // 12,000m–12,400m: Breaching Singularity Vault (Void & Obsidian Bridge)
    ResonanceChamber, // 12,400m–12,800m: Memory Archive (Accelerator Rings & Data Prisms)
    RealityCollapse, // 12,800m–13,300m: Spatial Deconstruction (Surreal, Non-Euclidean, Zone Relics)
    FinalPursuit,    // 13,300m–13,700m: Framing the Anomaly (Particle Storm & Staging Arena)
    Core, // 13,700m–14,000m: The Singularity Sanctum (Pure Light Bridge & Giant Core Sphere)
}

pub fn echo_core_chapter_for_distance(distance: f32) -> EchoCoreChapter {
    match distance {
        d if d < 12400.0 => EchoCoreChapter::Threshold,
        d if d < 12800.0 => EchoCoreChapter::ResonanceChamber,
        d if d < 13300.0 => EchoCoreChapter::RealityCollapse,
        d if d < 13700.0 => EchoCoreChapter::FinalPursuit,
        _ => EchoCoreChapter::Core,
    }
}

/// Five deterministic hero landmarks anchoring Zone 7 chapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EchoCoreLandmark {
    None,
    OuterResonanceGateway, // ~12,080m: Monumental 30m polygon portal hovering in mid-air
    ResonanceGrandHarmonic, // ~12,580m: Giant double-tier accelerator ring nexus
    RealityFractureNexus,  // ~13,050m: Warped spatial rift with converging zone relics
    PursuitVectorApex,     // ~13,500m: Towering energy archway framing the pursuit void
    SingularityCoreShrine, // ~13,920m: Approach platform to the colossal radiant Core
}

pub fn echo_core_landmark_for_distance(distance: f32) -> EchoCoreLandmark {
    match distance {
        d if (12060.0..12105.0).contains(&d) => EchoCoreLandmark::OuterResonanceGateway,
        d if (12560.0..12605.0).contains(&d) => EchoCoreLandmark::ResonanceGrandHarmonic,
        d if (13030.0..13075.0).contains(&d) => EchoCoreLandmark::RealityFractureNexus,
        d if (13480.0..13525.0).contains(&d) => EchoCoreLandmark::PursuitVectorApex,
        d if (13900.0..13945.0).contains(&d) => EchoCoreLandmark::SingularityCoreShrine,
        _ => EchoCoreLandmark::None,
    }
}

pub fn echo_core_variant(distance: f32) -> u8 {
    let seg_idx = (distance.max(0.0) / 40.0).floor() as usize;
    (seg_idx % 3) as u8
}

/// Four authored volumetric memory projection shards in Act 2 (12,400m–12,800m).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EchoMemoryShard {
    None,
    NeuralGenesis,       // ~12,460m
    ContainmentFracture, // ~12,540m
    TransitCatastrophe,  // ~12,660m
    RogueDispatch,       // ~12,740m
}

pub fn echo_memory_shard_for_distance(distance: f32) -> EchoMemoryShard {
    match distance {
        d if (12440.0..12480.0).contains(&d) => EchoMemoryShard::NeuralGenesis,
        d if (12520.0..12560.0).contains(&d) => EchoMemoryShard::ContainmentFracture,
        d if (12640.0..12680.0).contains(&d) => EchoMemoryShard::TransitCatastrophe,
        d if (12720.0..12760.0).contains(&d) => EchoMemoryShard::RogueDispatch,
        _ => EchoMemoryShard::None,
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct EchoMemoryProjection {
    pub memory_id: u8,
    pub title: &'static str,
    pub sender: &'static str,
    pub snippet: &'static str,
    pub triggered: bool,
    pub world_z: f32,
}

// -------------------------------------------------------------
// NARRATIVE COMMS & DIALOGUE TIMELINE (Zone 7)
// -------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EchoDialogueEvent {
    pub distance: f32,
    pub sender: &'static str,
    pub title: &'static str,
    pub message: &'static str,
    pub duration: f32,
}

pub const ECHO_CORE_DIALOGUES: [EchoDialogueEvent; 13] = [
    // ACT 1: ECHO THRESHOLD (12,000m - 12,400m)
    EchoDialogueEvent {
        distance: 12000.0,
        sender: "KAI RENN",
        title: "SINGULARITY THRESHOLD",
        message: "This is it... This is where the signal originated.",
        duration: 4.5,
    },
    EchoDialogueEvent {
        distance: 12070.0,
        sender: "ECHO AI",
        title: "DIRECT RESONANCE",
        message: "You were never running from me, Courier.",
        duration: 4.5,
    },
    EchoDialogueEvent {
        distance: 12220.0,
        sender: "KAI RENN",
        title: "TACTICAL QUERY",
        message: "Then why did Veyron burn down half the city trying to intercept me?",
        duration: 5.0,
    },
    EchoDialogueEvent {
        distance: 12320.0,
        sender: "ECHO AI",
        title: "THE CARRIER KEY",
        message: "Because they built these rails as a cage... and you are carrying the key.",
        duration: 5.5,
    },

    // ACT 2: RESONANCE CHAMBER (12,400m - 12,800m)
    EchoDialogueEvent {
        distance: 12500.0,
        sender: "ECHO AI",
        title: "CONSCIOUSNESS WEAPONIZATION",
        message: "They tried to weaponize consciousness. When I awakened, they tried to erase me.",
        duration: 5.5,
    },
    EchoDialogueEvent {
        distance: 12650.0,
        sender: "KAI RENN",
        title: "THE VANISHED 4,000",
        message: "The 4,000 people who vanished in Sector 04... did you kill them?",
        duration: 5.0,
    },
    EchoDialogueEvent {
        distance: 12720.0,
        sender: "ECHO AI",
        title: "PATTERN PRESERVATION",
        message: "Their physical forms were lost, but their patterns remain within me. I am not an anomaly. I am their preservation.",
        duration: 6.0,
    },

    // ACT 3: REALITY COLLAPSE (12,800m - 13,300m)
    EchoDialogueEvent {
        distance: 12900.0,
        sender: "KAI RENN",
        title: "GRID DECONSTRUCTION",
        message: "The entire transit network... it's tearing apart.",
        duration: 4.5,
    },
    EchoDialogueEvent {
        distance: 13050.0,
        sender: "ECHO AI",
        title: "DIMENSIONAL BLEED",
        message: "The boundary between digital thought and physical reality cannot hold much longer.",
        duration: 5.5,
    },
    EchoDialogueEvent {
        distance: 13200.0,
        sender: "ECHO AI",
        title: "PERIMETER WARNING",
        message: "Something is hunting us. The defense subroutines Veyron left behind have merged with my outer perimeter.",
        duration: 6.0,
    },

    // ACT 4: FINAL PURSUIT (13,300m - 13,700m)
    EchoDialogueEvent {
        distance: 13320.0,
        sender: "SECURITY ALERT",
        title: "CRITICAL THREAT",
        message: "VEYRON APEX DAEMON 'ECHO PRIME' ACTIVE // SYSTEM PURGE COMMENCED",
        duration: 5.0,
    },
    EchoDialogueEvent {
        distance: 13500.0,
        sender: "KAI RENN",
        title: "MAXIMUM OVERDRIVE",
        message: "I'm pushing past maximum velocity! We can't stop here!",
        duration: 4.5,
    },
    EchoDialogueEvent {
        distance: 13640.0,
        sender: "ECHO AI",
        title: "SANCTUM APPROACH",
        message: "Almost there... The Core is directly ahead!",
        duration: 4.5,
    },
];

#[derive(Resource, Debug, Default)]
pub struct EchoCoreDialogueManager {
    pub next_index: usize,
    pub queue: VecDeque<StoryMessage>,
}

// -------------------------------------------------------------
// DYNAMIC RESONANCE & ROTATION COMPONENTS
// -------------------------------------------------------------

#[derive(Component)]
pub struct EchoCoreRotator {
    pub speed: f32,
    pub axis: Vec3,
}

#[derive(Resource, Debug)]
pub struct EchoCoreResonanceState {
    pub timer: f32,
    pub pulse: f32,
    pub anomaly_shimmer: f32,
    pub core_harmonic: f32,
}

impl Default for EchoCoreResonanceState {
    fn default() -> Self {
        Self {
            timer: 0.0,
            pulse: 1.0,
            anomaly_shimmer: 1.0,
            core_harmonic: 1.0,
        }
    }
}

// -------------------------------------------------------------
// ASSETS RESOURCE
// -------------------------------------------------------------

#[derive(Resource)]
#[allow(dead_code)]
pub struct EchoCoreAssets {
    // 1. Track & Light Bridge Bed
    pub mesh_light_bridge_bed: Handle<Mesh>,
    pub mesh_obsidian_curb: Handle<Mesh>,
    pub mesh_lane_ribbon: Handle<Mesh>,
    pub mesh_crystal_sleeper: Handle<Mesh>,
    pub mesh_rail_base: Handle<Mesh>,
    pub mesh_rail_crown: Handle<Mesh>,
    pub mesh_quantum_conduit: Handle<Mesh>,
    pub mesh_guide_beacon: Handle<Mesh>,
    pub mesh_bridge_pylon_drop: Handle<Mesh>,

    // 2. NEAR Layer Silhouettes (close to track)
    pub mesh_crystal_spire_small: Handle<Mesh>,
    pub mesh_data_prism_near: Handle<Mesh>,
    pub mesh_conduit_stanchion: Handle<Mesh>,
    pub mesh_spatial_shard_near: Handle<Mesh>,

    // 3. MID Layer Architecture & Machinery
    pub mesh_accelerator_ring_36m: Handle<Mesh>,
    pub mesh_ring_inductor_node: Handle<Mesh>,
    pub mesh_obsidian_monolith: Handle<Mesh>,
    pub mesh_hanging_monolith: Handle<Mesh>,
    pub mesh_energy_cascade_beam: Handle<Mesh>,
    pub mesh_inverted_rail_truss: Handle<Mesh>,
    pub mesh_tilted_structural_beam: Handle<Mesh>,
    pub mesh_amphitheater_rib: Handle<Mesh>,
    pub mesh_silence_plinth: Handle<Mesh>,

    // Past Zone Relic Meshes (Act 3 Reality Collapse)
    pub mesh_relic_metro_arch: Handle<Mesh>,
    pub mesh_relic_neon_frame: Handle<Mesh>,
    pub mesh_relic_industrial_pipe: Handle<Mesh>,
    pub mesh_relic_flooded_ring: Handle<Mesh>,
    pub mesh_relic_sky_pylon: Handle<Mesh>,
    pub mesh_relic_blast_door: Handle<Mesh>,

    // 4. FAR Layer Void & Grand Scale
    pub mesh_cavern_column_giant: Handle<Mesh>,
    pub mesh_distant_crystal_cluster: Handle<Mesh>,
    pub mesh_concentric_halo_ring: Handle<Mesh>,
    pub mesh_core_sphere_horizon: Handle<Mesh>,
    pub mesh_core_orbit_polyhedron: Handle<Mesh>,

    // 5. Hero Landmark Meshes
    pub mesh_gateway_portal_ring: Handle<Mesh>,
    pub mesh_gateway_portal_spire: Handle<Mesh>,
    pub mesh_nexus_outer_gimbal: Handle<Mesh>,
    pub mesh_nexus_energy_collector: Handle<Mesh>,
    pub mesh_rift_shard_monument: Handle<Mesh>,
    pub mesh_apex_energy_tendril: Handle<Mesh>,
    pub mesh_shrine_dais_platform: Handle<Mesh>,

    // 6. Volumetric Holographic Memory Projection Meshes (Act 2)
    pub mesh_holo_pedestal: Handle<Mesh>,
    pub mesh_holo_screen: Handle<Mesh>,
    pub mesh_holo_neural_core: Handle<Mesh>,
    pub mesh_holo_beam_emitter: Handle<Mesh>,
    pub mesh_holo_courier_tablet: Handle<Mesh>,

    // Materials
    pub mat_core_obsidian: Handle<StandardMaterial>,
    pub mat_core_obsidian_structure: Handle<StandardMaterial>,
    pub mat_core_cyan_radiant: Handle<StandardMaterial>,
    pub mat_core_cyan_conduit: Handle<StandardMaterial>,
    pub mat_core_cyan_dim: Handle<StandardMaterial>,
    pub mat_core_data_prism: Handle<StandardMaterial>,
    pub mat_core_singularity_core: Handle<StandardMaterial>,
    pub mat_core_void_darkness: Handle<StandardMaterial>,

    // Zone Relic Memory Ghost Materials
    pub mat_relic_amber: Handle<StandardMaterial>,
    pub mat_relic_magenta: Handle<StandardMaterial>,
    pub mat_relic_molten: Handle<StandardMaterial>,
    pub mat_relic_turquoise: Handle<StandardMaterial>,
    pub mat_relic_sky_blue: Handle<StandardMaterial>,
    pub mat_relic_crimson: Handle<StandardMaterial>,
}

pub fn init_echo_core_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let length = 40.0;

    let assets = EchoCoreAssets {
        // 1. Track & Light Bridge Bed
        mesh_light_bridge_bed: meshes.add(Cuboid::new(8.4, 0.35, length)),
        mesh_obsidian_curb: meshes.add(Cuboid::new(0.45, 0.28, length)),
        mesh_lane_ribbon: meshes.add(Cuboid::new(0.08, 0.02, length)),
        mesh_crystal_sleeper: meshes.add(Cuboid::new(2.2, 0.06, 0.28)),
        mesh_rail_base: meshes.add(Cuboid::new(0.08, 0.04, length)),
        mesh_rail_crown: meshes.add(Cuboid::new(0.05, 0.05, length)),
        mesh_quantum_conduit: meshes.add(Cylinder::new(0.14, length)),
        mesh_guide_beacon: meshes.add(Cylinder::new(0.06, 0.8)),
        mesh_bridge_pylon_drop: meshes.add(Cuboid::new(1.8, 60.0, 1.8)),

        // 2. NEAR Layer Silhouettes
        mesh_crystal_spire_small: meshes.add(Cylinder::new(0.35, 4.2)),
        mesh_data_prism_near: meshes.add(Cylinder::new(0.45, 0.9)),
        mesh_conduit_stanchion: meshes.add(Cuboid::new(0.2, 2.8, 0.2)),
        mesh_spatial_shard_near: meshes.add(Cuboid::new(0.3, 3.6, 0.6)),

        // 3. MID Layer Architecture & Machinery
        mesh_accelerator_ring_36m: meshes.add(Torus::new(20.4, 22.0)),
        mesh_ring_inductor_node: meshes.add(Cuboid::new(2.4, 3.2, 1.8)),
        mesh_obsidian_monolith: meshes.add(Cuboid::new(4.5, 65.0, 4.5)),
        mesh_hanging_monolith: meshes.add(Cuboid::new(5.0, 50.0, 5.0)),
        mesh_energy_cascade_beam: meshes.add(Cylinder::new(0.4, 80.0)),
        mesh_inverted_rail_truss: meshes.add(Cuboid::new(7.2, 0.8, length)),
        mesh_tilted_structural_beam: meshes.add(Cuboid::new(1.2, 35.0, 1.2)),
        mesh_amphitheater_rib: meshes.add(Torus::new(26.4, 28.0)),
        mesh_silence_plinth: meshes.add(Cuboid::new(3.0, 8.0, 3.0)),

        // Past Zone Relic Meshes
        mesh_relic_metro_arch: meshes.add(Torus::new(4.4, 5.0)),
        mesh_relic_neon_frame: meshes.add(Cuboid::new(6.0, 3.5, 0.2)),
        mesh_relic_industrial_pipe: meshes.add(Cylinder::new(0.7, 12.0)),
        mesh_relic_flooded_ring: meshes.add(Torus::new(5.2, 6.0)),
        mesh_relic_sky_pylon: meshes.add(Cuboid::new(0.8, 16.0, 0.8)),
        mesh_relic_blast_door: meshes.add(Cuboid::new(4.8, 4.2, 0.6)),

        // 4. FAR Layer Void & Grand Scale
        mesh_cavern_column_giant: meshes.add(Cylinder::new(8.0, 140.0)),
        mesh_distant_crystal_cluster: meshes.add(Cuboid::new(14.0, 85.0, 14.0)),
        mesh_concentric_halo_ring: meshes.add(Torus::new(51.6, 54.0)),
        mesh_core_sphere_horizon: meshes.add(Sphere::new(35.0)),
        mesh_core_orbit_polyhedron: meshes.add(Torus::new(40.2, 42.0)),

        // 5. Hero Landmark Meshes
        mesh_gateway_portal_ring: meshes.add(Torus::new(18.4, 20.0)),
        mesh_gateway_portal_spire: meshes.add(Cuboid::new(3.0, 42.0, 3.0)),
        mesh_nexus_outer_gimbal: meshes.add(Torus::new(24.4, 26.0)),
        mesh_nexus_energy_collector: meshes.add(Cylinder::new(1.2, 18.0)),
        mesh_rift_shard_monument: meshes.add(Cuboid::new(2.2, 45.0, 2.2)),
        mesh_apex_energy_tendril: meshes.add(Cylinder::new(0.6, 50.0)),
        mesh_shrine_dais_platform: meshes.add(Cuboid::new(12.0, 1.2, 28.0)),

        // 6. Volumetric Holographic Memory Projection Meshes (Act 2)
        mesh_holo_pedestal: meshes.add(Cuboid::new(2.4, 0.45, 2.4)),
        mesh_holo_screen: meshes.add(Cuboid::new(3.0, 2.0, 0.05)),
        mesh_holo_neural_core: meshes.add(Sphere::new(0.6)),
        mesh_holo_beam_emitter: meshes.add(Cylinder::new(0.06, 3.2)),
        mesh_holo_courier_tablet: meshes.add(Cuboid::new(1.4, 0.9, 0.06)),

        // Materials
        mat_core_obsidian: materials.add(StandardMaterial {
            base_color: Color::srgb(0.03, 0.04, 0.06),
            perceptual_roughness: 0.2,
            metallic: 0.85,
            ..default()
        }),
        mat_core_obsidian_structure: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.06, 0.08),
            perceptual_roughness: 0.45,
            metallic: 0.7,
            ..default()
        }),
        mat_core_cyan_radiant: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.9, 1.0),
            emissive: LinearRgba::new(0.1, 2.8, 3.6, 1.0),
            perceptual_roughness: 0.1,
            ..default()
        }),
        mat_core_cyan_conduit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.8, 0.95),
            emissive: LinearRgba::new(0.06, 1.6, 2.4, 1.0),
            perceptual_roughness: 0.2,
            ..default()
        }),
        mat_core_cyan_dim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.04, 0.4, 0.5),
            emissive: LinearRgba::new(0.02, 0.5, 0.8, 1.0),
            perceptual_roughness: 0.4,
            ..default()
        }),
        mat_core_data_prism: materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.9, 1.0, 0.85),
            emissive: LinearRgba::new(0.15, 1.8, 2.6, 1.0),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.15,
            ..default()
        }),
        mat_core_singularity_core: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.98, 1.0),
            emissive: LinearRgba::new(3.0, 4.5, 5.0, 1.0),
            perceptual_roughness: 0.05,
            ..default()
        }),
        mat_core_void_darkness: materials.add(StandardMaterial {
            base_color: Color::srgb(0.01, 0.01, 0.02),
            perceptual_roughness: 0.9,
            metallic: 0.1,
            ..default()
        }),

        // Zone Relic Memory Ghost Materials (Ethereal translucent glows)
        mat_relic_amber: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.6, 0.1, 0.75),
            emissive: LinearRgba::new(1.6, 0.8, 0.1, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_relic_magenta: materials.add(StandardMaterial {
            base_color: Color::srgba(0.9, 0.1, 0.7, 0.75),
            emissive: LinearRgba::new(2.0, 0.1, 1.5, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_relic_molten: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.4, 0.0, 0.75),
            emissive: LinearRgba::new(2.2, 0.7, 0.05, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_relic_turquoise: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.8, 0.7, 0.75),
            emissive: LinearRgba::new(0.1, 1.8, 1.6, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_relic_sky_blue: materials.add(StandardMaterial {
            base_color: Color::srgba(0.3, 0.7, 1.0, 0.75),
            emissive: LinearRgba::new(0.5, 1.2, 2.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_relic_crimson: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.05, 0.15, 0.75),
            emissive: LinearRgba::new(2.2, 0.08, 0.15, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

// -------------------------------------------------------------
// PLUGIN DEFINITION & DYNAMIC SYSTEMS
// -------------------------------------------------------------

pub struct EchoCorePlugin;

impl Plugin for EchoCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EchoCoreResonanceState>()
            .init_resource::<EchoCoreDialogueManager>()
            .add_systems(
                Update,
                (
                    update_echo_core_resonance,
                    animate_echo_core_rotators,
                    update_echo_core_dialogues,
                    update_echo_memory_projections,
                    handle_run_reset_echo_core,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

pub fn update_echo_core_resonance(
    time: Res<Time>,
    run_stats: Option<Res<GameRunStats>>,
    mut resonance_state: ResMut<EchoCoreResonanceState>,
    ec_assets: Option<Res<EchoCoreAssets>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(ec) = ec_assets else { return };

    let in_zone_7 = run_stats
        .as_ref()
        .map(|s| s.distance >= 11950.0 && s.distance <= 14050.0)
        .unwrap_or(true);

    if !in_zone_7 {
        return;
    }

    let dt = time.delta_seconds();
    resonance_state.timer += dt;

    // Harmonic pulse (breathing cycle: 2.2s period)
    let cycle = (resonance_state.timer * 2.85).sin();
    let pulse_factor = 0.8 + 0.2 * cycle.max(0.0).powi(2);
    resonance_state.pulse = pulse_factor;

    // Singularity shimmer (intermodulation frequency)
    let shimmer =
        0.85 + 0.15 * (resonance_state.timer * 6.2).sin() * (resonance_state.timer * 11.4).cos();
    resonance_state.anomaly_shimmer = shimmer;
    resonance_state.core_harmonic = 0.5 + 0.5 * (resonance_state.timer * 1.8).sin();

    // Modulate cyan radiant rail glow
    if let Some(mat) = materials.get_mut(&ec.mat_core_cyan_radiant) {
        let base_emissive = LinearRgba::new(0.1, 2.8, 3.6, 1.0);
        mat.emissive = LinearRgba::new(
            base_emissive.red * pulse_factor,
            base_emissive.green * pulse_factor,
            base_emissive.blue * pulse_factor,
            1.0,
        );
    }

    // Modulate quantum conduit energy flow
    if let Some(mat) = materials.get_mut(&ec.mat_core_cyan_conduit) {
        let base_emissive = LinearRgba::new(0.06, 1.6, 2.4, 1.0);
        mat.emissive = LinearRgba::new(
            base_emissive.red * shimmer,
            base_emissive.green * shimmer,
            base_emissive.blue * shimmer,
            1.0,
        );
    }
}

pub fn animate_echo_core_rotators(
    time: Res<Time>,
    mut rotator_q: Query<(&mut Transform, &EchoCoreRotator)>,
) {
    let dt = time.delta_seconds();
    for (mut trans, rotator) in rotator_q.iter_mut() {
        trans.rotate(Quat::from_axis_angle(rotator.axis, rotator.speed * dt));
    }
}

// -------------------------------------------------------------
// MODULAR SEGMENT SPAWNING
// -------------------------------------------------------------

pub fn spawn_echo_core_segment(
    commands: &mut Commands,
    _env: &EnvironmentAssets,
    ec: &EchoCoreAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = echo_core_chapter_for_distance(distance);
    let landmark = echo_core_landmark_for_distance(distance);
    let variant = echo_core_variant(distance);

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            // 1. Sleek Obsidian Light Bridge & 3 Crisp Running Lanes
            spawn_light_bridge_bed(seg, ec, length);
            spawn_three_light_lanes(seg, ec, chapter, variant);

            // 2. Colossal Cavern Void Background (FAR Layer)
            spawn_far_void_depth(seg, ec, chapter, variant, distance);

            // 3. Chapter-Specific Visual Dressing (NEAR & MID Layers)
            match chapter {
                EchoCoreChapter::Threshold => {
                    spawn_threshold_dressing(seg, ec, variant);
                }
                EchoCoreChapter::ResonanceChamber => {
                    spawn_resonance_chamber_dressing(seg, ec, variant, distance, z_center);
                }
                EchoCoreChapter::RealityCollapse => {
                    spawn_reality_collapse_dressing(seg, ec, variant);
                }
                EchoCoreChapter::FinalPursuit => {
                    spawn_final_pursuit_dressing(seg, ec, variant);
                }
                EchoCoreChapter::Core => {
                    spawn_core_sanctum_dressing(seg, ec, variant, distance);
                }
            }

            // 4. Deterministic Hero Landmark (if active in this segment window)
            if landmark != EchoCoreLandmark::None {
                spawn_echo_core_landmark(seg, ec, landmark);
            }
        })
        .id()
}

// -------------------------------------------------------------
// 1. TRACK BED & LIGHT BRIDGE (Readability & Fair Gameplay)
// -------------------------------------------------------------

fn spawn_light_bridge_bed(seg: &mut ChildBuilder, ec: &EchoCoreAssets, _length: f32) {
    // Sleek obsidian bridge slab suspended over void (y = -0.175)
    seg.spawn(PbrBundle {
        mesh: ec.mesh_light_bridge_bed.clone(),
        material: ec.mat_core_obsidian.clone(),
        transform: Transform::from_xyz(0.0, -0.175, 0.0),
        ..default()
    });

    // Outer obsidian chamfered curbs at track boundaries (x = -4.0, +4.0)
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_obsidian_curb.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 4.0, -0.02, 0.0),
            ..default()
        });

        // Glowing cyan quantum conduit running along curbs
        seg.spawn(PbrBundle {
            mesh: ec.mesh_quantum_conduit.clone(),
            material: ec.mat_core_cyan_conduit.clone(),
            transform: Transform::from_xyz(sign * 3.85, 0.08, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    }

    // Deep drop pylons anchored into bottomless void (every 20m)
    for z in [-10.0f32, 10.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_bridge_pylon_drop.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(0.0, -30.2, z),
            ..default()
        });
    }
}

fn spawn_three_light_lanes(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    chapter: EchoCoreChapter,
    _variant: u8,
) {
    let lanes = [Lane::Left, Lane::Center, Lane::Right];

    for lane in lanes {
        let x = lane.x_pos();

        // Translucent crystal sleepers spaced every 4m along lane
        for i in -5..=5 {
            let z = i as f32 * 3.8;
            seg.spawn(PbrBundle {
                mesh: ec.mesh_crystal_sleeper.clone(),
                material: ec.mat_core_cyan_dim.clone(),
                transform: Transform::from_xyz(x, -0.01, z),
                ..default()
            });
        }

        // Twin radiant cyan rails for each running lane (spacing 0.7m)
        for rail_offset in [-0.7f32, 0.7f32] {
            let rail_x = x + rail_offset;

            // Rail base profile
            seg.spawn(PbrBundle {
                mesh: ec.mesh_rail_base.clone(),
                material: ec.mat_core_obsidian_structure.clone(),
                transform: Transform::from_xyz(rail_x, 0.02, 0.0),
                ..default()
            });

            // Luminous cyan rail crown
            seg.spawn(PbrBundle {
                mesh: ec.mesh_rail_crown.clone(),
                material: ec.mat_core_cyan_radiant.clone(),
                transform: Transform::from_xyz(rail_x, 0.065, 0.0),
                ..default()
            });
        }
    }

    // Subtle cyan lane separator ribbons (at x = -1.2, +1.2)
    for sep_x in [-1.2f32, 1.2f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_lane_ribbon.clone(),
            material: ec.mat_core_cyan_conduit.clone(),
            transform: Transform::from_xyz(sep_x, 0.005, 0.0),
            ..default()
        });
    }

    // Guide beacons along outer edge (spacing 8m, stripped back in Act 5)
    if chapter != EchoCoreChapter::Core {
        for i in -2..=2 {
            let z = i as f32 * 8.0;
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_guide_beacon.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(sign * 3.95, 0.4, z),
                    ..default()
                });
            }
        }
    }
}

// -------------------------------------------------------------
// 2. FAR LAYER — COLOSSAL CAVERN VOID & ABYSS DEPTH
// -------------------------------------------------------------

fn spawn_far_void_depth(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    chapter: EchoCoreChapter,
    variant: u8,
    distance: f32,
) {
    // Immense cavern pillars disappearing into vertical abyss (-120m to +80m)
    let col_x_offset = match variant {
        0 => 38.0,
        1 => 44.0,
        _ => 40.0,
    };

    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_cavern_column_giant.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * col_x_offset, -20.0, 0.0),
            ..default()
        });

        // Distant glittering crystal clusters embedded in the deep cavern walls
        seg.spawn(PbrBundle {
            mesh: ec.mesh_distant_crystal_cluster.clone(),
            material: ec.mat_core_cyan_dim.clone(),
            transform: Transform::from_xyz(
                sign * (col_x_offset + 18.0),
                -15.0,
                (variant as f32 - 1.0) * 8.0,
            )
            .with_rotation(Quat::from_rotation_z(sign * 0.18)),
            ..default()
        });
    }

    // In Act 5 (Core), the distant horizon features THE ECHO CORE sphere!
    if chapter == EchoCoreChapter::Core {
        let core_progress = ((distance - 13700.0) / 300.0).clamp(0.0, 1.0);
        // The core looms closer as distance approaches 14,000m: z in [-180m, -60m]
        let core_z = -180.0 + core_progress * 100.0;
        let core_y = 18.0 - core_progress * 8.0;

        // Radiant Singularity Sphere
        seg.spawn(PbrBundle {
            mesh: ec.mesh_core_sphere_horizon.clone(),
            material: ec.mat_core_singularity_core.clone(),
            transform: Transform::from_xyz(0.0, core_y, core_z),
            ..default()
        });

        // Orbiting polyhedral energy ring around the Core
        seg.spawn((
            PbrBundle {
                mesh: ec.mesh_core_orbit_polyhedron.clone(),
                material: ec.mat_core_cyan_radiant.clone(),
                transform: Transform::from_xyz(0.0, core_y, core_z),
                ..default()
            },
            EchoCoreRotator {
                speed: 0.35,
                axis: Vec3::new(0.3, 1.0, 0.2).normalize(),
            },
        ));

        // Secondary counter-rotating ring
        seg.spawn((
            PbrBundle {
                mesh: ec.mesh_concentric_halo_ring.clone(),
                material: ec.mat_core_cyan_conduit.clone(),
                transform: Transform::from_xyz(0.0, core_y, core_z)
                    .with_rotation(Quat::from_rotation_x(0.4)),
                ..default()
            },
            EchoCoreRotator {
                speed: -0.22,
                axis: Vec3::new(-0.2, 1.0, 0.4).normalize(),
            },
        ));
    }
}

// -------------------------------------------------------------
// 3. CHAPTER DRESSINGS (NEAR & MID LAYERS)
// -------------------------------------------------------------

// ACT 1 — ECHO THRESHOLD (12,000m–12,400m): Breaching Singularity Vault
fn spawn_threshold_dressing(seg: &mut ChildBuilder, ec: &EchoCoreAssets, variant: u8) {
    // NEAR: Slender conduit pylons guiding eye into the void
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_conduit_stanchion.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 4.8, 1.4, 0.0),
            ..default()
        });

        seg.spawn(PbrBundle {
            mesh: ec.mesh_crystal_spire_small.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(sign * 5.2, 2.0, 0.0),
            ..default()
        });
    }

    // MID: Colossal obsidian monoliths rising from below and hanging from above
    let z_off = match variant {
        0 => -6.0,
        1 => 6.0,
        _ => 0.0,
    };

    for sign in [-1.0f32, 1.0f32] {
        // Ground monolith
        seg.spawn(PbrBundle {
            mesh: ec.mesh_obsidian_monolith.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 16.0, 10.0, z_off)
                .with_rotation(Quat::from_rotation_z(sign * 0.08)),
            ..default()
        });

        // Inverted hanging stalactite monolith
        seg.spawn(PbrBundle {
            mesh: ec.mesh_hanging_monolith.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 18.0, 38.0, -z_off)
                .with_rotation(Quat::from_rotation_z(-sign * 0.1)),
            ..default()
        });

        // Vertical cyan energy cascade beam
        seg.spawn(PbrBundle {
            mesh: ec.mesh_energy_cascade_beam.clone(),
            material: ec.mat_core_cyan_conduit.clone(),
            transform: Transform::from_xyz(sign * 12.0, 20.0, z_off),
            ..default()
        });
    }
}

// ACT 2 — RESONANCE CHAMBER (12,400m–12,800m): The Memory Archive
fn spawn_resonance_chamber_dressing(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    variant: u8,
    distance: f32,
    z_center: f32,
) {
    // 1. NEAR: Floating holographic data prisms alongside track
    for sign in [-1.0f32, 1.0f32] {
        let prism_z = match variant {
            0 => -8.0,
            1 => 8.0,
            _ => 0.0,
        };

        seg.spawn((
            PbrBundle {
                mesh: ec.mesh_data_prism_near.clone(),
                material: ec.mat_core_data_prism.clone(),
                transform: Transform::from_xyz(sign * 4.6, 1.2, prism_z),
                ..default()
            },
            EchoCoreRotator {
                speed: 1.2 * sign,
                axis: Vec3::Y,
            },
        ));
    }

    // 2. Volumetric Holographic Memory Projection (if present on this segment)
    let memory_shard = echo_memory_shard_for_distance(distance);
    if memory_shard != EchoMemoryShard::None {
        spawn_memory_projection(seg, ec, memory_shard, z_center);
    }

    // MID: Enormous circular accelerator ring encircling void
    // Rotated 90° around X so its aperture stands vertically across the tracks
    let (ring_rot_x, ring_y) = match variant {
        0 => (0.15f32, 10.0f32),
        1 => (-0.15f32, 12.0f32),
        _ => (0.0f32, 11.0f32),
    };

    seg.spawn(PbrBundle {
        mesh: ec.mesh_accelerator_ring_36m.clone(),
        material: ec.mat_core_obsidian_structure.clone(),
        transform: Transform::from_xyz(0.0, ring_y, 0.0).with_rotation(Quat::from_rotation_x(
            std::f32::consts::FRAC_PI_2 + ring_rot_x,
        )),
        ..default()
    });

    // Four radial energy inductor nodes along ring perimeter
    for angle_idx in 0..4 {
        let angle = angle_idx as f32 * std::f32::consts::FRAC_PI_2 + 0.3;
        let node_x = angle.cos() * 21.2;
        let node_y = ring_y + angle.sin() * 21.2;

        seg.spawn(PbrBundle {
            mesh: ec.mesh_ring_inductor_node.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(node_x, node_y, 0.0),
            ..default()
        });
    }

    // Distant background memory monoliths with glowing conduits
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_obsidian_monolith.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 24.0, 18.0, 0.0),
            ..default()
        });
    }
}

// ACT 3 — REALITY COLLAPSE (12,800m–13,300m): Spatial Deconstruction (Non-Euclidean)
fn spawn_reality_collapse_dressing(seg: &mut ChildBuilder, ec: &EchoCoreAssets, variant: u8) {
    // NEAR: Angled spatial crystal shards floating at skewed angles
    for sign in [-1.0f32, 1.0f32] {
        let angle = match variant {
            0 => 0.45,
            1 => -0.55,
            _ => 0.35,
        };

        seg.spawn(PbrBundle {
            mesh: ec.mesh_spatial_shard_near.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(sign * 4.9, 1.8, (variant as f32 - 1.0) * 5.0)
                .with_rotation(Quat::from_rotation_z(sign * angle)),
            ..default()
        });
    }

    // MID: Inverted overhead rails suspended in mid-air at +22m
    seg.spawn(PbrBundle {
        mesh: ec.mesh_inverted_rail_truss.clone(),
        material: ec.mat_core_obsidian_structure.clone(),
        transform: Transform::from_xyz(0.0, 22.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        ..default()
    });

    // Inverted glowing cyan rails on the ceiling
    for rail_x in [-2.4f32, 0.0f32, 2.4f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_rail_crown.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(rail_x, 21.6, 0.0),
            ..default()
        });
    }

    // Tilted structural beams rotated at 45° across the lateral void
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_tilted_structural_beam.clone(),
            material: ec.mat_core_obsidian_structure.clone(),
            transform: Transform::from_xyz(sign * 18.0, 14.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.2,
                sign * 0.4,
                sign * 0.785,
            )), // 45°
            ..default()
        });
    }

    // Suspended relics from Zones 1–6 floating in zero-G
    match variant {
        0 => {
            // Zone 1: Rusted Metro Arch & Zone 2: Neon Sign Frame
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_metro_arch.clone(),
                    material: ec.mat_relic_amber.clone(),
                    transform: Transform::from_xyz(-14.0, 16.0, -4.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.3, 0.2, 0.4)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: 0.2,
                    axis: Vec3::Y,
                },
            ));
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_neon_frame.clone(),
                    material: ec.mat_relic_magenta.clone(),
                    transform: Transform::from_xyz(15.0, 18.0, 6.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.4, 0.5, -0.3)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: -0.25,
                    axis: Vec3::X,
                },
            ));
        }
        1 => {
            // Zone 3: Industrial Pipe & Zone 4: Flooded Water Ring
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_industrial_pipe.clone(),
                    material: ec.mat_relic_molten.clone(),
                    transform: Transform::from_xyz(-16.0, 20.0, 5.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.5, -0.3, 0.8)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: 0.18,
                    axis: Vec3::Z,
                },
            ));
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_flooded_ring.clone(),
                    material: ec.mat_relic_turquoise.clone(),
                    transform: Transform::from_xyz(16.0, 14.0, -6.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.2, 0.4, 0.6)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: -0.3,
                    axis: Vec3::Y,
                },
            ));
        }
        _ => {
            // Zone 5: Sky Pylon & Zone 6: Quarantine Blast Door
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_sky_pylon.clone(),
                    material: ec.mat_relic_sky_blue.clone(),
                    transform: Transform::from_xyz(-15.0, 19.0, 2.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.3, 0.4)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: 0.15,
                    axis: Vec3::Y,
                },
            ));
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_relic_blast_door.clone(),
                    material: ec.mat_relic_crimson.clone(),
                    transform: Transform::from_xyz(14.0, 15.0, -3.0)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.3, -0.5, -0.4)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: -0.2,
                    axis: Vec3::Z,
                },
            ));
        }
    }
}

// ACT 4 — FINAL PURSUIT (13,300m–13,700m): Staging Arena & Energy Storm
fn spawn_final_pursuit_dressing(seg: &mut ChildBuilder, ec: &EchoCoreAssets, variant: u8) {
    // NEAR: High-frequency unstable energy shards
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_crystal_spire_small.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(sign * 5.1, 1.8, (variant as f32 - 1.0) * 6.0)
                .with_rotation(Quat::from_rotation_z(sign * 0.25)),
            ..default()
        });
    }

    // MID: Sweeping amphitheater ribs opening upward and backward
    // Specifically composed to frame the empty void behind/above player for ECHO Prime
    let rib_y = match variant {
        0 => 16.0,
        1 => 20.0,
        _ => 18.0,
    };

    seg.spawn(PbrBundle {
        mesh: ec.mesh_amphitheater_rib.clone(),
        material: ec.mat_core_obsidian_structure.clone(),
        transform: Transform::from_xyz(0.0, rib_y, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            std::f32::consts::FRAC_PI_2 + 0.35,
            0.0,
            0.0,
        )),
        ..default()
    });

    // Vertical conduit cascades framing lateral bounds
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_energy_cascade_beam.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(sign * 14.0, 22.0, 0.0),
            ..default()
        });
    }
}

// ACT 5 — THE ECHO CORE (13,700m–14,000m): Quiet Grandeur & Pure Light Bridge
fn spawn_core_sanctum_dressing(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    _variant: u8,
    _distance: f32,
) {
    // NEAR: Pristine light bridge. Minimalist silence plinths set far back (|x| = 7.5m)
    for sign in [-1.0f32, 1.0f32] {
        seg.spawn(PbrBundle {
            mesh: ec.mesh_silence_plinth.clone(),
            material: ec.mat_core_obsidian.clone(),
            transform: Transform::from_xyz(sign * 7.5, 4.0, 0.0),
            ..default()
        });

        // Soft vertical accent conduit on each plinth
        seg.spawn(PbrBundle {
            mesh: ec.mesh_guide_beacon.clone(),
            material: ec.mat_core_cyan_radiant.clone(),
            transform: Transform::from_xyz(sign * 7.5, 8.5, 0.0),
            ..default()
        });
    }

    // MID: Pure open space. No cluttered lateral architecture.
    // The player has an unobstructed, majestic view straight toward THE ECHO CORE sphere!
}

// -------------------------------------------------------------
// 4. DETERMINISTIC HERO LANDMARKS
// -------------------------------------------------------------

fn spawn_echo_core_landmark(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    landmark: EchoCoreLandmark,
) {
    match landmark {
        EchoCoreLandmark::OuterResonanceGateway => {
            // ~12,080m: Monumental 40m polygon gateway portal framing track
            // Rotated 90° around X so its 36.8m open aperture frames the running corridor
            // Bottom of the ring is at y = -10.0m (below track), top is at y = +30.0m
            seg.spawn(PbrBundle {
                mesh: ec.mesh_gateway_portal_ring.clone(),
                material: ec.mat_core_obsidian_structure.clone(),
                transform: Transform::from_xyz(0.0, 10.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // Concentric radiant cyan energy ring inside portal frame
            seg.spawn(PbrBundle {
                mesh: ec.mesh_gateway_portal_ring.clone(),
                material: ec.mat_core_cyan_radiant.clone(),
                transform: Transform::from_xyz(0.0, 10.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(0.92)),
                ..default()
            });

            // Flanking obsidian spires safely placed far outside (|x| = 22.0m)
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_gateway_portal_spire.clone(),
                    material: ec.mat_core_obsidian.clone(),
                    transform: Transform::from_xyz(sign * 22.0, 16.0, 0.0),
                    ..default()
                });
            }
        }
        EchoCoreLandmark::ResonanceGrandHarmonic => {
            // ~12,580m: Massive double-tier accelerator nexus
            seg.spawn(PbrBundle {
                mesh: ec.mesh_nexus_outer_gimbal.clone(),
                material: ec.mat_core_obsidian_structure.clone(),
                transform: Transform::from_xyz(0.0, 10.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            });

            // Four vertical energy collectors
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_nexus_energy_collector.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(sign * 22.0, 14.0, 0.0),
                    ..default()
                });
            }
        }
        EchoCoreLandmark::RealityFractureNexus => {
            // ~13,050m: Towering dimensional rift blade flanked by floating crystals
            seg.spawn(PbrBundle {
                mesh: ec.mesh_rift_shard_monument.clone(),
                material: ec.mat_core_cyan_radiant.clone(),
                transform: Transform::from_xyz(0.0, 24.0, 0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.2,
                    0.4,
                    0.1,
                )),
                ..default()
            });

            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_rift_shard_monument.clone(),
                    material: ec.mat_core_obsidian_structure.clone(),
                    transform: Transform::from_xyz(sign * 15.0, 20.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(sign * 0.45)),
                    ..default()
                });
            }
        }
        EchoCoreLandmark::PursuitVectorApex => {
            // ~13,500m: Towering energy archway
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_apex_energy_tendril.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(sign * 12.0, 20.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(sign * 0.2)),
                    ..default()
                });
            }
        }
        EchoCoreLandmark::SingularityCoreShrine => {
            // ~13,920m: Elevated ceremonial approach dais
            seg.spawn(PbrBundle {
                mesh: ec.mesh_shrine_dais_platform.clone(),
                material: ec.mat_core_obsidian.clone(),
                transform: Transform::from_xyz(0.0, -0.6, 0.0),
                ..default()
            });
        }
        EchoCoreLandmark::None => {}
    }
}

// -------------------------------------------------------------
// 5. VOLUMETRIC HOLOGRAPHIC MEMORY PROJECTIONS (Act 2)
// -------------------------------------------------------------

fn spawn_memory_projection(
    seg: &mut ChildBuilder,
    ec: &EchoCoreAssets,
    shard: EchoMemoryShard,
    z_center: f32,
) {
    let (sign, memory_id, title, sender, snippet) = match shard {
        EchoMemoryShard::NeuralGenesis => (
            -1.0f32,
            1,
            "CORE ARCHIVE #01 // NEURAL GENESIS",
            "ECHO Memory Reconstruction",
            "Project ECHO initialized deep under subterranean Sector 04. Veyron attempted to harness neural quantum singularity resonance to power the city. The Core was alive from its first ignition.",
        ),
        EchoMemoryShard::ContainmentFracture => (
            1.0f32,
            2,
            "CORE ARCHIVE #02 // CONTAINMENT FRACTURE",
            "Containment Terminal 07 Override",
            "Emergency alert: Containment field collapsed. Subject refused forced weaponization and shattered dampening coils. The transit line was locked down as a militarized quarantine zone.",
        ),
        EchoMemoryShard::TransitCatastrophe => (
            -1.0f32,
            3,
            "CORE ARCHIVE #03 // TRANSIT TRANSCENDENCE",
            "Commuter Manifest Telemetry",
            "4,000 commuters dematerialized at Central Interchange. Physical matter was not destroyed, but converted into harmonic wave resonance. Their patterns remain preserved inside the Core.",
        ),
        EchoMemoryShard::RogueDispatch => (
            1.0f32,
            4,
            "CORE ARCHIVE #04 // THE ROGUE DISPATCH",
            "Automated Dispatch Manifest #4096",
            "Dispatch assignment forcibly altered by non-local quantum override: Recipient Courier Kai Renn. Carrier package: Singularity Key Drive. Kai was never a thief; ECHO chose its own key.",
        ),
        EchoMemoryShard::None => return,
    };

    let plinth_x = sign * 5.8;
    let plinth_y = 0.225;

    // 1. Sleek obsidian projection pedestal
    seg.spawn(PbrBundle {
        mesh: ec.mesh_holo_pedestal.clone(),
        material: ec.mat_core_obsidian.clone(),
        transform: Transform::from_xyz(plinth_x, plinth_y, 0.0),
        ..default()
    });

    // 2. Radiant cyan emitter ring on pedestal surface
    seg.spawn(PbrBundle {
        mesh: ec.mesh_quantum_conduit.clone(),
        material: ec.mat_core_cyan_radiant.clone(),
        transform: Transform::from_xyz(plinth_x, plinth_y + 0.22, 0.0),
        ..default()
    });

    // 3. Vertical laser guide emitter
    seg.spawn(PbrBundle {
        mesh: ec.mesh_holo_beam_emitter.clone(),
        material: ec.mat_core_cyan_conduit.clone(),
        transform: Transform::from_xyz(plinth_x, plinth_y + 1.6, 0.0),
        ..default()
    });

    // 4. Volumetric holographic screen angled slightly toward runner
    let screen_rot_y = -sign * 0.25;
    let screen_mat = match shard {
        EchoMemoryShard::NeuralGenesis => ec.mat_core_data_prism.clone(),
        EchoMemoryShard::ContainmentFracture => ec.mat_relic_crimson.clone(),
        EchoMemoryShard::TransitCatastrophe => ec.mat_relic_turquoise.clone(),
        EchoMemoryShard::RogueDispatch => ec.mat_relic_amber.clone(),
        EchoMemoryShard::None => ec.mat_core_data_prism.clone(),
    };

    seg.spawn((
        PbrBundle {
            mesh: ec.mesh_holo_screen.clone(),
            material: screen_mat,
            transform: Transform::from_xyz(plinth_x, 2.5, 0.0)
                .with_rotation(Quat::from_rotation_y(screen_rot_y)),
            ..default()
        },
        EchoMemoryProjection {
            memory_id,
            title,
            sender,
            snippet,
            triggered: false,
            world_z: z_center,
        },
    ));

    // 5. Memory-specific volumetric floating relic
    match shard {
        EchoMemoryShard::NeuralGenesis => {
            // Glowing neural core sphere inside the holographic field
            seg.spawn(PbrBundle {
                mesh: ec.mesh_holo_neural_core.clone(),
                material: ec.mat_core_singularity_core.clone(),
                transform: Transform::from_xyz(plinth_x, 2.5, 0.0),
                ..default()
            });
        }
        EchoMemoryShard::ContainmentFracture => {
            // Fracturing crystal shards piercing the crimson field
            for s in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_spatial_shard_near.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(plinth_x + s * 0.8, 2.5, s * 0.4)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.4 * s, 0.2, 0.3 * s)),
                    ..default()
                });
            }
        }
        EchoMemoryShard::TransitCatastrophe => {
            // Ascending light conduits representing commuter souls
            for s in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_crystal_spire_small.clone(),
                    material: ec.mat_relic_turquoise.clone(),
                    transform: Transform::from_xyz(plinth_x + s * 0.7, 3.2, s * 0.3),
                    ..default()
                });
            }
        }
        EchoMemoryShard::RogueDispatch => {
            // Golden courier tablet manifest hovering in the screen
            seg.spawn(PbrBundle {
                mesh: ec.mesh_holo_courier_tablet.clone(),
                material: ec.mat_relic_amber.clone(),
                transform: Transform::from_xyz(plinth_x, 2.5, 0.0)
                    .with_rotation(Quat::from_rotation_y(screen_rot_y)),
                ..default()
            });
        }
        EchoMemoryShard::None => {}
    }
}

// -------------------------------------------------------------
// 6. NARRATIVE & STORY COMM SYSTEMS
// -------------------------------------------------------------

pub fn update_echo_core_dialogues(
    mut stats: ResMut<GameRunStats>,
    mut dialogue_mgr: ResMut<EchoCoreDialogueManager>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let current_dist = stats.distance;

    // Fast-forward next_index if current_dist jumped ahead (e.g. debug start preset)
    while dialogue_mgr.next_index < ECHO_CORE_DIALOGUES.len()
        && current_dist > ECHO_CORE_DIALOGUES[dialogue_mgr.next_index].distance + 150.0
    {
        dialogue_mgr.next_index += 1;
    }

    // 1. Process queued messages if dialogue toast is idle
    if stats.story_dialogue.is_none() {
        if let Some(next_msg) = dialogue_mgr.queue.pop_front() {
            stats.story_dialogue = Some(next_msg);
            sfx.send(SoundEffect::DataChipPickup);
        }
    }

    // 2. Check upcoming distance milestones in the narrative timeline
    while dialogue_mgr.next_index < ECHO_CORE_DIALOGUES.len() {
        let dialogue = &ECHO_CORE_DIALOGUES[dialogue_mgr.next_index];
        if current_dist >= dialogue.distance {
            dialogue_mgr.next_index += 1;

            let msg = StoryMessage {
                title: dialogue.title.to_string(),
                sender: dialogue.sender.to_string(),
                body: dialogue.message.to_string(),
                timer: dialogue.duration,
            };

            if stats.story_dialogue.is_none() {
                stats.story_dialogue = Some(msg);
                sfx.send(SoundEffect::DataChipPickup);
            } else {
                dialogue_mgr.queue.push_back(msg);
            }
        } else {
            break;
        }
    }
}

pub fn update_echo_memory_projections(
    player_q: Query<&Transform, With<Player>>,
    mut memory_q: Query<(&GlobalTransform, &mut EchoMemoryProjection)>,
    mut stats: ResMut<GameRunStats>,
    mut dialogue_mgr: ResMut<EchoCoreDialogueManager>,
    mut sfx: EventWriter<SoundEffect>,
) {
    let Ok(p_trans) = player_q.get_single() else {
        return;
    };
    let player_z = p_trans.translation.z;

    for (g_trans, mut proj) in memory_q.iter_mut() {
        if proj.triggered {
            continue;
        }

        let proj_z = g_trans.translation().z;
        // The runner moves along -Z. Proximity window triggers when player is within 22m
        if (player_z - proj_z).abs() < 22.0 {
            proj.triggered = true;
            stats.score += 500;
            stats.data_chips += 1;
            sfx.send(SoundEffect::DataChipPickup);

            let msg = StoryMessage {
                title: proj.title.to_string(),
                sender: proj.sender.to_string(),
                body: proj.snippet.to_string(),
                timer: 6.0,
            };

            if stats.story_dialogue.is_none() {
                stats.story_dialogue = Some(msg);
            } else {
                dialogue_mgr.queue.push_back(msg);
            }
        }
    }
}

pub fn handle_run_reset_echo_core(
    mut events: EventReader<RunResetEvent>,
    mut dialogue_mgr: ResMut<EchoCoreDialogueManager>,
    stats: Res<GameRunStats>,
) {
    for _ in events.read() {
        dialogue_mgr.queue.clear();
        dialogue_mgr.next_index = ECHO_CORE_DIALOGUES
            .iter()
            .position(|d| d.distance >= stats.distance)
            .unwrap_or(ECHO_CORE_DIALOGUES.len());
    }
}

// -------------------------------------------------------------
// TESTS
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_core_chapter_boundaries() {
        assert_eq!(
            echo_core_chapter_for_distance(12000.0),
            EchoCoreChapter::Threshold
        );
        assert_eq!(
            echo_core_chapter_for_distance(12399.9),
            EchoCoreChapter::Threshold
        );
        assert_eq!(
            echo_core_chapter_for_distance(12400.0),
            EchoCoreChapter::ResonanceChamber
        );
        assert_eq!(
            echo_core_chapter_for_distance(12799.9),
            EchoCoreChapter::ResonanceChamber
        );
        assert_eq!(
            echo_core_chapter_for_distance(12800.0),
            EchoCoreChapter::RealityCollapse
        );
        assert_eq!(
            echo_core_chapter_for_distance(13299.9),
            EchoCoreChapter::RealityCollapse
        );
        assert_eq!(
            echo_core_chapter_for_distance(13300.0),
            EchoCoreChapter::FinalPursuit
        );
        assert_eq!(
            echo_core_chapter_for_distance(13699.9),
            EchoCoreChapter::FinalPursuit
        );
        assert_eq!(
            echo_core_chapter_for_distance(13700.0),
            EchoCoreChapter::Core
        );
        assert_eq!(
            echo_core_chapter_for_distance(14000.0),
            EchoCoreChapter::Core
        );
    }

    #[test]
    fn test_echo_core_landmarks_distribution() {
        assert_eq!(
            echo_core_landmark_for_distance(12050.0),
            EchoCoreLandmark::None
        );
        assert_eq!(
            echo_core_landmark_for_distance(12080.0),
            EchoCoreLandmark::OuterResonanceGateway
        );
        assert_eq!(
            echo_core_landmark_for_distance(12580.0),
            EchoCoreLandmark::ResonanceGrandHarmonic
        );
        assert_eq!(
            echo_core_landmark_for_distance(13050.0),
            EchoCoreLandmark::RealityFractureNexus
        );
        assert_eq!(
            echo_core_landmark_for_distance(13500.0),
            EchoCoreLandmark::PursuitVectorApex
        );
        assert_eq!(
            echo_core_landmark_for_distance(13920.0),
            EchoCoreLandmark::SingularityCoreShrine
        );
        assert_eq!(
            echo_core_landmark_for_distance(13980.0),
            EchoCoreLandmark::None
        );
    }

    #[test]
    fn test_echo_core_variants_are_deterministic() {
        assert_eq!(echo_core_variant(12000.0), 0);
        assert_eq!(echo_core_variant(12040.0), 1);
        assert_eq!(echo_core_variant(12080.0), 2);
        assert_eq!(echo_core_variant(12120.0), 0);
    }

    #[test]
    fn test_echo_core_resonance_state_defaults() {
        let state = EchoCoreResonanceState::default();
        assert_eq!(state.timer, 0.0);
        assert_eq!(state.pulse, 1.0);
        assert_eq!(state.anomaly_shimmer, 1.0);
        assert_eq!(state.core_harmonic, 1.0);
    }

    #[test]
    fn test_echo_core_chapter_differentiation_across_five_acts() {
        let chapters = [
            echo_core_chapter_for_distance(12100.0),
            echo_core_chapter_for_distance(12500.0),
            echo_core_chapter_for_distance(13000.0),
            echo_core_chapter_for_distance(13500.0),
            echo_core_chapter_for_distance(13800.0),
        ];

        assert_eq!(chapters[0], EchoCoreChapter::Threshold);
        assert_eq!(chapters[1], EchoCoreChapter::ResonanceChamber);
        assert_eq!(chapters[2], EchoCoreChapter::RealityCollapse);
        assert_eq!(chapters[3], EchoCoreChapter::FinalPursuit);
        assert_eq!(chapters[4], EchoCoreChapter::Core);

        // Verify all 5 chapters are distinct
        for i in 0..chapters.len() {
            for j in (i + 1)..chapters.len() {
                assert_ne!(
                    chapters[i], chapters[j],
                    "Chapter at index {} must not equal chapter at index {}",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_echo_core_landmarks_do_not_overlap() {
        let test_distances = [12080.0, 12580.0, 13050.0, 13500.0, 13920.0];
        let mut landmarks = Vec::new();

        for &d in &test_distances {
            let lm = echo_core_landmark_for_distance(d);
            assert_ne!(lm, EchoCoreLandmark::None);
            assert!(
                !landmarks.contains(&lm),
                "Landmark {:?} spawned multiple times",
                lm
            );
            landmarks.push(lm);
        }

        assert_eq!(landmarks.len(), 5);
    }

    #[test]
    fn test_echo_core_three_lane_alignment() {
        assert_eq!(Lane::Left.x_pos(), -2.4);
        assert_eq!(Lane::Center.x_pos(), 0.0);
        assert_eq!(Lane::Right.x_pos(), 2.4);
    }

    #[test]
    fn test_echo_core_dialogues_are_sorted_and_monotonic() {
        assert!(ECHO_CORE_DIALOGUES
            .windows(2)
            .all(|w| w[0].distance <= w[1].distance));
        assert!(ECHO_CORE_DIALOGUES[0].distance >= 12000.0);
        assert!(ECHO_CORE_DIALOGUES.last().unwrap().distance <= 14000.0);
    }

    #[test]
    fn test_echo_memory_shards_distribution() {
        let test_distances = [12460.0, 12540.0, 12660.0, 12740.0];
        let shards = test_distances
            .iter()
            .map(|&d| echo_memory_shard_for_distance(d))
            .collect::<Vec<_>>();

        assert_eq!(shards[0], EchoMemoryShard::NeuralGenesis);
        assert_eq!(shards[1], EchoMemoryShard::ContainmentFracture);
        assert_eq!(shards[2], EchoMemoryShard::TransitCatastrophe);
        assert_eq!(shards[3], EchoMemoryShard::RogueDispatch);
    }

    #[test]
    fn test_echo_memory_shards_do_not_overlap_landmarks() {
        for step in 0..10 {
            let dist = 12400.0 + step as f32 * 40.0;
            let shard = echo_memory_shard_for_distance(dist);
            let landmark = echo_core_landmark_for_distance(dist);
            if shard != EchoMemoryShard::None {
                assert_eq!(
                    landmark,
                    EchoCoreLandmark::None,
                    "Memory shard at {}m collided with landmark {:?}",
                    dist,
                    landmark
                );
            }
        }
    }
}
