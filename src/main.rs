mod audio;
mod benchmark;
mod collectibles;
mod director;
mod enemies;
mod environment;
mod environment_lighting;
mod environment_props;
mod environment_signage;
mod neon_district;
mod neon_district_landmarks;
mod obstacles;
mod old_metro;
mod old_metro_landmarks;
mod patterns;
mod player;
mod pooling;
mod story;
mod track;
mod types;
mod ui;
mod zones;

use audio::AudioSystemPlugin;
use benchmark::BenchmarkPlugin;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use collectibles::CollectiblePlugin;
use director::DirectorPlugin;
use enemies::EnemyPlugin;
use environment::EnvironmentPlugin;
use environment_lighting::EnvironmentLightingPlugin;
use obstacles::ObstaclePlugin;
use player::PlayerPlugin;
use pooling::PoolingPlugin;
use track::TrackPlugin;
use types::*;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RUNLINE — The Last Signal (Aurelia 2097)".to_string(),
                resolution: (1280.0_f32, 720.0_f32).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Performance instrumentation diagnostics
        .add_plugins((FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin))
        // Game States & Core Resources
        .init_state::<AppState>()
        .init_resource::<GameRunStats>()
        .init_resource::<ActivePowerUps>()
        .init_resource::<BossBattleState>()
        .init_resource::<PlayerMovementHistory>()
        .init_resource::<ThreatAlertState>()
        .init_resource::<QualitySettings>()
        .add_event::<RunResetEvent>()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.07)))
        // Architectural Plugins
        .add_plugins((
            DirectorPlugin,
            AudioSystemPlugin,
            PlayerPlugin,
            TrackPlugin,
            ObstaclePlugin,
            CollectiblePlugin,
            EnemyPlugin,
            UiPlugin,
            PoolingPlugin,
            BenchmarkPlugin,
            EnvironmentPlugin,
            EnvironmentLightingPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                camera_follow_player.run_if(in_state(AppState::InGame)),
                handle_run_reset_camera,
            ),
        )
        .run();
}

fn setup_scene(mut commands: Commands) {
    // 3D Third-Person Follow Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.8, 6.8)
                .looking_at(Vec3::new(0.0, 1.4, -6.0), Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // Directional Lighting (Low-angle subterranean rake)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.40, 0.45, 0.52),
            illuminance: 3200.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.75, -0.35, 0.0)),
        ..default()
    });

    // Strategic Ambient Lighting (Dark subterranean envelope)
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.08, 0.10, 0.14),
        brightness: 80.0,
    });
}

fn camera_follow_player(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut cam_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    stats: Res<GameRunStats>,
    boss_state: Res<BossBattleState>,
    history: Res<PlayerMovementHistory>,
) {
    let p_trans = match player_q.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut c_trans = match cam_q.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();

    // Dynamic camera framing: smoothly pull back slightly as velocity increases for heightened speed sensation
    let speed_factor = ((stats.speed - 16.0) / 16.0).clamp(0.0, 1.0);
    let target_x = p_trans.translation.x * 0.45;
    let target_y = 3.8 + speed_factor * 0.35;
    let target_z = p_trans.translation.z + 6.8 + speed_factor * 0.9;

    // Stumble & Boss encounter tension screen-shake
    let boss_shake = if boss_state.is_active {
        (t * 24.0).sin() * 0.06
    } else {
        0.0
    };
    let shake_x = if stats.stumble_intensity > 0.0 {
        (t * 35.0).sin() * stats.stumble_intensity * 0.30 + boss_shake
    } else {
        boss_shake
    };
    let shake_y = if stats.stumble_intensity > 0.0 {
        (t * 40.0).cos() * stats.stumble_intensity * 0.20
    } else {
        0.0
    };

    c_trans.translation.x += (target_x + shake_x - c_trans.translation.x) * (16.0 * dt).min(1.0);
    c_trans.translation.y = target_y + shake_y;
    c_trans.translation.z = target_z;

    let look_target = Vec3::new(
        p_trans.translation.x * 0.3,
        1.4,
        p_trans.translation.z - 7.0,
    );
    c_trans.look_at(look_target, Vec3::Y);

    // Subtle lateral roll banking into turn on lane shifts
    let lateral_bank = (-history.current_lateral_velocity * 0.0030).clamp(-0.045, 0.045);
    c_trans.rotation *= Quat::from_rotation_z(lateral_bank);

    // Subtle rotational roll disorientation during stumble
    if stats.stumble_intensity > 0.0 {
        let shake_roll = (t * 28.0).sin() * stats.stumble_intensity * 0.025;
        c_trans.rotation *= Quat::from_rotation_z(shake_roll);
    }
}

fn handle_run_reset_camera(
    mut events: EventReader<RunResetEvent>,
    mut cam_q: Query<&mut Transform, With<MainCamera>>,
) {
    for _ in events.read() {
        if let Ok(mut c_trans) = cam_q.get_single_mut() {
            *c_trans =
                Transform::from_xyz(0.0, 3.8, 6.8).looking_at(Vec3::new(0.0, 1.4, -6.0), Vec3::Y);
        }
    }
}
