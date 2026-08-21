fn main() {
    let mut list = vec![1, 2, 3];
    //       "クロージャの定義前: {:?}"
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    //       "クロージャの呼び出し後: {:?}"
    println!("After calling closure: {:?}", list);
}
