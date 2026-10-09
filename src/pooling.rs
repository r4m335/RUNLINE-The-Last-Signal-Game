use crate::types::*;
use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct PoolAssets {
    // Meshes (Legacy / Base roots)
    pub mesh_barrier: Handle<Mesh>,
    pub mesh_wire: Handle<Mesh>,
    pub mesh_pillar: Handle<Mesh>,
    pub mesh_train_static: Handle<Mesh>,
    pub mesh_train_moving: Handle<Mesh>,
    pub mesh_train_light: Handle<Mesh>,
    pub mesh_fragment: Handle<Mesh>,
    pub mesh_chip: Handle<Mesh>,
    pub mesh_powerup: Handle<Mesh>,

    // Low Barrier Modular Prefab Meshes & Materials
    pub mesh_barrier_stanchion: Handle<Mesh>,
    pub mesh_barrier_foot: Handle<Mesh>,
    pub mesh_barrier_rail: Handle<Mesh>,
    pub mesh_barrier_led_strip: Handle<Mesh>,
    pub mesh_barrier_lock_box: Handle<Mesh>,
    pub mesh_barrier_skirt: Handle<Mesh>,
    pub mat_barrier_steel: Handle<StandardMaterial>,
    pub mat_barrier_led: Handle<StandardMaterial>,

    // High Hanging Wire Prefab Meshes & Materials
    pub mesh_wire_anchor: Handle<Mesh>,
    pub mesh_wire_catenary: Handle<Mesh>,
    pub mesh_wire_bundle: Handle<Mesh>,
    pub mesh_wire_dangle: Handle<Mesh>,
    pub mesh_wire_spark: Handle<Mesh>,
    pub mat_wire_copper: Handle<StandardMaterial>,
    pub mat_wire_spark: Handle<StandardMaterial>,
    pub mat_wire_insulation: Handle<StandardMaterial>,

    // Tall Pillar Fortified Column Meshes & Materials
    pub mesh_pillar_base: Handle<Mesh>,
    pub mesh_pillar_body: Handle<Mesh>,
    pub mesh_pillar_trunk: Handle<Mesh>,
    pub mesh_pillar_placard: Handle<Mesh>,
    pub mesh_pillar_beacon: Handle<Mesh>,
    pub mesh_pillar_collar: Handle<Mesh>,
    pub mat_pillar_beacon: Handle<StandardMaterial>,
    pub mat_pillar_armor: Handle<StandardMaterial>,
    pub mat_pillar_pedestal: Handle<StandardMaterial>,

    // Metro Train Prefab Meshes & Materials
    pub mesh_train_bumper: Handle<Mesh>,
    pub mesh_train_windscreen: Handle<Mesh>,
    pub mesh_train_headlamp: Handle<Mesh>,
    pub mesh_train_marker_red: Handle<Mesh>,
    pub mesh_train_bogie: Handle<Mesh>,
    pub mesh_train_hvac: Handle<Mesh>,
    pub mesh_train_windows_static: Handle<Mesh>,
    pub mesh_train_windows_moving: Handle<Mesh>,
    pub mat_train_windscreen: Handle<StandardMaterial>,
    pub mat_train_marker_red: Handle<StandardMaterial>,
    pub mat_train_windows: Handle<StandardMaterial>,
    pub mat_train_chassis: Handle<StandardMaterial>,

    // Materials (Base)
    pub mat_hazard: Handle<StandardMaterial>,
    pub mat_pillar: Handle<StandardMaterial>,
    pub mat_wire: Handle<StandardMaterial>,
    pub mat_train_body: Handle<StandardMaterial>,
    pub mat_train_light: Handle<StandardMaterial>,
    pub mat_fragment: Handle<StandardMaterial>,
    pub mat_chip: Handle<StandardMaterial>,
    pub mat_powerup_shield: Handle<StandardMaterial>,
    pub mat_powerup_magnet: Handle<StandardMaterial>,
    pub mat_powerup_overdrive: Handle<StandardMaterial>,

    // Powerups Custom 3D Iconography Meshes & Materials
    pub mesh_shield_hex: Handle<Mesh>,
    pub mesh_shield_plate: Handle<Mesh>,
    pub mesh_shield_ring: Handle<Mesh>,
    pub mesh_magnet_arch: Handle<Mesh>,
    pub mesh_magnet_prong: Handle<Mesh>,
    pub mesh_magnet_pole: Handle<Mesh>,
    pub mesh_magnet_ring: Handle<Mesh>,
    pub mat_magnet_pole: Handle<StandardMaterial>,
    pub mesh_overdrive_diamond: Handle<Mesh>,
    pub mesh_overdrive_ring: Handle<Mesh>,
    pub mesh_jump_chevron: Handle<Mesh>,
    pub mesh_jump_ring: Handle<Mesh>,
    pub mat_powerup_jump: Handle<StandardMaterial>,

    // Power-up Vertical Holographic Beacon
    pub mesh_powerup_beacon_beam: Handle<Mesh>,
    pub mat_beacon_shield: Handle<StandardMaterial>,
    pub mat_beacon_magnet: Handle<StandardMaterial>,
    pub mat_beacon_overdrive: Handle<StandardMaterial>,

    // High Visibility Obstacle Cues
    pub mesh_barrier_strobe: Handle<Mesh>,
    pub mat_wire_beacon: Handle<StandardMaterial>,
    pub mat_wire_live: Handle<StandardMaterial>,
    pub mesh_wire_chevron: Handle<Mesh>,
    pub mat_wire_chevron: Handle<StandardMaterial>,

    // Zone 2 Neon District Dedicated Road Obstacle Meshes & Materials
    // A. Autonomous Delivery Van / Cyber Car (Static Vehicle)
    pub mesh_neon_van_body: Handle<Mesh>,
    pub mesh_neon_van_cabin: Handle<Mesh>,
    pub mesh_neon_van_headlights: Handle<Mesh>,
    pub mesh_neon_van_taillights: Handle<Mesh>,
    pub mesh_neon_van_skid: Handle<Mesh>,
    pub mesh_neon_van_display: Handle<Mesh>,
    pub mat_neon_van_body: Handle<StandardMaterial>,
    pub mat_neon_van_cabin: Handle<StandardMaterial>,
    pub mat_neon_van_headlight: Handle<StandardMaterial>,
    pub mat_neon_van_taillight: Handle<StandardMaterial>,
    pub mat_neon_van_display: Handle<StandardMaterial>,

    // B. Futuristic Cyber Bus (Moving Vehicle)
    pub mesh_neon_bus_body: Handle<Mesh>,
    pub mesh_neon_bus_windscreen: Handle<Mesh>,
    pub mesh_neon_bus_destination: Handle<Mesh>,
    pub mesh_neon_bus_headlights: Handle<Mesh>,
    pub mesh_neon_bus_taillights: Handle<Mesh>,
    pub mesh_neon_bus_windows: Handle<Mesh>,
    pub mesh_neon_bus_skirt: Handle<Mesh>,
    pub mat_neon_bus_body: Handle<StandardMaterial>,
    pub mat_neon_bus_destination: Handle<StandardMaterial>,

    // C. Illuminated Road Barricade (Low Obstacle)
    pub mesh_neon_barricade_frame: Handle<Mesh>,
    pub mesh_neon_barricade_stripe: Handle<Mesh>,
    pub mesh_neon_barricade_strobe: Handle<Mesh>,
    pub mesh_neon_barricade_foot: Handle<Mesh>,
    pub mat_neon_barricade_frame: Handle<StandardMaterial>,
    pub mat_neon_barricade_hazard: Handle<StandardMaterial>,
    pub mat_neon_barricade_strobe: Handle<StandardMaterial>,

    // D. Overhead Traffic Scanner (Overhead Obstacle)
    pub mesh_neon_scanner_gantry: Handle<Mesh>,
    pub mesh_neon_scanner_housing: Handle<Mesh>,
    pub mesh_neon_scanner_laser: Handle<Mesh>,
    pub mesh_neon_scanner_arrow: Handle<Mesh>,
    pub mat_neon_scanner_metal: Handle<StandardMaterial>,
    pub mat_neon_scanner_laser: Handle<StandardMaterial>,
    pub mat_neon_scanner_warning: Handle<StandardMaterial>,

    // E. Veyron Checkpoint Pillar (Lane-Blocking Structure)
    pub mesh_neon_checkpoint_column: Handle<Mesh>,
    pub mesh_neon_checkpoint_collar: Handle<Mesh>,
    pub mesh_neon_checkpoint_beacon: Handle<Mesh>,
    pub mesh_neon_checkpoint_sign: Handle<Mesh>,
    pub mat_neon_checkpoint_armor: Handle<StandardMaterial>,
    pub mat_neon_checkpoint_red: Handle<StandardMaterial>,
    pub mat_neon_checkpoint_display: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct EntityPool {
    pub barriers: Vec<Entity>,
    pub wires: Vec<Entity>,
    pub pillars: Vec<Entity>,
    pub static_trains: Vec<Entity>,
    pub moving_trains: Vec<Entity>,
    pub neon_barriers: Vec<Entity>,
    pub neon_scanners: Vec<Entity>,
    pub neon_pillars: Vec<Entity>,
    pub neon_vans: Vec<Entity>,
    pub neon_buses: Vec<Entity>,
    pub fragments: Vec<Entity>,
    pub data_chips: Vec<Entity>,
    pub powerups: Vec<Entity>,
}

impl EntityPool {
    pub fn pop(&mut self, pool_type: PoolType) -> Option<Entity> {
        match pool_type {
            PoolType::LowBarrier => self.barriers.pop(),
            PoolType::HighHangingWire => self.wires.pop(),
            PoolType::TallPillar => self.pillars.pop(),
            PoolType::StaticTrain => self.static_trains.pop(),
            PoolType::MovingTrain => self.moving_trains.pop(),
            PoolType::NeonRoadBarrier => self.neon_barriers.pop(),
            PoolType::NeonOverheadScanner => self.neon_scanners.pop(),
            PoolType::NeonCheckpointPillar => self.neon_pillars.pop(),
            PoolType::NeonAutoVan => self.neon_vans.pop(),
            PoolType::NeonCyberBus => self.neon_buses.pop(),
            PoolType::EchoFragment => self.fragments.pop(),
            PoolType::DataChip => self.data_chips.pop(),
            PoolType::PowerUp => self.powerups.pop(),
        }
    }

    pub fn push(&mut self, pool_type: PoolType, entity: Entity) {
        match pool_type {
            PoolType::LowBarrier => self.barriers.push(entity),
            PoolType::HighHangingWire => self.wires.push(entity),
            PoolType::TallPillar => self.pillars.push(entity),
            PoolType::StaticTrain => self.static_trains.push(entity),
            PoolType::MovingTrain => self.moving_trains.push(entity),
            PoolType::NeonRoadBarrier => self.neon_barriers.push(entity),
            PoolType::NeonOverheadScanner => self.neon_scanners.push(entity),
            PoolType::NeonCheckpointPillar => self.neon_pillars.push(entity),
            PoolType::NeonAutoVan => self.neon_vans.push(entity),
            PoolType::NeonCyberBus => self.neon_buses.push(entity),
            PoolType::EchoFragment => self.fragments.push(entity),
            PoolType::DataChip => self.data_chips.push(entity),
            PoolType::PowerUp => self.powerups.push(entity),
        }
    }

    pub fn total_dormant(&self) -> usize {
        self.barriers.len()
            + self.wires.len()
            + self.pillars.len()
            + self.static_trains.len()
            + self.moving_trains.len()
            + self.neon_barriers.len()
            + self.neon_scanners.len()
            + self.neon_pillars.len()
            + self.neon_vans.len()
            + self.neon_buses.len()
            + self.fragments.len()
            + self.data_chips.len()
            + self.powerups.len()
    }
}

pub struct PoolingPlugin;

impl Plugin for PoolingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityPool>()
            .add_systems(Startup, init_pool_assets);
    }
}

