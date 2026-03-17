/*
　Result enumは以下のように定義されている
　enum Result<T, E> {
　    Ok(T),
　    Err(E),
　}

　・T, Eはジェネリックな型引数
　・Tは成功したときにOk列挙子に含まれて返される値の型
　・EはErr列挙子に含まれて返されるエラーの型
*/

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // 同じロジックでmatchを使用しない記法
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    /*
    　unwrap
    　・ResultがOk列挙子なら、Okの中身を返す
    　・ResultがErr列挙子ならpanic!マクロを呼ぶ
     */
    let greeting_file = File::open("hello.txt").unwrap();

    /*
    　expect
    　・Ok列挙子の場合の挙動はunwrapと同じ
    　・Err列挙子の場合の、panic!時にエラーメッセージを渡せる
     */
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

/*
　?演算子
　・Resultの値がOkならOkの中身が返り、プログラム継続
　・Resultの値がerrなら関数全体からErrが返る
　・?演算子が呼ばれる対象となったエラー値はFromトレイトのfrom関数を呼び出す
　　-> 受け取ったエラー型が現在の関数の戻り値で定義されているエラー型に変換される
　・?を使用する対象の値と戻り値の型に互換性がある関数でしか使用できない
*/
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// さらに短く書ける
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

/*
　・?はOptionを返す関数の中で使用できる
　・Noneならばその時点でNoneが関数から早期リターン
　・SomeならSomeの内側の値がその式の結果の値となり関数は継続する
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/*
　・main関数は戻り値型として指定できる型に制限がある
　・Result<(), E>を返せる
　・Ok(()) は成功したが、返すデータがない場合に用いる
*/
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }