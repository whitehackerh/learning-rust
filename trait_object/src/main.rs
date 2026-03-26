/*
　トレイトオブジェクト
　・指定されたトレイトを実装する、ある型のインスタンスと、実行時にその型に対するトレイトメソッドを探すのに
　　使用されるテーブルの、両方を指す
　・&参照やBox<T>スマートポインタなどの、何らかのポインタを指定し、それからdynキーワード、そして対象の
　　トレイトを指定することでトレイトオブジェクトを作成する
*/

use trait_object::{Screen, Button, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };
    screen.run();
}
