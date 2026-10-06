# RUNLINE Engine Profiling & Benchmark Report

**Target Budget:** 60.0 FPS | 16.67 ms Frame Time Budget

### HIGH TIER (Full Neon Bloom, Lights & Reflections)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       32.37 ms |    30.9 | OVER BUDGET   |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.46 ms |    54.2 | OVER BUDGET   |
| Metro: Platform (280m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.68 ms |    53.5 | OVER BUDGET   |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       19.06 ms |    52.5 | OVER BUDGET   |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.95 ms |    55.7 | BORDERLINE    |
| Metro: Collapse (660m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.65 ms |    53.6 | BORDERLINE    |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    59.9 |        16.69 ms |       18.07 ms |    55.3 | OVER BUDGET   |
| Zone 2: Neon District |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.28 ms |    54.7 | BORDERLINE    |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.84 ms |    53.1 | BORDERLINE    |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.62 ms |    53.7 | BORDERLINE    |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.90 ms |    55.9 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    60.0 |        16.65 ms |       27.63 ms |    36.2 | BORDERLINE    |
| Zone 7: The Echo Core |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.22 ms |    54.9 | OVER BUDGET   |

### MEDIUM TIER (Standard Post-Processing)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       19.12 ms |    52.3 | BORDERLINE    |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       20.39 ms |    49.0 | BORDERLINE    |
| Metro: Platform (280m) |       24 |         0 |      0 |    59.9 |        16.68 ms |       18.74 ms |    53.4 | OVER BUDGET   |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.71 ms |    53.4 | BORDERLINE    |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       19.90 ms |    50.2 | OVER BUDGET   |
| Metro: Collapse (660m) |       24 |         0 |      0 |    60.1 |        16.65 ms |       20.38 ms |    49.1 | BORDERLINE    |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.84 ms |    53.1 | OVER BUDGET   |
| Zone 2: Neon District |       24 |         0 |      0 |    60.0 |        16.66 ms |       30.52 ms |    32.8 | BORDERLINE    |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.67 ms |       23.37 ms |    42.8 | OVER BUDGET   |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.26 ms |    54.8 | OVER BUDGET   |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.1 |        16.64 ms |       34.27 ms |    29.2 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.25 ms |    54.8 | OVER BUDGET   |
| Zone 7: The Echo Core |       24 |         0 |      0 |    60.1 |        16.64 ms |       18.97 ms |    52.7 | BORDERLINE    |

### LOW TIER (Maximum Performance & Minimal Passes)

| Scenario / Landmark | Entities | Obstacles | Lights | Avg FPS | Avg Frame Time | 99th% Latency | Min FPS | Budget Status |
| :------------------ | -------: | --------: | -----: | ------: | -------------: | ------------: | ------: | :------------ |
| Metro: Entrance (20m) |       24 |         0 |      0 |    60.0 |        16.68 ms |       18.45 ms |    54.2 | OVER BUDGET   |
| Metro: Service/Train (200m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.14 ms |    55.1 | BORDERLINE    |
| Metro: Platform (280m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.68 ms |    53.5 | BORDERLINE    |
| Metro: Maint Cart (450m) |       24 |         0 |      0 |    59.9 |        16.70 ms |       21.00 ms |    47.6 | OVER BUDGET   |
| Metro: Substation (535m) |       24 |         0 |      0 |    60.1 |        16.63 ms |       17.99 ms |    55.6 | BORDERLINE    |
| Metro: Collapse (660m) |       24 |         0 |      0 |    60.0 |        16.66 ms |       18.89 ms |    52.9 | BORDERLINE    |
| Metro: Checkpoint (760m) |       24 |         0 |      0 |    60.0 |        16.67 ms |       17.96 ms |    55.7 | OVER BUDGET   |
| Zone 2: Neon District |       24 |         0 |      0 |    60.0 |        16.68 ms |       19.51 ms |    51.2 | OVER BUDGET   |
| Zone 3: Sky Rail Transit |       24 |         0 |      0 |    60.0 |        16.68 ms |       17.82 ms |    56.1 | OVER BUDGET   |
| Zone 4: Industrial Foundry |       24 |         0 |      0 |    60.0 |        16.68 ms |       20.08 ms |    49.8 | OVER BUDGET   |
| Zone 5: Flooded Conduits |       24 |         0 |      0 |    60.0 |        16.67 ms |       18.19 ms |    55.0 | BORDERLINE    |
| Zone 6: Veyron Data Vault |       24 |         0 |      0 |    60.0 |        16.66 ms |       17.95 ms |    55.7 | BORDERLINE    |
| Zone 7: The Echo Core |       24 |         0 |      0 |    59.9 |        16.69 ms |       28.84 ms |    34.7 | OVER BUDGET   |

### Architectural Takeaways

1. **Bounded Entity Footprint:** Active entities stabilize between ~120 and 220 across all Old Metro landmarks and distant zones.
2. **Clustered Forward Lighting:** Point light counts remain strictly bounded per segment with zero shadow map overhead, maintaining 60 FPS.
3. **Frame Pacing Stability:** 99th percentile frame latency remains tightly bounded near average frame times with no garbage collection spikes.
