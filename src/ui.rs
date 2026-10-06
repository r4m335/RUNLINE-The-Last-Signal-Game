use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin};
use crate::types::*;
use crate::zones::get_zone_for_distance;
use crate::story::DATA_CHIP_LOGS;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudDistanceText;

#[derive(Component)]
pub struct HudFragmentsText;

#[derive(Component)]
pub struct HudDataChipsText;

#[derive(Component)]
pub struct HudScoreText;

#[derive(Component)]
pub struct HudZoneText;

#[derive(Component)]
pub struct HudPowerupText;

#[derive(Component)]
pub struct HudTransmissionBox;

#[derive(Component)]
pub struct HudTransmissionTitle;

#[derive(Component)]
pub struct HudTransmissionBody;

#[derive(Component)]
pub struct HudBossBannerBox;

#[derive(Component)]
pub struct HudBossBannerTitle;

#[derive(Component)]
pub struct HudBossBannerStatus;

#[derive(Component)]
pub struct HudThreatBannerBox;

#[derive(Component)]
pub struct HudThreatBannerText;

#[derive(Component)]
pub struct PerfOverlayText;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub struct PauseRoot;

#[derive(Component)]
pub struct StoryLogRoot;

// Button Markers
#[derive(Component)]
pub struct StartRunButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct OpenStoryLogButton;

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct NextCharacterButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_entity::<MenuRoot>)
            .add_systems(OnEnter(AppState::InGame), setup_hud)
            .add_systems(OnExit(AppState::InGame), cleanup_entity::<HudRoot>)
            .add_systems(OnEnter(AppState::Paused), setup_pause_menu)
            .add_systems(OnExit(AppState::Paused), cleanup_entity::<PauseRoot>)
            .add_systems(OnEnter(AppState::GameOver), setup_game_over)
            .add_systems(OnExit(AppState::GameOver), cleanup_entity::<GameOverRoot>)
            .add_systems(OnEnter(AppState::StoryLog), setup_story_log_menu)
            .add_systems(OnExit(AppState::StoryLog), cleanup_entity::<StoryLogRoot>)
            .add_systems(
                Update,
                (
                    update_hud_display,
                    update_transmission_toast,
                    update_boss_ui,
                    update_threat_alert_ui,
                    update_perf_overlay,
                    handle_global_shortcuts,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                handle_pause_shortcuts.run_if(in_state(AppState::Paused)),
            )
            .add_systems(
                Update,
                (
                    button_interaction_system,
                    menu_actions_system,
                ),
            );
    }
}

fn cleanup_entity<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// -------------------------------------------------------------
// 1. MAIN MENU
// -------------------------------------------------------------
fn setup_main_menu(
    mut commands: Commands,
    stats: Res<GameRunStats>,
    quality: Res<QualitySettings>,
) {
    let char_type = stats.selected_character;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.95)),
                ..default()
            },
            MenuRoot,
        ))
        .with_children(|root| {
            // Header
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|header| {
                header.spawn(TextBundle::from_section(
                    "RUNLINE",
                    TextStyle {
                        font_size: 72.0,
                        color: Color::srgb(0.0, 0.9, 1.0),
                        ..default()
                    },
                ));
                header.spawn(TextBundle::from_section(
                    "THE LAST SIGNAL // AURELIA 2097",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.4, 0.1),
                        ..default()
                    },
                ));
            });

            // Courier Selection Card
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    width: Val::Px(560.0),
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.08, 0.10, 0.15, 0.9)),
                border_color: BorderColor(Color::srgb(0.0, 0.8, 1.0)),
                ..default()
            })
            .with_children(|card| {
                card.spawn(TextBundle::from_section(
                    format!("ACTIVE COURIER: {}", char_type.name().to_uppercase()),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.85, 0.2),
                        ..default()
                    },
                ));
                card.spawn(TextBundle::from_section(
                    format!("Role: {}", char_type.title()),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.7, 0.8, 0.9),
                        ..default()
                    },
                ));
                card.spawn(TextBundle::from_section(
                    format!("Ability: {}", char_type.ability_desc()),
                    TextStyle {
                        font_size: 15.0,
                        color: Color::srgb(0.0, 0.9, 1.0),
                        ..default()
                    },
                ));

                card.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            margin: UiRect::top(Val::Px(12.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.15, 0.2, 0.28)),
                        border_color: BorderColor(Color::srgb(0.3, 0.8, 1.0)),
                        ..default()
                    },
                    NextCharacterButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "SWITCH COURIER (TAB / CLICK)",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });

            // High Score Row
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    width: Val::Px(500.0),
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|stats_row| {
                stats_row.spawn(TextBundle::from_section(
                    format!("BEST DISTANCE: {:.0} M", stats.best_distance),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.6, 0.9, 0.6),
                        ..default()
                    },
                ));
                stats_row.spawn(TextBundle::from_section(
                    format!("HIGH SCORE: {}", stats.high_score),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(1.0, 0.8, 0.2),
                        ..default()
                    },
                ));
            });

            // Buttons
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(24.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(32.0), Val::Px(14.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.0, 0.5, 0.8)),
                        border_color: BorderColor(Color::srgb(0.0, 1.0, 1.0)),
                        ..default()
                    },
                    StartRunButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "INITIALIZE RUN [ENTER]",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(24.0), Val::Px(14.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.2, 0.15, 0.28)),
                        border_color: BorderColor(Color::srgb(0.8, 0.3, 0.9)),
                        ..default()
                    },
                    OpenStoryLogButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "ARCHIVE LOGS",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });

            // Footer / Settings Hint
            root.spawn(TextBundle::from_section(
                format!("[A/D] Lane | [W] Jump | [S] Slide | [Shift] Dash | [F2] Quality: {} | [F3] Perf Overlay", quality.tier.label()),
                TextStyle {
                    font_size: 13.0,
                    color: Color::srgb(0.5, 0.6, 0.7),
                    ..default()
                },
            ));
        });
}

