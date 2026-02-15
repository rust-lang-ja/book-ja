fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        // 1か2
        1 | 2 => println!("one or two"),
        // 3
        3 => println!("three"),
        // なにか
        _ => println!("anything"),
    }
    // ANCHOR_END: here
}
