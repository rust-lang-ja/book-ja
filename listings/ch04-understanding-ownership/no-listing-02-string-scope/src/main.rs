fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // sはここから有効になる

        // sで作業をする
    }                                  // このスコープはここでおしまい。sは
                                       // もう有効ではない
    // ANCHOR_END: here
}
