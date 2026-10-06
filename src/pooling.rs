use bevy::prelude::*;
use crate::types::*;

#[derive(Resource)]
pub struct PoolAssets {
    // Meshes
    pub mesh_barrier: Handle<Mesh>,
    pub mesh_wire: Handle<Mesh>,
    pub mesh_pillar: Handle<Mesh>,
    pub mesh_train_static: Handle<Mesh>,
    pub mesh_train_moving: Handle<Mesh>,
    pub mesh_train_light: Handle<Mesh>,
    pub mesh_fragment: Handle<Mesh>,
    pub mesh_chip: Handle<Mesh>,
    pub mesh_powerup: Handle<Mesh>,

    // Materials
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
}

#[derive(Resource, Default)]
pub struct EntityPool {
    pub barriers: Vec<Entity>,
    pub wires: Vec<Entity>,
    pub pillars: Vec<Entity>,
    pub static_trains: Vec<Entity>,
    pub moving_trains: Vec<Entity>,
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

        mat_hazard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.45, 0.1),
            emissive: LinearRgba::new(1.6, 0.5, 0.0, 1.0),
            perceptual_roughness: 0.3,
            ..default()
        }),
        mat_pillar: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.22, 0.26),
            metallic: 0.8,
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_wire: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.1),
            emissive: LinearRgba::new(1.8, 1.3, 0.0, 1.0),
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
            base_color: Color::srgb(0.0, 0.9, 1.0),
            emissive: LinearRgba::new(0.6, 2.5, 3.5, 1.0),
            perceptual_roughness: 0.1,
            metallic: 0.9,
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
            base_color: Color::srgb(0.1, 0.6, 1.0),
            emissive: LinearRgba::new(0.5, 1.8, 3.5, 1.0),
            ..default()
        }),
        mat_powerup_magnet: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.1, 0.9),
            emissive: LinearRgba::new(2.8, 0.2, 2.8, 1.0),
            ..default()
        }),
        mat_powerup_overdrive: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            emissive: LinearRgba::new(3.5, 1.2, 0.0, 1.0),
            ..default()
        }),
    };

    commands.insert_resource(pool_assets);
}
