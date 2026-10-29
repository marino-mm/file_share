use std::{env, fs::File, io::Write, thread::current, time::SystemTime};

pub struct LogFile {
    path: File,
}

impl LogFile {
    fn wirite_header(&mut self) {
        self.path
            .write_all("TIMESTAMP\tTHREAD_ID\tTHREAD_NAME\tMESSAGE\n".as_bytes())
            .expect("Failed to write header to log file");
    }

    fn quick_timestamp(&self) -> String {
        let dur = SystemTime::UNIX_EPOCH
            .elapsed()
            .unwrap();
        let dur = dur.as_millis();
        let milis = dur % 1_000;
        let secs = dur / 1_000 % 60;
        let mins = dur / 60_000 % 60;
        let hours = dur / 360_000 % 24;

        format!("{hours}:{mins}:{secs}.{milis}")
    }

    pub fn new() -> LogFile {
        let mut path = env::current_dir().expect("Total fuckup");
        path.push("log.txt");
        let mut file = LogFile {
            path: File::create(path.as_path()).expect("Wrong file path"),
        };
        file.wirite_header();
        file
    }

    pub fn log(&mut self, msg: &str) {
        let thread = current();
        let thread_id = thread.id();
        let thread_name = thread
            .name()
            .unwrap_or_default();

        let row = format!(
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            self.quick_timestamp(),
            thread_id,
            thread_name,
            msg
        );
        self.path
            .write_all(row.as_bytes())
            .expect("Failed to write row to log");
    }
}
