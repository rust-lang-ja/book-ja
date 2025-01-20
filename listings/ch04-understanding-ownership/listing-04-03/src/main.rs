fn main() {
    let s = String::from("hello");  // s comes into scope
                                    // sがスコープに入る

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
                                    // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // x comes into scope
                                    // xがスコープに入る

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
                                    // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) { // some_string comes into scope
                                          // some_stringがスコープに入る。
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
  // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integer comes into scope
                                   // some_integerがスコープに入る
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
  // ここでsome_integerがスコープを抜ける。何も特別なことはない。
