use std::collections::BTreeMap;


#[derive(Debug, PartialEq)]
pub struct AdbDeviceInfo {
    serial_number: String,
    info: BTreeMap<String, String>
}


impl AdbDeviceInfo {
    pub fn new(serial_number: String, info: BTreeMap<String, String>) -> AdbDeviceInfo {
        return AdbDeviceInfo { serial_number, info }
    }

    pub fn parse_info(string: &str) -> Option<AdbDeviceInfo> {
        let mut info_pairs = string.split_whitespace();

        let serial_code = info_pairs.next();
        let device_state = info_pairs.next();

        if let (Some(serial_number), Some("device")) = (serial_code, device_state) {
            let info: BTreeMap<String, String> = info_pairs.filter_map(|pair| {
                let mut kv = pair.split(':');

                if let (Some(key), Some(value), None) = (kv.next(), kv.next(), kv.next()) {
                    Some((key.to_owned(), value.to_owned()))
                }
                else {
                    None
                }
            }).collect();

            Some(AdbDeviceInfo { serial_number: serial_number.to_owned(), info })
        }
        else {
            None
        }
    }

    pub fn get_serial_number(&self) -> String {
        self.serial_number.to_owned()
    }

    pub fn get_info(&self) -> BTreeMap<String, String> {
        self.info.to_owned()
    }
}