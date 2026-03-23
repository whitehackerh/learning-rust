/*
　Dropトレイト
　・値がスコープを抜けた時に走るコードを指定
　・どんな型に対してもDropトレイトの実装を提供できる
　・selfへの可変参照を取るdropという1つのメソッドを実装する必要がある
*/

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!

    /*
    　Dropトレイトのdropメソッドは主導では実行できない
      -> 値を早期に片付けたい場合はstd::mem::drop関数を呼ぶ
    */
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");

    // CustomSmartPointer created.
    // Dropping CustomSmartPointer with data `some data`!
    // CustomSmartPointer dropped before the end of main.
}


