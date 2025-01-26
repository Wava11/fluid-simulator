use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_fps)
            .add_systems(Update, update_fps);
    }
}

fn spawn_fps(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 32.,
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

#[derive(Component)]
struct FpsText;
