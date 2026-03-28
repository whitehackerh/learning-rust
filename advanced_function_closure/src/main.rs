fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// 関数に関数を渡す(関数ポインタ　fn型)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/*
　・関数ポインタはクロージャトレイト3つ全て(Fn, FnMut, FnOnce)を実装する
　・関数ポインタを引数として、クロージャを期待する関数に渡せる
*/

// クロージャの返却　-> 直接返せないため、Boxなどを用いる
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}