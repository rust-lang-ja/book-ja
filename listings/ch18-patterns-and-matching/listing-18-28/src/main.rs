fn main() {
    // ANCHOR: here
    let x = 4;
    let y = false;

    match x {
        // はい
        4 | 5 | 6 if y => println!("yes"),
        // いいえ
        _ => println!("no"),
    }
    // ANCHOR_END: here
}
