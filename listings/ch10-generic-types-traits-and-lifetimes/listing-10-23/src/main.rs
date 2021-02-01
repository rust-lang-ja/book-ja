// ANCHOR: here
fn main() {
    // 長い文字列は長い
    let string1 = String::from("long string is long");
    // （訳注：この言葉自体に深い意味はない。下の"xyz"より長いということだけが重要）

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // 一番長い文字列は{}
        println!("The longest string is {}", result);
    }
}
// ANCHOR_END: here

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
