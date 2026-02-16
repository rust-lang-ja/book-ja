fn main() {
    // ANCHOR: here
    let x = Some(5);
    let y = 10;

    match x {
        // 50だったよ
        Some(50) => println!("Got 50"),
        // マッチしたよ、y = {y}
        Some(y) => println!("Matched, y = {y}"),
        // 既定のケース、x = {:?}
        _ => println!("Default case, x = {:?}", x),
    }

    // 最後には: x = {:?}, y = {y}
    println!("at the end: x = {:?}, y = {y}", x);
    // ANCHOR_END: here
}
