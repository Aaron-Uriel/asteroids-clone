pub enum Quadrant { First, Second, Third, Fourth }

pub struct GenericMathVec<'a> {
    pub magnitude: &'a mut f32,
    pub angle: &'a mut f32
}

impl<'a> GenericMathVec<'a> {
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
    pub fn vec_add(&mut self, other_vec: GenericMathVec) {
        use std::f32;

        let x_axis_sum = *self.magnitude * self.angle.cos() + *other_vec.magnitude * other_vec.angle.cos() + f32::EPSILON;
        let y_axis_sum = *self.magnitude * self.angle.sin() + *other_vec.magnitude * other_vec.angle.sin() + f32::EPSILON;

        *self.magnitude = f32::sqrt(x_axis_sum.powi(2) + y_axis_sum.powi(2));
        *self.angle = f32::atan(y_axis_sum / x_axis_sum) + match Self::get_quadrant_from_components(x_axis_sum, y_axis_sum) {
            Quadrant::First => 0.0,
            Quadrant::Second | Quadrant::Third => f32::consts::PI,
            Quadrant::Fourth => 2.0 * f32::consts::PI
        };
    }
}