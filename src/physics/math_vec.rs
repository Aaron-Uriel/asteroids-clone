pub enum Quadrant { First, Second, Third, Fourth }


pub struct InternalMathVec {
    pub magnitude: f32,
    pub angle: f32
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

pub fn add(imvec1: InternalMathVec, imvec2: InternalMathVec) -> InternalMathVec {
    use std::f32;

    let x_axis_sum = imvec1.magnitude * imvec1.angle.cos() + imvec2.magnitude * imvec2.angle.cos();
    let y_axis_sum = imvec1.magnitude * imvec1.angle.sin() + imvec2.magnitude * imvec2.angle.sin();

    InternalMathVec {
        magnitude: f32::sqrt(x_axis_sum.powi(2) + y_axis_sum.powi(2)),
        angle: f32::atan(y_axis_sum / x_axis_sum) + match get_quadrant_from_components(x_axis_sum, y_axis_sum) {
            Quadrant::First => 0.0,
            Quadrant::Second | Quadrant::Third => f32::consts::PI,
            Quadrant::Fourth => 2.0 * f32::consts::PI
        }
    }
}