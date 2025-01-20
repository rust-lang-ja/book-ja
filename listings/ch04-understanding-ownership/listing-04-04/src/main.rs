fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
                                        // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let s2 = String::from("hello");     // s2 comes into scope
                                        // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
                                        // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
  // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
                                             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("yours"); // some_string comes into scope
                                             // some_stringがスコープに入る

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
                                             // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// This function takes a String and returns one
// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
                                                      // a_stringがスコープに入る。

    a_string  // a_string is returned and moves out to the calling function
              // a_stringが返され、呼び出し元関数にムーブされる
}
