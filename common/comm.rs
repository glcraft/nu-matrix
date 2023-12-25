use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serde::{Serialize, de::DeserializeOwned};
use std::io::{self, Write, Read};
use futures::io::{AsyncWrite, AsyncRead};
use interprocess::local_socket::{LocalSocketStream, tokio::LocalSocketStream as AsyncLocalSocketStream};
use log::debug;


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
    debug!("Sending request ({0} bytes): {req_str}", req_str.len());
    let mut bytes = [0u8; 8];
    (&mut bytes as &mut [u8]).write_u64::<BigEndian>(req_str.len() as u64).map_err(Error::IntToBytes)?;
    // let mut stream = LocalSocketStream::connect(get_socket_name()).expect("Failed to connect to socket");
    let mut writer = BufWriter::new(stream);
    writer.write_all(&bytes)?;
    writer.write_all(req_str.as_bytes())?;
    writer.flush()?;
    debug!("Request sent");
    Ok(())
}

pub fn receive<T: DeserializeOwned>(stream: &mut LocalSocketStream) -> Result<T, Error> {
    use std::io::BufReader;
    let mut reader = BufReader::new(stream);
    let mut bytes = [0u8; 8];
    debug!("Receiving response size");
    reader.read_exact(&mut bytes)?;
    let len = io::Cursor::new(bytes).read_u64::<BigEndian>().map_err(Error::BytesToInt)?;
    debug!("    Response size: {0} bytes", len);
    let mut res_vec = vec![0; len as usize];
    reader.read_exact(&mut res_vec)?;
    debug!("Response received: {}", String::from_utf8_lossy(&res_vec));
    let res: T = serde_json::from_slice(&res_vec)?;
    Ok(res)
}

pub async fn async_send<T: Serialize>(stream: &mut AsyncLocalSocketStream, method: T) -> Result<(), Error> {
    use futures::io::AsyncWriteExt;
    let req_str = serde_json::to_string(&method)?;
    debug!("Sending request ({0} bytes): {req_str}", req_str.len());
    let mut bytes = [0u8; 8];
    (&mut bytes as &mut [u8]).write_u64::<BigEndian>(req_str.len() as u64).map_err(Error::IntToBytes)?;
    stream.write_all(&bytes).await?;
    stream.write_all(req_str.as_bytes()).await?;
    stream.flush().await?;
    debug!("Request sent");
    Ok(())
}

pub async fn async_receive<T: DeserializeOwned>(stream: &mut AsyncLocalSocketStream) -> Result<T, Error> {
    use futures::io::AsyncReadExt;
    let mut bytes = [0u8; 8];
    debug!("Receiving response size");
    stream.read_exact(&mut bytes).await?;
    let len = io::Cursor::new(bytes).read_u64::<BigEndian>().map_err(Error::BytesToInt)?;
    debug!("    Response size: {0} bytes", len);
    let mut res_vec = vec![0; len as usize];
    stream.read_exact(&mut res_vec).await?;
    debug!("Response received: {}", String::from_utf8_lossy(&res_vec));
    let res: T = serde_json::from_slice(&res_vec)?;
    Ok(res)
}