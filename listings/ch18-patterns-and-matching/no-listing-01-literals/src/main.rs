fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 => println!("one"),       // 1
        2 => println!("two"),       // 2
        3 => println!("three"),     // 3
        _ => println!("anything"),  // なにか
    }
    // ANCHOR_END: here
}
