/*
　Unsafe Rust
　・メモリ安全保証を強制しない
　・unsafe superpowers
　　・生ポインタを参照外しすること
　　・unsafeな関数やメソッドを呼ぶこと
　　・可変で静的な変数にアクセスしたり変更すること
　　・unsafeなトレイトを実装すること
*/

/*
　生ポインタ
　・不変や可変になりうる
　・不変 *const T
　・可変 *mut T
　・* は参照外し演算子ではなく型名の一部
　・生ポインタの不変は、参照外し後に直接ポインタに代入できない
　・参照やスマートポインタと異なる点
　　・同じ場所への不変と可変なポインタや複数の可変なポインタが存在することで借用規則を無視できる
　　・有効なメモリを指しているとは保証されない
　　・nullの可能性がある
　　・自動的な型付けは実装されていない
*/

/*
　グローバル変数
　・Rustではstatic変数と呼ばれる
*/
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    /*
    　生ポインタの生成
    　・safeコードでも生成はできる
    　・unsafeブロックの外では生ポインタを参照外しできない
     */
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        // unsafe関数の呼び出しにはunsafeブロック内で呼ばなければならない
        dangerous();
    }

    // unsafeなコードを含んでいたとしても関数をunsafeにする必要があることにはならない
    // split_at_mut はunsafeなコードを含んでいるが、関数はunsafeではない
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        // 現在のバージョンだとエラーになる
        // println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


// unsafeなトレイト
// 少なくとも、1つのメソッドにコンパイラが確かめられないなんらかの不変条件があると、 トレイトはunsafeになる
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}