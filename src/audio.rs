use bevy::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::types::{SoundEffect, BossStartedEvent, BossDefeatedEvent};

#[derive(Resource)]
pub struct SoundHandles {
    pub jump: Handle<AudioSource>,
    pub slide: Handle<AudioSource>,
    pub lane: Handle<AudioSource>,
    pub fragment: Handle<AudioSource>,
    pub chip: Handle<AudioSource>,
    pub powerup: Handle<AudioSource>,
    pub shield_break: Handle<AudioSource>,
    pub crash: Handle<AudioSource>,
    pub dash: Handle<AudioSource>,
    pub transition: Handle<AudioSource>,
}

pub struct AudioSystemPlugin;

impl Plugin for AudioSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEffect>()
            .add_systems(Startup, setup_audio_assets)
            .add_systems(Update, (play_sound_effects, handle_boss_audio_events));
    }
}

fn write_wav_file(path: &Path, samples: &[i16], sample_rate: u32) {
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let data_len = (samples.len() * 2) as u32;
    let file_len = 36 + data_len;

    // RIFF header
    let _ = file.write_all(b"RIFF");
    let _ = file.write_all(&file_len.to_le_bytes());
    let _ = file.write_all(b"WAVE");

    // "fmt " sub-chunk
    let _ = file.write_all(b"fmt ");
    let _ = file.write_all(&16u32.to_le_bytes()); // subchunk1 size (16 for PCM)
    let _ = file.write_all(&1u16.to_le_bytes());  // audio format (1 = PCM)
    let _ = file.write_all(&1u16.to_le_bytes());  // num channels (1 = mono)
    let _ = file.write_all(&sample_rate.to_le_bytes()); // sample rate
    let byte_rate = sample_rate * 2;
    let _ = file.write_all(&byte_rate.to_le_bytes()); // byte rate
    let _ = file.write_all(&2u16.to_le_bytes());  // block align (num channels * bits/8)
    let _ = file.write_all(&16u16.to_le_bytes()); // bits per sample

    // "data" sub-chunk
    let _ = file.write_all(b"data");
    let _ = file.write_all(&data_len.to_le_bytes());

    // write 16-bit PCM samples
    for sample in samples {
        let _ = file.write_all(&sample.to_le_bytes());
    }
}

pub fn generate_sound_assets() {
    let base_dir = Path::new("assets/sounds");
    let _ = fs::create_dir_all(base_dir);
    let sample_rate = 44100;

    // 1. Jump: Pitch sweep upward from 220Hz to 660Hz with fast decay (0.2s)
    let jump_samples = generate_pitch_sweep(sample_rate, 0.22, 220.0, 720.0, 0.7);
    write_wav_file(&base_dir.join("jump.wav"), &jump_samples, sample_rate);

    // 2. Slide: Sci-fi air swoosh (white noise + lowpass decay, 0.3s)
    let slide_samples = generate_swoosh(sample_rate, 0.35, 0.5);
    write_wav_file(&base_dir.join("slide.wav"), &slide_samples, sample_rate);

    // 3. Lane Switch: Quick digital tick / blip (0.08s)
    let lane_samples = generate_tone(sample_rate, 0.08, 480.0, 0.4);
    write_wav_file(&base_dir.join("lane.wav"), &lane_samples, sample_rate);

    // 4. Fragment: Crystalline bell shimmer (1250Hz + 2500Hz, 0.25s)
    let frag_samples = generate_crystal_ping(sample_rate, 0.25, 1320.0, 0.6);
    write_wav_file(&base_dir.join("fragment.wav"), &frag_samples, sample_rate);

    // 5. Data Chip: High-tech three-tone arpeggio (0.35s)
    let chip_samples = generate_arpeggio(sample_rate, &[880.0, 1174.0, 1760.0], 0.12, 0.7);
    write_wav_file(&base_dir.join("chip.wav"), &chip_samples, sample_rate);

    // 6. Power-up: Ascending chord surge (0.45s)
    let power_samples = generate_arpeggio(sample_rate, &[440.0, 659.0, 880.0, 1318.0], 0.10, 0.8);
    write_wav_file(&base_dir.join("powerup.wav"), &power_samples, sample_rate);

    // 7. Shield Break: Harmonic crack and frequency drop (0.4s)
    let shield_samples = generate_pitch_sweep(sample_rate, 0.35, 880.0, 120.0, 0.8);
    write_wav_file(&base_dir.join("shield_break.wav"), &shield_samples, sample_rate);

    // 8. Crash: Low rumble and impact (0.5s)
    let crash_samples = generate_crash(sample_rate, 0.55, 0.9);
    write_wav_file(&base_dir.join("crash.wav"), &crash_samples, sample_rate);

    // 9. Dash: High velocity wind blast (0.3s)
    let dash_samples = generate_pitch_sweep(sample_rate, 0.28, 300.0, 950.0, 0.6);
    write_wav_file(&base_dir.join("dash.wav"), &dash_samples, sample_rate);

    // 10. Transition: Dramatic synth milestone chime (0.6s)
    let trans_samples = generate_arpeggio(sample_rate, &[523.25, 659.25, 783.99, 1046.50], 0.15, 0.8);
    write_wav_file(&base_dir.join("transition.wav"), &trans_samples, sample_rate);
}

fn generate_tone(sample_rate: u32, duration: f32, freq: f32, volume: f32) -> Vec<i16> {
    let count = (sample_rate as f32 * duration) as usize;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let t = i as f32 / sample_rate as f32;
        let env = 1.0 - (t / duration);
        let sample = (2.0 * std::f32::consts::PI * freq * t).sin() * env * volume;
        out.push((sample * 32767.0) as i16);
    }
    out
}

