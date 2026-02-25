/*
　match
　・一連のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行
　・各アームに紐づけられるコードは式であり、マッチしたアームの式の結果がmatch式全体の戻り値
　・アームで複数行のコードを実行したい場合は波かっこを使う
　　・波かっこを使う場合はカンマ省略可能
　・パターンにマッチした値の一部に束縛できる
　・パターンはすべての可能性を網羅しないとコンパイルできない
*/

#[derive(Debug)]
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
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        /*
        　・3でも7でもないあらゆる値を変数に束縛　(catch-allアーム)
        　・catch-allアームは最後に書く
         */
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch-allアームで値を使用しない場合は _ を使う
        _ => reroll(),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 何もしたくない場合は()などを使う (catch-all以外のパターンでも可)
        _ => ()
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // 1を返す
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Option<T> の Someケースの場合の中身のTを取り出す
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}