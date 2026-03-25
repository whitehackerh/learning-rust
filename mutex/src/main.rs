/*
　ミューテックス mutual exclusion
　・複数のスレッドやプロセスが共有リソースに同時にアクセスする際、一度に1つのタスクのみがアクセスできるようにする（排他制御）仕組み
　・ミューテックスにあるデータにアクセスするにはミューテックスのロックを所望することでアクセスしたいことをまず、スレッドは通知しなければならない
　　・ロックとは、現在誰がデータへの排他的アクセスを行っているかを追跡するミューテックスの一部をなすデータ構造

　つまり、
　・データを使用する前にロックの獲得を試みなければならない
　・ミューテックスが死守しているデータの使用が終わったら、他のスレッドがロックを獲得できるように、データをアンロックしなければならない

　Mutex<T>
　・lockを呼び出すとLockResultに包まれた形でMutexGuardというスマートポインタを返却

　MutexGuard
　・内部のデータを指すDerefを実装している
　・MutexGuardがスコープを外れたときに、自動的にロックを解除するDrop実装もしている

　Arc<T>
　・Rc<T>のマルチスレッド版
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex生成　Mutex<T>
    let m = Mutex::new(5);

    {
        /*
        　・lockでロックを獲得　ただし、現在のスレッドをブロックするため、ロックを得られる順番が来るまで何もできない
        　・ロックを保持している他のスレッドがパニックしたら失敗する -> unwrapでこのスレッドをパニックさせる
        　・ロックを獲得した後、戻り値を中に入っているデータ(Mutex<T> の T)への可変参照として扱える
        */
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // エラー
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();  <- counter がループの初回にムーブされ、次のループでもムーブされたcounterを使おうとしているため
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap()); <- 別のスレッドにムーブされたcounterを使おうとしている


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
