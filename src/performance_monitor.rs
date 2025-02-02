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
            .add_systems(Update, (update_fps, update_collision_detection_duration));
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

    commands.insert_resource(CollisionDetectionDuration(Duration::new(0, 0)));
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
    collision_detection_duration: Res<CollisionDetectionDuration>,
    mut text_query: Query<&mut TextSpan, With<CollisionDetectionDurationText>>,
) {
    for mut span in &mut text_query {
        **span = format!("{:?}", collision_detection_duration.0);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct CollisionDetectionDurationText;
#[derive(Resource)]
pub struct CollisionDetectionDuration(pub Duration);
