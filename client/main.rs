mod process;

use nu_matrix_common::{
    jrpc::Request,
    methods::Method
};
use interprocess::local_socket::{LocalSocketStream, NameTypeSupport};
use std::io::BufWriter;

fn main() -> std::io::Result<()>{
    let ppid = process::parent_id().expect("Failed to get parent process ID");
    let name = match NameTypeSupport::query() {
        NameTypeSupport::OnlyPaths => {
            let tmp = std::env::temp_dir();
            tmp.join("nu-matrix.sock").to_str().expect("Failed to convert path to string").to_string()
        },
        NameTypeSupport::OnlyNamespaced | NameTypeSupport::Both => "@nu-matrix.sock".into(),
    };

    let stream = LocalSocketStream::connect(name)?;

    let writer = BufWriter::new(stream);

    serde_json::to_writer(writer, &Request::new(ppid, Method::Stop, None))?;
    Ok(())
}
