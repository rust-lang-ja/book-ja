use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    //       "{}を検索します"
    println!("Searching for {}", query);
    //       "ファイル{}の中で"
    println!("In file {}", file_path);
}
