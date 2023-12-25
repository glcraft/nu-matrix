mod process;

use nu_matrix_common::{
    jrpc::{Request, Response},
    methods::{self, Method},
    comm::{Error, receive, send},
};
use interprocess::local_socket::{LocalSocketStream, NameTypeSupport};


fn get_socket_name() -> String {
    let name = match NameTypeSupport::query() {
        NameTypeSupport::OnlyPaths => {
            let tmp = std::env::temp_dir();
            tmp.join("nu-matrix.sock").to_str().expect("Failed to convert path to string").to_string()
        },
        NameTypeSupport::OnlyNamespaced | NameTypeSupport::Both => "@nu-matrix.sock".into(),
    };
    name
}

fn send_request(stream: &mut LocalSocketStream, method: Method) -> Result<(), Error> {
    let ppid = process::parent_id().expect("Failed to get parent process ID");
    let current_time: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_millis()
        .try_into()
        .map_err(Error::ConvertInt)?;
    let req = Request::new(ppid as _, method, Some(current_time));
    
    send(stream, req)?;
    
    let _res = receive::<Response<methods::Response>>(stream)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    nu_matrix_common::init_log().expect("Failed to initialize logger");

    let mut stream = LocalSocketStream::connect(get_socket_name()).expect("Failed to connect to socket");
    send_request(&mut stream, Method::NewIdentity(3, 3))?;
    send_request(&mut stream, Method::Stop)?;
    Ok(())
}
