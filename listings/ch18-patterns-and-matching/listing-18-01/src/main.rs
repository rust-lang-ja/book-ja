fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        //       "あなたのお気に入りの色、{color}を背景色に使用します"
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        //       "火曜日は緑の日！"
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            //       "紫を背景色に使用します"
            println!("Using purple as the background color");
        } else {
            //       "オレンジを背景色に使用します"
            println!("Using orange as the background color");
        }
    } else {
        //       "青を背景色に使用します"
        println!("Using blue as the background color");
    }
}
