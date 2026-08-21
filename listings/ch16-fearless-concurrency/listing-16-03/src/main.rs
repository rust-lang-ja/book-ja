use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        //       "こちらがベクタ: {:?}"
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
