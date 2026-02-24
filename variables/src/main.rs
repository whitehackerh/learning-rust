fn main() {
    // 不変
    let y: i32;
    y = 5;
    let z: i32 = 6;
    println!("The value of x is: {}", y);
    println!("The value of x is: {}", z);
    // 可変
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 定数
    const A: usize = 100;
    println!("The value of constant is: {}", A);


    // シャドーイング
    let a: i32 = 5;
    let b = a;
    // 変数名が同じだが再宣言が可能　（シャドーイングの場合、違う型で宣言可能 ex.最初は数値で宣言したが次は文字列で宣言など）
    let a: i32 = y + 1; // 6 

    {
        // ブロック内のaは{}内がスコープ
        let a: i32 = a * 2;
        println!("The value of x in the inner scope is: {a}"); // 12
    }

    println!("The value of x in the inner scope is: {a}"); // 6
    println!("The value of x in the inner scope is: {b}"); // 5


    // NG
    /* 
    let mut a = 1; 
    a = "2";
    */
}
