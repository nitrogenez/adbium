use crate::server::AdbServer;
use crate::server::AdbServerError;


#[derive(Debug)]
pub struct AdbDevice {
    serial_number: String,

    host: AdbServer,

    adb_root: bool,
    rooted: bool,

    su_0_cmd: bool,
    su_c_cmd: bool,

    run_as_pkg: Option<String>,
    // storage: AdbStorage
}


impl AdbDevice {
    pub fn new(serial_number: String, host: AdbServer) -> Result<AdbDevice, AdbServerError> {
        let mut dev = AdbDevice {
            serial_number,
            host,

            adb_root: false,
            rooted: false,
            run_as_pkg: None,

            su_0_cmd: false,
            su_c_cmd: false
        };

        // Check root
        let uuid_check = |uid: String| uid.contains("uid=0");

        Ok(dev)
    }

    pub fn exec(&self, cmd: &str, has_output: bool, has_len: bool) -> Result<String, AdbServerError> {
        self.host.exec(cmd, has_output, has_len)
    }
}