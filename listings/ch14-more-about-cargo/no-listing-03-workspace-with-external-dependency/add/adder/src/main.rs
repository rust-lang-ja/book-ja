use add_one;

fn main() {
    let num = 10;
    println!(
    	// こんにちは世界！{}+1は{}!
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
