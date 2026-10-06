import os
import math
import struct

def write_wav(path, samples, sample_rate=44100):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    num_samples = len(samples)
    data_size = num_samples * 2
    riff_size = 36 + data_size
    
    with open(path, "wb") as f:
        # RIFF header
        f.write(b"RIFF")
        f.write(struct.pack("<I", riff_size))
        f.write(b"WAVE")
        
        # fmt chunk
        f.write(b"fmt ")
        f.write(struct.pack("<I", 16)) # Subchunk1Size
        f.write(struct.pack("<H", 1))  # AudioFormat (PCM)
        f.write(struct.pack("<H", 1))  # NumChannels (Mono)
        f.write(struct.pack("<I", sample_rate))
        f.write(struct.pack("<I", sample_rate * 2)) # ByteRate
        f.write(struct.pack("<H", 2))  # BlockAlign
        f.write(struct.pack("<H", 16)) # BitsPerSample
        
        # data chunk
        f.write(b"data")
        f.write(struct.pack("<I", data_size))
        for s in samples:
            val = max(-32767, min(32767, int(s * 32767)))
            f.write(struct.pack("<h", val))

def gen_sweep(dur, start_f, end_f, vol=0.7, sr=44100):
    count = int(sr * dur)
    samples = []
    phase = 0.0
    for i in range(count):
        t = i / count
        freq = start_f + (end_f - start_f) * t
        env = (1.0 - t) ** 1.4
        phase += 2.0 * math.pi * freq / sr
        samples.append(math.sin(phase) * env * vol)
    return samples

def gen_swoosh(dur, vol=0.5, sr=44100):
    count = int(sr * dur)
    samples = []
    seed = 123456789
    for i in range(count):
        t = i / sr
        env = math.exp(-((t - 0.15) ** 2) * 20.0)
        seed = (seed * 1664525 + 1013904223) & 0xFFFFFFFF
        noise = ((seed >> 16) / 32768.0) - 1.0
        samples.append(noise * env * vol)
    return samples

def gen_tone(dur, freq, vol=0.5, sr=44100):
    count = int(sr * dur)
    return [math.sin(2.0 * math.pi * freq * (i / sr)) * (1.0 - i / count) * vol for i in range(count)]

def gen_crystal(dur, freq=1320.0, vol=0.6, sr=44100):
    count = int(sr * dur)
    samples = []
    for i in range(count):
        t = i / sr
        env = math.exp(-t * 12.0)
        s1 = math.sin(2.0 * math.pi * freq * t)
        s2 = math.sin(2.0 * math.pi * freq * 2.0 * t) * 0.4
        s3 = math.sin(2.0 * math.pi * freq * 3.01 * t) * 0.2
        samples.append((s1 + s2 + s3) * env * vol * 0.6)
    return samples

def gen_arpeggio(notes, note_dur, vol=0.7, sr=44100):
    samples = []
    for f in notes:
        count = int(sr * note_dur)
        for i in range(count):
            t = i / sr
            env = 1.0 - (i / count)
            s = math.sin(2.0 * math.pi * f * t) + math.sin(2.0 * math.pi * f * 1.5 * t) * 0.25
            samples.append(s * env * vol * 0.7)
    return samples

def gen_crash(dur, vol=0.85, sr=44100):
    count = int(sr * dur)
    samples = []
    seed = 987654321
    for i in range(count):
        t = i / count
        env = (1.0 - t) ** 2
        seed = (seed * 1664525 + 1013904223) & 0xFFFFFFFF
        noise = ((seed >> 16) / 32768.0) - 1.0
        sub = math.sin(2.0 * math.pi * 65.0 * (i / sr)) * 0.6
        samples.append((noise * 0.6 + sub * 0.4) * env * vol)
    return samples

def main():
    base = "assets/sounds"
    print("Generating cyberpunk sound effects...")
    write_wav(f"{base}/jump.wav", gen_sweep(0.22, 220.0, 720.0, 0.7))
    write_wav(f"{base}/slide.wav", gen_swoosh(0.35, 0.5))
    write_wav(f"{base}/lane.wav", gen_tone(0.08, 480.0, 0.4))
    write_wav(f"{base}/fragment.wav", gen_crystal(0.25, 1320.0, 0.6))
    write_wav(f"{base}/chip.wav", gen_arpeggio([880.0, 1174.0, 1760.0], 0.12, 0.7))
    write_wav(f"{base}/powerup.wav", gen_arpeggio([440.0, 659.0, 880.0, 1318.0], 0.10, 0.8))
    write_wav(f"{base}/shield_break.wav", gen_sweep(0.35, 880.0, 120.0, 0.8))
    write_wav(f"{base}/crash.wav", gen_crash(0.55, 0.85))
    write_wav(f"{base}/dash.wav", gen_sweep(0.28, 300.0, 950.0, 0.6))
    write_wav(f"{base}/transition.wav", gen_arpeggio([523.25, 659.25, 783.99, 1046.50], 0.15, 0.8))
    print("Sound generation complete!")

if __name__ == "__main__":
    main()
