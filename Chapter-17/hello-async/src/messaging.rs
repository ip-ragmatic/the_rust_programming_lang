use std::{
    future::Future,
    pin::{pin, Pin},
    time::Duration,
};

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx_fut = pin!(async move {
            let msgs = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for msg in msgs {
                tx.send(msg).unwrap();
                trpl::sleep(Duration::from_millis(250)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("received: `{val}`");
            }
        });

        let tx1_fut = pin!(async move {
            let msgs = vec![
                String::from("hello"),
                String::from("from"),
                String::from("future 1"),
            ];
            for msg in msgs {
                tx1.send(msg).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx1_fut, rx_fut];
        trpl::join_all(futures).await;
    });
}
