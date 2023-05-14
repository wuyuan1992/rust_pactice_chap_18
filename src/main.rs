enum Event {
    Click,
    Move { x: f32, y: f32 },
    PickColor(u8, u8, u8),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    match_points();
    match_events();
    omit_param(1, 2, 3);
    match_with_if();
}

fn match_points() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 3 },
        Point { x: 5, y: 7 },
    ];

    for point in points {
        match point {
            Point { x: 0, y: 0 } => println!("origin"),
            Point { x: 0, y } => println!("x axis, y = {}", y),
            Point { x, y: 0 } => println!("y axis, x = {}", x),
            Point { x, y } => println!("regular ({}, {})", x, y),
        }
    }
}

fn match_events() {
    let events = vec![
        Event::PickColor(0, 160, 255),
        Event::Move { x: 1.0, y: 2.0 },
        Event::Click,
    ];

    for evt in events {
        match evt {
            Event::Click => println!("Click"),
            Event::Move { x, y } => println!("Move: {}, {}", x, y),
            Event::PickColor(r, g, b) => println!("PickColor: {}, {}, {}", r, g, b),
        }
    }
}

fn omit_param(_: i32, _: i32, z: i32) {
    println!("z: {}", z);
}

fn omit_match() {
    let (_, _, z) = (1, 2, 3);
    println!("z: {}", z);
}

fn match_with_if() {
    let default_value = 5;

    let val = Some(10);

    match val {
        Some(x) if x == default_value => println!("default value"),
        Some(x) => println!("non-default value: {}", x),
        None => println!("no value"),
    }
}
