/*
　構造体
　・意味のあるグループを形成する複数の関連した値をまとめ、名前付けできる独自のデータ型
　・各データ片に名前を付ける
　　・フィールド：構造体に定義するデータ片の名前と型
  ・使用するには各フィールドに対して具体的な値を指定してインスタンスを生成する
  ・フィールドは構造体宣言通りの順番に指定する必要はない
  ・構造体から値を得るにはドット記法を使う
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // インスタンスが可変ならば、すべてのフィールドは可変
    // 一部のフィールドのみ可変にすることはできない
    user1.email = String::from("anotheremail@example.com");

    let user = build_user(String::from("someusername123"), String::from("someone@example.com"));

    /*
    　構造体更新記法
    　・構造体の一部を変更する形で新しいインスタンスを生成
     */
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // ..　という記法により明示的にセットされていないフィールドが与えられたインスタンスのフィールドと同じ値になる
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    /* 
    　構造体更新記法を使う場合、
    　・1つでもCopyトレイトが実装されていない型のフィールドを元のインスタンスの値で指定すると、更新後のインスタンスにムーブされる
    　  -> 元のインスタンスの値を指定するフィールドがすべてCopyトレイトが実装されている型であれば、ムーブされない

    　・上記のuser2はuser3にムーブされている
    　　・String型のusernameの値はuser2の値であるため
     */

    /*
    　タプル構造体
    　・フィールドに紐づけられた名前がなく、型だけの構造体
    　・.や添え字で個々の値にアクセス
    */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    　ユニット様構造体
    　・ある型にトレイトを実装するものの、型自体に保持させるデータが一切ない場面で有効
     */
    let subject = AlwaysEqual;

}

// 構造体の新規インスタンスを関数本体の最後の式として生成して、そのインスタンスを返す
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// 仮引数名と構造体のフィールド名が同じであれば、フィールド初期化省略記法を使える
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ユニット様構造体
struct AlwaysEqual;


