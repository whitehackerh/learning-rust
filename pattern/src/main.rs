fn main() {
    /*
    　match
    　・必須事項
    　　・網羅的
    　　-> 最後のアームに包括的なパターンを入れる (最後のアームに _ がよく使われる)
     */
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    /*
    　if let
    　・if let, else if , else if letを組み合わせられる
    　　(最初がif でも else if letを組み合わせられる)
    　・一連のアームの条件は、関連している必要はない
     */
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    /*
    　while let
    　・パターンが合致し続ける限り、ループが続く
     */
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    /*
    　for
    　・for x in y では xがパターン
     */
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate()  {
        println!("{} is at index {}", value, index);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

/*
　引数
　・パターン&(x, y)
*/
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}