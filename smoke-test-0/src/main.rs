use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};

use std::io;

async fn process_socket(mut socket: TcpStream) {
    println!("got one");
    let mut buf = vec![0; 1024];

    // TODO: use a buffered reader here?
    let n = socket
        .read(&mut buf)
        .await
        .expect("Failed to read data from socket.");
    
    println!("{}", String::from_utf8(buf).unwrap());
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:38888").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}
