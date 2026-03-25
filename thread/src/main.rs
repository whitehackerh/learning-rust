/*
　・実行中のプログラムのコードはプロセスで走る
　・OSは同時に複数のプロセスを管理する
　・プログラム内では独立した部分を同時に実行することもできる
　　-> スレッド : 独立した部分を走らせる機能
*/

use std::thread;
use std::time::Duration;

fn main() {
    /*
    　新規スレッド生成
    　スレッドで走らせたいコードを含むクロージャを渡す
     */
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // hi number 1 from the main thread!
    // hi number 1 from the spawned thread!
    // hi number 2 from the spawned thread!
    // hi number 2 from the main thread!
    // hi number 3 from the spawned thread!
    // hi number 3 from the main thread!
    // hi number 4 from the spawned thread!
    // hi number 4 from the main thread!
    // hi number 5 from the spawned thread!

    /*
    　・Rustプログラムのメインスレッドが完了するときには、立ち上げられたすべてのスレッドは、
    　　その実行が完了したかどうかにかかわらず、停止される
    　・thread::sleep
    　　少々の間、スレッドの実行を止め、違うスレッドを走らせることができる　※ただし保証はない
     */

    /*
    　JoinHandle
    　・thread::spawnの戻り値の型
    　・joinメソッドを呼び出した時にスレッドの終了を待つ所有された値
    　・joinを呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックする
    　　・ブロック : そのスレッドが動いたり、終了したりすることを防ぐ
     */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("handle {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    /*
    　以下の順にするとメインスレッドは立ち上げたスレッドが終了するまで待つ
     　-> 出力は混ざらない

    　1. thread::spawn
    　2. handle.join
    　3. forループ
     */

    // 別のスレッドに値を渡す
    // error
    // let v = vec![1, 2, 3];
    // let handle2 = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // handle2.join().unwrap();

    // moveキーワードで所有権をクロージャに奪わせる
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle2.join().unwrap();
}
