mod tests;

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
    future,
};

use std::{io, time::Duration, sync::Arc};

async fn on_connection(ctx: future::Instance<context::ApplicationContext>, mut stream: LocalSocketStream) {
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
        let pid = request.session;

        if request.method == Method::Stop {
            if ctx.lock().await.stop_session(pid).is_err() {
                error!("Failed to stop session: Session {pid} not found");
            }
            return;
        }
        let session = ctx.lock().await.session(pid);
        let res = match run::run(session, request.method) {
            Ok(r) => Response::ok(request.id, r),
            Err(_) => Response::err(request.id, Error::InternalError)
        };
        if let Err(e) = comm::async_send(&mut stream, res).await {
            error!("Failed to read from stream: {e}");
            break;
        }
    }
}


#[tokio::main]
async fn main() {
    nu_matrix_common::init_log().expect("Failed to initialize logger");

    let name = socket::get_socket_name();

    info!("Socket at {name}");
    let listener = socket::make_listener(name).expect("Failed to bind to socket");

    let ctx = future::new_instance(context::ApplicationContext::new());

    loop {
        match future::timeout(listener.accept(), Duration::from_secs(1)).await {
            Some(Ok(stream)) => {
                tokio::spawn(on_connection(Arc::clone(&ctx), stream));
            },
            Some(Err(e)) => {
                eprintln!("Incoming connection failed: {e}");
                continue;
            },
            None if ctx.lock().await.is_finishable() => break,
            _ => ()
        };
    }
    info!("Shutting down");
}
