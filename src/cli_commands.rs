use std::ffi::OsString;

pub enum CliCommans {
    Start(),
    Stop(),
    Status(),
    List(),
    Scan(OsString),
}

impl CliCommans {
    pub fn execute(&self) {
        match self {
            CliCommans::Start() => println!("I entered this command: Start"),
            CliCommans::Stop() => println!("I entered this command: Stop"),
            CliCommans::Status() => println!("I entered this command: Status"),
            CliCommans::List() => println!("I entered this command: List"),
            CliCommans::Scan(os_string) => {
                println!("I got this path: {:?}", os_string)
            }
        }
    }
}
