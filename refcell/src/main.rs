/*
　内部可変性
　・そのデータへの不変参照があるときもデータを可変化できるデザインパターン
　・unsafeコードを使用し、通常の規則を捻じ曲げる
　・unsafeコードはコンパイラが規則をチェックすることに頼らず、自分でチェックしていることをコンパイラに表明する
　・コンパイラが保証できなくても、借用規則に実行時に従うことが保証できる場合に限り、内部可変性パターンを使用した型を使用できる
　・
*/

/*
　RefCell<T>
　・保持するデータに対して単独の所有権を表す
　・借用規則は実行時に強制される
　　cf 参照やBox<T>は借用規則はコンパイル時に強制される
　・借用規則を破ると、プログラムはパニックし、終了する
　・現在活動中のRef<T>, RefMut<T>スマートポインタの数を追いかける
　　・borrowを呼び出すたびに、活動中の不変参照の数を増やす
　　・Ref<T>の値がスコープを抜けると不変参照の数は1下がる
　　・RefCell<T>はいかなる時も、複数の不変借用または1つの可変借用を持たせることができる
　・シングルスレッドでのみ機能する
　　cf スレッドセーフなバージョンはMutex<T>
*/

/*
　スマートポインタの整理
　・Rc<T>は、同じデータに複数の所有者を持たせてくれる; Box<T>とRefCell<T>は単独の所有者
　・Box<T>では、不変借用も可変借用もコンパイル時に精査できる; Rc<T>では不変借用のみがコンパイル時に精査できる; RefCell<T>では、不変借用も可変借用も実行時に精査される。
　・RefCell<T>は実行時に精査される可変借用を許可するので、RefCell<T>が不変でも、 RefCell<T>内の値を可変化できる。 <- 内部可変性
*/

/*
　borrow
 ・スマートポインタ型のRef<T>を返す

　borrow_mut
　・スマートポインタ型のRefMut<T>を返す

　Ref<T> も RefMut<T> もDerefを実装しているため、普通の参照のように扱える
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 不変値に対して可変で借用はできない
    // let x = 5;
    // let y = &mut x;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
