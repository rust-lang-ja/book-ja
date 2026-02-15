enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // Quit列挙子には分配すべきデータがない
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            // x方向に{x}、y方向に{y}だけ動く
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            // テキストメッセージ: {text}
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            // 色を赤{r}, 緑{g}, 青{b}に変更
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
