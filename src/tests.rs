use std::{collections::BTreeMap, net::{Ipv4Addr, TcpStream}, time::Duration};

use crate::{device_info::*, server::{AdbServer, AdbServerError}};
// use crate::device::*;


#[test]
fn test_device_info_parsing() {
    let string: &str = &"serial\tdevice k1:v1 k2:v2";

    let testserial: String = "serial".to_string();
    let mut testinfo: BTreeMap<String, String> = BTreeMap::new();

    testinfo.insert("k1".to_string(), "v1".to_string());
    testinfo.insert("k2".to_string(), "v2".to_string());

    let devinfo = AdbDeviceInfo::parse_info(string);


    assert_eq!(devinfo, Some(AdbDeviceInfo::new(testserial.to_owned(), testinfo.to_owned())));
}


#[test]
fn test_connection() {
    let serv = AdbServer::new(Ipv4Addr::from([127,0,0,1]), 5037, Duration::from_secs(2), Duration::from_secs(2)).expect("Couldn't create server");
    let conn = serv.connect().expect("Failed to estabilish connection");
}

#[test]
fn test_device_search() {
    let serv = AdbServer::new(Ipv4Addr::from([127,0,0,1]), 5037, Duration::from_secs(2), Duration::from_secs(2)).expect("Couldn't create server");
    let dev = serv.get_active_device().expect("Couldn't find device");
}

#[test]
fn test_server_io() {
    let serv = AdbServer::new(Ipv4Addr::from([127,0,0,1]), 5037, Duration::from_secs(2), Duration::from_secs(2)).expect("Failed to estabilish connection");

    serv.exec("devices", true, true).expect("Failed to test server io");
}