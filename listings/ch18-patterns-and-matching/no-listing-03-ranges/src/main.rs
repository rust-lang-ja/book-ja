fn main() {
    // ANCHOR: here
    let x = 5;

    match x {
        // 1から5まで
        1..=5 => println!("one through five"),
        // それ以外
        _ => println!("something else"),
    }
    // ANCHOR_END: here
}
