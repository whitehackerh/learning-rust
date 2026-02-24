/*
　列挙型 enum
　・取りうる列挙子を列挙することで型を定義
　・ある値が、取りうる値の集合のうちのいずれか一つであることを表現する
*/

// IPアドレスはIPV4かIPV6のどちらか。両方にはなりえない。
enum IpAddrKind {
    V4,
    V6,
}

/*
　・enumの列挙子の中にデータを格納できる
　・自動でコンストラクタ関数が定義される
*/ 
enum IpAddr {
    V4(String),
    V6(String),
}

/*
　・列挙子に紐づけるデータ型と量は異なってもよい
*/
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
　enumもimplを使ってメソッドを定義できる
*/
impl Message {
    fn call (&self) {}
}

/*
　・Rustにはnull機能がない
　・値が存在するか不在かという概念をコード化するenumが Option<T>

    enum Option<T> {
        None,
        Some(T),
    }

　・Option<T>は Option:: の接頭辞なしに Some と None を直接使える
　・<T>はジェネリック型引数
　・Some列挙子はあらゆる型のデータを1つだけ持てる
*/


fn main() {
    /* 
    　・各列挙子のインスタンス生成
    　・以下の変数はどちらもIpAddrKind型
    */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    // None値を見ただけではそれに対するSome列挙子が保持する型を推論できないため、型を明記している
    let absent_number: Option<i32> = None;

    /* error
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y;

        ・Option<T>でない型は常に有効な値であることをコンパイラが確認してくれる
        ・Option<T>である場合のみ、値を保持していない可能性を確認する必要がある
        ・nullになる可能性がある場合は型をOption<T>で定義する必要がある
        ・上記の処理を行う場合、Option<T>をTに変換する必要がある
     */

}

// IpAddrKindであればどんな列挙子でも受け取る関数
fn route(ip_kind: IpAddrKind) {}
