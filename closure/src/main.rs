/*
　クロージャ
　・変数に保存したり、引数として他の関数に渡すことのできる匿名関数
　・定義されたスコープから値をキャプチャできる
*/

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_elseの引数はクロージャ
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
        println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // クロージャは型注釈を取り除くこともできる
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    let a = add_one_v3(5);
    let a = add_one_v4(5);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // クロージャは↑でStringと推論しているため、次に異なる型で使用するとエラー
    // let n = example_closure(5);

    /*
    　listへの不変参照は複数同時に存在可能
      -> クロージャ定義より前のコード、クロージャ定義からクロージャ呼び出しまでの間のコード、 クロージャ呼び出し後のコードからでも、アクセス可能
     */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    // クロージャ定義時にlist2への可変参照がキャプチャされる
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    /* 
    　クロージャを呼び出した後はクロージャを再度使用していないため、可変借用は終了
    　-> 最後に呼ぶところまで可変借用は継続
    */ 
    println!("After calling closure: {:?}", list2);

    // 可変借用と不変借用は同時に存在できないためにエラーになるケース 1
    // let mut list2 = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list2);
    // let mut borrows_mutably = || list2.push(7);
    // println!("Before calling closure: {:?}", list);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list2);

    // 可変借用と不変借用は同時に存在できないためにエラーになるケース 2
    // let mut list2 = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list2);
    // let mut borrows_mutably = || list2.push(7);
    // println!("Before calling closure: {:?}", list);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list2); <- ↓で呼ばれるため、可変借用はまだ有効
    // borrows_mutably();
    // println!("After calling closure: {:?}", list2);

    /*
    　クロージャの本体が所有権を必要としない場合でも、所有権を奪うように強制したい場合は、引数リストの前でmoveキーワードを使用する
     */
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    /*
    　クロージャ本体は以下のいずれかを行える
    　・キャプチャされた値をクロージャの外にムーブ
    　・　　　　　　　　　　変更
    　・　　　　　　　　　　ムーブも変更もしない
    　・何もキャプチャしない
     */

    /*
    　Fn系トレイト
    　-> クロージャはFn系トレイトのうち1つ, 2つ, または3つ全てを付加的に自動的に実装する
　　　・FnOnce
　　　　・所有権を奪うクロージャに適用
　　　　・一度だけ呼び出せる
　　　・FnMut
　　　　・ムーブしない かつ キャプチャされた値を変更するかもしれないクロージャに適用
　　　　・複数回呼び出せる
　　　・Fn
　　　　・不変参照 または 何もキャプチャしないクロージャに適用
　　　　・複数回呼び出せる

    　・FnができることはFnMutでもでき、FnMutでできることはFnOnceもできる
     */
}
