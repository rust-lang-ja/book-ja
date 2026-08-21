// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    // ANCHOR: here
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        //      "ファイルを読み込むことができるはずでした"
        .expect("Should have been able to read the file");

    //       "テキスト:\n{contents}"
    println!("With text:\n{contents}");
}
// ANCHOR_END: here
