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
use crate::device::AdbDevice;
use crate::device_info::AdbDeviceInfo;


#[derive(Debug, Clone, PartialEq)]
pub struct AdbServer {
    host: Ipv4Addr,
    port: u32,

    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>
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
        AdbServer {
            host: Ipv4Addr::from([127,0,0,1]),
            port: 5037,

            read_timeout: Some(Duration::from_secs(2)),
            write_timeout: Some(Duration::from_secs(2))
        }
    }
}


impl AdbServer {
    pub fn new(host: Ipv4Addr, port: u32, read_timeout: Duration, write_timeout: Duration) -> AdbServerResult {
        Ok(AdbServer {
            host,
            port,

            read_timeout: Some(read_timeout),
            write_timeout: Some(write_timeout),
        })
    }

    pub fn encode_msg(msg: &str) -> Result<String, TryFromIntError> {
        let hexlen = u16::try_from(msg.len()).map(|len| format!("{:0>4X}", len))?;

        Ok(format!("{}{}", hexlen, msg))
    }

    pub fn connect(&self) -> Result<TcpStream, std::io::Error> {
        let addr_str: String = self.host.to_string().to_owned();
        let addr = format!("{}:{}", addr_str, self.port.to_owned());

        let adb_stream = TcpStream::connect(addr)?;

        adb_stream.set_read_timeout(self.read_timeout)?;
        adb_stream.set_write_timeout(self.write_timeout)?;

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

            let message = std::str::from_utf8(&bytes[0..n]).map(|s| format!("AdbError: {}", s))?;
            return Err(AdbServerError::AdbError(message))
        }

        let mut response = Vec::new();

        if has_output {
            stream.read_to_end(&mut response)?;

            if response.starts_with(SyncCmd::Okay.code()) {
                response = response.split_off(4);
            }

            if response.starts_with(SyncCmd::Fail.code()) {
                response = response.split_off(8);

                let message = std::str::from_utf8(&response).map(|s| format!("AdbError: {}", s))?;
                return Err(AdbServerError::AdbError(message))
            }

            if has_len {
                if response.len() >= 4 {
                    let message = response.split_off(4);
                    let message_slice: &mut &[u8] = &mut &*response;

                    let n = AdbServer::read_length(message_slice)?;

                    if n != message.len() {
                        println!("warning: unknown adb response (response length: {}, message length: {})", n, message.len());
                    }

                    return Ok(message);
                }
                else {
                    return Err(AdbServerError::AdbError(format!("adb server responded but didn't send any hex string length: {:?}", std::str::from_utf8(&response)?)))
                }
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

    pub fn get_devices_info<D: FromIterator<AdbDeviceInfo>>(&self) -> Result<D, AdbServerError> {
        let response: String = self.exec("devices -l", true, true)?;
        let device_infos: D = response.lines().filter_map(AdbDeviceInfo::parse_info).collect();

        Ok(device_infos)
    }

    pub fn get_active_device(&self) -> Result<AdbDevice, AdbServerError> {
        let device_infos: Vec<AdbDeviceInfo> = self.get_devices_info()?;

        let active_info: &AdbDeviceInfo = device_infos.get(0).to_owned().unwrap().to_owned();
        let active: AdbDevice = AdbDevice::new(active_info.get_serial_number().to_owned(), self.to_owned())?;

        Ok(active)
    }
}