trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

fn main() {
    // 赤ちゃん犬は{}と呼ばれる
    println!("A baby dog is called a {}", Dog::baby_name());
}
