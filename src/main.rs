//! main 函数入口
mod base;

use base::myenum;
use base::myfn;
use base::myio;
use base::mystruct;
use base::mytrait;
use std::cell;

/// 新手教程测试
fn main() {
    let point = mystruct::Point { x: 90.0, y: 100.0 };
    println! {"{:?}",point};

    let color = mystruct::Color::new(0xa4, 0xc6, 0x39);
    println! {"{:?}",color};

    let point1 = mystruct::Point { x: 63.3, ..point };
    println! {"{:?}",point1};

    let square = mystruct::Square::new(0.3, 0.3, cell::Cell::new(6.0));
    println! {"{:?}",square};
    square.side.set(9.3);
    println! {"{:?}",square};

    let this_enum = myenum::Message::MouseMove { x: 3, y: 6 };
    match this_enum {
        myenum::Message::MouseMove { x, y } => println! {"{},{}",x,y},
        myenum::Message::Quit => println! {"quit"},
        myenum::Message::ChangeColor(x, y, z) => println! {"{},{},{}",x,y,z},
        myenum::Message::Write(s) => println! {"{}",s},
    }

    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    let number = Option::Some(7);
    if let Option::Some(i) = number {
        println!("{:?}", i);
    } else {
        println!("not match");
    }
    let mut optional = Option::Some(0);
    while let Option::Some(i) = optional {
        if i > 9 {
            println!("more than 9");
            optional = Option::None;
        } else {
            println!("try again!{}", i);
            optional = Option::Some(i + 1);
        }
    }

    let y = 5;
    let z: i32 = if y == 5 { 10 } else { 15 };
    println! {"{}",z};

    for value in 0..20 {
        if value % 4 == 0 {
            println! {"{}",value};
        }
    }

    let mut i = 0;
    loop {
        i += 1;
        println!("Entered the outer loop");
        if i == 10 {
            println!("Entered the inner loop");
            break;
        }
    }
    println!("This point will never be reached {}", i);

    let mut day = 1;
    match day {
        e @ 1..=5 => {
            println!("got a range element {}", e);
            day += 1;
        }
        6 | 7 => println! {"weekday"},
        _ => println!("anything"),
    }
    println! {"day:{}",day};

    let box_fn = myfn::factory(1);
    let f2 = box_fn(2) * 2;
    let f3 = myfn::apply(box_fn, 2);
    println!("{},{}", f2, f3);

    let c = myfn::Circle::new(0.1, 0.5, 3.5);
    println!("{}", c.area());

    let c1 = mytrait::Circle {
        x: 0.0,
        y: 0.5,
        r: 3.0,
    };
    mytrait::print_area(c1);

    let s1 = mytrait::Square {
        x: 0.0,
        y: 0.5,
        side: 3.0,
    };
    mytrait::print_area(s1);

    let mystr = myio::read_io();
    println! {"{}",mystr.trim()};

    myio::std_out();

    myio::file_open();

    myio::file_write();

    let s = format!(
        "{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子\n",
        81,
        "wayslog",
        width = 4,
        height = 178
    );
    print!("{}", s);

    let s1 = format!(
        "rust.cc社区的唐{CaiNiao}眼睛度数足有{0:>06.1}度却还是每天辛苦码代码才能赚到100个{3}。\n但是{2}却只需睡{1:^4$}个小时就可以迎娶白富美了。\n",
        500.0,
        12,
        "ELTON",
        "QB",
        4,
        CaiNiao = "Mike"
    );
    print!("{}", s1);

    let mut x = 5;
    match x {
        ref mut mr => println!("mut ref :{}", mr),
    }
    // 当然了……在let表达式里也能用
    println! {"{}",x};

    let x1 = 4;
    let y1 = false;
    match x1 {
        4 | 5 if y1 => println!("yes"),
        _ => println!("no"),
    }
    println!("{}", y1);

    let p1 = mytrait::Point { x: 6u8, y: 8u8 };
    let p2 = mytrait::Point { x: 3u8, y: 9u8 };
    println! {"{:?}",p1+p2};
}
