fn main() {
    // ANCHOR: here
    let num = Some(4);

    match num {
    	// 数{}は偶数です
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    	// 数{}は奇数です
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
    // ANCHOR_END: here
}
