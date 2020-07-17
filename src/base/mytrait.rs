//特性实践

pub trait HasArea {
    fn area(&self) -> f64;
}


#[derive(Debug)]
pub struct Circle {
   pub x: f64,
   pub y: f64,
   pub  r: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r * self.r)
    }
}

#[derive(Debug)]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn print_area<T: HasArea>(shape: T) {
    println!("area is {}", shape.area());
}