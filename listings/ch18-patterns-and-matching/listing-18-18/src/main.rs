fn main() {
    // ANCHOR: here
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            // 既存の値の変更を上書きできません
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // 設定は{:?}です
    println!("setting is {:?}", setting_value);
    // ANCHOR_END: here
}
