use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
    	//      "hello.txtがこのプロジェクトに含まれているべきです"
        .expect("hello.txt should be included in this project");
}
