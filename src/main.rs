mod base;

use base::fnstruct;
use base::mytrait;

fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    let number = Some(7);
    if let Some(i) = number {
        println!("{:?}", i);
    } else {
        println!("not match");
    }
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("more than 9");
           optional = None;
        }else {
            println!("try again!{}", i);
            optional = Some(i + 1);
        }
    }

    let box_fn = mycircle::factory(1);
    let f2 = box_fn(2) * 2;
    let f3 = mycircle::apply(box_fn, 2);
    println!("{},{}", f2, f3);

    let c = mycircle::Circle::new(0.1, 0.5, 3.5);
    println!("{}", c.area());

    let c1 = mytrait::Circle {x: 0.0, y: 0.5, r: 3.0};
    mytrait::print_area(c1);

    let s1 = mytrait::Square {x: 0.0, y: 0.5, side: 3.0};
    mytrait::print_area(s1);
}

