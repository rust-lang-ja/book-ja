struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        //       "データ`{}`を持つCustomSmartPointerをドロップ中!"
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
    	//                 "俺のもの"
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
    	//                 "別のもの"
        data: String::from("other stuff"),
    };
    //       "CustomSmartPointerが作成された。"
    println!("CustomSmartPointers created.");
}
