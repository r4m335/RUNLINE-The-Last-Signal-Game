use crate::environment::EnvironmentAssets;
use crate::types::*;
use bevy::prelude::*;

/// Authored visual chapters for Zone 4 (6,000m–8,000m).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodedMetroChapter {
    FloodgateDescent,
    SubmergedStation,
    ReservoirCrossing,
    SunkenTransit,
    DeepLineApproach,
}

pub fn flooded_metro_chapter_for_distance(distance: f32) -> FloodedMetroChapter {
    match distance {
        d if d < 6300.0 => FloodedMetroChapter::FloodgateDescent,
        d if d < 6750.0 => FloodedMetroChapter::SubmergedStation,
        d if d < 7200.0 => FloodedMetroChapter::ReservoirCrossing,
        d if d < 7650.0 => FloodedMetroChapter::SunkenTransit,
        _ => FloodedMetroChapter::DeepLineApproach,
    }
}

fn flooded_metro_variant(distance: f32) -> u8 {
    let segment_index = (distance.max(0.0) / 40.0).floor() as i32;
    (segment_index.rem_euclid(3)) as u8
}

#[derive(Resource)]
pub struct FloodedMetroAssets {
    pub mesh_wall: Handle<Mesh>,
    pub mesh_support: Handle<Mesh>,
    pub mesh_beam: Handle<Mesh>,
    pub mesh_gate_leaf: Handle<Mesh>,
    pub mesh_pipe: Handle<Mesh>,
    pub mesh_platform: Handle<Mesh>,
    pub mesh_arch_column: Handle<Mesh>,
    pub mesh_bridge: Handle<Mesh>,
    pub mesh_railing: Handle<Mesh>,
    pub mesh_water: Handle<Mesh>,
    pub mesh_train_body: Handle<Mesh>,
    pub mesh_train_roof: Handle<Mesh>,
    pub mesh_train_window: Handle<Mesh>,
    pub mesh_train_door: Handle<Mesh>,
    pub mesh_bulkhead: Handle<Mesh>,
    pub mesh_bulkhead_door: Handle<Mesh>,
    pub mesh_warning_panel: Handle<Mesh>,
    pub mesh_fixture: Handle<Mesh>,
    pub mesh_equipment: Handle<Mesh>,
    pub mat_concrete: Handle<StandardMaterial>,
    pub mat_wet_concrete: Handle<StandardMaterial>,
    pub mat_steel: Handle<StandardMaterial>,
    pub mat_rusted_steel: Handle<StandardMaterial>,
    pub mat_teal: Handle<StandardMaterial>,
    pub mat_water: Handle<StandardMaterial>,
    pub mat_train: Handle<StandardMaterial>,
    pub mat_train_window: Handle<StandardMaterial>,
    pub mat_warning: Handle<StandardMaterial>,
    pub mat_fixture: Handle<StandardMaterial>,
}

