use std::time::Duration;

// fn main() {
//     trpl::block_on(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         handle.await.unwrap();
//     });
// }

// fn main() {
//     trpl::block_on(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         trpl::join(fut1, fut2).await;
//     });
// }

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let val = String::from("hi");
//         tx.send(val).unwrap();

//         let received = rx.recv().await.unwrap();
//         println!("received '{received}'");
//     });
// }

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("future"),
//         ];

//         // 特定のasyncブロック内では、コード内にawaitキーワードが現れる順序が、
//         // プログラム実行時にそれらが実行される順序でもある
//         for val in vals {
//             tx.send(val).unwrap();
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//         while let Some(value) = rx.recv().await {
//             println!("received '{value}'");
//         }
//     });
// }

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let tx_fut = async {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         trpl::join(tx_fut, rx_fut).await;
//     });
// }

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         // moveすることでtxがドロップされ、プログラムが終了するようになる
//         let tx_fut = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         trpl::join(tx_fut, rx_fut).await;
//     });
// }

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for i in vals {
                tx1.send(i).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for i in vals {
                tx.send(i).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // マクロは任意の数のfutureをawaitできる
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}