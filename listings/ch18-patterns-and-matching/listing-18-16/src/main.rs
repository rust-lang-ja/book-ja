enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            // 色を赤{r}, 緑{g}, 青{b}に変更
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
        	// 色を色相{h}, 彩度{s}, 明度{v}に変更
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