pub fn init_flooded_metro_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let assets = FloodedMetroAssets {
        mesh_wall: meshes.add(Cuboid::new(0.42, 7.8, 40.0)),
        mesh_support: meshes.add(Cuboid::new(0.72, 7.4, 0.72)),
        mesh_beam: meshes.add(Cuboid::new(16.0, 0.48, 0.72)),
        mesh_gate_leaf: meshes.add(Cuboid::new(0.34, 5.7, 4.8)),
        mesh_pipe: meshes.add(Cylinder::new(0.28, 40.0)),
        mesh_platform: meshes.add(Cuboid::new(4.0, 0.36, 40.0)),
        mesh_arch_column: meshes.add(Cuboid::new(0.34, 5.8, 0.44)),
        mesh_bridge: meshes.add(Cuboid::new(12.0, 0.28, 7.0)),
        mesh_railing: meshes.add(Cuboid::new(0.12, 0.72, 7.0)),
        mesh_water: meshes.add(Cuboid::new(8.5, 0.08, 40.0)),
        mesh_train_body: meshes.add(Cuboid::new(3.3, 2.8, 13.0)),
        mesh_train_roof: meshes.add(Cuboid::new(3.5, 0.28, 12.5)),
        mesh_train_window: meshes.add(Cuboid::new(0.07, 0.82, 1.7)),
        mesh_train_door: meshes.add(Cuboid::new(0.08, 2.0, 1.15)),
        mesh_bulkhead: meshes.add(Cuboid::new(0.52, 7.3, 0.74)),
        mesh_bulkhead_door: meshes.add(Cuboid::new(0.30, 5.7, 5.2)),
        mesh_warning_panel: meshes.add(Cuboid::new(0.10, 1.1, 2.6)),
        mesh_fixture: meshes.add(Cuboid::new(0.16, 0.16, 0.16)),
        mesh_equipment: meshes.add(Cuboid::new(1.8, 1.4, 2.4)),
        mat_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.16, 0.18),
            perceptual_roughness: 0.88,
            ..default()
        }),
        mat_wet_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.13, 0.16),
            metallic: 0.22,
            perceptual_roughness: 0.48,
            ..default()
        }),
        mat_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.22, 0.24),
            metallic: 0.88,
            perceptual_roughness: 0.34,
            ..default()
        }),
        mat_rusted_steel: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.17, 0.13),
            metallic: 0.62,
            perceptual_roughness: 0.74,
            ..default()
        }),
        mat_teal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.45, 0.52),
            emissive: LinearRgba::new(0.03, 0.75, 0.90, 1.0),
            metallic: 0.30,
            perceptual_roughness: 0.28,
            ..default()
        }),
        mat_water: materials.add(StandardMaterial {
            base_color: Color::srgb(0.015, 0.18, 0.22),
            emissive: LinearRgba::new(0.0, 0.10, 0.14, 1.0),
            metallic: 0.18,
            perceptual_roughness: 0.20,
            ..default()
        }),
        mat_train: materials.add(StandardMaterial {
            base_color: Color::srgb(0.09, 0.12, 0.14),
            metallic: 0.72,
            perceptual_roughness: 0.42,
            ..default()
        }),
        mat_train_window: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.09, 0.11),
            emissive: LinearRgba::new(0.01, 0.24, 0.30, 1.0),
            metallic: 0.25,
            perceptual_roughness: 0.18,
            ..default()
        }),
        mat_warning: materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.35, 0.08),
            emissive: LinearRgba::new(0.80, 0.16, 0.02, 1.0),
            perceptual_roughness: 0.44,
            ..default()
        }),
        mat_fixture: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.95, 1.0),
            emissive: LinearRgba::new(0.20, 1.6, 2.2, 1.0),
            ..default()
        }),
    };

    commands.insert_resource(assets);
}

pub fn spawn_flooded_metro_segment(
    commands: &mut Commands,
    env: &EnvironmentAssets,
    flooded: &FloodedMetroAssets,
    z_center: f32,
    length: f32,
    distance: f32,
) -> Entity {
    let chapter = flooded_metro_chapter_for_distance(distance);
    let variant = flooded_metro_variant(distance);

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, z_center)),
            TrackSegmentMarker,
            Despawnable { z_center },
        ))
        .with_children(|seg| {
            spawn_flooded_track(seg, env, length);
            match chapter {
                FloodedMetroChapter::FloodgateDescent => {
                    spawn_floodgate_descent(seg, flooded, variant)
                }
                FloodedMetroChapter::SubmergedStation => {
                    spawn_submerged_station(seg, flooded, variant)
                }
                FloodedMetroChapter::ReservoirCrossing => {
                    spawn_reservoir_crossing(seg, flooded, variant)
                }
                FloodedMetroChapter::SunkenTransit => spawn_sunken_transit(seg, flooded, variant),
                FloodedMetroChapter::DeepLineApproach => {
                    spawn_deep_line_approach(seg, flooded, variant)
                }
            }
        })
        .id()
}