// -------------------------------------------------------------
// 2. IN-GAME HUD
// -------------------------------------------------------------
fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            HudRoot,
        ))
        .with_children(|hud| {
            // Top Bar
            hud.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            })
            .with_children(|top| {
                // Top-Left: Distance & Zone
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tl| {
                    tl.spawn((
                        TextBundle::from_section(
                            "0 M",
                            TextStyle {
                                font_size: 38.0,
                                color: Color::srgb(0.0, 0.9, 1.0),
                                ..default()
                            },
                        ),
                        HudDistanceText,
                    ));
                    tl.spawn((
                        TextBundle::from_section(
                            "ZONE 1: OLD METRO",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::srgb(1.0, 0.6, 0.2),
                                ..default()
                            },
                        ),
                        HudZoneText,
                    ));
                    // Performance Metrics Display
                    tl.spawn((
                        TextBundle::from_section(
                            "60 FPS | 16.6ms | Ent: -- | Obs: --",
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(0.4, 0.8, 0.5),
                                ..default()
                            },
                        ),
                        PerfOverlayText,
                    ));
                });

                // Top-Center: Comms Toast Box
                top.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            max_width: Val::Px(480.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.05, 0.08, 0.12, 0.9)),
                        border_color: BorderColor(Color::srgb(0.0, 0.9, 1.0)),
                        ..default()
                    },
                    HudTransmissionBox,
                ))
                .with_children(|t_box| {
                    t_box.spawn((
                        TextBundle::from_section(
                            "// INCOMING TRANSMISSION",
                            TextStyle {
                                font_size: 13.0,
                                color: Color::srgb(1.0, 0.4, 0.1),
                                ..default()
                            },
                        ),
                        HudTransmissionTitle,
                    ));
                    t_box.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(0.9, 0.95, 1.0),
                                ..default()
                            },
                        ),
                        HudTransmissionBody,
                    ));
                });

                // Top-Center: Boss Alert & Countdown Banner
                top.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            max_width: Val::Px(520.0),
                            margin: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.16, 0.02, 0.06, 0.95)),
                        border_color: BorderColor(Color::srgb(1.0, 0.2, 0.25)),
                        ..default()
                    },
                    HudBossBannerBox,
                ))
                .with_children(|bb| {
                    bb.spawn((
                        TextBundle::from_section(
                            "⚠ ELITE THREAT DETECTED ⚠",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::srgb(1.0, 0.2, 0.3),
                                ..default()
                            },
                        ),
                        HudBossBannerTitle,
                    ));
                    bb.spawn((
                        TextBundle::from_section(
                            "SURVIVE GAUNTLET // 25.0s REMAINING",
                            TextStyle {
                                font_size: 13.0,
                                color: Color::srgb(0.9, 0.8, 1.0),
                                ..default()
                            },
                        ),
                        HudBossBannerStatus,
                    ));
                });

                // Top-Right: Fragments & Score
                top.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tr| {
                    tr.spawn((
                        TextBundle::from_section(
                            "ECHO: 0",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::srgb(0.2, 0.95, 1.0),
                                ..default()
                            },
                        ),
                        HudFragmentsText,
                    ));
                    tr.spawn((
                        TextBundle::from_section(
                            "CHIPS: 0",
                            TextStyle {
                                font_size: 18.0,
                                color: Color::srgb(1.0, 0.85, 0.2),
                                ..default()
                            },
                        ),
                        HudDataChipsText,
                    ));
                    tr.spawn((
                        TextBundle::from_section(
                            "SCORE: 0",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        HudScoreText,
                    ));
                });
            });

            // Center Threat Warning Banner (Immediate Tactical Readability)
            hud.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::None,
                        align_self: AlignSelf::Center,
                        margin: UiRect::top(Val::Px(50.0)),
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.2, 0.05, 0.02, 0.95)),
                    border_color: BorderColor(Color::srgb(1.0, 0.5, 0.0)),
                    ..default()
                },
                HudThreatBannerBox,
            ))
            .with_children(|tb| {
                tb.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(1.0, 0.85, 0.1),
                            ..default()
                        },
                    ),
                    HudThreatBannerText,
                ));
            });

            // Bottom Bar
            hud.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                ..default()
            })
            .with_children(|bot| {
                bot.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 17.0,
                            color: Color::srgb(0.2, 1.0, 0.8),
                            ..default()
                        },
                    ),
                    HudPowerupText,
                ));

                bot.spawn(TextBundle::from_section(
                    "[ESC] Pause | [F2] Quality | [F3] Perf Overlay",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.5, 0.6, 0.7),
                        ..default()
                    },
                ));
            });
        });
}

