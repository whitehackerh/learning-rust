/*
　スタック
　・後入れ先出し
　・スタック上のデータはすべて既知の固定サイズ
　　・コンパイル時にサイズが分からなかったりサイズが可変のデータはヒープに格納しなければならない
　
　ヒープ
　・ヒープ上に空の領域を見つけ、そこを使用中とし、ポインタを返す
　　・ポインタ：アドレス
　・ヒープへのポインタは既知の固定サイズであるためスタックに保管可能

　・速度
　　・スタックにデータを積む　＞　ヒープ上にメモリ確保
　　・スタックへのデータアクセス　＞　ヒープへのデータアクセス

　・コードが関数を呼び出すと、関数に渡された値、関数のローカル変数がスタックに載る
　・関数の実行が終了すると取り除かれる

　・所有権の主な目的はヒープデータを管理すること

　所有権
　・Rustの各値には所有者が存在する
　・いかなる時も所有者は一つ
　・所有者がスコープから外れたら、値は破棄される

　スコープ
　・要素が有効になるプログラム内の範囲
*/

fn main() {
    {                      // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // sは、ここから有効になる

        // sで作業をする
    } // このスコープは終わり。もうsは有効ではない

    /*
    　String型
    　・ヒープにメモリを確保
    　・from関数を用いて文字列リテラルから生成
     */
    let s = String::from("hello");
    let mut s = String::from("hello"); // 変更可能
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // hello, world!

    {
        let s = String::from("hello"); // sはここから有効になる

        // sで作業をする
    }                                  // ブロック内で宣言したsはもう有効ではない

    // ヒープを使う所有権のある型はスコープを抜けるときに自動的にdrop関数が実行されメモリを返還する

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); s1はs2にムーブされたためerror

    // deep copyが必要な場合、cloneメソッドを使う
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    // xはyに代入されても有効
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    /*
    　整数などの既知のサイズを持つ型はスタック上に保持される

    　Copyトレイト
    　・値がムーブ（所有権移動）せずに、単純にコピーされて新しい所有者になれる
    　・整数などのスタックに保持される型が対象
    　・他の変数に代入した後もムーブされず、元の変数は有効なまま

    　Copyを実装する型の例
    　・あらゆる整数型。u32など。
　　　・論理値型であるbool。trueとfalseという値がある。
　　　・あらゆる浮動小数点型、f64など。
　　　・文字型であるchar。
　　　・タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。
     */

    /*
    　関数に値を渡すとムーブやコピーがされる
     */
    let s = String::from("hello");  // sがスコープに入る
    takes_ownership(s);             // sの値が関数にムーブされる
                                    // sは無効
    let x = 5;                      // xがスコープに入る
    makes_copy(x);                  // xが関数にコピーされる
                                    // xは有効

    /*
    　値を返すことでも所有権は移動する
     */
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする
    let s2 = String::from("hello");     // s2がスコープに入る
    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる

    //　複数の値を関数から受け取るにはタプルを使う
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

}


fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を呼び出した関数にムーブする
    let some_string = String::from("yours"); // some_stringがスコープに入る
    some_string                              // some_stringが返され、呼び出し元にムーブされる
}

// この関数は、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返す
    (s, length)
}
