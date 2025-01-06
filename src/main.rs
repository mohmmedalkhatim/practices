use std::{collections::HashMap, fmt::Alignment, io::Error};
mod game;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone, Default)]
struct Resqust {
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
}
impl Resqust {
    async fn new(mut stream: TcpStream) -> Result<Resqust, Error> {
        let mut orginel = String::new();
        let _ = stream.read_to_string(&mut orginel).await?;
        let mut lines = orginel.lines();
        let head = lines.next();
        let mut req = Resqust::default();
        match head {
            Some(state) => {
                let head: Vec<&str> = state.split(" ").collect();
                req.method = head[0].to_string();
                req.path = head[1].to_string();
            }
            None => {}
        }
        todo!()
    }
}
struct Responce {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
struct Router {
    routes: HashMap<String, Box<dyn Fn(Resqust) -> Responce>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let init = TcpListener::bind("127.0.0.1:3000").await;
    match init {
        Ok(server) => loop {
            let (mut tcp, _socket) = server.accept().await.unwrap();
            let mut s = String::new();
            let _ = tcp.read_to_string(&mut s).await;
            println!("{}", s);
        },
        Err(e) => Err(e),
    }
}
