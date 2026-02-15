extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // -3の絶対値は、Cによると{}
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
