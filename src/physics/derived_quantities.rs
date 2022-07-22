use bevy_inspector_egui::Inspectable;
use super::math_vec::*;

#[derive(Inspectable)]
pub struct Velocity {
    pub magnitude: f32,
    pub angle: f32
}

impl Velocity {
    pub fn vec_add(&mut self, mut other_vec: Velocity) {
        self.as_generic().vec_add(other_vec.as_generic());
    }

    fn as_generic(&mut self) -> GenericMathVec {
        GenericMathVec {
            magnitude: &mut self.magnitude,
            angle: &mut self.angle
        }
    }
}

#[derive(Inspectable)]
pub struct Momentum {
    pub magnitude: f32,
    pub angle: f32
}

impl Momentum {
    pub fn vec_add(&mut self, mut other_vec: Momentum) {
        self.as_generic().vec_add(other_vec.as_generic());
    }

    fn as_generic(&mut self) -> GenericMathVec {
        GenericMathVec {
            magnitude: &mut self.magnitude,
            angle: &mut self.angle
        }
    }
}
