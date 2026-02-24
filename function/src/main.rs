fn main() {
    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    /*
    　文：なんらかの動作をして値を返さない命令
    　式：結果値に評価される
     */

    // この1行は文
    let a = 1;
    // let文を他の変数に代入することはできない
    // let x = (let y = 6);  error

    /* 
    　・関数定義も文に該当する
    　・式は文の一部
    　・let a = 1 の 1 は式
    　・関数呼び出しやマクロ呼び出し、{}で括られるスコープも式 
    */


    let y = {
        let x = 3;
        x + 1
    };
    println!("value: {}", y);  // y = 4

    /*   
    　x + 1にセミコロンをつけてしまうと文になってしまう。文は値を返さない
    　-> yに値が束縛されない
    */


    let b = five();
    println!("The value of b is: {b}");
    let c = plus_one(5);
    println!("The value of c is: {c}");
}

// 関数名はスネークケースが慣例
fn another_function() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5 
    /* 
    　・基本的に最後の式が戻り値となる
    　・returnキーワードで関数から早期リターンも可能
    */
}

fn plus_one(x: i32) -> i32 {
    x + 1 // セミコロンをつけると文となり、値を返さないためエラー
}