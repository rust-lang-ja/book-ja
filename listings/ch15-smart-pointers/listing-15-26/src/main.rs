use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    //       "aの最初の参照カウント = {}"
    println!("a initial rc count = {}", Rc::strong_count(&a));
    //       "aの次の要素は = {:?}"
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    //       "b作成後のaの参照カウント = {}"
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    //       "bの最初の参照カウント = {}"
    println!("b initial rc count = {}", Rc::strong_count(&b));
    //       "bの次の要素 = {:?}"
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    //       "aを変更後のbの参照カウント = {}"
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    //       "aを変更後のaの参照カウント = {}"
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 次の行のコメントを外して循環していると確認してください;
    // スタックオーバーフローします
    // //       "aの次の要素 = {:?}"
    // println!("a next item = {:?}", a.tail());
}
// ANCHOR_END: here
