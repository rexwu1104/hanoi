use opencv::core;

use super::traits::Renderable;

pub struct Hanoi {
    pub width: i32,
    pub height: i32,
    pub columns: i32,
    pub renderables: Vec<Box<dyn Renderable>>,
}

pub struct Stack {
    pub width: i32,
    pub height: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub renderables: Vec<Box<dyn Renderable>>
}

pub struct StackPlate {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: core::VecN<f64, 4>
}

pub struct ColorGenerator(pub usize, pub [usize; 3], pub usize, pub usize);