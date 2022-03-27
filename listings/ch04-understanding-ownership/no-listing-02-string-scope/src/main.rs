fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // sはここから有効になる

        // do stuff with s
        // sで作業をする
    }                                  // this scope is now over, and s is no
                                       // longer valid
                                       // このスコープはここでおしまい。sは
                                       // もう有効ではない
    // ANCHOR_END: here
}
