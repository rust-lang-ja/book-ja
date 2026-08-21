fn main() {
    let list = vec![1, 2, 3];
    //       "クロージャの定義前: {:?}"
    println!("Before defining closure: {:?}", list);

    //                             "クロージャから: {:?}"
    let only_borrows = || println!("From closure: {:?}", list);

    //       "クロージャの呼び出し前: {:?}"
    println!("Before calling closure: {:?}", list);
    only_borrows();
    //       "クロージャの呼び出し後: {:?}"
    println!("After calling closure: {:?}", list);
}
