use bevy_inspector_egui::Inspectable;
use super::math_vec::{*, self};

#[derive(Inspectable)]
pub struct Velocity {
    pub magnitude: f32,
    pub angle: f32
}

impl Velocity {
    pub fn vec_add(&mut self, other_vec: Velocity) {
        let result = math_vec::add(
            InternalMathVec { magnitude: self.magnitude, angle: self.angle },
            InternalMathVec { magnitude: other_vec.magnitude, angle: other_vec.angle }
        );

        self.magnitude = result.magnitude;
        self.angle = result.angle;
    }
}

#[derive(Inspectable)]
pub struct Momentum {
    pub magnitude: f32,
    pub angle: f32
}

impl Momentum {
    pub fn vec_add(&mut self, other_vec: Momentum) {
        let result = math_vec::add(
            InternalMathVec { magnitude: self.magnitude, angle: self.angle },
            InternalMathVec { magnitude: other_vec.magnitude, angle: other_vec.angle }
        );

        self.magnitude = result.magnitude;
        self.angle = result.angle;
    }
}
