fn main() {
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    //       "最初の要素は: {first}"
    println!("The first element is: {first}");
    // ANCHOR_END: here
}
