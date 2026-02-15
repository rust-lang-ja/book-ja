fn main() {
    // ANCHOR: here
    let x = 'c';

    match x {
        // ASCII文字前半
        'a'..='j' => println!("early ASCII letter"),
        // ASCII文字後半
        'k'..='z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }
    // ANCHOR_END: here
}
