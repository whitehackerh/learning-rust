fn main() {
    // 型エイリアス　typeキーワードを使う
    type Kilometers = i32; // <- 同義語になる
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 長い型に使うと有用
    type Thunk = Box<Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
}

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

/*
　Empty型　!
　・関数が値を返すことが決してない時に使う
　・continueも!値
*/
fn bar() -> ! {
    // --snip--
}

/*
　動的サイズ決定型 (DST)
　・実行時にしかサイズを知ることのできない値の型
　・ex) str, トレイト
　・Sizedトレイト
　　・コンパイル時にサイズの判明するすべてのものに自動的に実装される
　　・暗黙的にすべてのジェネリック関数にSizedの境界を追加する
　　・ジェネリック関数は通常コンパイル時に判明するサイズがある型に対してのみ動くが、?Sizedで制限を緩められる
*/
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}