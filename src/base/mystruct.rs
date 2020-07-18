//结构体和枚举的实践
use std::cell::Cell;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color(red, green, blue)
    }
}

#[derive(Debug)]
pub struct Square {
    x: f64,
    y: f64,
    pub side: Cell<f64>,
}

impl Square {
    pub fn new(x: f64, y: f64, side: Cell<f64>) -> Square {
        Square { x, y, side }
    }
}
