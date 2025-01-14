use bevy::prelude::*;

#[derive(Component,Clone, Copy)]
pub struct FluidParticle {
    pub radius: f32,
    pub restitution_coeff: f32
}

impl Into<Mesh> for FluidParticle {
    fn into(self) -> Mesh {
        Circle::new(self.radius).into()
    }
}
