/*
　if let
　・一つのパターンにマッチする値を扱うことができ、記述を減らせる
　・等号で区切られたパターンと式をとり、式がmatchに与えられ、パターンが最初のアームになった
　　matchと同じ動作
　・ただし包括性チェックは失われる(if letに合致しないパターンは無視)
　・elseも使える
*/

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    /*
    　・Some列挙子の値をmaxに束縛
    　・Noneに対してはなにもしない (matchの _ => () と同じ動き)
     */ 
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        // if letに該当しない全パターン
        count += 1;
    }

    /*
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
     */
}
