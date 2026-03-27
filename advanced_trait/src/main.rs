/*
　関連型
　・トレイトのメソッド定義がシグニチャでプレースホルダーの型を使用できるように、
　　トレイトと型のプレースホルダーを結びつける
*/

/*
　演算子オーバーロード
　・特定の状況で演算子の振る舞いをカスタマイズする
　・演算子に紐づいたトレイトを実装することで、対応するトレイトをオーバーロードできる
*/

/*
　スーパートレイト
　・トレイトを定義する際に、他のトレイトとの依存関係を指定する仕組み
*/

fn main() {
    println!("Hello, world!");

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

    let person = Human;
    person.fly();
    // トレイトに定義されている同名のメソッドの呼び方
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person); // <- これもok

    // 同名の関連関数を呼び出す
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for Counter {
//     type Item = u32;　// implで具体的な型を指定する
//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// RHS=Self <- デフォルト型引数
// トレイトを実装する際に具体的な型を指定していない場合の型になる
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


// スーパートレイト（親）の定義
trait Player {
    fn play(&self);
}
// スーパートレイトを必要とするトレイトの定義
trait Artist: Player {
    fn perform(&self);
}

struct Musician;

// スーパートレイトの実装
impl Player for Musician {
    fn play(&self) {
        println!("楽器を演奏します。");
    }
}

// スーパートレイトを必要とするトレイトの実装
impl Artist for Musician {
    fn perform(&self) {
        // スーパートレイトのメソッドを内部で呼び出すこともできる
        self.play();
        println!("さらにパフォーマンスを披露します！");
    }
}

/*
　ニュータイプパターン
　・Rustには「自分が定義した型、または自分が定義したトレイトのどちらか一方が自作のものでないと impl できない」というルール（孤児規則 / Orphan Rules）がある
　　例えば、Vec<String>（標準ライブラリ）に対して、Display（標準ライブラリ）を直接実装することはできない
　・新しい型でラップすることでimplできるようになる
*/
use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0で中身のVec<T>にアクセス
        write!(f, "[{}]", self.0.join(", "))
    }
}
