/*
　Derefトレイト
　・参照外し演算子 * の振る舞いをカスタマイズできる
*/

/*
　参照外し型強制
　・Derefトレイトを実装する型への参照を、別の型への参照に変換する
　ex) Stringは&strを返すようにDerefトレイトを実装しているため、参照外し型強制で&Stringを&strに変換できる
　・特定の型の値への参照を関数やメソッド定義の引数型と一致しない引数として渡すときに自動的に発生する
　・derefメソッドの呼び出しが提供した型を引数が必要とする型に変換
*/

/*
　DerefMutトレイト
　・可変参照の*演算子をオーバーライドできる
*/

/*
　以下の3つの場合に型やトレイト実装を見つけたときにコンパイラは参照外し型強制を行う
　・T: Deref<Target=U>のとき、&T から &U
　・T: DerefMut<Target=U>のとき、 &mut T から &mut U
　・T: Deref<Target=U>のとき、&mut T から &U
    -> 可変参照を不変参照にも型強制する (ただし、不変参照は可変参照に型強制されない)
*/

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // *y -> 参照外しして、値にアクセス

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 参照外しでBox<T>のポインタを辿る

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // assert_eq!(5, *y); // MyBoxが参照を外すための実装(-> Derefトレイト)がされていないためエラー

    let x = 5;
    let y = MyBox2::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 水面下で*(y.deref())が実行されている

    let m = MyBox2::new(String::from("Rust"));
    hello(&m); // &mは値への参照　&MyBox2<String> を &Stringに変換し、さらに&strに変換している
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

struct MyBox2<T>(T);

impl<T> Deref for MyBox2<T> {
    // Derefトレイトが使用する関連型を定義
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // .0はタプルの最初の要素にアクセス
    }
}

impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
