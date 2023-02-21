use std::{collections::BTreeMap, net::{Ipv4Addr, TcpStream}};

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
