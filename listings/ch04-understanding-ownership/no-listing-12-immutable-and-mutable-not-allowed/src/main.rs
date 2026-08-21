fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！

    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
