mod socket;

use nu_matrix_common::{
    jrpc::Request,
    methods::Method
};

use interprocess::local_socket::tokio::LocalSocketStream;
use std::io;
use futures::io::{
    AsyncReadExt, BufReader
};

fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    match conn {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("Incoming connection failed: {e}");
            None
        }
    }
}

#[tokio::main]
async fn main() {
    
    let name = socket::get_socket_name();

    eprintln!("Socket at {name}");
    let listener = socket::make_listener(name).expect("Failed to bind to socket");

    let mut buffer = String::with_capacity(1024);

    loop {

        let conn = match listener.accept().await {
            Ok(c) => c,
            e => {
                handle_error(e);
                continue;
            },
        };
        let mut stream = BufReader::new(conn);
        if let Err(e) = stream.read_to_string(&mut buffer).await {
            eprintln!("Failed to read from stream: {e}");
            continue;
        }
        let request =  match serde_json::from_str::<Request>(&buffer) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse JSON: {e}");
                continue;
            }
        };

        eprintln!("Got request: {request:?}");

        if request.method == Method::Stop {
            break;
        }
    }
}