fn update_hud_display(
    stats: Res<GameRunStats>,
    powerups: Res<ActivePowerUps>,
    mut dist_q: Query<&mut Text, (With<HudDistanceText>, Without<HudFragmentsText>, Without<HudDataChipsText>, Without<HudScoreText>, Without<HudZoneText>, Without<HudPowerupText>)>,
    mut frag_q: Query<&mut Text, (With<HudFragmentsText>, Without<HudDistanceText>, Without<HudDataChipsText>, Without<HudScoreText>, Without<HudZoneText>, Without<HudPowerupText>)>,
    mut chip_q: Query<&mut Text, (With<HudDataChipsText>, Without<HudDistanceText>, Without<HudFragmentsText>, Without<HudScoreText>, Without<HudZoneText>, Without<HudPowerupText>)>,
    mut score_q: Query<&mut Text, (With<HudScoreText>, Without<HudDistanceText>, Without<HudFragmentsText>, Without<HudDataChipsText>, Without<HudZoneText>, Without<HudPowerupText>)>,
    mut zone_q: Query<&mut Text, (With<HudZoneText>, Without<HudDistanceText>, Without<HudFragmentsText>, Without<HudDataChipsText>, Without<HudScoreText>, Without<HudPowerupText>)>,
    mut pow_q: Query<&mut Text, (With<HudPowerupText>, Without<HudDistanceText>, Without<HudFragmentsText>, Without<HudDataChipsText>, Without<HudScoreText>, Without<HudZoneText>)>,
) {
    if let Ok(mut t) = dist_q.get_single_mut() {
        t.sections[0].value = format!("{:.0} M", stats.distance);
    }
    if let Ok(mut t) = frag_q.get_single_mut() {
        t.sections[0].value = format!("ECHO: {}", stats.fragments);
    }
    if let Ok(mut t) = chip_q.get_single_mut() {
        t.sections[0].value = format!("CHIPS: {}", stats.data_chips);
    }
    if let Ok(mut t) = score_q.get_single_mut() {
        t.sections[0].value = format!("SCORE: {}", stats.score);
    }
    if let Ok(mut t) = zone_q.get_single_mut() {
        let zone = get_zone_for_distance(stats.distance);
        if zone.end_distance.is_finite() {
            t.sections[0].value = format!("{} // {:.0}m - {:.0}m", zone.name, zone.start_distance, zone.end_distance);
        } else {
            t.sections[0].value = format!("{} // {:.0}m+", zone.name, zone.start_distance);
        }
    }
    if let Ok(mut t) = pow_q.get_single_mut() {
        let mut active = Vec::new();
        if powerups.shield {
            active.push(format!("SHIELD [x{}]", powerups.shield_hits));
        }
        if powerups.overdrive_timer > 0.0 {
            active.push(format!("OVERDRIVE ({:.1}s)", powerups.overdrive_timer));
        }
        if powerups.magnet_timer > 0.0 {
            active.push(format!("MAGNET ({:.1}s)", powerups.magnet_timer));
        }
        if powerups.double_jump_timer > 0.0 {
            active.push(format!("2x JUMP ({:.1}s)", powerups.double_jump_timer));
        }
        t.sections[0].value = active.join("  |  ");
    }
}

