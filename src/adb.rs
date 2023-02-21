#[derive(Debug, PartialEq)]
pub enum SyncCmd {
    Data,
    Dent,
    Done,
    Fail,
    List,
    Okay,
    Quit,
    Recv,
    Send,
    Stat
}


impl SyncCmd {
    pub fn code(&self) -> &'static [u8; 4] {
        use self::SyncCmd::*;

        match *self {
            Data => b"DATA",
            Dent => b"DENT",
            Done => b"DONE",
            Fail => b"FAIL",
            List => b"LIST",
            Okay => b"OKAY",
            Quit => b"QUIT",
            Recv => b"RECV",
            Send => b"SEND",
            Stat => b"STAT"
        }
    }
}