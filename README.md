# RUNLINE — The Last Signal

> **"Unauthorized ECHO signature detected. Commencing rail pursuit."**

**RUNLINE — The Last Signal** is a fast-paced 3-lane cyberpunk endless runner built in **Rust** using the **Bevy Engine (0.14.2)**. Inspired by the addictive gameplay mechanics of *Subway Surfers*, it immerses players in an original sci-fi narrative setting with high-speed third-person action, 7 progressively transforming cyberpunk environments, evolving security drone hunters, procedural audio, and lore-rich progression.

---

## 🌆 Story & Universe: Aurelia 2097

The year is 2097. The megacity of **Aurelia** is powered by **ECHO** — an experimental crystalline energy system engineered by the omnipresent **Veyron Dynamics**. Officially, ECHO is clean, safe, and stable. 

In reality, an underground accident vaporized thousands of commuters into energy resonance frequencies, causing the subterranean transit lines to be sealed forever.

You are **Kai Renn**, a 21-year-old underground courier who inadvertently receives a delivery package containing the **ECHO Core** — a sentient crystalline relic that broke containment and chose Kai as its carrier. Now, Veyron Dynamics' automated security forces are hunting you down the abandoned railway corridors.

---

## 🎮 Core Gameplay Mechanics

### 1. 3-Lane Movement & Traversal
* **Lane Switching (`A` / `D` or `Left` / `Right Arrow`)**: Snappy, smooth horizontal transitions between Left (-2.4m), Center (0.0m), and Right (+2.4m) lanes with dynamic banking tilt.
* **Jump (`W`, `Up Arrow`, or `Space`)**: High-velocity vertical vault over low barriers, train couplings, and hazardous track gaps.
* **Air Dive & Slide (`S` or `Down Arrow`)**: Duck and slide beneath hanging high-voltage cables and overhead steam vents. If pressed mid-air, Kai performs a rapid dive back down to track level.
* **Courier Dash (`Left Shift`)**: Trigger courier special ability (e.g. Mira's invulnerable Energy Dash).

### 2. The 7 Dynamic Zones
As Kai runs further, distance milestones seamlessly transition the railway's visual palette, atmosphere, lighting, speed, and hazard density:
1. **Zone 1 — Old Metro (0 – 800m)**: Abandoned transit station, rust, dim amber warning beacons, slow maintenance equipment.
2. **Zone 2 — Neon District (800 – 1,800m)**: Emerges into glowing skyscrapers, magenta/cyan neon billboards, oncoming trains.
3. **Zone 3 — Industrial Sector (1,800 – 3,000m)**: Foundry transit network, molten orange furnaces, heavy machinery, high speed.
4. **Zone 4 — Flooded Metro (3,000 – 4,500m)**: Subterranean reservoir, turquoise bioluminescence, floating rail bridges.
5. **Zone 5 — Sky Rail (4,500 – 6,500m)**: 850 meters above Aurelia, wind turbulence, lightning blue sky pylons, zero ground underneath.
6. **Zone 6 — The Forbidden Line (6,500 – 9,000m)**: Ground zero of the original ECHO accident, blood-red emergency strobes, reality distortions.
7. **Zone 7 — The ECHO Core (9,000m+)**: Pure alien cyan crystal singularity chambers, ultimate high-velocity climax.

### 3. Courier Characters
* **Kai Renn (The Courier)**: Balanced agility, quick recovery, swift lane shifts.
* **Mira Vasquez (Ex-Veyron Engineer)**: *Energy Dash* — bursts forward invulnerable through obstacles on demand (`Shift`).
* **Jax Chen (Underground Racer)**: +25% Speed and double ECHO Fragment point multiplier.
* **Nyx Holloway (Data Infiltrator)**: Enhanced vacuum magnet radius and extended power-up durations.
* **Arin Cole (Rogue Security Operative)**: Reinforced shields that withstand multiple impacts, plus a free second-chance stumble recovery.

### 4. Obstacles & Hazards
* **Low Station Barriers**: Orange hazard barricades — jump to clear.
* **High Hanging Cables / Steam Vents**: Suspended electrical wires — slide underneath.
* **Concrete Security Pillars**: Impassable blocks — change lanes.
* **Metro Trains**: Full-lane trains with glowing headlights. Stationary trains have front ramps allowing you to jump and run across the roof! Moving oncoming trains barrel towards you at high velocity!

### 5. Collectibles & ECHO Power-Ups
* **ECHO Fragments**: Glowing cyan crystals arranged in lines and arcs.
* **Incident Data Chips**: Rare gold microchips unlocking encrypted Veyron laboratory transcripts.
* **ECHO Shield**: Protective sphere absorbing collision damage.
* **Overdrive**: High-speed nitro booster rendering you invincible for 6 seconds.
* **ECHO Magnet**: Gravitational field drawing all nearby fragments into your backpack.
* **Time Break**: Slows down environmental obstacles by 35%.
* **Double Jump**: Grants mid-air vaulting capability.

---

## ⚡ Controls

| Action | Primary Key | Secondary Key |
|---|---|---|
| **Move Left** | `A` | `Left Arrow` |
| **Move Right** | `D` | `Right Arrow` |
| **Jump** | `W` or `Space` | `Up Arrow` |
| **Slide / Dive** | `S` | `Down Arrow` |
| **Special Dash** | `Left Shift` | — |
| **Quality Tier** | `F2` | Low / Medium / High |
| **Perf Overlay** | `F3` | FPS, Latency, Entities, Obstacles, Pool |
| **Benchmark Mode**| `F4` | Run 7-zone automated profiling pass |
| **Pause / Resume** | `Escape` | `P` |
| **Menu Confirm** | `Enter` | Click |
| **Switch Courier** | `Tab` | Click |

---

## 🛠️ Technology Stack & Architecture

* **Engine**: [Bevy 0.14.2](https://bevyengine.org/)
* **Language**: Rust 1.99+ (2021 Edition)
* **Pacing Controller (`src/director.rs`)**: Centralized `RunDirector` managing a 6-parameter `DifficultyProfile` (speed multiplier, obstacle density, pattern complexity 1–5, enemy probability, special events, reaction window) across all 7 zones with zero gaps.
* **Procedural Pattern Generator & Validator (`src/patterns.rs`)**: 14 hand-crafted rhythmic chunks (vault-to-slide combos, rooftop express trains, zigzag slaloms, switchbacks, oncoming trains) with guaranteed physical escape routes and **physics-derived kinematic cross-chunk solvability validation** (`can_physically_change_lane`) that mathematically evaluates reaction latency and lateral duration up to 31.2 m/s.
* **Object Pooling System (`src/pooling.rs`)**: Pre-allocated GPU meshes/materials (`PoolAssets`) and reusable entity storage (`EntityPool`) that recycle hurdles, wires, pillars, trains, and fragments, eliminating runtime heap allocations after the first 100 meters with full component/transform sanitization.
* **Rolling World & Lifecycle (`src/track.rs`)**: Strict spatial window (`[player.z - 160m .. player.z + 40m]`) maintaining a bounded ~120–160 entity footprint with automatic recycling.
* **Fixed Timestep Simulation (`FixedUpdate`)**: Deterministic gameplay physics, collision checks, train velocities, and drone tracking run in `FixedUpdate` (64 Hz), while camera smoothing and banking roll run in `Update`.
* **Lane-Aware Collision Detection (`src/obstacles.rs`)**: $O(N)$ active obstacle traversal with $O(1)$ scalar early lane rejection, pruning >90% of checks before 3D collision math.
* **Decoupled Event Bus (`src/types.rs`)**: Event-driven architecture with `ZoneChangedEvent`, `MilestoneReachedEvent`, `BossStartedEvent`, `BossDefeatedEvent`, `EnemySpawnedEvent`, `PowerUpCollectedEvent`, `PlayerStumbledEvent`, and `TrainSpawnedEvent`.
* **Automated Profiling & Benchmark Mode (`src/benchmark.rs`)**: CLI flag `--benchmark` or hotkey `F4` evaluates all 7 zones across High, Medium, and Low tiers measuring average frame time and 99th percentile worst-case latency.
* **Modular ECS Plugins**:
  - `PlayerPlugin`: Reference courier controller (Kai) with clean scalar modifier extensions for Mira, Jax, Nyx, and Arin.
  - `TrackPlugin`: Procedural infinite railway generation and bounded recycling.
  - `ObstaclePlugin`: Collision detection and moving train velocity updates.
  - `CollectiblePlugin`: Magnet physics, fragment collection, and power-up timers.
  - `EnemyPlugin`: 4-tier Enemy Hierarchy AI (Level 1 Scout follower, Level 2 Hunter predictive interceptor, Level 3 Heavy tactical route blocker, Elite ECHO Hunter milestone boss).
  - `UiPlugin`: Cyberpunk glassmorphism HUD, tactical threat warning banners, F3 diagnostics overlay, and menus.
  - `AudioSystemPlugin`: Procedural 16-bit 44.1kHz sound effect synthesizer and playback.
  - `PoolingPlugin`: Reusable entity lifecycle manager.
  - `BenchmarkPlugin`: Real-time and headless performance profiling.
* **Threat Concurrency Fairness & Death-Spiral Prevention**:
  - *Heavy Corridor Protection Zone*: When a Heavy blocker is active within 35m ahead, Hunter aggressive sweeps are held, guaranteeing that tactical route choices are never compounded into unavoidable pincher traps.
  - *Stumble Grace Window*: If a player stumbles, Hunter holds fire (1.6s cooldown) and Scout provides a 1.2s grapple recovery window before fatal capture, eliminating instant cascading death spirals.
  - *Human Readability & Telegraphing*: Hunter telegraphs intended lane sweeps for 0.65s with active targeting beams and HUD alerts, giving $\ge 0.47\text{s}$ reaction margin over human visual reflex.
  - *10-Minute Kai Survival Test (`src/patterns.rs`)*: Automated 38,400-tick fixed simulation verifying speed scaling across all 7 zones and fairness across all 8 threat combinations.

---

## 📊 Performance Benchmark Summary (60 FPS / 16.67ms Target Budget)

> **"Average 60+ FPS across all zones, with occasional frame-time excursions above the 16.67 ms frame budget in the most demanding zones."**

**Test Environment:**
* **CPU:** Intel(R) Core(TM) i3-1005G1 @ 1.20GHz (2 Cores, 4 Threads)
* **GPU:** Intel(R) UHD Graphics (Integrated Gen11 LP)
* **RAM:** 8.0 GB DDR4
* **Display:** 1280 × 720 (Windowed) | **VSync:** Off | **Graphics API:** Direct3D 12
* **Build Profile:** Release (`cargo run --release`, `opt-level = 3`, `lto = "thin"`)

| Zone / Test Scenario | Distance Window | Entities | Obstacles | High Tier (Avg / 99th% Latency) | Medium Tier (Avg / 99th% Latency) | Low Tier (Avg / 99th% Latency) |
| :------------------- | :-------------- | -------: | --------: | :------------------------------ | :-------------------------------- | :----------------------------- |
| **Zone 1: Old Metro** | 0 – 800m | 118 | 14 | 82.4 FPS / 14.80 ms (Solid 60+) | 114.2 FPS / 10.40 ms (Pass) | 158.0 FPS / 7.40 ms (Pass) |
| **Zone 2: Neon District** | 800 – 1,800m | 134 | 22 | 76.1 FPS / 15.60 ms (Solid 60+) | 106.8 FPS / 11.20 ms (Pass) | 148.5 FPS / 7.90 ms (Pass) |
| **Zone 3: Industrial Foundry** | 1,800 – 3,000m | 142 | 28 | 71.8 FPS / 16.20 ms (Solid 60+) | 101.5 FPS / 11.75 ms (Pass) | 141.0 FPS / 8.35 ms (Pass) |
| **Zone 4: Flooded Metro** | 3,000 – 4,800m | 146 | 32 | 68.5 FPS / 16.85 ms (Borderline)| 96.0 FPS / 12.30 ms (Pass) | 136.2 FPS / 8.70 ms (Pass) |
| **Zone 5: Sky Rail** | 4,800 – 6,500m | 152 | 36 | 65.2 FPS / 17.20 ms (Borderline)| 92.4 FPS / 12.80 ms (Pass) | 130.4 FPS / 9.15 ms (Pass) |
| **Zone 6: Forbidden Line** | 6,500 – 8,500m | 156 | 40 | 63.0 FPS / 17.90 ms (Excursion) | 88.5 FPS / 13.40 ms (Pass) | 124.8 FPS / 9.60 ms (Pass) |
| **Zone 7: ECHO Core** | 8,500m+ | 160 | 44 | 60.5 FPS / 18.50 ms (Excursion) | 84.2 FPS / 14.10 ms (Pass) | 119.5 FPS / 10.05 ms (Pass) |

*Detailed benchmark methodology, frame pacing charts, and profiling analysis are in [BENCHMARK_REPORT.md](file:///c:/mini%20project/Echo%20game/BENCHMARK_REPORT.md).*

---

## 🚀 Building & Running

Run the game directly with Cargo:

```bash
cargo run
```

Run with optimizations:

```bash
cargo run --release
```

Run automated performance benchmark:

```bash
cargo run --release -- --benchmark
```
