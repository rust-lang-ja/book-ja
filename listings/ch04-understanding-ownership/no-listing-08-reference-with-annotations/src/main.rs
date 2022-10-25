fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
  // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
  // 何も起こらない
// ANCHOR_END: here
