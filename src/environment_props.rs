use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct PropAssets {
    // -------------------------------------------------------------
    // MESHES
    // -------------------------------------------------------------
    // Pipes & Conduits
    pub mesh_pipe_large: Handle<Mesh>,
    pub mesh_pipe_medium: Handle<Mesh>,
    pub mesh_pipe_small: Handle<Mesh>,
    pub mesh_pipe_flange: Handle<Mesh>,
    pub mesh_pipe_bracket: Handle<Mesh>,

    // Subway Wall Tiles (4 Variants)
    pub mesh_tile_panel_normal: Handle<Mesh>,
    pub mesh_tile_panel_dirty: Handle<Mesh>,
    pub mesh_tile_panel_cracked: Handle<Mesh>,
    pub mesh_tile_panel_broken: Handle<Mesh>,

    // Station Furniture
    pub mesh_bench_seat: Handle<Mesh>,
    pub mesh_bench_legs: Handle<Mesh>,
    pub mesh_timetable_board: Handle<Mesh>,
    pub mesh_ticket_machine: Handle<Mesh>,
    pub mesh_emergency_phone: Handle<Mesh>,
    pub mesh_trash_bin: Handle<Mesh>,
    pub mesh_station_clock: Handle<Mesh>,

    // Wall Signs & Storytelling Posters
    pub mesh_entrance_sign: Handle<Mesh>,
    pub mesh_station_nameplate: Handle<Mesh>,
    pub mesh_directional_sign: Handle<Mesh>,
    pub mesh_poster_board: Handle<Mesh>,
    pub mesh_graffiti_panel: Handle<Mesh>,
    pub mesh_security_placard: Handle<Mesh>,
    pub mesh_checkpoint_gantry: Handle<Mesh>,
    pub mesh_checkpoint_scanner: Handle<Mesh>,

    // Heavy Props & Vehicles
    pub mesh_derelict_carriage: Handle<Mesh>,
    pub mesh_carriage_window: Handle<Mesh>,
    pub mesh_carriage_door: Handle<Mesh>,
    pub mesh_maintenance_cart: Handle<Mesh>,
    pub mesh_cart_wheel: Handle<Mesh>,
    pub mesh_toolbox: Handle<Mesh>,
    pub mesh_cable_spool: Handle<Mesh>,
    pub mesh_transformer_body: Handle<Mesh>,
    pub mesh_transformer_fin: Handle<Mesh>,
    pub mesh_insulator_bushing: Handle<Mesh>,
    pub mesh_breaker_cabinet: Handle<Mesh>,

    // Structural Collapse & Debris
    pub mesh_rubble_chunk_large: Handle<Mesh>,
    pub mesh_rubble_chunk_med: Handle<Mesh>,
    pub mesh_rubble_chunk_small: Handle<Mesh>,
    pub mesh_bent_rebar: Handle<Mesh>,
    pub mesh_emergency_shore_jack: Handle<Mesh>,

    // Surveillance & Security
    pub mesh_camera_mount: Handle<Mesh>,
    pub mesh_camera_housing: Handle<Mesh>,
    pub mesh_camera_lens: Handle<Mesh>,
    pub mesh_security_barrier: Handle<Mesh>,

    // VFX Geometry
    pub mesh_steam_plume: Handle<Mesh>,
    pub mesh_electric_arc: Handle<Mesh>,

    // -------------------------------------------------------------
    // MATERIALS
    // -------------------------------------------------------------
    pub mat_tile_normal: Handle<StandardMaterial>,
    pub mat_tile_dirty: Handle<StandardMaterial>,
    pub mat_tile_cracked: Handle<StandardMaterial>,
    pub mat_tile_broken: Handle<StandardMaterial>,

    pub mat_pipe_dark: Handle<StandardMaterial>,
    pub mat_pipe_red: Handle<StandardMaterial>,
    pub mat_pipe_yellow: Handle<StandardMaterial>,
    pub mat_pipe_blue: Handle<StandardMaterial>,
    pub mat_pipe_flange: Handle<StandardMaterial>,

    pub mat_bench_wood: Handle<StandardMaterial>,
    pub mat_poster_veyron: Handle<StandardMaterial>,
    pub mat_graffiti_cyan: Handle<StandardMaterial>,
    pub mat_station_sign_amber: Handle<StandardMaterial>,
    pub mat_station_sign_flicker: Handle<StandardMaterial>,

    pub mat_derelict_train: Handle<StandardMaterial>,
    pub mat_train_window_dark: Handle<StandardMaterial>,
    pub mat_train_window_lit: Handle<StandardMaterial>,

    pub mat_transformer_metal: Handle<StandardMaterial>,
    pub mat_ceramic_insulator: Handle<StandardMaterial>,
    pub mat_rubble_concrete: Handle<StandardMaterial>,
    pub mat_bent_rebar: Handle<StandardMaterial>,

    pub mat_security_black: Handle<StandardMaterial>,
    pub mat_security_red_led: Handle<StandardMaterial>,
    pub mat_emergency_red_light: Handle<StandardMaterial>,

    pub mat_steam: Handle<StandardMaterial>,
    pub mat_arc_spark: Handle<StandardMaterial>,
}

