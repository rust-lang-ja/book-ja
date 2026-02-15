fn main() {
    // ANCHOR: here
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // 範囲内のidが見つかりました: {}
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // 別の範囲内のidが見つかりました
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // それ以外のidが見つかりました
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    // ANCHOR_END: here
}
