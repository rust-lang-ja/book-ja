use add_one;

fn main() {
    let num = 10;
    //       "こんにちは世界！{num}+1は{}!"
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
