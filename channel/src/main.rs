/*
　チャンネル
　・あるスレッドから別のスレッドへデータを送信する手段
　・転送機
　　・データを送信する側
　・受信機
　　・データを受信する側
　・転送機と受信機のどちらかがドロップされるとチャンネルは閉じられる
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    　・チャンネルを生成するが、これだけでは何もしないし、コンパイルできない
    　・1つのチャンネルが値を生成する複数の送信側と、その値を消費するたった1つの受信側を持つ
    　・tx -> 転送機, rx -> 受信機
     */
    let (tx, rx) = mpsc::channel();
   
   // txをクロージャにムーブ
    thread::spawn(move || {
        let val = String::from("hi");
        // sendで送信
        // sendに渡した値(Copyトレイトを実装しない型)はムーブされる
        tx.send(val).unwrap();
    });

    /*
    　recv
    　・受信できる
    　・メインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機
    　・値が送信されたら、Result<T, E>に含んで返す
    　・転送機が閉じたらエラーを返し、もう値は来ないと通知する
     */
    /*
    　try_recv
    　・ブロックしない
    　・即座にResult<T, E>を返す
    　・メッセージがあったらそれを含むOk値、何もなければErr値
    　・メッセージを待つ間にこのスレッドにすることがほかにある場合に有用
    　　-> メッセージがあれば処理し、それ以外の場合は再度チェックするまでの間に他の作業をするループの実装
    　・値がないのか、転送機が閉じたのかはエラーの中身で判断する
     */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    /*
    　・受信機をイテレータとして扱い、受信した値それぞれを出力する
    　・チャンネルが閉じられると繰り返しも終わる
     */
    for received in rx2 {
        println!("Got: {}", received);
    }

    // 転送機をクローンして複数のスレッドから同じ受信機に送信
    let (tx3, rx3) = mpsc::channel();
    let tx4 = tx3.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx4.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx3 {
        println!("Got: {}", received);
    }
}
