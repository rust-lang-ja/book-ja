struct Point {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        // x軸上の{x}
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // y軸上の{y}
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // どちらの軸上でもない: ({x}, {y})
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
// ANCHOR_END: here
