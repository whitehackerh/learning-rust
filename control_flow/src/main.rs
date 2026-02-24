fn main() {
    let number = 3;

    // 条件式はbool型でなければならない
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if は式であるため右辺にもってくることができる
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}"); // 5

    // 条件分岐が複数ある場合、それぞれの分岐の結果は同じデータ型でなければならない
    /* NG 
    let condition = true;
    let number = if condition { 5 } else { "six" };  ←型が違う
    println!("The value of number is: {number}");
    */

    
    
    /*
    　ループ
    　・loop
    　・while
    　・for
    */

    /* 
    　loop
    　・明示的にやめさせるまで実行する
    　・式
     */

    // 無限ループ
    /* 
    loop {
        println!("again!");
    }
     */

    // 値を返すパターン
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}"); // 20

    // 同じブロック内に複数breakがあり、それぞれで値を返す場合、同じ型でなければならない(if-else if-else と同じ)

    /*
    　ループ内にループがある場合
    　・break, continueは最も内側のループに適用
    　・ループラベルを使用すれば、breakやcontinueが適用されるループを指定できる
    　・ループラベルはシングルクォートで始める
     */

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // counting_up のループを抜ける
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // 2

    // break 値; で値を返せるが、continueは不可。あくまで次のループに移るためのもの

    /*
    　while
    　・条件付きでループさせたい場合などで使う
    　・条件式がtrueでなくなったとき、プログラム側でbreakが実行されてループを終了
    　・式
     */
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    /*
    　for
    　・コレクションの各要素に対して処理をしたい場合などで使う
    　・式にはなれない
     */
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // x..y <- Range型
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
