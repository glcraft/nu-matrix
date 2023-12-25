mod matrix;
mod socket;
mod run;
mod context;

use interprocess::local_socket::tokio::LocalSocketStream;
use log::{info, error};
use nu_matrix_common::{
    jrpc::{Request, Response, Error},
    methods::Method,
    comm,
};

use std::io;

async fn on_connection(ctx: &mut context::ApplicationContext, mut stream: LocalSocketStream) -> bool {
    loop {
        let request = match comm::async_receive::<Request>(&mut stream).await {
            Ok(r) => r,
            Err(comm::Error::Exchange(e)) if matches!(e.kind(), io::ErrorKind::BrokenPipe | io::ErrorKind::UnexpectedEof) => {
                info!("Connection closed");
                break;
            }
            Err(e) => {
                error!("Failed to read from stream: {e:?}");
                break;
            }
        };

        if request.method == Method::Stop {
            return true;
        }
        let pid = request.session;
        let session = match ctx.session_mut(pid) {
            Some(s) => s,
            None => ctx.new_session(pid)
        };
        let res = match run::run(session, request.method) {
            Ok(r) => Response::ok(request.id, r),
            Err(_) => Response::err(request.id, Error::InternalError)
        };
        if let Err(e) = comm::async_send(&mut stream, res).await {
            error!("Failed to read from stream: {e}");
            break;
        }
    }
    false
}


#[tokio::main]
async fn main() {
    nu_matrix_common::init_log().expect("Failed to initialize logger");

    let name = socket::get_socket_name();

    info!("Socket at {name}");
    let listener = socket::make_listener(name).expect("Failed to bind to socket");

    let mut ctx = context::ApplicationContext::new();

    loop {
        let stream = match listener.accept().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Incoming connection failed: {e}");
                continue;
            },
        };
        if on_connection(&mut ctx, stream).await {
            break;
        }
    }
    info!("Shutting down");
}
