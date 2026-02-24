/*
　参照
　・アドレス
　　・アドレスに置かれているデータにアクセスできる
　・参照は生存期間中を通して、特定の型の有効な値を指していることが保証されている

　借用
　・参照を作成すること
*/

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 関数に渡してもムーブしていない
    println!("The length of '{}' is {}.", s1, len); // s1は有効

    let s = String::from("hello");
    // change(&s);
    
    let mut s = String::from("hello");
    change(&mut s);

    /*
    　可変参照
    　・&mut 変数名 で渡す
    　・関数では &mut データ型 で受け入れる
    　・ある値への可変参照が存在する場合、その値への参照を他に作ることはできない
    　・不変参照をしている間、同じ値に対して可変参照はできない

    　不変参照
    　・複数作ることが可能
     */

    // NG
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // ただし、r1またはr2が実際に使われて初めてエラーになる。複数可変参照を作るだけならエラーにはならない。

    // OK
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるため、問題なく新しい参照を作ることができる。
    let r2 = &mut s;

    // NG
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);

    /*
    　参照のスコープ
    　・作られたところか始まり、その参照が最後に使用される時点まで
     */

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // r1,r2が最後に使用される時点
    let r3 = &mut s;
    println!("{}", r3);

}

fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になるが、参照しているものの所有権を持っているわけではないためドロップはされない。

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 不変参照への変更はエラー
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}