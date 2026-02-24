/*
` スライス
　・参照の一種
　・所有権を持たない
　・コレクション全体または連続した要素の列を参照できる
*/

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    /*
    　文字列スライス
    　・&str
    　・start indexはスライスの最初の位置、ending indexはスライスの終端位置よりも1大きい値
    　・長さはending index - start index
     */

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; // [0..2] と等価

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; // [0..末端]と等価

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..]; // 全体を参照

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error! clear()は可変参照を必要とする
    println!("the first word is: {}", word); // 可変参照と不変参照が同時に存在できない

    // sは&str型
    let s = "Hello, world!";

    // 他の型のスライス
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // &[i32]型
    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> &str {
// 引数の型を&strとすることで&strも&Stringも受け取れる
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
