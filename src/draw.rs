use bevy::prelude::*;
use crate::kinetics::{acceleration::Acceleration, forces::Forces, mass::Mass, velocity::Velocity};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_circle);
    }
}

fn draw_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::hsl(260 as f32, 0.95, 0.7))),
        Transform::from_xyz(
            -300.,
            100.,
            200.,
        ),
        Velocity(Vec2::new(10.,10.)),
        Acceleration(Vec2::new(0.,0.)),
        Mass(1.),
        Forces(vec![]),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::hsl(160 as f32, 0.95, 0.7))),
        Transform::from_xyz(
            -300.,
            0.,
            200.,
        ),
        Velocity(Vec2::new(10.,10.)),
        Acceleration(Vec2::new(0.,-9.8)),
        Mass(1.),
        Forces(vec![]),
    ));
}
