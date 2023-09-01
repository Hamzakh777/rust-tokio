use mini_redis::{client, Result};


/// Rust transforms `async fn` at compile time into a routine that operates
/// asynchronously. Any calls to `.await` within the `async fn` yield control back to the thread. 
/// 
/// Calling async functions in rust does not result in the body of the function executing. 
/// Instead calling `async fn` returns a value representing the operation. To run the operation 
/// you should use the `.await` operator on the return value. The return value of an `async fn` 
/// is an anonymous type that implements the `Future` trait.
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    client.set("hello", "workd".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
