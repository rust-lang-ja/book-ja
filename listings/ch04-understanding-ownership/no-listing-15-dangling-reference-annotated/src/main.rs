fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle returns a reference to a String
                         // dangleはStringへの参照を返す

    let s = String::from("hello"); // s is a new String
                                   // sは新しいString

    &s // we return a reference to the String, s
       // String sへの参照を返す
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
  // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ
// ANCHOR_END: here