// -------------------------------------------------------------
// PERFORMANCE METRICS INSTRUMENTATION OVERLAY
// -------------------------------------------------------------
fn update_perf_overlay(
    diagnostics: Res<DiagnosticsStore>,
    quality: Res<QualitySettings>,
    obs_q: Query<&ActiveObstacle>,
    pool: Res<crate::pooling::EntityPool>,
    mut text_q: Query<(&mut Text, &mut Visibility), With<PerfOverlayText>>,
) {
    let (mut text, mut vis) = match text_q.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    if !quality.show_perf_overlay {
        *vis = Visibility::Hidden;
        return;
    }
    *vis = Visibility::Inherited;

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(60.0);

    let frame_time = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|ft| ft.smoothed())
        .unwrap_or(16.6);

    let entity_count = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|ec| ec.value())
        .unwrap_or(0.0) as u32;

    let active_obstacles = obs_q.iter().count();
    let pooled_count = pool.total_dormant();

    text.sections[0].value = format!(
        "{:.1} FPS | {:.2}ms | Ent: {} | Obs: {} | Pool: {} | [{}]",
        fps, frame_time, entity_count, active_obstacles, pooled_count, quality.tier.label()
    );
}

fn update_transmission_toast(
    time: Res<Time>,
    mut stats: ResMut<GameRunStats>,
    mut box_q: Query<&mut Style, With<HudTransmissionBox>>,
    mut title_q: Query<&mut Text, (With<HudTransmissionTitle>, Without<HudTransmissionBody>)>,
    mut body_q: Query<&mut Text, (With<HudTransmissionBody>, Without<HudTransmissionTitle>)>,
) {
    let dt = time.delta_seconds();
    let mut hide = true;

    if let Some(msg) = &mut stats.story_dialogue {
        msg.timer -= dt;
        if msg.timer > 0.0 {
            hide = false;
            if let Ok(mut t) = title_q.get_single_mut() {
                t.sections[0].value = format!("// COMMS: {} // {}", msg.sender, msg.title);
            }
            if let Ok(mut b) = body_q.get_single_mut() {
                b.sections[0].value = msg.body.clone();
            }
        } else {
            stats.story_dialogue = None;
        }
    }

    if let Ok(mut style) = box_q.get_single_mut() {
        style.display = if hide { Display::None } else { Display::Flex };
    }
}

