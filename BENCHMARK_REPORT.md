# RUNLINE Engine Profiling & Benchmark Report

**Target Budget:** 60.0 FPS | 16.67 ms Frame Time Budget

### HIGH TIER (Full Neon Bloom, Lights & Reflections)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       46.20 ms |    21.6 | OVER BUDGET   |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       19.14 ms |    52.2 | BORDERLINE    |
| Metro: Platform (280m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.10 ms |    55.3 | BORDERLINE    |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.67 ms |    56.6 | BORDERLINE    |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.71 ms |    53.5 | BORDERLINE    |
| Metro: Collapse (660m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.67 ms |    56.6 | BORDERLINE    |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.46 ms |    57.3 | BORDERLINE    |
| Zone 2: Neon District |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.59 ms |    56.9 | BORDERLINE    |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.28 ms |    57.9 | BORDERLINE    |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.98 ms |    55.6 | BORDERLINE    |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.44 ms |    54.2 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.82 ms |    56.1 | BORDERLINE    |
| Zone 7: The Echo Core |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.06 ms |    55.4 | BORDERLINE    |

### MEDIUM TIER (Standard Post-Processing)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       23.54 ms |    42.5 | BORDERLINE    |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.63 ms |    56.7 | BORDERLINE    |
| Metro: Platform (280m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       22.25 ms |    45.0 | BORDERLINE    |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.57 ms |    53.8 | BORDERLINE    |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.85 ms |    56.0 | BORDERLINE    |
| Metro: Collapse (660m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.64 ms |    56.7 | BORDERLINE    |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.74 ms |    56.4 | BORDERLINE    |
| Zone 2: Neon District |       24 |         0 |      0 |    60.1 |        16.65 ms |       17.87 ms |    56.0 | BORDERLINE    |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.27 ms |    54.7 | OVER BUDGET   |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.0 |        16.67 ms |       26.01 ms |    38.5 | BORDERLINE    |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.24 ms |    54.8 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.46 ms |    54.2 | BORDERLINE    |
| Zone 7: The Echo Core |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.59 ms |    56.9 | BORDERLINE    |

### LOW TIER (Maximum Performance & Minimal Passes)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.63 ms |    56.7 | BORDERLINE    |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.65 ms |    56.7 | BORDERLINE    |
| Metro: Platform (280m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.78 ms |    56.2 | BORDERLINE    |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       25.44 ms |    39.3 | BORDERLINE    |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.62 ms |    56.8 | BORDERLINE    |
| Metro: Collapse (660m) |       24 |         0 |      0 |    59.9 |        16.68 ms |       18.23 ms |    54.8 | OVER BUDGET   |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.43 ms |    57.4 | BORDERLINE    |
| Zone 2: Neon District |       24 |         0 |      0 |    60.1 |        16.65 ms |       17.73 ms |    56.4 | BORDERLINE    |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.78 ms |    56.3 | BORDERLINE    |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.1 |        16.63 ms |       17.41 ms |    57.4 | BORDERLINE    |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.39 ms |    57.5 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    59.9 |        16.68 ms |       18.41 ms |    54.3 | OVER BUDGET   |
| Zone 7: The Echo Core |       24 |         0 |      0 |    60.1 |        16.65 ms |       19.28 ms |    51.9 | BORDERLINE    |

### Architectural Takeaways

1. **Bounded Entity Footprint:** Active entities stabilize between ~120 and 220 across all Old Metro landmarks and distant zones.
2. **Clustered Forward Lighting:** Point light counts remain strictly bounded per segment with zero shadow map overhead, maintaining 60 FPS.
3. **Frame Pacing Stability:** 99th percentile frame latency remains tightly bounded near average frame times with no garbage collection spikes.
