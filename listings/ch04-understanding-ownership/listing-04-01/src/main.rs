fn main() {
    // ANCHOR: here
    {                      // s is not valid here, it’s not yet declared
                           // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // s is valid from this point forward
                           // sは、ここから有効になる

        // do stuff with s
        // sで作業をする
    }                      // this scope is now over, and s is no longer valid
                           // このスコープは終わり。もうsは有効ではない
    // ANCHOR_END: here
}