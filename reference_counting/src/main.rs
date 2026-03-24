/*
　Rc<T> reference couting
　・値がまだ使用中かどうか決定するために、値への参照の数を追跡する
　・値への参照が0なら、値は片付けられる
　・単独の値に複数の所有者を持たせることができ、 所有者のいずれかが存在している限り、値が有効であり続けることをカウントは保証する
　・不変参照経由でプログラムの複数箇所間でデータを共有できる
*/

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); <- a は bにムーブされてるため、エラー

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
