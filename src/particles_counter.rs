use bevy::prelude::*;

use crate::fluids::particle::FluidParticle;

pub struct ParticlesCounterPlugin;

impl Plugin for ParticlesCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_counter)
            .add_systems(Update, update_counter);
    }
}

fn spawn_counter(mut commands: Commands) {
    commands
        .spawn((
            Text::new("#particles: "),
            TextFont {
                font_size: 32.,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                right: Val::Px(5.),
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
            CounterText,
        ));
}

fn update_counter(
    particles_q: Query<&FluidParticle>,
    mut text_q: Query<&mut TextSpan, With<CounterText>>,
) {
    let amount = particles_q.iter().count();
    let mut span = text_q.single_mut();
    **span = format!("{}", amount);
}

#[derive(Component)]
struct CounterText;
