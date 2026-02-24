/*
　メソッド
　・構造体の文脈の中で定義
　　・他にもenum, トレイトオブジェクトの中　でも定義される
　・第1引数は必ずself
　　・&をつけなければ所有権を奪える
　　・&をつければ借用
　　・&mut なら可変借用
　・構造体のフィールドと同じ名前のメソッドを定義できる
*/

/*
　関連関数
　・implブロック内で定義されたすべての関数
　・selfを第1引数として持たない関連関数を定義することもできる -> メソッドではない
　・構造体の新規インスタンスを返すコンストラクタによく使用される
*/

/*
　・各構造体には複数のimplブロックを存在させることができる
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implブロック内の定義は構造体に関連付けられたもの
impl Rectangle {
    // 第1引数の &self は self: &Self の省略記法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 戻り値と関数内のSelfはRectangleを指す
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    // メソッドではない関連関数の呼び出し
    let square = Rectangle::square(3);
}
