use std::{f32, marker::PhantomData};
use bevy_inspector_egui::Inspectable;

pub enum Quadrant { First, Second, Third, Fourth }

pub trait IsAVectorQuantity {}

#[derive(Inspectable)]
pub struct MathVec<T: IsAVectorQuantity> {
    pub magnitude: f32,
    pub angle: f32,

    #[inspectable(ignore)]
    _kind: PhantomData<T>
}

impl<T: IsAVectorQuantity> MathVec<T> {
    pub fn new(magnitude: f32, angle: f32) -> MathVec<T> {
        MathVec { magnitude: magnitude, angle: angle, _kind: PhantomData }
    }

    fn get_quadrant_from_components(x_comp: f32, y_comp: f32) -> Quadrant {
        if x_comp.is_sign_positive() && y_comp.is_sign_positive() {
            Quadrant::First
        }
        else if x_comp.is_sign_positive() {
            Quadrant::Fourth
        }
        else if y_comp.is_sign_positive() {
            Quadrant::Second
        }
        else {
            Quadrant::Third
        }
    }

    pub fn vec_add(&mut self, mvec: MathVec<T>) {
        let x_axis_sum = self.magnitude * self.angle.cos() + mvec.magnitude * mvec.angle.cos() + f32::EPSILON;
        let y_axis_sum = self.magnitude * self.angle.sin() + mvec.magnitude * mvec.angle.sin() + f32::EPSILON;

        self.magnitude = f32::sqrt(x_axis_sum.powi(2) + y_axis_sum.powi(2));
        self.angle = f32::atan(y_axis_sum / x_axis_sum) + match MathVec::<T>::get_quadrant_from_components(x_axis_sum, y_axis_sum) {
            Quadrant::First => 0.0,
            Quadrant::Second | Quadrant::Third => f32::consts::PI,
            Quadrant::Fourth => 2.0 * f32::consts::PI
        };
    }
}