fn main() {
    // リテラルにマッチ
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 名前付き変数にマッチ
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // yは新しい変数(10で宣言したyではない)
        // Some内のあらゆる値に合致 -> yはxの値に束縛される
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 複数のパターン
    let x = 1;
    match x {
        // 1 or 2 の場合
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ..= で限度値を含む値の範囲にマッチさせる 
    // 数値かcharのみ
    let x = 5;
    match x {
        // 1~5
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 分配して値を分解
    // 構造体, enum, タプル, 参照
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // 構造体のフィールドをa, b に分解
    let Point { x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    // 省略記法
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        },
        Message::Write(text) => {
            println!("Text message: {}", text)
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 参照
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    // 構造体とタプルを分配
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    // _で値の一部を無視
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    // 名前を_で始めて未使用の変数を無視
    // アンダースコアで始めることで、コンパイラに未使用変数について警告しないよう指示できる
    let _x = 5;
    let y = 10;

    // 値は束縛される
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {　<- ここでムーブされる
    //     println!("found a string");
    // }
    // println!("{:?}", s);
    // _には値は束縛されない
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // ..で値の残りの部分を無視
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
    // どの値が無視されるべきか不明瞭だとエラーになる
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    // refとref mutでパターンに参照を生成
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        // &nameでは生成できないためref を用いる
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
    // 可変参照
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // マッチガードで追加の条件式
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @束縛
    // 値を保持する変数を生成するのと同時にその値がパターンに一致するかを調べる
    enum Message2 {
        Hello { id: i32 },
    }
    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

// _で無視
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}