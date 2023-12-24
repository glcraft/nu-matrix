use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, Write, Read};
use interprocess::local_socket::LocalSocketStream;

use crate::jrpc::Request;


pub fn send<T: Serialize>(stream: &mut LocalSocketStream, method: T) -> io::Result<()> {
    use std::io::BufWriter;
    let req_str = serde_json::to_string(&method)?;
    println!("Sending request: {req_str:?}");
    let mut bytes = [0u8; 8];
    (&mut bytes as &mut [u8]).write_u64::<BigEndian>(req_str.len() as u64).expect("Failed to write length of request to stream");
    println!("size: {}, bytes: {:?}", req_str.len(), bytes);
    // let mut stream = LocalSocketStream::connect(get_socket_name()).expect("Failed to connect to socket");
    let mut writer = BufWriter::new(stream);
    writer.write_all(&bytes).expect("Failed to write to stream");
    writer.write_all(req_str.as_bytes()).expect("Failed to write to stream");
    writer.flush().expect("Failed to flush stream");
    Ok(())
}

pub fn receive<T: DeserializeOwned>(stream: &mut LocalSocketStream) -> io::Result<T> {
    use std::io::BufReader;
    let mut reader = BufReader::new(stream);
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes).expect("Failed to read length of request from stream");
    
    let len = io::Cursor::new(bytes).read_u64::<BigEndian>().expect("Failed to read length of request from stream");
    println!("size: {}, bytes: {:?}", len, bytes);
    let mut res_vec = vec![0; len as usize];
    reader.read_exact(&mut res_vec).expect("Failed to read request from stream");
    let res: T = serde_json::from_slice(&res_vec).expect("Failed to parse request from stream");
    Ok(res)
}