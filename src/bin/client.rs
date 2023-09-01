use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>
    }
}

#[tokio::main]
async fn main() {
    // Create a channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    // ignore errors
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // ignore errors
                    let _ = resp.send(res);
                }
            }
        } 
    });

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: String::from("foo"),
            resp: resp_tx
        };
        
        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Got = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: String::from("foo"),
            val: "bar".into(),
            resp: resp_tx
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Got = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}