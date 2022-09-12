use tokio::net::TcpListener;

use std::io;


async fn process_socket<T>(socket: T) {
    println!("got one")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        process_socket(socket).await;
    }

    Ok(())
}
