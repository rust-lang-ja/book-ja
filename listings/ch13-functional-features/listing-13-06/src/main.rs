use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    //       "クロージャの定義前: {:?}"
    println!("Before defining closure: {:?}", list);

    //                             "スレッドから: {:?}"
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
