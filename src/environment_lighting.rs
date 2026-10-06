use bevy::prelude::*;

#[derive(Component)]
pub struct FlickeringLight {
    pub base_intensity: f32,
    pub flicker_speed: f32,
    pub min_mult: f32,
    pub seed: f32,
}

#[derive(Component)]
pub struct EmergencyBeacon {
    pub frequency: f32,
    pub phase: f32,
    pub base_intensity: f32,
}

#[derive(Component)]
pub struct SteamVent {
    pub timer: f32,
    pub cycle: f32,
}

#[derive(Component)]
pub struct ElectricalArc {
    pub timer: f32,
    pub interval: f32,
}

#[derive(Component)]
pub struct RotatingCameraMount {
    pub sweep_speed: f32,
    pub max_angle: f32,
}

pub struct EnvironmentLightingPlugin;

impl Plugin for EnvironmentLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_flickering_lights,
                update_emergency_beacons,
                update_steam_and_arcs,
                update_surveillance_cameras,
            )
                .run_if(in_state(crate::types::AppState::InGame)),
        );
    }
}

fn update_flickering_lights(
    time: Res<Time>,
    mut query: Query<(&FlickeringLight, &mut PointLight)>,
) {
    let t = time.elapsed_seconds();
    for (flicker, mut light) in query.iter_mut() {
        let noise1 = ((t * flicker.flicker_speed + flicker.seed).sin() * 0.5 + 0.5).powi(2);
        let noise2 = (t * flicker.flicker_speed * 2.37 + flicker.seed * 3.1).cos() * 0.5 + 0.5;
        let combined = (noise1 * 0.7 + noise2 * 0.3).clamp(flicker.min_mult, 1.0);
        light.intensity = flicker.base_intensity * combined;
    }
}

fn update_emergency_beacons(
    time: Res<Time>,
    mut query: Query<(&EmergencyBeacon, &mut PointLight)>,
) {
    let t = time.elapsed_seconds();
    for (beacon, mut light) in query.iter_mut() {
        let pulse = ((t * beacon.frequency * std::f32::consts::TAU + beacon.phase).sin() * 0.5 + 0.5).powi(3);
        light.intensity = beacon.base_intensity * pulse;
    }
}

fn update_steam_and_arcs(
    time: Res<Time>,
    mut steam_q: Query<(&mut SteamVent, &mut Transform), Without<ElectricalArc>>,
    mut arc_q: Query<(&mut ElectricalArc, &mut Visibility)>,
) {
    let dt = time.delta_seconds();

    for (mut steam, mut trans) in steam_q.iter_mut() {
        steam.timer += dt;
        if steam.timer > steam.cycle {
            steam.timer -= steam.cycle;
        }

        // Intermittent steam puff: active during 0.0s..1.2s of cycle
        if steam.timer < 1.2 {
            let progress = steam.timer / 1.2;
            let scale_y = 1.0 + progress * 2.5;
            let alpha = 1.0 - progress;
            trans.scale = Vec3::new(alpha, scale_y, alpha);
        } else {
            trans.scale = Vec3::ZERO;
        }
    }

    for (mut arc, mut vis) in arc_q.iter_mut() {
        arc.timer += dt;
        if arc.timer > arc.interval {
            arc.timer -= arc.interval;
        }

        // Quick electric spark flash during 0.0s..0.12s
        if arc.timer < 0.12 {
            *vis = Visibility::Inherited;
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

fn update_surveillance_cameras(
    time: Res<Time>,
    mut query: Query<(&RotatingCameraMount, &mut Transform)>,
) {
    let t = time.elapsed_seconds();
    for (cam, mut trans) in query.iter_mut() {
        let angle = (t * cam.sweep_speed).sin() * cam.max_angle;
        trans.rotation = Quat::from_rotation_y(angle);
    }
}
