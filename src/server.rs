use std::error::Error;

use std::net::Ipv4Addr;
use std::net::TcpStream;

use std::num::ParseIntError;
use std::num::TryFromIntError;
use std::str::Utf8Error;
use std::time::Duration;

use std::io::Read;
use std::io::Write;

use crate::adb::SyncCmd;


#[derive(Debug, Clone, PartialEq)]
pub struct AdbServer {
    host: Ipv4Addr,
    port: u32,
}


#[derive(Debug)]
pub enum AdbServerError {
    Offline,
    NoDevices,
    MultipleDevices,
    MissingPackage,
    InvalidStorage,
    UnknownError(Box<dyn Error>),

    AdbError(String)
}

pub type AdbServerResult = Result<AdbServer, AdbServerError>;

impl From<Utf8Error> for AdbServerError {
    fn from(value: Utf8Error) -> Self {
        AdbServerError::AdbError("Utf8 Conversion Error".to_string())
    }
}

impl From<std::io::Error> for AdbServerError {
    fn from(value: std::io::Error) -> Self {
        AdbServerError::UnknownError(Box::new(value))
    }
}

impl From<ParseIntError> for AdbServerError {
    fn from(value: ParseIntError) -> Self {
        AdbServerError::UnknownError(Box::new(value))
    }
}


impl Default for AdbServer {
    fn default() -> Self {
        AdbServer { host: Ipv4Addr::from([127,0,0,1]), port: 5037 }
    }
}


impl AdbServer {
    pub fn new(host: Ipv4Addr, port: u32) -> AdbServerResult {
        Ok(AdbServer { host, port })
    }

    pub fn encode_msg(msg: &str) -> Result<String, TryFromIntError> {
        let hexlen = u16::try_from(msg.len()).map(|len| format!("{:0>4X}", len))?;

        Ok(format!("{}{}", hexlen, msg))
    }

    pub fn connect(&self) -> Result<TcpStream, std::io::Error> {
        let adb_stream = TcpStream::connect(format!("{:?}{:?}", self.host.to_string().to_owned(), self.port.to_owned()))?;

        adb_stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        adb_stream.set_write_timeout(Some(Duration::from_secs(2)))?;

        Ok(adb_stream)
    }

    fn read_length<R: Read>(stream: &mut R) -> Result<usize, AdbServerError> {
        let mut bytes: [u8; 4] = [0; 4];

        stream.read_exact(&mut bytes)?;
        let response = std::str::from_utf8(&bytes)?;

        Ok(usize::from_str_radix(response, 16)?)
    }

    fn read_response(&self, stream: &mut TcpStream, has_output: bool, has_len: bool) -> Result<Vec<u8>, AdbServerError> {
        let mut bytes: [u8; 1024] = [0; 1024];

        stream.read_exact(&mut bytes[0..4])?;

        if !bytes.starts_with(SyncCmd::Okay.code()) {
            let n = bytes.len().min(AdbServer::read_length(stream)?);

            stream.read_exact(&mut bytes[0..n])?;

            let message = std::str::from_utf8(&bytes[0..n]).map(|s| format!("AdbError: {:?}", s))?;
            return Err(AdbServerError::AdbError(message))
        }

        let mut response = Vec::new();

        if has_output {
            stream.read_to_end(&mut response)?;

            if response.starts_with(SyncCmd::Fail.code()) {
                response = response.split_off(8);

                let message = std::str::from_utf8(&response).map(|s| format!("AdbError: {:?}", s))?;
                return Err(AdbServerError::AdbError(message))
            }
        }
        Ok(response)
    }

    pub fn exec(&self, cmd: &str, has_output: bool, has_len: bool) -> Result<String, AdbServerError> {
        let mut adb_stream = self.connect()?;

        adb_stream.write_all(AdbServer::encode_msg(cmd).unwrap().as_bytes())?;

        let bytes = self.read_response(&mut adb_stream, has_output, has_len)?;
        let response = std::str::from_utf8(&bytes)?;

        Ok(response.to_owned())
    }
}