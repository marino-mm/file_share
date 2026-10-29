use std::{
    env::{self},
    fmt::Debug,
    io,
    path::{Path, PathBuf},
};

mod cli_commands;
mod log_file;
mod sha_256;

#[derive(Debug)]
struct App {
    cwd: PathBuf,
}

fn main() -> io::Result<()> {
    let args = env::args_os();
    for arg in args {
        println!("{:?}", arg);
    }
    // println!("{}", env::current_dir()?.display());
    let app = App {
        cwd: env::current_dir()?.to_owned(),
    };
    print!("{:?}", app);
    Ok(())
}
