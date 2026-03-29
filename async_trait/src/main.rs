use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
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
        });

        // ピン留めされたトレイトオブジェクトの参照を Vec に入れる
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });
}