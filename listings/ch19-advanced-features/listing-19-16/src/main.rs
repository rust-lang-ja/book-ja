// ANCHOR: here
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        // こちら機長です。
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        // 上がれ！
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        // *激しく腕を振る*
        println!("*waving arms furiously*");
    }
}
// ANCHOR_END: here

fn main() {}
