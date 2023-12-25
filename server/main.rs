mod matrix;
mod socket;
mod run;
mod context;

use log::{info, error};
use nu_matrix_common::{
    jrpc::{Request, Response, Error},
    methods::Method,
    comm,
};

use interprocess::local_socket::LocalSocketStream;
use std::io;

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
    nu_matrix_common::init_log().expect("Failed to initialize logger");

    let name = socket::get_socket_name();

    info!("Socket at {name}");
    let listener = socket::make_listener(name).expect("Failed to bind to socket");
    
    let mut ctx = context::ApplicationContext::new();

    'mainloop: loop {
        let mut stream = match listener.accept() {
            Ok(c) => c,
            e => {
                handle_error(e);
                continue;
            },
        };
        loop {
            let request = match comm::receive::<Request>(&mut stream) {
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
                break 'mainloop;
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
            if let Err(e) = comm::send(&mut stream, res) {
                error!("Failed to read from stream: {e}");
                break;
            }
        }
    }
    info!("Shutting down");
}
