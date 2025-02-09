use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct PerformanceMonitorPlugin;

impl Plugin for PerformanceMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_monitor)
            .add_systems(
                Update,
                (
                    update_fps,
                    update_collision_detection_duration,
                    update_collision_detection_checked_pairs,
                    update_collision_detection_colliding_pairs,
                ),
            );
    }
}

fn spawn_monitor(mut commands: Commands) {
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 32.,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            },
        ))
        .with_child((
            (
                TextSpan::default(),
                TextFont {
                    font_size: 32.,
                    ..default()
                },
            ),
            FpsText,
        ));

    commands
        .spawn((
            Text::new("Collision detection took: "),
            TextFont {
                font_size: 32.,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(40.),
                left: Val::Px(5.),
                ..default()
            },
        ))
        .with_child((
            (
                TextSpan::default(),
                TextFont {
                    font_size: 32.,
                    ..default()
                },
            ),
            CollisionDetectionDurationText,
        ));
    commands
        .spawn((
            Text::new("Checked pairs: "),
            TextFont {
                font_size: 32.,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(80.),
                left: Val::Px(5.),
                ..default()
            },
        ))
        .with_child((
            (
                TextSpan::default(),
                TextFont {
                    font_size: 32.,
                    ..default()
                },
            ),
            CollisionDetectionCheckedPairsText,
        ));
    commands
        .spawn((
            Text::new("Colliding pairs: "),
            TextFont {
                font_size: 32.,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(120.),
                left: Val::Px(5.),
                ..default()
            },
        ))
        .with_child((
            (
                TextSpan::default(),
                TextFont {
                    font_size: 32.,
                    ..default()
                },
            ),
            CollisionDetectionCollidingPairsText,
        ));

    commands.insert_resource(CollisionDetectionMonitor {
        duration: Duration::new(0, 0),
        checked_pairs: 0,
        colliding_pairs: 0,
    })
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut TextSpan, With<FpsText>>) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}

fn update_collision_detection_duration(
    collision_detection_monitor: Res<CollisionDetectionMonitor>,
    mut duration_text_query: Query<&mut TextSpan, With<CollisionDetectionDurationText>>,
) {
    for mut span in &mut duration_text_query {
        **span = format!("{:?}", collision_detection_monitor.duration);
    }
}

fn update_collision_detection_checked_pairs(
    collision_detection_monitor: Res<CollisionDetectionMonitor>,
    mut checked_pairs_text_query: Query<&mut TextSpan, With<CollisionDetectionCheckedPairsText>>,
    // mut colliding_pairs_text_query: Query<
    //     &mut TextSpan,
    //     With<CollisionDetectionCollidingPairsText>,
    // >,
) {
    for mut span in &mut checked_pairs_text_query {
        **span = format!("{:?}", collision_detection_monitor.checked_pairs);
    }
    // for mut span in &mut colliding_pairs_text_query {
    //     **span = format!("{:?}", collision_detection_monitor.colliding_pairs);
    // }
}

fn update_collision_detection_colliding_pairs(
    collision_detection_monitor: Res<CollisionDetectionMonitor>,
    mut colliding_pairs_text_query: Query<
        &mut TextSpan,
        With<CollisionDetectionCollidingPairsText>,
    >,
) {
    for mut span in &mut colliding_pairs_text_query {
        **span = format!("{:?}", collision_detection_monitor.colliding_pairs);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct CollisionDetectionDurationText;

#[derive(Component)]
struct CollisionDetectionCheckedPairsText;
#[derive(Component)]
struct CollisionDetectionCollidingPairsText;

#[derive(Resource)]
pub struct CollisionDetectionMonitor {
    pub duration: Duration,
    pub checked_pairs: usize,
    pub colliding_pairs: usize,
}
