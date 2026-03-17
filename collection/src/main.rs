/*
　コレクション
　・データはヒープに確保される
　・伸縮可能

　ベクタ・・・可変長の値を並べて保持
　文字列・・・文字のコレクション
　ハッシュマップ・・・値を特定のキーと紐づけする

　ベクタ Vec<T>
　・同じ型の値のみ保持

　文字列 String
　・伸縮可能、可変、所有権あり
　・文字列スライス &strとは性質が異なる

　ハッシュマップ HashMap<K, V>
　・K型のキーとV型の値の対応関係を保持
　・全てのキーは互いに同じ型でなければならず、全ての値は互いに同じ型でなければならない
　・キーと値のペアの数は伸長可能
　・キーは一意でなければならない (1つのキーには1つの値しか紐づけられない)
*/

fn main() {
    // 空のベクタを生成
    let v: Vec<i32> = Vec::new();

    // 初期値を持たせる場合
    let v2 = vec![1, 2, 3];

    // ベクタに要素を追加
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);

    // ベクタの要素にアクセス
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2); // getメソッドを使用するとOption<&T>が得られる
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /*
    　・添え字の記法でアクセスする場合、範囲外にアクセスするとエラー
    　・getメソッドを使えば、範囲外にアクセスするとNoneを返す
     */

    // 同一スコープ上では可変と不変な参照を同時に存在させられないためエラー
    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v5.push(6); <- pushの第一引数が &mut self つまりv5の可変参照
    // println!("The first element is: {first}");

    // ベクタ内の値を順に処理
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // 要素に変更を加える場合
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let fruits = vec!["apple", "banana", "cherry", "date"];
    // インデックスと要素（参照）を同時に取り出す
    for (i, fruit) in fruits.iter().enumerate() {
        println!("Index: {}, Value: {}", i, fruit);
    }

    // 異なる型の要素を表現する単一の型が必要な場合、enumを定義して使用する
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // ベクタに格納しうるすべての型を網羅できない場合はトレイトを使う


    // ---------------------------------------------------------

    // 文字列生成
    let mut s = String::new();

    // 文字列リテラルからString生成
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // 文字列に追加
    let mut s = String::from("foo");
    s.push_str("bar");
    // 1文字だけ追加する場合はpush
    let mut s = String::from("lo");
    s.push('l');

    // 文字列連結
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされる
    
    // 複雑な文字列の連結
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format!は引数の所有権を奪わない

    // 文字列走査
    for c in "Зд".chars() {
        println!("{c}");
    }


    // ---------------------------------------------------------

    // ハッシュマップをnewで生成 -> insertで要素追加
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // ハッシュマップの値にアクセス getにキーを渡す
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    /*
    　・getはOption<&V>を返す
    　・キーに対応する値がハッシュマップになかったらgetはNoneを返す
     */

    // forでハッシュマップのキーと値のペアを走査　※順番は保証されない
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Copyトレイトが実装されていない型はハッシュマップに渡すとムーブされる
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 2つの変数はムーブされる

    /*
    　・Copyトレイトが実装されていない型はムーブされない
    　・Copyトレイトが実装されている型でも参照を渡せばムーブされない
    　・参照が指している値は最低でもハッシュマップが有効な間は有効でなければならない
     */

    // 上書き
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 同じキーを異なる値で挿入する
    println!("{:?}", scores); // {"Blue": 25}

    // キーが存在しない場合のみキーと値を追加する
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // 挿入される
    scores.entry(String::from("Blue")).or_insert(50); // 挿入されない
    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}

    // 古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insertは指定されたキーに対する値への可変参照を返す
        *count += 1; // 可変参照を保持しているため、値に代入するためには参照外ししなければならない
    }
    println!("{:?}", map);
}