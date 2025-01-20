fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
      // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let r2 = &mut s;
    // ANCHOR_END: here
}
