use crate::environment::EnvironmentAssets;
use crate::types::{AppState, Despawnable, GameRunStats, Lane, TrackSegmentMarker};
use bevy::prelude::*;

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
        mesh_accelerator_ring_36m: meshes.add(Torus::new(0.85, 18.0)),
        mesh_ring_inductor_node: meshes.add(Cuboid::new(2.4, 3.2, 1.8)),
        mesh_obsidian_monolith: meshes.add(Cuboid::new(4.5, 65.0, 4.5)),
        mesh_hanging_monolith: meshes.add(Cuboid::new(5.0, 50.0, 5.0)),
        mesh_energy_cascade_beam: meshes.add(Cylinder::new(0.4, 80.0)),
        mesh_inverted_rail_truss: meshes.add(Cuboid::new(7.2, 0.8, length)),
        mesh_tilted_structural_beam: meshes.add(Cuboid::new(1.2, 35.0, 1.2)),
        mesh_amphitheater_rib: meshes.add(Torus::new(1.2, 24.0)),
        mesh_silence_plinth: meshes.add(Cuboid::new(3.0, 8.0, 3.0)),

        // Past Zone Relic Meshes
        mesh_relic_metro_arch: meshes.add(Torus::new(0.35, 4.5)),
        mesh_relic_neon_frame: meshes.add(Cuboid::new(6.0, 3.5, 0.2)),
        mesh_relic_industrial_pipe: meshes.add(Cylinder::new(0.7, 12.0)),
        mesh_relic_flooded_ring: meshes.add(Torus::new(0.5, 5.0)),
        mesh_relic_sky_pylon: meshes.add(Cuboid::new(0.8, 16.0, 0.8)),
        mesh_relic_blast_door: meshes.add(Cuboid::new(4.8, 4.2, 0.6)),

        // 4. FAR Layer Void & Grand Scale
        mesh_cavern_column_giant: meshes.add(Cylinder::new(8.0, 140.0)),
        mesh_distant_crystal_cluster: meshes.add(Cuboid::new(14.0, 85.0, 14.0)),
        mesh_concentric_halo_ring: meshes.add(Torus::new(1.4, 45.0)),
        mesh_core_sphere_horizon: meshes.add(Sphere::new(35.0)),
        mesh_core_orbit_polyhedron: meshes.add(Torus::new(1.1, 40.0)),

        // 5. Hero Landmark Meshes
        mesh_gateway_portal_ring: meshes.add(Torus::new(1.2, 14.0)),
        mesh_gateway_portal_spire: meshes.add(Cuboid::new(2.8, 38.0, 2.8)),
        mesh_nexus_outer_gimbal: meshes.add(Torus::new(1.5, 22.0)),
        mesh_nexus_energy_collector: meshes.add(Cylinder::new(1.2, 18.0)),
        mesh_rift_shard_monument: meshes.add(Cuboid::new(2.2, 45.0, 2.2)),
        mesh_apex_energy_tendril: meshes.add(Cylinder::new(0.6, 50.0)),
        mesh_shrine_dais_platform: meshes.add(Cuboid::new(12.0, 1.2, 28.0)),

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
        app.init_resource::<EchoCoreResonanceState>();
        app.add_systems(
            Update,
            (update_echo_core_resonance, animate_echo_core_rotators)
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
                    spawn_resonance_chamber_dressing(seg, ec, variant);
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
fn spawn_resonance_chamber_dressing(seg: &mut ChildBuilder, ec: &EchoCoreAssets, variant: u8) {
    // NEAR: Floating holographic data prisms alongside track
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

    // MID: Enormous 36m-diameter circular accelerator ring encircling void
    // Alternates between overhead tilt and under-track tilt
    let (ring_rot_x, ring_y) = match variant {
        0 => (0.25f32, 10.0f32),
        1 => (-0.2f32, 12.0f32),
        _ => (0.0f32, 11.0f32),
    };

    seg.spawn(PbrBundle {
        mesh: ec.mesh_accelerator_ring_36m.clone(),
        material: ec.mat_core_obsidian_structure.clone(),
        transform: Transform::from_xyz(0.0, ring_y, 0.0)
            .with_rotation(Quat::from_rotation_x(ring_rot_x)),
        ..default()
    });

    // Four radial energy inductor nodes along ring perimeter
    for angle_idx in 0..4 {
        let angle = angle_idx as f32 * std::f32::consts::FRAC_PI_2 + 0.3;
        let node_x = angle.cos() * 18.0;
        let node_y = ring_y + angle.sin() * 18.0;

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
            0.35,
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
            // ~12,080m: Monumental 30m polygon gateway portal hovering over track
            seg.spawn(PbrBundle {
                mesh: ec.mesh_gateway_portal_ring.clone(),
                material: ec.mat_core_obsidian_structure.clone(),
                transform: Transform::from_xyz(0.0, 14.0, 0.0),
                ..default()
            });

            // Rotating inner energy ring
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_gateway_portal_ring.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(0.0, 14.0, 0.0).with_scale(Vec3::splat(0.85)),
                    ..default()
                },
                EchoCoreRotator {
                    speed: 0.6,
                    axis: Vec3::Z,
                },
            ));

            // Flanking obsidian spires (x = -16.0, +16.0)
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_gateway_portal_spire.clone(),
                    material: ec.mat_core_obsidian.clone(),
                    transform: Transform::from_xyz(sign * 16.0, 16.0, 0.0),
                    ..default()
                });
            }
        }
        EchoCoreLandmark::ResonanceGrandHarmonic => {
            // ~12,580m: Massive double-tier accelerator nexus
            seg.spawn((
                PbrBundle {
                    mesh: ec.mesh_nexus_outer_gimbal.clone(),
                    material: ec.mat_core_obsidian_structure.clone(),
                    transform: Transform::from_xyz(0.0, 16.0, 0.0),
                    ..default()
                },
                EchoCoreRotator {
                    speed: 0.25,
                    axis: Vec3::Y,
                },
            ));

            // Four vertical energy collectors
            for sign in [-1.0f32, 1.0f32] {
                seg.spawn(PbrBundle {
                    mesh: ec.mesh_nexus_energy_collector.clone(),
                    material: ec.mat_core_cyan_radiant.clone(),
                    transform: Transform::from_xyz(sign * 18.0, 14.0, 0.0),
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
}
