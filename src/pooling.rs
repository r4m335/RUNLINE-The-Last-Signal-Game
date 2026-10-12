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
    pub mesh_neon_van_nose: Handle<Mesh>,
    pub mesh_neon_van_hood: Handle<Mesh>,
    pub mesh_neon_van_wheel: Handle<Mesh>,
    pub mesh_neon_van_rim: Handle<Mesh>,
    pub mesh_neon_van_panel: Handle<Mesh>,
    pub mesh_neon_van_sensor: Handle<Mesh>,
    pub mesh_neon_van_headlamp_unit: Handle<Mesh>,
    pub mat_neon_van_body: Handle<StandardMaterial>,
    pub mat_neon_van_cabin: Handle<StandardMaterial>,
    pub mat_neon_van_glass: Handle<StandardMaterial>,
    pub mat_neon_van_rubber: Handle<StandardMaterial>,
    pub mat_neon_van_rim: Handle<StandardMaterial>,
    pub mat_neon_van_trim: Handle<StandardMaterial>,
    pub mat_neon_van_marking: Handle<StandardMaterial>,
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
    pub mesh_neon_bus_nose: Handle<Mesh>,
    pub mesh_neon_bus_wheel: Handle<Mesh>,
    pub mesh_neon_bus_rim: Handle<Mesh>,
    pub mesh_neon_bus_window_panel: Handle<Mesh>,
    pub mesh_neon_bus_trim: Handle<Mesh>,
    pub mesh_neon_bus_sensor: Handle<Mesh>,
    pub mat_neon_bus_body: Handle<StandardMaterial>,
    pub mat_neon_bus_glass: Handle<StandardMaterial>,
    pub mat_neon_bus_rubber: Handle<StandardMaterial>,
    pub mat_neon_bus_rim: Handle<StandardMaterial>,
    pub mat_neon_bus_trim: Handle<StandardMaterial>,
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

    // Zone 3 Industrial Sector Dedicated Obstacles
    pub mesh_industrial_barrier_frame: Handle<Mesh>,
    pub mesh_industrial_barrier_stripe: Handle<Mesh>,
    pub mesh_industrial_barrier_support: Handle<Mesh>,
    pub mesh_industrial_pipe: Handle<Mesh>,
    pub mesh_industrial_pipe_bundle: Handle<Mesh>,
    pub mesh_industrial_clearance: Handle<Mesh>,
    pub mesh_industrial_freight_body: Handle<Mesh>,
    pub mesh_industrial_freight_cab: Handle<Mesh>,
    pub mesh_industrial_freight_panel: Handle<Mesh>,
    pub mesh_industrial_freight_wheel: Handle<Mesh>,
    pub mat_industrial_steel: Handle<StandardMaterial>,
    pub mat_industrial_hazard: Handle<StandardMaterial>,
    pub mat_industrial_rust: Handle<StandardMaterial>,
    pub mat_industrial_pipe: Handle<StandardMaterial>,
    pub mat_industrial_warning: Handle<StandardMaterial>,
    pub mat_industrial_glass: Handle<StandardMaterial>,

    // Zone 4 Flooded Metro Dedicated Obstacles
    pub mesh_flooded_barrier_frame: Handle<Mesh>,
    pub mesh_flooded_barrier_plate: Handle<Mesh>,
    pub mesh_flooded_barrier_top_rail: Handle<Mesh>,
    pub mesh_flooded_barrier_stripe: Handle<Mesh>,
    pub mesh_flooded_barrier_skirt: Handle<Mesh>,
    pub mesh_flooded_barrier_strobe: Handle<Mesh>,
    pub mesh_flooded_barrier_support: Handle<Mesh>,
    pub mesh_flooded_pipe_column: Handle<Mesh>,
    pub mesh_flooded_column_collar: Handle<Mesh>,
    pub mesh_flooded_column_valve: Handle<Mesh>,
    pub mesh_flooded_cable_bundle: Handle<Mesh>,
    pub mesh_flooded_cable_tag: Handle<Mesh>,
    pub mesh_flooded_cable_conduit: Handle<Mesh>,
    pub mesh_flooded_gate_strip: Handle<Mesh>,
    pub mesh_flooded_gate_foot: Handle<Mesh>,
    pub mesh_flooded_gate_tooth: Handle<Mesh>,
    pub mesh_flooded_train_body: Handle<Mesh>,
    pub mesh_flooded_train_roof: Handle<Mesh>,
    pub mesh_flooded_train_window: Handle<Mesh>,
    pub mesh_flooded_service_body: Handle<Mesh>,
    pub mesh_flooded_service_cab: Handle<Mesh>,
    pub mesh_flooded_service_light: Handle<Mesh>,
    pub mesh_empty_gondola_wall_side: Handle<Mesh>,
    pub mesh_empty_gondola_wall_end: Handle<Mesh>,
    pub mesh_empty_gondola_floor: Handle<Mesh>,
    pub mesh_empty_gondola_rim: Handle<Mesh>,
    pub mesh_empty_gondola_stake: Handle<Mesh>,
    pub mesh_cargo_container_short: Handle<Mesh>,
    pub mesh_cargo_crate_stack: Handle<Mesh>,
    pub mesh_cargo_drum: Handle<Mesh>,
    pub mesh_cargo_cable_reel: Handle<Mesh>,
    pub mat_flooded_steel: Handle<StandardMaterial>,
    pub mat_flooded_hazard: Handle<StandardMaterial>,
    pub mat_flooded_rust: Handle<StandardMaterial>,
    pub mat_flooded_amber: Handle<StandardMaterial>,
    pub mat_flooded_cable: Handle<StandardMaterial>,
    pub mat_cargo_container: Handle<StandardMaterial>,
    pub mat_cargo_wood: Handle<StandardMaterial>,

    // Zone 5 Sky Rail Dedicated Obstacle Meshes & Materials
    // A. Sky Rail Maintenance Gate (Low Obstacle)
    pub mesh_sky_gate_frame: Handle<Mesh>,
    pub mesh_sky_gate_hazard_panel: Handle<Mesh>,
    pub mesh_sky_gate_top_rail: Handle<Mesh>,
    pub mesh_sky_gate_stanchion: Handle<Mesh>,
    pub mesh_sky_gate_foot: Handle<Mesh>,
    pub mesh_sky_gate_sensor: Handle<Mesh>,

    // B. Low-Clearance Aerodynamic Maintenance Beam (Overhead Obstacle)
    pub mesh_sky_beam_body: Handle<Mesh>,
    pub mesh_sky_beam_strut: Handle<Mesh>,
    pub mesh_sky_beam_plate: Handle<Mesh>,
    pub mesh_sky_beam_conduit: Handle<Mesh>,
    pub mesh_sky_beam_tooth: Handle<Mesh>,

    // C. Sky Rail Signal / Magnetic Maintenance Tower (Full-Height Column)
    pub mesh_sky_tower_core: Handle<Mesh>,
    pub mesh_sky_tower_pedestal: Handle<Mesh>,
    pub mesh_sky_tower_collar: Handle<Mesh>,
    pub mesh_sky_tower_truss: Handle<Mesh>,
    pub mesh_sky_tower_beacon: Handle<Mesh>,
    pub mesh_sky_tower_placard: Handle<Mesh>,

    // D. Derelict Sky Rail Mag-Car (Static Vehicle)
    pub mesh_sky_mag_car_body: Handle<Mesh>,
    pub mesh_sky_mag_car_nose: Handle<Mesh>,
    pub mesh_sky_mag_car_skirt: Handle<Mesh>,
    pub mesh_sky_mag_car_windscreen: Handle<Mesh>,
    pub mesh_sky_mag_car_window_strip: Handle<Mesh>,

    // E. High-Speed Sky Mag-Train (Moving Vehicle)
    pub mesh_sky_mag_train_body: Handle<Mesh>,
    pub mesh_sky_mag_train_nose: Handle<Mesh>,
    pub mesh_sky_mag_train_windscreen: Handle<Mesh>,
    pub mesh_sky_mag_train_skirt: Handle<Mesh>,
    pub mesh_sky_mag_train_fin: Handle<Mesh>,
    pub mesh_sky_mag_train_headlamp: Handle<Mesh>,

    // Zone 5 Obstacle Materials (90% darkness, 10% light)
    pub mat_sky_obs_steel: Handle<StandardMaterial>,
    pub mat_sky_obs_dark: Handle<StandardMaterial>,
    pub mat_sky_obs_hazard_amber: Handle<StandardMaterial>,
    pub mat_sky_obs_cyan_glow: Handle<StandardMaterial>,
    pub mat_sky_obs_headlight: Handle<StandardMaterial>,
    pub mat_sky_obs_red_beacon: Handle<StandardMaterial>,

    // Zone 6 Forbidden Line Dedicated Obstacles
    pub mesh_forbidden_gate_frame: Handle<Mesh>,
    pub mesh_forbidden_gate_plate: Handle<Mesh>,
    pub mesh_forbidden_gate_strobe: Handle<Mesh>,
    pub mesh_forbidden_gate_hazard_stripe: Handle<Mesh>,
    pub mesh_forbidden_conduit_bracket: Handle<Mesh>,
    pub mesh_forbidden_conduit_core: Handle<Mesh>,
    pub mesh_forbidden_conduit_spark: Handle<Mesh>,
    pub mesh_forbidden_pillar_shaft: Handle<Mesh>,
    pub mesh_forbidden_pillar_rib: Handle<Mesh>,
    pub mesh_forbidden_pillar_collar: Handle<Mesh>,
    pub mesh_forbidden_pillar_beacon: Handle<Mesh>,
    pub mesh_forbidden_transport_chassis: Handle<Mesh>,
    pub mesh_forbidden_transport_nose: Handle<Mesh>,
    pub mesh_forbidden_transport_skirt: Handle<Mesh>,
    pub mesh_forbidden_transport_marker: Handle<Mesh>,
    pub mesh_forbidden_mover_chassis: Handle<Mesh>,
    pub mesh_forbidden_mover_wedge: Handle<Mesh>,
    pub mesh_forbidden_mover_headlamp: Handle<Mesh>,
    pub mesh_forbidden_mover_strobe: Handle<Mesh>,
    pub mat_forbidden_obs_steel: Handle<StandardMaterial>,
    pub mat_forbidden_obs_concrete: Handle<StandardMaterial>,
    pub mat_forbidden_obs_crimson: Handle<StandardMaterial>,
    pub mat_forbidden_obs_amber: Handle<StandardMaterial>,
    pub mat_forbidden_obs_cyan: Handle<StandardMaterial>,
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
    pub industrial_barriers: Vec<Entity>,
    pub industrial_pipes: Vec<Entity>,
    pub industrial_pillars: Vec<Entity>,
    pub industrial_freight: Vec<Entity>,
    pub industrial_cargo: Vec<Entity>,
    pub flooded_barriers: Vec<Entity>,
    pub flooded_cables: Vec<Entity>,
    pub flooded_pillars: Vec<Entity>,
    pub flooded_freight: Vec<Entity>,
    pub flooded_cargo: Vec<Entity>,
    pub sky_rail_gates: Vec<Entity>,
    pub sky_rail_beams: Vec<Entity>,
    pub sky_rail_towers: Vec<Entity>,
    pub sky_rail_cars: Vec<Entity>,
    pub sky_rail_trains: Vec<Entity>,
    pub forbidden_barriers: Vec<Entity>,
    pub forbidden_conduits: Vec<Entity>,
    pub forbidden_pillars: Vec<Entity>,
    pub forbidden_transports: Vec<Entity>,
    pub forbidden_movers: Vec<Entity>,
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
            PoolType::IndustrialRoadBarrier => self.industrial_barriers.pop(),
            PoolType::IndustrialOverheadPipe => self.industrial_pipes.pop(),
            PoolType::IndustrialSupportPillar => self.industrial_pillars.pop(),
            PoolType::IndustrialFreightTransport => self.industrial_freight.pop(),
            PoolType::IndustrialCargoTransport => self.industrial_cargo.pop(),
            PoolType::FloodedBarrier => self.flooded_barriers.pop(),
            PoolType::FloodedPowerCable => self.flooded_cables.pop(),
            PoolType::FloodedPumpColumn => self.flooded_pillars.pop(),
            PoolType::FloodedSubmergedTrain => self.flooded_freight.pop(),
            PoolType::FloodedMovingTrain => self.flooded_cargo.pop(),
            PoolType::SkyRailMaintenanceGate => self.sky_rail_gates.pop(),
            PoolType::SkyRailClearanceBeam => self.sky_rail_beams.pop(),
            PoolType::SkyRailSignalTower => self.sky_rail_towers.pop(),
            PoolType::SkyRailDerelictMagCar => self.sky_rail_cars.pop(),
            PoolType::SkyRailMagTrain => self.sky_rail_trains.pop(),
            PoolType::ForbiddenQuarantineBarrier => self.forbidden_barriers.pop(),
            PoolType::ForbiddenConduitBundle => self.forbidden_conduits.pop(),
            PoolType::ForbiddenContainmentPillar => self.forbidden_pillars.pop(),
            PoolType::ForbiddenFacilityTransport => self.forbidden_transports.pop(),
            PoolType::ForbiddenHeavyMover => self.forbidden_movers.pop(),
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
            PoolType::IndustrialRoadBarrier => self.industrial_barriers.push(entity),
            PoolType::IndustrialOverheadPipe => self.industrial_pipes.push(entity),
            PoolType::IndustrialSupportPillar => self.industrial_pillars.push(entity),
            PoolType::IndustrialFreightTransport => self.industrial_freight.push(entity),
            PoolType::IndustrialCargoTransport => self.industrial_cargo.push(entity),
            PoolType::FloodedBarrier => self.flooded_barriers.push(entity),
            PoolType::FloodedPowerCable => self.flooded_cables.push(entity),
            PoolType::FloodedPumpColumn => self.flooded_pillars.push(entity),
            PoolType::FloodedSubmergedTrain => self.flooded_freight.push(entity),
            PoolType::FloodedMovingTrain => self.flooded_cargo.push(entity),
            PoolType::SkyRailMaintenanceGate => self.sky_rail_gates.push(entity),
            PoolType::SkyRailClearanceBeam => self.sky_rail_beams.push(entity),
            PoolType::SkyRailSignalTower => self.sky_rail_towers.push(entity),
            PoolType::SkyRailDerelictMagCar => self.sky_rail_cars.push(entity),
            PoolType::SkyRailMagTrain => self.sky_rail_trains.push(entity),
            PoolType::ForbiddenQuarantineBarrier => self.forbidden_barriers.push(entity),
            PoolType::ForbiddenConduitBundle => self.forbidden_conduits.push(entity),
            PoolType::ForbiddenContainmentPillar => self.forbidden_pillars.push(entity),
            PoolType::ForbiddenFacilityTransport => self.forbidden_transports.push(entity),
            PoolType::ForbiddenHeavyMover => self.forbidden_movers.push(entity),
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
            + self.industrial_barriers.len()
            + self.industrial_pipes.len()
            + self.industrial_pillars.len()
            + self.industrial_freight.len()
            + self.industrial_cargo.len()
            + self.flooded_barriers.len()
            + self.flooded_cables.len()
            + self.flooded_pillars.len()
            + self.flooded_freight.len()
            + self.flooded_cargo.len()
            + self.sky_rail_gates.len()
            + self.sky_rail_beams.len()
            + self.sky_rail_towers.len()
            + self.sky_rail_cars.len()
            + self.sky_rail_trains.len()
            + self.forbidden_barriers.len()
            + self.forbidden_conduits.len()
            + self.forbidden_pillars.len()
            + self.forbidden_transports.len()
            + self.forbidden_movers.len()
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
        mesh_neon_van_nose: meshes.add(Cuboid::new(1.92, 0.42, 1.10)),
        mesh_neon_van_hood: meshes.add(Cuboid::new(1.72, 0.12, 1.65)),
        mesh_neon_van_wheel: meshes.add(Cylinder::new(0.38, 0.20)),
        mesh_neon_van_rim: meshes.add(Cylinder::new(0.18, 0.22)),
        mesh_neon_van_panel: meshes.add(Cuboid::new(0.035, 0.55, 2.10)),
        mesh_neon_van_sensor: meshes.add(Cuboid::new(0.22, 0.16, 0.32)),
        mesh_neon_van_headlamp_unit: meshes.add(Cuboid::new(0.34, 0.16, 0.10)),
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
        mat_neon_van_glass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.03, 0.10, 0.16),
            metallic: 0.80,
            perceptual_roughness: 0.08,
            ..default()
        }),
        mat_neon_van_rubber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.015, 0.018, 0.025),
            perceptual_roughness: 0.88,
            metallic: 0.05,
            ..default()
        }),
        mat_neon_van_rim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.48, 0.58),
            metallic: 0.95,
            perceptual_roughness: 0.16,
            ..default()
        }),
        mat_neon_van_trim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.70, 0.82),
            emissive: LinearRgba::new(0.1, 1.2, 1.8, 1.0),
            metallic: 0.75,
            perceptual_roughness: 0.22,
            ..default()
        }),
        mat_neon_van_marking: materials.add(StandardMaterial {
            base_color: Color::srgb(0.78, 0.18, 0.78),
            emissive: LinearRgba::new(0.6, 0.05, 0.8, 1.0),
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
        mesh_neon_bus_nose: meshes.add(Cuboid::new(1.98, 0.42, 1.15)),
        mesh_neon_bus_wheel: meshes.add(Cylinder::new(0.48, 0.22)),
        mesh_neon_bus_rim: meshes.add(Cylinder::new(0.23, 0.24)),
        mesh_neon_bus_window_panel: meshes.add(Cuboid::new(0.06, 0.78, 1.72)),
        mesh_neon_bus_trim: meshes.add(Cuboid::new(0.08, 0.10, 11.60)),
        mesh_neon_bus_sensor: meshes.add(Cuboid::new(0.28, 0.18, 0.38)),
        mat_neon_bus_body: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.18, 0.25),
            metallic: 0.90,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_neon_bus_glass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.025, 0.09, 0.15),
            metallic: 0.82,
            perceptual_roughness: 0.08,
            ..default()
        }),
        mat_neon_bus_rubber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.012, 0.015, 0.022),
            perceptual_roughness: 0.90,
            metallic: 0.04,
            ..default()
        }),
        mat_neon_bus_rim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.38, 0.45, 0.56),
            metallic: 0.95,
            perceptual_roughness: 0.16,
            ..default()
        }),
        mat_neon_bus_trim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.75, 0.92),
            emissive: LinearRgba::new(0.15, 1.4, 2.0, 1.0),
            metallic: 0.70,
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

        // Zone 3 Industrial Sector Dedicated Obstacles
        mesh_industrial_barrier_frame: meshes.add(Cuboid::new(2.10, 0.34, 0.26)),
        mesh_industrial_barrier_stripe: meshes.add(Cuboid::new(2.00, 0.18, 0.28)),
        mesh_industrial_barrier_support: meshes.add(Cuboid::new(0.16, 0.74, 0.30)),
        mesh_industrial_pipe: meshes.add(Cuboid::new(2.20, 0.30, 0.35)),
        mesh_industrial_pipe_bundle: meshes.add(Cuboid::new(2.10, 0.16, 0.16)),
        mesh_industrial_clearance: meshes.add(Cuboid::new(1.70, 0.12, 0.06)),
        mesh_industrial_freight_body: meshes.add(Cuboid::new(2.20, 2.50, 12.0)),
        mesh_industrial_freight_cab: meshes.add(Cuboid::new(1.80, 0.86, 2.10)),
        mesh_industrial_freight_panel: meshes.add(Cuboid::new(0.06, 0.72, 2.40)),
        mesh_industrial_freight_wheel: meshes.add(Cylinder::new(0.38, 0.20)),
        mat_industrial_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.21, 0.21),
            metallic: 0.91,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_industrial_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.46, 0.04),
            emissive: LinearRgba::new(0.36, 0.12, 0.01, 1.0),
            metallic: 0.42,
            perceptual_roughness: 0.46,
            ..default()
        }),
        mat_industrial_rust: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.12, 0.06),
            metallic: 0.58,
            perceptual_roughness: 0.80,
            ..default()
        }),
        mat_industrial_pipe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.14, 0.14),
            metallic: 0.86,
            perceptual_roughness: 0.42,
            ..default()
        }),
        mat_industrial_warning: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.58, 0.08),
            emissive: LinearRgba::new(3.0, 0.78, 0.03, 1.0),
            ..default()
        }),
        mat_industrial_glass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.24, 0.26),
            emissive: LinearRgba::new(0.04, 0.35, 0.38, 1.0),
            metallic: 0.80,
            perceptual_roughness: 0.12,
            ..default()
        }),
        // Zone 4 Flooded Metro Dedicated Obstacles (Readability Pass - High-Visibility Jump Barrier)
        mesh_flooded_barrier_frame: meshes.add(Cuboid::new(2.15, 0.76, 0.30)),
        mesh_flooded_barrier_plate: meshes.add(Cuboid::new(2.10, 0.68, 0.12)),
        mesh_flooded_barrier_top_rail: meshes.add(Cuboid::new(2.25, 0.12, 0.22)),
        mesh_flooded_barrier_stripe: meshes.add(Cuboid::new(2.05, 0.16, 0.04)),
        mesh_flooded_barrier_skirt: meshes.add(Cuboid::new(2.15, 0.14, 0.28)),
        mesh_flooded_barrier_strobe: meshes.add(Sphere::new(0.14)),
        mesh_flooded_barrier_support: meshes.add(Cuboid::new(0.22, 0.82, 0.42)),
        mesh_flooded_pipe_column: meshes.add(Cylinder::new(0.58, 4.40)),
        mesh_flooded_column_collar: meshes.add(Cuboid::new(1.50, 0.75, 1.50)),
        mesh_flooded_column_valve: meshes.add(Cylinder::new(0.48, 0.14)),
        // Overhead Low-Clearance Hazard Gate
        mesh_flooded_cable_conduit: meshes.add(Cuboid::new(2.40, 0.38, 0.32)),
        mesh_flooded_cable_bundle: meshes.add(Cuboid::new(0.24, 2.10, 0.26)),
        mesh_flooded_cable_tag: meshes.add(Cuboid::new(2.20, 0.32, 0.08)),
        mesh_flooded_gate_strip: meshes.add(Cuboid::new(2.10, 0.10, 0.04)),
        mesh_flooded_gate_foot: meshes.add(Cuboid::new(0.40, 0.24, 0.40)),
        mesh_flooded_gate_tooth: meshes.add(Cuboid::new(0.12, 0.12, 0.04)),
        mesh_flooded_train_body: meshes.add(Cuboid::new(2.20, 2.50, 12.0)),
        mesh_flooded_train_roof: meshes.add(Cuboid::new(2.38, 0.38, 11.8)),
        mesh_flooded_train_window: meshes.add(Cuboid::new(0.06, 0.75, 2.40)),
        mesh_flooded_service_body: meshes.add(Cuboid::new(2.20, 0.60, 8.50)),
        mesh_flooded_service_cab: meshes.add(Cuboid::new(2.14, 1.70, 3.20)),
        mesh_flooded_service_light: meshes.add(Sphere::new(0.22)),
        mesh_empty_gondola_wall_side: meshes.add(Cuboid::new(0.12, 2.10, 11.8)),
        mesh_empty_gondola_wall_end: meshes.add(Cuboid::new(2.16, 2.10, 0.14)),
        mesh_empty_gondola_floor: meshes.add(Cuboid::new(2.16, 0.14, 11.8)),
        mesh_empty_gondola_rim: meshes.add(Cuboid::new(0.14, 0.10, 11.8)),
        mesh_empty_gondola_stake: meshes.add(Cuboid::new(0.08, 2.10, 0.14)),
        mesh_cargo_container_short: meshes.add(Cuboid::new(2.10, 1.55, 3.20)),
        mesh_cargo_crate_stack: meshes.add(Cuboid::new(0.95, 1.05, 1.45)),
        mesh_cargo_drum: meshes.add(Cylinder::new(0.32, 0.85)),
        mesh_cargo_cable_reel: meshes.add(Cylinder::new(0.65, 0.55)),
        mat_flooded_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.28, 0.32),
            metallic: 0.85,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_flooded_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.98, 0.74, 0.05),
            emissive: LinearRgba::new(0.80, 0.50, 0.02, 1.0),
            metallic: 0.10,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_flooded_rust: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.18, 0.10),
            metallic: 0.50,
            perceptual_roughness: 0.65,
            ..default()
        }),
        mat_flooded_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.50, 0.05),
            emissive: LinearRgba::new(4.5, 1.5, 0.05, 1.0),
            ..default()
        }),
        mat_flooded_cable: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.06, 0.07),
            metallic: 0.40,
            perceptual_roughness: 0.55,
            ..default()
        }),
        mat_cargo_container: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.32, 0.44),
            metallic: 0.65,
            perceptual_roughness: 0.38,
            ..default()
        }),
        mat_cargo_wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.36, 0.24, 0.15),
            perceptual_roughness: 0.85,
            ..default()
        }),

        // Zone 5 Sky Rail Dedicated Obstacle Meshes & Materials
        mesh_sky_gate_frame: meshes.add(Cuboid::new(2.05, 0.65, 0.20)),
        mesh_sky_gate_hazard_panel: meshes.add(Cuboid::new(1.95, 0.24, 0.06)),
        mesh_sky_gate_top_rail: meshes.add(Cuboid::new(2.08, 0.08, 0.12)),
        mesh_sky_gate_stanchion: meshes.add(Cuboid::new(0.12, 0.72, 0.24)),
        mesh_sky_gate_foot: meshes.add(Cuboid::new(0.24, 0.06, 0.36)),
        mesh_sky_gate_sensor: meshes.add(Cuboid::new(0.28, 0.16, 0.10)),

        mesh_sky_beam_body: meshes.add(Cuboid::new(2.28, 0.32, 0.32)),
        mesh_sky_beam_strut: meshes.add(Cuboid::new(0.14, 0.60, 0.24)),
        mesh_sky_beam_plate: meshes.add(Cuboid::new(2.08, 0.16, 0.08)),
        mesh_sky_beam_conduit: meshes.add(Cuboid::new(2.15, 0.06, 0.08)),
        mesh_sky_beam_tooth: meshes.add(Cuboid::new(0.20, 0.18, 0.06)),

        mesh_sky_tower_core: meshes.add(Cuboid::new(1.45, 3.50, 0.90)),
        mesh_sky_tower_pedestal: meshes.add(Cuboid::new(1.70, 0.45, 1.10)),
        mesh_sky_tower_collar: meshes.add(Cuboid::new(1.55, 0.25, 1.00)),
        mesh_sky_tower_truss: meshes.add(Cuboid::new(0.12, 3.20, 0.20)),
        mesh_sky_tower_beacon: meshes.add(Cylinder::new(0.08, 0.40)),
        mesh_sky_tower_placard: meshes.add(Cuboid::new(1.10, 0.80, 0.06)),

        mesh_sky_mag_car_body: meshes.add(Cuboid::new(2.15, 2.15, 10.5)),
        mesh_sky_mag_car_nose: meshes.add(Cuboid::new(2.05, 1.60, 1.8)),
        mesh_sky_mag_car_skirt: meshes.add(Cuboid::new(0.12, 0.22, 11.2)),
        mesh_sky_mag_car_windscreen: meshes.add(Cuboid::new(1.75, 0.75, 0.08)),
        mesh_sky_mag_car_window_strip: meshes.add(Cuboid::new(0.06, 0.32, 9.5)),

        mesh_sky_mag_train_body: meshes.add(Cuboid::new(2.18, 2.20, 12.5)),
        mesh_sky_mag_train_nose: meshes.add(Cuboid::new(2.08, 1.65, 2.4)),
        mesh_sky_mag_train_windscreen: meshes.add(Cuboid::new(1.80, 0.80, 0.08)),
        mesh_sky_mag_train_skirt: meshes.add(Cuboid::new(0.12, 0.24, 13.5)),
        mesh_sky_mag_train_fin: meshes.add(Cuboid::new(0.08, 0.35, 1.8)),
        mesh_sky_mag_train_headlamp: meshes.add(Cuboid::new(0.24, 0.16, 0.08)),

        mat_sky_obs_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.15, 0.20),
            metallic: 0.88,
            perceptual_roughness: 0.35,
            ..default()
        }),
        mat_sky_obs_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.08, 0.11),
            metallic: 0.75,
            perceptual_roughness: 0.45,
            ..default()
        }),
        mat_sky_obs_hazard_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.52, 0.06),
            emissive: LinearRgba::new(2.8, 1.2, 0.08, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_sky_obs_cyan_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.65, 0.82),
            emissive: LinearRgba::new(0.35, 1.85, 2.4, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_sky_obs_headlight: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.95, 1.0),
            emissive: LinearRgba::new(3.5, 5.0, 6.5, 1.0),
            ..default()
        }),
        mat_sky_obs_red_beacon: materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.06, 0.06),
            emissive: LinearRgba::new(2.4, 0.12, 0.08, 1.0),
            ..default()
        }),

        // Zone 6 Forbidden Line Dedicated Obstacles
        mesh_forbidden_gate_frame: meshes.add(Cuboid::new(2.10, 0.85, 0.35)),
        mesh_forbidden_gate_plate: meshes.add(Cuboid::new(1.88, 0.60, 0.12)),
        mesh_forbidden_gate_strobe: meshes.add(Sphere::new(0.12)),
        mesh_forbidden_gate_hazard_stripe: meshes.add(Cuboid::new(1.78, 0.10, 0.04)),

        mesh_forbidden_conduit_bracket: meshes.add(Cuboid::new(2.24, 0.22, 0.35)),
        mesh_forbidden_conduit_core: meshes.add(Cuboid::new(2.18, 0.28, 0.22)),
        mesh_forbidden_conduit_spark: meshes.add(Sphere::new(0.08)),

        mesh_forbidden_pillar_shaft: meshes.add(Cuboid::new(1.78, 3.95, 1.18)),
        mesh_forbidden_pillar_rib: meshes.add(Cuboid::new(1.98, 3.80, 0.22)),
        mesh_forbidden_pillar_collar: meshes.add(Cuboid::new(2.05, 0.40, 1.35)),
        mesh_forbidden_pillar_beacon: meshes.add(Sphere::new(0.15)),

        mesh_forbidden_transport_chassis: meshes.add(Cuboid::new(2.18, 2.45, 11.8)),
        mesh_forbidden_transport_nose: meshes.add(Cuboid::new(2.08, 1.95, 1.8)),
        mesh_forbidden_transport_skirt: meshes.add(Cuboid::new(0.12, 0.25, 11.2)),
        mesh_forbidden_transport_marker: meshes.add(Sphere::new(0.16)),

        mesh_forbidden_mover_chassis: meshes.add(Cuboid::new(2.18, 2.45, 13.8)),
        mesh_forbidden_mover_wedge: meshes.add(Cuboid::new(2.10, 2.15, 2.2)),
        mesh_forbidden_mover_headlamp: meshes.add(Cuboid::new(0.35, 0.22, 0.12)),
        mesh_forbidden_mover_strobe: meshes.add(Sphere::new(0.16)),

        mat_forbidden_obs_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.10),
            metallic: 0.88,
            perceptual_roughness: 0.32,
            ..default()
        }),
        mat_forbidden_obs_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.12, 0.15),
            metallic: 0.12,
            perceptual_roughness: 0.88,
            ..default()
        }),
        mat_forbidden_obs_crimson: materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.04, 0.08),
            emissive: LinearRgba::new(3.8, 0.08, 0.14, 1.0),
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_forbidden_obs_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.55, 0.04),
            emissive: LinearRgba::new(2.4, 1.1, 0.05, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_forbidden_obs_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.55, 0.75),
            emissive: LinearRgba::new(0.08, 2.0, 2.4, 1.0),
            perceptual_roughness: 0.18,
            ..default()
        }),
    };

    commands.insert_resource(pool_assets);
}
