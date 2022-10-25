fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
                            // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // This will print `hello, world!`
                       // これは`hello, world!`と出力する
                       // ANCHOR_END: here
}
