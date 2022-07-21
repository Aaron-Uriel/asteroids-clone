use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;
use super::math_vec::MathVec32;

#[derive(Inspectable, Component)]
pub struct Velocity(MathVec32);

impl Velocity {
    pub fn new(magnitude: f32, angle: f32) -> Velocity {
        Velocity(MathVec32::new(magnitude, angle))
    }

    pub fn vector_add(&mut self, other_vec: Velocity) {
        self.0.vec_add(other_vec.0);
    }

    pub fn get_magnitude(&self) -> f32{
        self.0.get_magnitude()
    }

    pub fn get_angle(&self) -> f32 {
        self.0.get_angle()
    }
}