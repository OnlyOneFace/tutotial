//函数和方法的实现

#[derive(Debug)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            x,
            y,
            r,
        }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r *self.r)
    }
}

pub fn apply<F>(f: F, y: i32) -> i32
    where F:Fn(i32) -> i32
{
    f(y)*y
}

pub fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}