fn generate_pitch_sweep(sample_rate: u32, duration: f32, start_f: f32, end_f: f32, volume: f32) -> Vec<i16> {
    let count = (sample_rate as f32 * duration) as usize;
    let mut out = Vec::with_capacity(count);
    let mut phase = 0.0f32;
    for i in 0..count {
        let t = i as f32 / count as f32;
        let cur_f = start_f + (end_f - start_f) * t;
        let env = (1.0 - t).powf(1.4);
        phase += 2.0 * std::f32::consts::PI * cur_f / sample_rate as f32;
        let sample = phase.sin() * env * volume;
        out.push((sample * 32767.0) as i16);
    }
    out
}

fn generate_crystal_ping(sample_rate: u32, duration: f32, freq: f32, volume: f32) -> Vec<i16> {
    let count = (sample_rate as f32 * duration) as usize;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let t = i as f32 / sample_rate as f32;
        let env = (-t * 12.0).exp();
        let s1 = (2.0 * std::f32::consts::PI * freq * t).sin();
        let s2 = (2.0 * std::f32::consts::PI * (freq * 2.0) * t).sin() * 0.4;
        let s3 = (2.0 * std::f32::consts::PI * (freq * 3.01) * t).sin() * 0.2;
        let sample = (s1 + s2 + s3) * env * volume * 0.6;
        out.push((sample * 32767.0) as i16);
    }
    out
}

fn generate_arpeggio(sample_rate: u32, notes: &[f32], note_dur: f32, volume: f32) -> Vec<i16> {
    let mut out = Vec::new();
    for &freq in notes {
        let count = (sample_rate as f32 * note_dur) as usize;
        for i in 0..count {
            let t = i as f32 / sample_rate as f32;
            let env = 1.0 - (t / note_dur);
            let s1 = (2.0 * std::f32::consts::PI * freq * t).sin();
            let s2 = (2.0 * std::f32::consts::PI * (freq * 1.5) * t).sin() * 0.25;
            let sample = (s1 + s2) * env * volume * 0.7;
            out.push((sample * 32767.0) as i16);
        }
    }
    out
}

fn generate_swoosh(sample_rate: u32, duration: f32, volume: f32) -> Vec<i16> {
    let count = (sample_rate as f32 * duration) as usize;
    let mut out = Vec::with_capacity(count);
    let mut seed = 123456789u32;
    for i in 0..count {
        let t = i as f32 / count as f32;
        let env = (-(t - 0.4).powi(2) * 12.0).exp();
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((seed >> 16) as f32 / 32768.0) - 1.0;
        let sample = noise * env * volume;
        out.push((sample * 32767.0) as i16);
    }
    out
}

fn generate_crash(sample_rate: u32, duration: f32, volume: f32) -> Vec<i16> {
    let count = (sample_rate as f32 * duration) as usize;
    let mut out = Vec::with_capacity(count);
    let mut seed = 987654321u32;
    for i in 0..count {
        let t = i as f32 / count as f32;
        let env = (1.0 - t).powi(2);
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((seed >> 16) as f32 / 32768.0) - 1.0;
        let sub = (2.0 * std::f32::consts::PI * 65.0 * (i as f32 / sample_rate as f32)).sin() * 0.6;
        let sample = (noise * 0.6 + sub * 0.4) * env * volume;
        out.push((sample * 32767.0) as i16);
    }
    out
}

fn setup_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    generate_sound_assets();

    let handles = SoundHandles {
        jump: asset_server.load("sounds/jump.wav"),
        slide: asset_server.load("sounds/slide.wav"),
        lane: asset_server.load("sounds/lane.wav"),
        fragment: asset_server.load("sounds/fragment.wav"),
        chip: asset_server.load("sounds/chip.wav"),
        powerup: asset_server.load("sounds/powerup.wav"),
        shield_break: asset_server.load("sounds/shield_break.wav"),
        crash: asset_server.load("sounds/crash.wav"),
        dash: asset_server.load("sounds/dash.wav"),
        transition: asset_server.load("sounds/transition.wav"),
    };
    commands.insert_resource(handles);
}

fn play_sound_effects(
    mut events: EventReader<SoundEffect>,
    handles: Option<Res<SoundHandles>>,
    mut commands: Commands,
) {
    let handles = match handles {
        Some(h) => h,
        None => return,
    };

    for event in events.read() {
        let source = match event {
            SoundEffect::Jump => handles.jump.clone(),
            SoundEffect::Slide => handles.slide.clone(),
            SoundEffect::LaneSwitch => handles.lane.clone(),
            SoundEffect::FragmentPickup => handles.fragment.clone(),
            SoundEffect::DataChipPickup => handles.chip.clone(),
            SoundEffect::PowerupPickup => handles.powerup.clone(),
            SoundEffect::ShieldBreak => handles.shield_break.clone(),
            SoundEffect::Stumble => handles.slide.clone(),
            SoundEffect::Crash => handles.crash.clone(),
            SoundEffect::Dash => handles.dash.clone(),
            SoundEffect::ZoneTransition => handles.transition.clone(),
        };

        commands.spawn(AudioBundle {
            source,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn handle_boss_audio_events(
    mut boss_started: EventReader<BossStartedEvent>,
    mut boss_defeated: EventReader<BossDefeatedEvent>,
    mut sfx: EventWriter<SoundEffect>,
) {
    for _ in boss_started.read() {
        sfx.send(SoundEffect::ZoneTransition);
    }
    for _ in boss_defeated.read() {
        sfx.send(SoundEffect::PowerupPickup);
    }
}

