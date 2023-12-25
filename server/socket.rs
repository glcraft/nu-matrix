use std::io;

use interprocess::local_socket::{
    NameTypeSupport,
    tokio::LocalSocketListener
};

pub fn get_socket_name() -> String {
    match NameTypeSupport::query() {
        NameTypeSupport::OnlyPaths => {
            let tmp = std::env::temp_dir();
            tmp.join("nu-matrix.sock").to_str().expect("Failed to convert path to string").to_string()
        },
        NameTypeSupport::OnlyNamespaced | NameTypeSupport::Both => "@nu-matrix.sock".into(),
    }
}

pub fn make_listener(name: impl AsRef<str>) -> io::Result<LocalSocketListener> {
    LocalSocketListener::bind(name.as_ref()).or_else(|e| {
        if e.kind() != io::ErrorKind::AddrInUse {
            return Err(e);
        }
        let path = std::path::Path::new(name.as_ref());
        if !path.exists() {
            return Err(e);
        }
        match std::fs::remove_file(path) {
            Ok(_) => LocalSocketListener::bind(name.as_ref()),
            Err(e) => Err(e),
        }
    })
}