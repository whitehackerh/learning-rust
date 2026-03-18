// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }

// 関数定義
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for item in list {
    //     if item > largest {  Tがなりうる可能性のある全ての型に対して動作するとは限らないためエラー
    //         largest = item;
    //     }
    // }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // charは二項演算できないためエラー
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 構造体定義
struct Point<T> {
    x: T, // <- x, y は同じ型
    y: T,
}

// T, U は同じ型でも異なる型でもいい
struct Point2<T, U> {
    x: T,
    y: U,
}

// enum定義
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// メソッド定義
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 具体的な型を指定する場合
impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 構造体定義のジェネリックな型引数は、必ずしも構造体のメソッドシグニチャで使用するものとは同じになると限らない。
struct Point4<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point4<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
        Point4 {
            x: self.x,
            y: other.y,
        }
    }
}