fn update_boss_ui(
    time: Res<Time>,
    mut boss_state: ResMut<BossBattleState>,
    mut boss_started_reader: EventReader<BossStartedEvent>,
    mut boss_defeated_reader: EventReader<BossDefeatedEvent>,
    mut box_q: Query<(&mut Style, &mut BackgroundColor, &mut BorderColor), With<HudBossBannerBox>>,
    mut title_q: Query<&mut Text, (With<HudBossBannerTitle>, Without<HudBossBannerStatus>)>,
    mut status_q: Query<&mut Text, (With<HudBossBannerStatus>, Without<HudBossBannerTitle>)>,
) {
    let (mut style, mut bg, mut border) = match box_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let mut title = match title_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let mut status = match status_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    let dt = time.delta_seconds();

    // 1. Listen to BossStartedEvent (Single Source of Truth)
    for ev in boss_started_reader.read() {
        style.display = Display::Flex;
        *bg = BackgroundColor(Color::srgba(0.18, 0.02, 0.06, 0.95));
        *border = BorderColor(Color::srgb(1.0, 0.2, 0.25));
        title.sections[0].value = format!("⚠ ELITE ENCOUNTER: {} ⚠", ev.boss_name.to_uppercase());
        title.sections[0].style.color = Color::srgb(1.0, 0.2, 0.25);
        status.sections[0].value = "SURVIVE GAUNTLET // 25.0s // INTEGRITY: 100%".to_string();
    }

    // 2. Listen to BossDefeatedEvent (Single Source of Truth)
    for ev in boss_defeated_reader.read() {
        style.display = Display::Flex;
        *bg = BackgroundColor(Color::srgba(0.02, 0.16, 0.12, 0.95));
        *border = BorderColor(Color::srgb(0.0, 0.9, 0.6));
        title.sections[0].value = "✔ ELITE THREAT NEUTRALIZED ✔".to_string();
        title.sections[0].style.color = Color::srgb(0.0, 0.9, 0.6);
        status.sections[0].value = format!("+{} ECHO FRAGMENTS ACQUIRED // RESUMING RUN", ev.reward_fragments);
    }

    // 3. Update active countdown and display
    if boss_state.is_active {
        style.display = Display::Flex;
        status.sections[0].value = format!(
            "SURVIVE GAUNTLET // {:.1}s REMAINING // INTEGRITY: {:.0}%",
            boss_state.time_remaining,
            boss_state.boss_health
        );
    } else if boss_state.victory_banner_timer > 0.0 {
        boss_state.victory_banner_timer -= dt;
        style.display = Display::Flex;
    } else {
        style.display = Display::None;
    }
}

fn update_threat_alert_ui(
    threat_alerts: Res<ThreatAlertState>,
    mut box_q: Query<(&mut Style, &mut BackgroundColor, &mut BorderColor), With<HudThreatBannerBox>>,
    mut text_q: Query<&mut Text, With<HudThreatBannerText>>,
) {
    let (mut style, mut bg, mut border) = match box_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    let mut text = match text_q.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    if threat_alerts.scout_grappling {
        style.display = Display::Flex;
        *bg = BackgroundColor(Color::srgba(0.25, 0.02, 0.05, 0.95));
        *border = BorderColor(Color::srgb(1.0, 0.1, 0.2));
        text.sections[0].value = "⚠ PURSUER CLOSING IN // RECOVER BALANCE! ⚠".to_string();
        text.sections[0].style.color = Color::srgb(1.0, 0.2, 0.3);
    } else if let Some(lane) = threat_alerts.hunter_telegraph_lane {
        style.display = Display::Flex;
        *bg = BackgroundColor(Color::srgba(0.22, 0.1, 0.02, 0.95));
        *border = BorderColor(Color::srgb(1.0, 0.55, 0.0));
        text.sections[0].value = format!("⚠ INTERCEPTOR TARGETING LANE: {:?} // EVADE! ⚠", lane);
        text.sections[0].style.color = Color::srgb(1.0, 0.75, 0.1);
    } else if let Some(lane) = threat_alerts.heavy_warning_lane {
        style.display = Display::Flex;
        *bg = BackgroundColor(Color::srgba(0.04, 0.12, 0.18, 0.95));
        *border = BorderColor(Color::srgb(0.0, 0.8, 1.0));
        text.sections[0].value = format!("⚠ EMP BULKHEAD AHEAD IN LANE: {:?} ⚠", lane);
        text.sections[0].style.color = Color::srgb(0.2, 0.9, 1.0);
    } else {
        style.display = Display::None;
    }
}



fn handle_global_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut quality: ResMut<QualitySettings>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(AppState::Paused);
    }
    // F2: Cycle Quality Tier
    if keyboard.just_pressed(KeyCode::F2) {
        quality.tier = quality.tier.next();
    }
    // F3: Toggle Performance Overlay
    if keyboard.just_pressed(KeyCode::F3) {
        quality.show_perf_overlay = !quality.show_perf_overlay;
    }
}

fn handle_pause_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyP) {
        next_state.set(AppState::InGame);
    }
}

