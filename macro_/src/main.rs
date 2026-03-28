/*
　マクロ
　・ある機能の集合
　・宣言的マクロ declarative macro
　・手続き的マクロ procedural macro
　　・構造体とenumにderive属性を使ったときに追加されるコードを指定する、カスタムの#[derive]マクロ
　　・任意の要素に使えるカスタムの属性を定義する属性風のマクロ
　　・関数のように見えるが、引数として指定されたトークンに対して作用する関数風のマクロ
　・可変長の引数をとれる
　・ファイル内で呼び出す前に定義したりスコープに導入しなければならない
*/

/*
　宣言的マクロ
　・定義するには macro_rules! 構文を使う
*/

/*
　手続き的マクロ
　・コードを入力として受け取り、そのコードに対して作用し、出力としてコードを生成する
*/

/*
　属性風マクロ
　・新しい属性を作れる
　ex)
  #[route(GET, "/")]
  fn index() {
*/

/*
　関数風マクロ
　・関数呼び出しのように見えるマクロを定義
　ex)
 let sql = sql!(SELECT * FROM posts WHERE id=1);
*/

fn main() {
    Pancakes::hello_macro();
}

// #[macro_export] はマクロを定義しているクレートがスコープに持ち込まれたなら無条件でこのマクロが利用可能になるべきということを示す
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)] // <- で宣言したトレイトの実装を使用できるようにする
struct Pancakes;