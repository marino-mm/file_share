use std::{env, fs::File, io::Write};

pub struct LogFile {
    path: File,
}

impl LogFile {
    pub fn new() -> LogFile {
        let mut path = env::current_dir().expect("Total fuckup");

        path.push("log.txt");

        LogFile {
            path: File::create(path.as_path()).expect("Wrong file path"),
        }
    }

    pub fn log(&mut self, msg: String) {
        // let timestamp = time::Instant::now().;

        self.path.write_all(msg.into_bytes().as_slice()).unwrap();
        self.path.write_all(b"\n").unwrap();
    }
}