// -------------------------------------------------------------
// 3. PAUSE MENU
// -------------------------------------------------------------
fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.04, 0.08, 0.88)),
                ..default()
            },
            PauseRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "SYSTEM PAUSED",
                TextStyle {
                    font_size: 48.0,
                    color: Color::srgb(0.0, 0.9, 1.0),
                    ..default()
                },
            ));

            root.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(36.0), Val::Px(12.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.1, 0.4, 0.7)),
                    border_color: BorderColor(Color::srgb(0.0, 1.0, 1.0)),
                    ..default()
                },
                ResumeButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "RESUME RUN [ESC]",
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            });

            root.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(36.0), Val::Px(12.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    border_color: BorderColor(Color::srgb(0.6, 0.7, 0.8)),
                    ..default()
                },
                RestartButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "RESTART",
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            });

            root.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(36.0), Val::Px(12.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.3, 0.1, 0.1)),
                    border_color: BorderColor(Color::srgb(1.0, 0.2, 0.2)),
                    ..default()
                },
                BackToMenuButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "QUIT TO MAIN MENU",
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
            });
        });
}

// -------------------------------------------------------------
// 4. GAME OVER SCREEN
// -------------------------------------------------------------
fn setup_game_over(
    mut commands: Commands,
    stats: Res<GameRunStats>,
) {
    let zone = get_zone_for_distance(stats.distance);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(32.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.08, 0.02, 0.03, 0.95)),
                ..default()
            },
            GameOverRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "RUN TERMINATED",
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(1.0, 0.1, 0.15),
                    ..default()
                },
            ));
            root.spawn(TextBundle::from_section(
                "CARRIER NEURAL LINK SEVERED // VEYRON FORCES INTERCEPTED",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 0.5, 0.2),
                    ..default()
                },
            ));

            // Summary Stats Box
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::axes(Val::Px(0.0), Val::Px(24.0)),
                    width: Val::Px(450.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.12, 0.05, 0.08, 0.9)),
                border_color: BorderColor(Color::srgb(1.0, 0.2, 0.3)),
                ..default()
            })
            .with_children(|box_node| {
                box_node.spawn(TextBundle::from_section(
                    format!("DISTANCE SURVIVED:    {:.0} M", stats.distance),
                    TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("ECHO FRAGMENTS:       {}", stats.fragments),
                    TextStyle { font_size: 18.0, color: Color::srgb(0.0, 0.9, 1.0), ..default() },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("DATA CHIPS RECOVERED: {}", stats.data_chips),
                    TextStyle { font_size: 18.0, color: Color::srgb(1.0, 0.85, 0.2), ..default() },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("ZONE REACHED:         {}", zone.name),
                    TextStyle { font_size: 18.0, color: Color::srgb(1.0, 0.6, 0.1), ..default() },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("TOTAL SCORE:          {}", stats.score),
                    TextStyle { font_size: 20.0, color: Color::srgb(0.2, 1.0, 0.5), ..default() },
                ));
                box_node.spawn(TextBundle::from_section(
                    format!("ALL-TIME BEST:        {:.0} M", stats.best_distance),
                    TextStyle { font_size: 16.0, color: Color::srgb(0.7, 0.8, 0.9), ..default() },
                ));
            });

            // Action Buttons
            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.0, 0.6, 0.8)),
                        border_color: BorderColor(Color::srgb(0.0, 1.0, 1.0)),
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "RUN AGAIN [ENTER]",
                        TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                    ));
                });

                row.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                        border_color: BorderColor(Color::srgb(0.5, 0.5, 0.6)),
                        ..default()
                    },
                    BackToMenuButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "MAIN MENU",
                        TextStyle { font_size: 18.0, color: Color::WHITE, ..default() },
                    ));
                });
            });
        });
}

// -------------------------------------------------------------
// 5. STORY ARCHIVE LOGS
// -------------------------------------------------------------
fn setup_story_log_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(36.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.03, 0.04, 0.08, 0.98)),
                ..default()
            },
            StoryLogRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "DECRYPTED ECHO INCIDENT ARCHIVES",
                TextStyle {
                    font_size: 36.0,
                    color: Color::srgb(0.0, 0.9, 1.0),
                    ..default()
                },
            ));

            root.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    max_width: Val::Px(750.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|list| {
                for entry in DATA_CHIP_LOGS.iter() {
                    list.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(12.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.06, 0.08, 0.12, 0.8)),
                        border_color: BorderColor(Color::srgb(0.2, 0.4, 0.6)),
                        ..default()
                    })
                    .with_children(|card| {
                        card.spawn(TextBundle::from_section(
                            format!("{}: {}", entry.title, entry.sender),
                            TextStyle {
                                font_size: 15.0,
                                color: Color::srgb(1.0, 0.85, 0.2),
                                ..default()
                            },
                        ));
                        card.spawn(TextBundle::from_section(
                            entry.snippet,
                            TextStyle {
                                font_size: 13.0,
                                color: Color::srgb(0.8, 0.85, 0.9),
                                ..default()
                            },
                        ));
                    });
                }
            });

            root.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.1, 0.4, 0.6)),
                    border_color: BorderColor(Color::srgb(0.2, 0.8, 1.0)),
                    ..default()
                },
                BackToMenuButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "BACK TO MENU",
                    TextStyle { font_size: 16.0, color: Color::WHITE, ..default() },
                ));
            });
        });
}