pub fn init_prop_assets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let length = 40.0;

    let props = PropAssets {
        // Pipes
        mesh_pipe_large: meshes.add(Cylinder::new(0.18, length)),
        mesh_pipe_medium: meshes.add(Cylinder::new(0.09, length)),
        mesh_pipe_small: meshes.add(Cylinder::new(0.04, length)),
        mesh_pipe_flange: meshes.add(Cylinder::new(0.24, 0.08)),
        mesh_pipe_bracket: meshes.add(Cuboid::new(0.12, 0.45, 0.12)),

        // Subway Tiles
        mesh_tile_panel_normal: meshes.add(Cuboid::new(0.06, 2.2, 3.8)),
        mesh_tile_panel_dirty: meshes.add(Cuboid::new(0.06, 2.2, 3.8)),
        mesh_tile_panel_cracked: meshes.add(Cuboid::new(0.06, 2.2, 3.8)),
        mesh_tile_panel_broken: meshes.add(Cuboid::new(0.06, 2.2, 3.8)),

        // Station Furniture
        mesh_bench_seat: meshes.add(Cuboid::new(0.65, 0.08, 2.2)),
        mesh_bench_legs: meshes.add(Cuboid::new(0.55, 0.45, 0.12)),
        mesh_timetable_board: meshes.add(Cuboid::new(0.08, 1.4, 2.4)),
        mesh_ticket_machine: meshes.add(Cuboid::new(0.7, 1.8, 0.8)),
        mesh_emergency_phone: meshes.add(Cuboid::new(0.25, 0.55, 0.2)),
        mesh_trash_bin: meshes.add(Cylinder::new(0.22, 0.75)),
        mesh_station_clock: meshes.add(Cylinder::new(0.42, 0.18)),

        // Wall Signs & Posters
        mesh_entrance_sign: meshes.add(Cuboid::new(6.8, 1.1, 0.15)),
        mesh_station_nameplate: meshes.add(Cuboid::new(0.08, 0.75, 3.2)),
        mesh_directional_sign: meshes.add(Cuboid::new(0.08, 0.55, 2.8)),
        mesh_poster_board: meshes.add(Cuboid::new(0.04, 1.8, 1.3)),
        mesh_graffiti_panel: meshes.add(Cuboid::new(0.04, 1.4, 2.0)),
        mesh_security_placard: meshes.add(Cuboid::new(0.04, 0.8, 1.1)),
        mesh_checkpoint_gantry: meshes.add(Cuboid::new(9.6, 1.4, 1.6)),
        mesh_checkpoint_scanner: meshes.add(Cuboid::new(0.8, 4.8, 0.8)),

        // Heavy Props & Vehicles
        mesh_derelict_carriage: meshes.add(Cuboid::new(2.8, 3.2, 14.0)),
        mesh_carriage_window: meshes.add(Cuboid::new(0.04, 0.9, 1.6)),
        mesh_carriage_door: meshes.add(Cuboid::new(0.05, 2.0, 1.2)),
        mesh_maintenance_cart: meshes.add(Cuboid::new(1.8, 0.35, 2.6)),
        mesh_cart_wheel: meshes.add(Cylinder::new(0.24, 0.08)),
        mesh_toolbox: meshes.add(Cuboid::new(0.45, 0.35, 0.75)),
        mesh_cable_spool: meshes.add(Cylinder::new(0.48, 0.65)),
        mesh_transformer_body: meshes.add(Cuboid::new(1.6, 2.4, 1.8)),
        mesh_transformer_fin: meshes.add(Cuboid::new(0.04, 1.8, 1.6)),
        mesh_insulator_bushing: meshes.add(Cylinder::new(0.12, 0.65)),
        mesh_breaker_cabinet: meshes.add(Cuboid::new(0.45, 1.7, 0.9)),

        // Structural Collapse & Debris
        mesh_rubble_chunk_large: meshes.add(Cuboid::new(1.2, 0.6, 0.9)),
        mesh_rubble_chunk_med: meshes.add(Cuboid::new(0.65, 0.4, 0.55)),
        mesh_rubble_chunk_small: meshes.add(Cuboid::new(0.35, 0.25, 0.35)),
        mesh_bent_rebar: meshes.add(Cylinder::new(0.03, 1.8)),
        mesh_emergency_shore_jack: meshes.add(Cylinder::new(0.08, 4.8)),

        // Surveillance & Security
        mesh_camera_mount: meshes.add(Cuboid::new(0.35, 0.12, 0.12)),
        mesh_camera_housing: meshes.add(Cuboid::new(0.18, 0.18, 0.45)),
        mesh_camera_lens: meshes.add(Sphere::new(0.05)),
        mesh_security_barrier: meshes.add(Cuboid::new(1.6, 1.1, 0.15)),

        // VFX Geometry
        mesh_steam_plume: meshes.add(Cylinder::new(0.4, 1.6)),
        mesh_electric_arc: meshes.add(Cuboid::new(0.06, 0.06, 1.2)),

        // -------------------------------------------------------------
        // MATERIALS
        // -------------------------------------------------------------
        mat_tile_normal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.74, 0.70),
            perceptual_roughness: 0.35,
            metallic: 0.05,
            ..default()
        }),
        mat_tile_dirty: materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.43, 0.40),
            perceptual_roughness: 0.65,
            metallic: 0.02,
            ..default()
        }),
        mat_tile_cracked: materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.54, 0.50),
            perceptual_roughness: 0.70,
            metallic: 0.05,
            ..default()
        }),
        mat_tile_broken: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.22, 0.24),
            perceptual_roughness: 0.88,
            metallic: 0.05,
            ..default()
        }),

        mat_pipe_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.14, 0.16),
            perceptual_roughness: 0.45,
            metallic: 0.80,
            ..default()
        }),
        mat_pipe_red: materials.add(StandardMaterial {
            base_color: Color::srgb(0.65, 0.15, 0.15),
            perceptual_roughness: 0.55,
            metallic: 0.40,
            ..default()
        }),
        mat_pipe_yellow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.65, 0.05),
            perceptual_roughness: 0.50,
            metallic: 0.45,
            ..default()
        }),
        mat_pipe_blue: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.35, 0.65),
            perceptual_roughness: 0.50,
            metallic: 0.45,
            ..default()
        }),
        mat_pipe_flange: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.20),
            perceptual_roughness: 0.40,
            metallic: 0.85,
            ..default()
        }),

        mat_bench_wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.20, 0.15),
            perceptual_roughness: 0.80,
            metallic: 0.10,
            ..default()
        }),
        mat_poster_veyron: materials.add(StandardMaterial {
            base_color: Color::srgb(0.80, 0.78, 0.75),
            emissive: LinearRgba::new(0.5, 0.4, 0.2, 1.0),
            perceptual_roughness: 0.40,
            ..default()
        }),
        mat_graffiti_cyan: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.85, 0.95),
            emissive: LinearRgba::new(0.4, 2.2, 3.2, 1.0),
            perceptual_roughness: 0.25,
            ..default()
        }),
        mat_station_sign_amber: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.70, 0.20),
            emissive: LinearRgba::new(2.8, 1.4, 0.2, 1.0),
            ..default()
        }),
        mat_station_sign_flicker: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.55, 0.15),
            emissive: LinearRgba::new(1.4, 0.7, 0.1, 1.0),
            ..default()
        }),

        mat_derelict_train: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.24, 0.26),
            perceptual_roughness: 0.65,
            metallic: 0.70,
            ..default()
        }),
        mat_train_window_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.07, 0.08),
            perceptual_roughness: 0.10,
            metallic: 0.95,
            ..default()
        }),
        mat_train_window_lit: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.75, 0.35),
            emissive: LinearRgba::new(1.8, 1.2, 0.3, 1.0),
            ..default()
        }),

        mat_transformer_metal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.16, 0.18),
            perceptual_roughness: 0.50,
            metallic: 0.80,
            ..default()
        }),
        mat_ceramic_insulator: materials.add(StandardMaterial {
            base_color: Color::srgb(0.65, 0.40, 0.25),
            perceptual_roughness: 0.20,
            metallic: 0.10,
            ..default()
        }),
        mat_rubble_concrete: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.24, 0.26),
            perceptual_roughness: 0.90,
            metallic: 0.05,
            ..default()
        }),
        mat_bent_rebar: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.16, 0.12),
            perceptual_roughness: 0.70,
            metallic: 0.65,
            ..default()
        }),

        mat_security_black: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.07, 0.08),
            perceptual_roughness: 0.25,
            metallic: 0.88,
            ..default()
        }),
        mat_security_red_led: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.15),
            emissive: LinearRgba::new(4.5, 0.2, 0.2, 1.0),
            ..default()
        }),
        mat_emergency_red_light: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.15, 0.15),
            emissive: LinearRgba::new(3.8, 0.3, 0.3, 1.0),
            ..default()
        }),

        mat_steam: materials.add(StandardMaterial {
            base_color: Color::srgba(0.85, 0.90, 0.95, 0.35),
            emissive: LinearRgba::new(0.2, 0.25, 0.3, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        mat_arc_spark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.9, 1.0),
            emissive: LinearRgba::new(3.0, 5.0, 8.0, 1.0),
            ..default()
        }),
    };

    commands.insert_resource(props);
}
