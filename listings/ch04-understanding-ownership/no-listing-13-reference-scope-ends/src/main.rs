fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // no problem
                 // 問題なし
    let r2 = &s; // no problem
                 // 問題なし
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // r1とr2はこれ以降使われません

    let r3 = &mut s; // no problem
                     // 問題なし
    println!("{}", r3);
    // ANCHOR_END: here
}