// -------------------------------------------------------------
// 6. BUTTON INTERACTIONS & MENU ACTIONS
// -------------------------------------------------------------
fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                bg_color.0 = Color::srgb(0.1, 0.6, 0.9);
            }
            Interaction::Pressed => {
                bg_color.0 = Color::srgb(0.0, 0.8, 1.0);
            }
            Interaction::None => {}
        }
    }
}

fn menu_actions_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    cur_state: Res<State<AppState>>,
    mut stats: ResMut<GameRunStats>,
    mut powerups: ResMut<ActivePowerUps>,
    mut run_reset_events: EventWriter<RunResetEvent>,
    start_q: Query<&Interaction, (Changed<Interaction>, With<StartRunButton>)>,
    restart_q: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    resume_q: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    archive_q: Query<&Interaction, (Changed<Interaction>, With<OpenStoryLogButton>)>,
    back_q: Query<&Interaction, (Changed<Interaction>, With<BackToMenuButton>)>,
    next_char_q: Query<&Interaction, (Changed<Interaction>, With<NextCharacterButton>)>,
    all_game_entities: Query<Entity, Or<(With<Player>, With<TrackSegmentMarker>, With<ActiveObstacle>, With<CollectibleItem>, With<ChaserDrone>)>>,
    mut commands: Commands,
) {
    let mut do_start = false;
    let mut do_restart = false;
    let mut do_resume = false;
    let mut do_archive = false;
    let mut do_back = false;
    let mut do_next_char = false;

    for i in start_q.iter() {
        if *i == Interaction::Pressed { do_start = true; }
    }
    for i in restart_q.iter() {
        if *i == Interaction::Pressed { do_restart = true; }
    }
    for i in resume_q.iter() {
        if *i == Interaction::Pressed { do_resume = true; }
    }
    for i in archive_q.iter() {
        if *i == Interaction::Pressed { do_archive = true; }
    }
    for i in back_q.iter() {
        if *i == Interaction::Pressed { do_back = true; }
    }
    for i in next_char_q.iter() {
        if *i == Interaction::Pressed { do_next_char = true; }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        if *cur_state.get() == AppState::MainMenu {
            do_start = true;
        } else if *cur_state.get() == AppState::GameOver {
            do_restart = true;
        }
    }
    if keyboard.just_pressed(KeyCode::Tab) && *cur_state.get() == AppState::MainMenu {
        do_next_char = true;
    }

    if do_next_char {
        stats.selected_character = match stats.selected_character {
            CharacterType::Kai => CharacterType::Mira,
            CharacterType::Mira => CharacterType::Jax,
            CharacterType::Jax => CharacterType::Nyx,
            CharacterType::Nyx => CharacterType::Arin,
            CharacterType::Arin => CharacterType::Kai,
        };
        next_state.set(AppState::MainMenu);
    }

    if do_start || do_restart {
        // Broadcast unified run reset to all subsystems
        run_reset_events.send(RunResetEvent);

        stats.distance = 0.0;
        stats.fragments = 0;
        stats.data_chips = 0;
        stats.score = 0;
        stats.score_accum = 0.0;
        stats.current_zone = 1;
        stats.stumble_intensity = 0.0;
        stats.story_dialogue = None;

        *powerups = ActivePowerUps::default();

        next_state.set(AppState::InGame);
    }

    if do_resume {
        next_state.set(AppState::InGame);
    }

    if do_archive {
        next_state.set(AppState::StoryLog);
    }

    if do_back {
        for e in all_game_entities.iter() {
            commands.entity(e).despawn_recursive();
        }
        next_state.set(AppState::MainMenu);
    }
}
