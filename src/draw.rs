use bevy::prelude::*;
use crate::{fluids::particle::FluidParticle, kinetics::{acceleration::Acceleration, forces::Forces, mass::Mass, velocity::Velocity}};

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
    let p1 = FluidParticle{radius:20.};

    for i in 1..2000 {
        commands.spawn((
            p1,
            Mesh2d(meshes.add(p1)),
            MeshMaterial2d(materials.add(Color::hsl((260/i) as f32 , 0.95, 0.7))),
            Transform::from_xyz(
                (400/i)as f32 -200.  ,
                (200/i) as f32,
                i as f32,
            ),
            Velocity(Vec2::new(10.,10.)),
            Acceleration(Vec2::new(0.,0.)),
            Mass(1.),
            Forces(vec![]),
        ));
    }
}
