fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // no problem
                 // 問題なし
    let r2 = &s; // no problem
                 // 問題なし
    let r3 = &mut s; // BIG PROBLEM
                     // 大問題！

    println!("{}, {}, and {}", r1, r2, r3);
    // ANCHOR_END: here
}
