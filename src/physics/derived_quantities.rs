use bevy_inspector_egui::Inspectable;
use super::math_vec::*;

#[derive(Inspectable)]
pub struct Velocity;
impl IsAVectorQuantity for Velocity {}

#[derive(Inspectable)]
pub struct Momentum;
impl IsAVectorQuantity for Momentum {}