fn spawn_flooded_track(seg: &mut ChildBuilder, env: &EnvironmentAssets, length: f32) {
    seg.spawn(PbrBundle {
        mesh: env.mesh_bed.clone(),
        material: env.mat_concrete_bed.clone(),
        transform: Transform::from_xyz(0.0, -0.225, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_drain_trough.clone(),
        material: env.mat_drainage.clone(),
        transform: Transform::from_xyz(0.0, -0.05, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_curb_shoulder.clone(),
        material: env.mat_ballast_curb.clone(),
        transform: Transform::from_xyz(-4.2, 0.12, 0.0),
        ..default()
    });
    seg.spawn(PbrBundle {
        mesh: env.mesh_curb_shoulder.clone(),
        material: env.mat_ballast_curb.clone(),
        transform: Transform::from_xyz(4.2, 0.12, 0.0),
        ..default()
    });

    let sleeper_count = (length / 4.0) as i32;
    for i in -sleeper_count / 2..=sleeper_count / 2 {
        seg.spawn(PbrBundle {
            mesh: env.mesh_sleeper_heavy.clone(),
            material: env.mat_sleeper_stained.clone(),
            transform: Transform::from_xyz(0.0, 0.01, i as f32 * 4.0),
            ..default()
        });
    }

    for lane_idx in -1..=1 {
        let x = Lane::from_index(lane_idx).x_pos();
        for rail_x in [x - 0.48, x + 0.48] {
            seg.spawn(PbrBundle {
                mesh: env.mesh_rail_base.clone(),
                material: env.mat_rail_base.clone(),
                transform: Transform::from_xyz(rail_x, 0.04, 0.0),
                ..default()
            });
            seg.spawn(PbrBundle {
                mesh: env.mesh_rail_crown.clone(),
                material: env.mat_rail_crown.clone(),
                transform: Transform::from_xyz(rail_x, 0.09, 0.0),
                ..default()
            });
        }
        for i in -4..=4 {
            seg.spawn(PbrBundle {
                mesh: env.mesh_lane_stripe.clone(),
                material: env.mat_echo_conduit.clone(),
                transform: Transform::from_xyz(x, 0.13, i as f32 * 4.5),
                ..default()
            });
        }
    }
}

fn spawn_floodgate_descent(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    spawn_side_walls(seg, a, 0.0);
    for x in [-6.0, 6.0] {
        for z in [-14.0, 14.0] {
            spawn_mesh(
                seg,
                &a.mesh_support,
                &a.mat_concrete,
                Transform::from_xyz(x, 3.7, z),
            );
        }
    }
    spawn_mesh(
        seg,
        &a.mesh_beam,
        &a.mat_concrete,
        Transform::from_xyz(0.0, 7.2, 0.0),
    );
    for x in [-3.45, 3.45] {
        spawn_mesh(
            seg,
            &a.mesh_gate_leaf,
            &a.mat_steel,
            Transform::from_xyz(x, 2.8, -12.0),
        );
    }
    spawn_side_pipes(seg, a, 3.0);
    spawn_waterline(seg, a, 0.0);
    if variant == 0 {
        spawn_warning_pair(seg, a, -8.0);
        spawn_fixture_pair(seg, a, 5.8, -5.0);
    }
}

fn spawn_submerged_station(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    let side = if variant % 2 == 0 { 1.0 } else { -1.0 };
    spawn_side_walls(seg, a, 0.0);
    spawn_mesh(
        seg,
        &a.mesh_platform,
        &a.mat_wet_concrete,
        Transform::from_xyz(side * 6.4, 0.65, 0.0),
    );
    for z in [-14.0, 0.0, 14.0] {
        spawn_mesh(
            seg,
            &a.mesh_arch_column,
            &a.mat_concrete,
            Transform::from_xyz(side * 4.9, 3.2, z),
        );
    }
    spawn_mesh(
        seg,
        &a.mesh_beam,
        &a.mat_concrete,
        Transform::from_xyz(0.0, 6.5, 0.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_water,
        &a.mat_water,
        Transform::from_xyz(-side * 8.5, -0.02, 0.0),
    );
    spawn_warning_pair(seg, a, 9.0);
    spawn_fixture_pair(seg, a, 5.8, 0.0);
    if variant == 1 {
        spawn_mesh(
            seg,
            &a.mesh_bridge,
            &a.mat_steel,
            Transform::from_xyz(0.0, 5.0, -8.0),
        );
        spawn_mesh(
            seg,
            &a.mesh_railing,
            &a.mat_rusted_steel,
            Transform::from_xyz(0.0, 5.42, -8.0),
        );
    }
}

fn spawn_reservoir_crossing(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    spawn_side_walls(seg, a, 2.0);
    for x in [-8.0, 8.0, -11.0, 11.0] {
        spawn_mesh(
            seg,
            &a.mesh_support,
            &a.mat_concrete,
            Transform::from_xyz(x, 3.6, if x.abs() > 9.0 { 10.0 } else { -10.0 }),
        );
    }
    for side in [-1.0, 1.0] {
        spawn_mesh(
            seg,
            &a.mesh_water,
            &a.mat_water,
            Transform::from_xyz(side * 8.5, -0.02, 0.0),
        );
    }
    spawn_mesh(
        seg,
        &a.mesh_bridge,
        &a.mat_steel,
        Transform::from_xyz(0.0, 5.4, 6.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_railing,
        &a.mat_rusted_steel,
        Transform::from_xyz(0.0, 5.82, 6.0),
    );
    spawn_mesh(
        seg,
        &a.mesh_equipment,
        &a.mat_rusted_steel,
        Transform::from_xyz(if variant == 0 { -6.6 } else { 6.6 }, 1.0, -7.0),
    );
    spawn_side_pipes(seg, a, 4.2);
    spawn_fixture_pair(seg, a, 6.8, 0.0);
}

fn spawn_sunken_transit(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    let side = if variant == 2 { -1.0 } else { 1.0 };
    spawn_side_walls(seg, a, 4.0);
    spawn_mesh(
        seg,
        &a.mesh_platform,
        &a.mat_wet_concrete,
        Transform::from_xyz(side * 6.4, 0.55, 0.0),
    );
    spawn_train_car(seg, a, side, -4.0);
    if variant != 1 {
        spawn_train_car(seg, a, -side, 13.0);
    }
    for z in [-14.0, 14.0] {
        spawn_mesh(
            seg,
            &a.mesh_support,
            &a.mat_rusted_steel,
            Transform::from_xyz(side * 5.0, 3.5, z),
        );
    }
    spawn_side_pipes(seg, a, 2.6);
    spawn_mesh(
        seg,
        &a.mesh_water,
        &a.mat_water,
        Transform::from_xyz(-side * 8.6, -0.01, 0.0),
    );
    spawn_warning_pair(seg, a, -10.0);
}

fn spawn_deep_line_approach(seg: &mut ChildBuilder, a: &FloodedMetroAssets, variant: u8) {
    spawn_side_walls(seg, a, 6.0);
    for x in [-6.3, 6.3] {
        spawn_mesh(
            seg,
            &a.mesh_bulkhead,
            &a.mat_steel,
            Transform::from_xyz(x, 3.7, -12.0),
        );
    }
    for x in [-3.0, 3.0] {
        spawn_mesh(
            seg,
            &a.mesh_bulkhead_door,
            &a.mat_wet_concrete,
            Transform::from_xyz(x, 2.8, -12.0),
        );
    }
    spawn_mesh(
        seg,
        &a.mesh_beam,
        &a.mat_steel,
        Transform::from_xyz(0.0, 7.0, -12.0),
    );
    spawn_side_pipes(seg, a, 3.8);
    spawn_waterline(seg, a, 3.0);
    spawn_fixture_pair(seg, a, 6.4, -12.0);
    if variant == 0 {
        spawn_warning_pair(seg, a, -12.0);
    }
}

fn spawn_side_walls(seg: &mut ChildBuilder, a: &FloodedMetroAssets, offset: f32) {
    for x in [-9.0, 9.0] {
        spawn_mesh(
            seg,
            &a.mesh_wall,
            &a.mat_wet_concrete,
            Transform::from_xyz(x, 3.8, offset),
        );
    }
}

fn spawn_side_pipes(seg: &mut ChildBuilder, a: &FloodedMetroAssets, y: f32) {
    for x in [-7.2, 7.2] {
        spawn_mesh(
            seg,
            &a.mesh_pipe,
            &a.mat_rusted_steel,
            Transform::from_xyz(x, y, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        );
    }
}

fn spawn_waterline(seg: &mut ChildBuilder, a: &FloodedMetroAssets, y: f32) {
    for x in [-8.78, 8.78] {
        spawn_mesh(
            seg,
            &a.mesh_warning_panel,
            &a.mat_teal,
            Transform::from_xyz(x, y + 1.1, 0.0),
        );
    }
}

fn spawn_warning_pair(seg: &mut ChildBuilder, a: &FloodedMetroAssets, z: f32) {
    for x in [-7.75, 7.75] {
        spawn_mesh(
            seg,
            &a.mesh_warning_panel,
            &a.mat_warning,
            Transform::from_xyz(x, 2.4, z),
        );
    }
}

fn spawn_fixture_pair(seg: &mut ChildBuilder, a: &FloodedMetroAssets, y: f32, z: f32) {
    for x in [-5.3, 5.3] {
        spawn_mesh(
            seg,
            &a.mesh_fixture,
            &a.mat_fixture,
            Transform::from_xyz(x, y, z),
        );
    }
}

fn spawn_train_car(seg: &mut ChildBuilder, a: &FloodedMetroAssets, side: f32, z: f32) {
    let x = side * 8.4;
    let inward = -side;
    spawn_mesh(
        seg,
        &a.mesh_train_body,
        &a.mat_train,
        Transform::from_xyz(x, 2.1, z),
    );
    spawn_mesh(
        seg,
        &a.mesh_train_roof,
        &a.mat_steel,
        Transform::from_xyz(x, 3.58, z),
    );
    for window_z in [-4.0, 0.0, 4.0] {
        spawn_mesh(
            seg,
            &a.mesh_train_window,
            &a.mat_train_window,
            Transform::from_xyz(x + inward * 1.67, 2.35, z + window_z),
        );
    }
    for door_z in [-6.0, 6.0] {
        spawn_mesh(
            seg,
            &a.mesh_train_door,
            &a.mat_rusted_steel,
            Transform::from_xyz(x + inward * 1.68, 1.7, z + door_z),
        );
    }
}

fn spawn_mesh(
    seg: &mut ChildBuilder,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    transform: Transform,
) {
    seg.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform,
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flooded_metro_chapters_follow_authored_boundaries() {
        assert_eq!(
            flooded_metro_chapter_for_distance(6000.0),
            FloodedMetroChapter::FloodgateDescent
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6299.99),
            FloodedMetroChapter::FloodgateDescent
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6300.0),
            FloodedMetroChapter::SubmergedStation
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(6750.0),
            FloodedMetroChapter::ReservoirCrossing
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(7200.0),
            FloodedMetroChapter::SunkenTransit
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(7650.0),
            FloodedMetroChapter::DeepLineApproach
        );
        assert_eq!(
            flooded_metro_chapter_for_distance(8000.0),
            FloodedMetroChapter::DeepLineApproach
        );
    }

    #[test]
    fn flooded_metro_variants_are_deterministic_and_bounded() {
        for distance in [6000.0, 6300.0, 6750.0, 7200.0, 7650.0, 7999.0] {
            assert_eq!(
                flooded_metro_variant(distance),
                flooded_metro_variant(distance)
            );
            assert!(flooded_metro_variant(distance) < 3);
        }
    }
}
