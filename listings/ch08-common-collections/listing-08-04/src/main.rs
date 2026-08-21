fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    //       "3つ目の要素は{third}です"
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        //                      "3つ目の要素は{third}です"
        Some(third) => println!("The third element is {third}"),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }
    // ANCHOR_END: here
}