fn init_pool_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pool_assets = PoolAssets {
        mesh_barrier: meshes.add(Cuboid::new(2.1, 0.85, 0.4)),
        mesh_wire: meshes.add(Cuboid::new(2.2, 0.5, 0.3)),
        mesh_pillar: meshes.add(Cuboid::new(1.8, 4.0, 1.2)),
        mesh_train_static: meshes.add(Cuboid::new(2.2, 2.5, 12.0)),
        mesh_train_moving: meshes.add(Cuboid::new(2.2, 2.5, 14.0)),
        mesh_train_light: meshes.add(Sphere::new(0.25)),
        mesh_fragment: meshes.add(Sphere::new(0.24)),
        mesh_chip: meshes.add(Cuboid::new(0.4, 0.6, 0.1)),
        mesh_powerup: meshes.add(Cuboid::new(0.6, 0.6, 0.6)),

        // Low Barrier Modular Prefab Meshes & Materials
        mesh_barrier_stanchion: meshes.add(Cuboid::new(0.14, 0.74, 0.26)),
        mesh_barrier_foot: meshes.add(Cuboid::new(0.28, 0.05, 0.38)),
        mesh_barrier_rail: meshes.add(Cuboid::new(2.10, 0.28, 0.20)),
        mesh_barrier_led_strip: meshes.add(Cuboid::new(2.05, 0.06, 0.08)),
        mesh_barrier_lock_box: meshes.add(Cuboid::new(0.28, 0.28, 0.16)),
        mesh_barrier_skirt: meshes.add(Cuboid::new(1.76, 0.20, 0.06)),
        mesh_barrier_strobe: meshes.add(Sphere::new(0.08)),
        mat_barrier_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.15, 0.18),
            metallic: 0.85,
            perceptual_roughness: 0.30,
            ..default()
        }),
        mat_barrier_led: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.50, 0.05),
            emissive: LinearRgba::new(5.8, 2.4, 0.1, 1.0),
            ..default()
        }),

        // High Hanging Wire Prefab Meshes & Materials
        mesh_wire_anchor: meshes.add(Cuboid::new(0.20, 0.25, 0.28)),
        mesh_wire_catenary: meshes.add(Cuboid::new(2.20, 0.08, 0.08)),
        mesh_wire_bundle: meshes.add(Cuboid::new(2.10, 0.16, 0.16)),
        mesh_wire_dangle: meshes.add(Cuboid::new(0.04, 0.28, 0.04)),
        mesh_wire_spark: meshes.add(Sphere::new(0.12)),
        mesh_wire_chevron: meshes.add(Cuboid::new(0.40, 0.06, 0.04)),
        mat_wire_copper: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.70, 0.15),
            emissive: LinearRgba::new(4.8, 2.6, 0.3, 1.0),
            metallic: 0.95,
            perceptual_roughness: 0.15,
            ..default()
        }),
        mat_wire_spark: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.90),
            emissive: LinearRgba::new(6.5, 6.0, 2.5, 1.0),
            ..default()
        }),
        mat_wire_insulation: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.88, 0.10),
            emissive: LinearRgba::new(4.2, 3.6, 0.2, 1.0),
            perceptual_roughness: 0.25,
            metallic: 0.30,
            ..default()
        }),
        mat_wire_live: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.40),
            emissive: LinearRgba::new(5.5, 5.0, 1.2, 1.0),
            ..default()
        }),
        mat_wire_beacon: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.10, 0.05),
            emissive: LinearRgba::new(5.2, 0.2, 0.1, 1.0),
            ..default()
        }),
        mat_wire_chevron: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.90, 0.15, 0.85),
            emissive: LinearRgba::new(3.8, 3.2, 0.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),

        // Tall Pillar Fortified Column Meshes & Materials
        mesh_pillar_base: meshes.add(Cuboid::new(1.90, 0.50, 1.30)),
        mesh_pillar_body: meshes.add(Cuboid::new(1.65, 3.00, 1.05)),
        mesh_pillar_trunk: meshes.add(Cuboid::new(0.15, 3.00, 0.30)),
        mesh_pillar_placard: meshes.add(Cuboid::new(1.20, 0.90, 0.06)),
        mesh_pillar_beacon: meshes.add(Cuboid::new(0.06, 2.40, 0.04)),
        mesh_pillar_collar: meshes.add(Cuboid::new(1.80, 0.30, 1.20)),
        mat_pillar_beacon: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.05, 0.05),
            emissive: LinearRgba::new(5.8, 0.1, 0.1, 1.0),
            ..default()
        }),
        mat_pillar_armor: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.20, 0.24),
            metallic: 0.90,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_pillar_pedestal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.11, 0.11, 0.13),
            perceptual_roughness: 0.85,
            metallic: 0.15,
            ..default()
        }),

        // Metro Train Prefab Meshes & Materials
        mesh_train_bumper: meshes.add(Cuboid::new(2.20, 0.50, 0.60)),
        mesh_train_windscreen: meshes.add(Cuboid::new(1.80, 0.85, 0.12)),
        mesh_train_headlamp: meshes.add(Cuboid::new(0.35, 0.25, 0.15)),
        mesh_train_marker_red: meshes.add(Cuboid::new(0.15, 0.15, 0.08)),
        mesh_train_bogie: meshes.add(Cuboid::new(2.00, 0.35, 2.20)),
        mesh_train_hvac: meshes.add(Cuboid::new(1.40, 0.30, 7.00)),
        mesh_train_windows_static: meshes.add(Cuboid::new(0.05, 0.65, 9.00)),
        mesh_train_windows_moving: meshes.add(Cuboid::new(0.05, 0.65, 11.00)),
        mat_train_windscreen: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.25, 0.30),
            emissive: LinearRgba::new(0.1, 0.8, 1.0, 1.0),
            perceptual_roughness: 0.10,
            metallic: 0.85,
            ..default()
        }),
        mat_train_marker_red: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.1),
            emissive: LinearRgba::new(4.2, 0.2, 0.2, 1.0),
            ..default()
        }),
        mat_train_windows: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.7, 0.8),
            emissive: LinearRgba::new(0.8, 2.2, 2.8, 1.0),
            ..default()
        }),
        mat_train_chassis: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.14, 0.18),
            metallic: 0.92,
            perceptual_roughness: 0.35,
            ..default()
        }),

        mat_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.50, 0.05),
            emissive: LinearRgba::new(3.5, 1.2, 0.0, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_pillar: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.22, 0.26),
            metallic: 0.85,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_wire: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.88, 0.10),
            emissive: LinearRgba::new(4.2, 3.6, 0.2, 1.0),
            ..default()
        }),
        mat_train_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.18, 0.25),
            metallic: 0.9,
            perceptual_roughness: 0.3,
            ..default()
        }),
        mat_train_light: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.85),
            emissive: LinearRgba::new(4.5, 4.0, 2.5, 1.0),
            ..default()
        }),
        mat_fragment: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.95, 1.0),
            emissive: LinearRgba::new(0.6, 3.8, 5.2, 1.0), // Piercing cyber cyan path beacon
            perceptual_roughness: 0.08,
            metallic: 0.95,
            ..default()
        }),
        mat_chip: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.2),
            emissive: LinearRgba::new(3.5, 2.5, 0.3, 1.0),
            perceptual_roughness: 0.2,
            metallic: 0.8,
            ..default()
        }),
        mat_powerup_shield: materials.add(StandardMaterial {
            base_color: Color::srgba(0.05, 0.85, 1.0, 0.90),
            emissive: LinearRgba::new(0.6, 3.6, 5.5, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_powerup_magnet: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.9),
            emissive: LinearRgba::new(4.8, 0.3, 4.8, 1.0),
            ..default()
        }),
        mat_powerup_overdrive: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            emissive: LinearRgba::new(5.5, 2.2, 0.0, 1.0),
            ..default()
        }),

        // Powerup 3D Iconography Meshes & Materials
        mesh_shield_hex: meshes.add(Cylinder::new(0.32, 0.14)),
        mesh_shield_plate: meshes.add(Cuboid::new(0.10, 0.36, 0.05)),
        mesh_shield_ring: meshes.add(Torus::new(0.04, 0.45)),
        mesh_magnet_arch: meshes.add(Cuboid::new(0.50, 0.14, 0.14)),
        mesh_magnet_prong: meshes.add(Cuboid::new(0.14, 0.42, 0.14)),
        mesh_magnet_pole: meshes.add(Cuboid::new(0.15, 0.10, 0.15)),
        mesh_magnet_ring: meshes.add(Torus::new(0.03, 0.35)),
        mat_magnet_pole: materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.92, 0.96),
            metallic: 0.95,
            perceptual_roughness: 0.10,
            ..default()
        }),
        mesh_overdrive_diamond: meshes.add(Cuboid::new(0.32, 0.65, 0.32)),
        mesh_overdrive_ring: meshes.add(Torus::new(0.04, 0.42)),
        mesh_jump_chevron: meshes.add(Cuboid::new(0.40, 0.12, 0.10)),
        mesh_jump_ring: meshes.add(Torus::new(0.03, 0.36)),
        mat_powerup_jump: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.9, 1.0),
            emissive: LinearRgba::new(0.6, 4.2, 5.5, 1.0),
            ..default()
        }),

        // Power-up Vertical Holographic Beacon Beam (Elevated above pickups)
        mesh_powerup_beacon_beam: meshes.add(Cylinder::new(0.035, 1.10)),
        mat_beacon_shield: materials.add(StandardMaterial {
            base_color: Color::srgba(0.08, 0.85, 1.0, 0.40),
            emissive: LinearRgba::new(0.5, 2.5, 3.8, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_beacon_magnet: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.15, 0.90, 0.40),
            emissive: LinearRgba::new(3.8, 0.4, 3.8, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_beacon_overdrive: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.60, 0.05, 0.40),
            emissive: LinearRgba::new(4.0, 2.0, 0.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),

        // Zone 2 Neon District Dedicated Road Obstacle Meshes & Materials
        // A. Autonomous Delivery Van / Cyber Car (Static Vehicle)
        mesh_neon_van_body: meshes.add(Cuboid::new(2.10, 1.40, 9.80)),
        mesh_neon_van_cabin: meshes.add(Cuboid::new(1.90, 0.85, 4.20)),
        mesh_neon_van_headlights: meshes.add(Cuboid::new(1.80, 0.18, 0.08)),
        mesh_neon_van_taillights: meshes.add(Cuboid::new(1.80, 0.15, 0.08)),
        mesh_neon_van_skid: meshes.add(Cuboid::new(0.25, 0.30, 9.20)),
        mesh_neon_van_display: meshes.add(Cuboid::new(0.04, 0.40, 3.50)),
        mat_neon_van_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.16, 0.20),
            metallic: 0.92,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_neon_van_cabin: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.18, 0.24),
            metallic: 0.85,
            perceptual_roughness: 0.12,
            ..default()
        }),
        mat_neon_van_headlight: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.85),
            emissive: LinearRgba::new(4.5, 4.0, 2.5, 1.0),
            ..default()
        }),
        mat_neon_van_taillight: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.10, 0.12),
            emissive: LinearRgba::new(4.5, 0.15, 0.20, 1.0),
            ..default()
        }),
        mat_neon_van_display: materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.85, 1.0),
            emissive: LinearRgba::new(0.8, 3.2, 4.2, 1.0),
            ..default()
        }),

        // B. Futuristic Cyber Bus (Moving Vehicle)
        mesh_neon_bus_body: meshes.add(Cuboid::new(2.15, 2.20, 13.60)),
        mesh_neon_bus_windscreen: meshes.add(Cuboid::new(1.95, 1.10, 0.10)),
        mesh_neon_bus_destination: meshes.add(Cuboid::new(1.60, 0.32, 0.08)),
        mesh_neon_bus_headlights: meshes.add(Cuboid::new(0.40, 0.22, 0.10)),
        mesh_neon_bus_taillights: meshes.add(Cuboid::new(0.18, 0.85, 0.08)),
        mesh_neon_bus_windows: meshes.add(Cuboid::new(0.06, 0.75, 11.20)),
        mesh_neon_bus_skirt: meshes.add(Cuboid::new(2.10, 0.35, 13.20)),
        mat_neon_bus_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.18, 0.25),
            metallic: 0.90,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_neon_bus_destination: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.65, 0.05),
            emissive: LinearRgba::new(4.5, 2.2, 0.1, 1.0),
            ..default()
        }),

        // C. Illuminated Road Barricade (Low Obstacle)
        mesh_neon_barricade_frame: meshes.add(Cuboid::new(2.10, 0.32, 0.22)),
        mesh_neon_barricade_stripe: meshes.add(Cuboid::new(2.05, 0.18, 0.24)),
        mesh_neon_barricade_strobe: meshes.add(Sphere::new(0.09)),
        mesh_neon_barricade_foot: meshes.add(Cuboid::new(0.35, 0.06, 0.45)),
        mat_neon_barricade_frame: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.16, 0.18),
            metallic: 0.70,
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_neon_barricade_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.45, 0.05),
            emissive: LinearRgba::new(5.0, 2.0, 0.1, 1.0),
            ..default()
        }),
        mat_neon_barricade_strobe: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.10),
            emissive: LinearRgba::new(6.0, 4.5, 0.5, 1.0),
            ..default()
        }),

        // D. Overhead Traffic Scanner (Overhead Obstacle)
        mesh_neon_scanner_gantry: meshes.add(Cuboid::new(2.20, 0.28, 0.35)),
        mesh_neon_scanner_housing: meshes.add(Cuboid::new(1.60, 0.22, 0.40)),
        mesh_neon_scanner_laser: meshes.add(Cuboid::new(1.90, 0.04, 0.04)),
        mesh_neon_scanner_arrow: meshes.add(Cuboid::new(0.30, 0.08, 0.06)),
        mat_neon_scanner_metal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.19, 0.22),
            metallic: 0.90,
            perceptual_roughness: 0.22,
            ..default()
        }),
        mat_neon_scanner_laser: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.90, 0.15),
            emissive: LinearRgba::new(5.5, 4.8, 0.8, 1.0),
            ..default()
        }),
        mat_neon_scanner_warning: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.30, 0.10),
            emissive: LinearRgba::new(4.5, 1.2, 0.2, 1.0),
            ..default()
        }),

        // E. Veyron Checkpoint Pillar (Lane-Blocking Structure)
        mesh_neon_checkpoint_column: meshes.add(Cuboid::new(1.65, 3.80, 1.10)),
        mesh_neon_checkpoint_collar: meshes.add(Cuboid::new(1.85, 0.35, 1.25)),
        mesh_neon_checkpoint_beacon: meshes.add(Cuboid::new(0.08, 3.20, 0.06)),
        mesh_neon_checkpoint_sign: meshes.add(Cuboid::new(1.10, 0.80, 0.06)),
        mat_neon_checkpoint_armor: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.15, 0.18),
            metallic: 0.92,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_neon_checkpoint_red: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.05, 0.15),
            emissive: LinearRgba::new(5.8, 0.2, 0.4, 1.0),
            ..default()
        }),
        mat_neon_checkpoint_display: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.40, 0.05),
            emissive: LinearRgba::new(3.8, 1.5, 0.1, 1.0),
            ..default()
        }),
    };

    commands.insert_resource(pool_assets);
}
