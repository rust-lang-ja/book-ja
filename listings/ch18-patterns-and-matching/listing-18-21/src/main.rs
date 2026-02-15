fn main() {
    // ANCHOR: here
    // こんにちは！
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        // 文字列が見つかりました
        println!("found a string");
    }

    println!("{:?}", s);
    // ANCHOR_END: here
}
