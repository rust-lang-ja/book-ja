fn main() {
    // ANCHOR: here
    {                      // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // sは、ここから有効になる

        // sで作業をする
    }                      // このスコープは終わり。もうsは有効ではない
    // ANCHOR_END: here
}