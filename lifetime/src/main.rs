
// error
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// 返している参照がxかyかわからない
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

/*
　ライフタイム注釈
　・いかなる参照の生存期間を変えることはない
　・ライフタイムに影響することなく、複数の参照のライフタイムの関係性を記述
　・ライフタイム引数を指定された関数はあらゆる雷二むの参照を受け取れる
　・'a のようにアポストロフィーから始まり小文字で記述する
*/

// 何らかのライフタイム'a に対して関数は2つの引数をとり、どちらも少なくともライフタイム 'a と同じだけ生きる文字列スライス
// 戻り値の文字列スライスも少なくともライフタイム 'a と同じだけ生きる
// 厳密にいうと、戻り値のライフタイムは、x,yの短いほうのライフタイムと同じ
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let result2 = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result2);
    let result3 = longest3(string1.as_str(), string2);
    println!("The longest string is {}", result3);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// error
// fn main2() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str()); <- resultのライフタイムはstring2と同じである
//     }
//     println!("The longest string is {}", result); <- resultは無効
// }

// ライフタイム引数を指定する必要があるかは、関数が行っていることに依存する
// ↓はyには不要
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/*
　・関数から参照を返す際、戻り値型のライフタイム引数は、引数のうち、どれかのライフタイム引数と一致する必要がある
　・引数を返さない場合は参照を返すのではなく所有権を渡してしまう　etc...
*/

fn longest3(x: &str, y: &str) -> String {
    String::from("really long string")
}

// 構造体定義のライフタイム注釈
// 参照を保持する構造体を定義する場合、構造体定義の全参照にライフタイム注釈をつける必要がある
struct ImportantExcerpt<'a> {
    part: &'a str, // この構造体のインスタンスはフィールドに保持している参照よりも長生きしない
}

/*
　ライフタイム省略則
　・コンパイラは参照である各引数に、ライフタイム引数を割り当てる
　・1つだけ入力ライフタイム引数があるなら、そのライフタイムが全ての出力ライフタイム引数に代入される
　・複数の入力ライフタイム引数があるけれども、メソッドでそのうちの一つが&selfや&mut selfだったら、 selfのライフタイムが全出力ライフタイム引数に代入される
*/

// メソッド定義におけるライフタイム注釈
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
　静的ライフタイム
　・'static
　・影響を受ける参照がプログラムの全期間生存
　・文字列リテラルは静的ライフタイム
*/


// ジェネリック型引数、トレイト境界、ライフタイムが混在すると以下の書き方になる
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    //       "アナウンス！ {}"
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}