use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serde::{Serialize, de::DeserializeOwned};
use std::io::{self, Write, Read};
use interprocess::local_socket::LocalSocketStream;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Failed to exchange data: {0}")]
    Exchange(#[from] io::Error),
    #[error("Failed to read integer from bytes: {0}")]
    BytesToInt(io::Error),
    #[error("Failed to write integer to bytes: {0}")]
    IntToBytes(io::Error),
    #[error("Failed to convert integers: {0}")]
    ConvertInt(std::num::TryFromIntError),
}


pub fn send<T: Serialize>(stream: &mut LocalSocketStream, method: T) -> Result<(), Error> {
    use std::io::BufWriter;
    let req_str = serde_json::to_string(&method)?;
    println!("Sending request: {req_str:?}");
    let mut bytes = [0u8; 8];
    (&mut bytes as &mut [u8]).write_u64::<BigEndian>(req_str.len() as u64).map_err(Error::IntToBytes)?;
    println!("size: {}, bytes: {:?}", req_str.len(), bytes);
    // let mut stream = LocalSocketStream::connect(get_socket_name()).expect("Failed to connect to socket");
    let mut writer = BufWriter::new(stream);
    writer.write_all(&bytes)?;
    writer.write_all(req_str.as_bytes())?;
    writer.flush()?;
    Ok(())
}

pub fn receive<T: DeserializeOwned>(stream: &mut LocalSocketStream) -> Result<T, Error> {
    use std::io::BufReader;
    let mut reader = BufReader::new(stream);
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes)?;
    
    let len = io::Cursor::new(bytes).read_u64::<BigEndian>().map_err(Error::BytesToInt)?;
    println!("size: {}, bytes: {:?}", len, bytes);
    let mut res_vec = vec![0; len as usize];
    reader.read_exact(&mut res_vec)?;
    let res: T = serde_json::from_slice(&res_vec)?;
    Ok(res)
}