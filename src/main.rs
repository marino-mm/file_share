use std::{
    io::{Read, Write, stdin, stdout},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use file_share::LogFile;

fn test() {
    let log: Arc<Mutex<LogFile>> = Arc::new(Mutex::new(LogFile::new()));

    log.lock().unwrap().log("My first message!".into());

    let socket = 9999;
    let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let socket = SocketAddr::new(ip, socket);

    let mut run = Arc::new(Mutex::new(true));

    let s_run_flag = Arc::clone(&mut run);
    let r_run_flag = Arc::clone(&run);

    let reciver_thread = reciver_thread(r_run_flag, &socket, Arc::clone(&log));
    let sender_thread = sender_thread(s_run_flag, &socket, Arc::clone(&log));

    reciver_thread
        .join()
        .expect("Failed to join reciver thread");
    sender_thread.join().expect("Failed to join sender thread");
}

fn reciver_thread(
    r_run_flag: Arc<Mutex<bool>>,
    socket: &SocketAddr,
    log: Arc<Mutex<LogFile>>,
) -> thread::JoinHandle<()> {
    let listener =
        TcpListener::bind(socket).expect(&format_args!("Failed to bind to {}", socket).to_string());

    log.lock()
        .unwrap()
        .log("TcpListener created and bind".into());

    let reciver = thread::Builder::new()
        .name("Reciver".to_owned())
        .spawn(move || {
            let mut buf = [0; 1000];
            while *r_run_flag.lock().unwrap() {
                match listener.accept() {
                    Ok((mut tcp, _socket)) => {
                        log.lock().unwrap().log("Connection accepted".into());
                        match tcp.read(&mut buf) {
                            Ok(_msg_size) => {
                                let msg = String::from_utf8_lossy(&buf);

                                println!("I have recived this message: {}", msg);
                                if msg.contains("Bye") {
                                    *r_run_flag.lock().unwrap() = false;
                                }
                                stdout().flush().unwrap();
                                buf = [0; 1000];
                            }
                            Err(e) => {
                                print!("I got this error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => print!("I got this error: {:?}", e),
                }
            }
        })
        .expect("Reciver thread crashed");
    reciver
}

fn sender_thread(
    s_run_flag: Arc<Mutex<bool>>,
    socket: &SocketAddr,
    log: Arc<Mutex<LogFile>>,
) -> thread::JoinHandle<()> {
    let mut sender = TcpStream::connect(socket)
        .expect(&format_args!("Failed to connect to {}", socket).to_string());

    log.lock().unwrap().log("TcpSender connect".into());

    let sender = thread::Builder::new()
        .name("Sender".to_owned())
        .spawn(move || {
            let stdin = stdin();
            let mut input_buffer = String::with_capacity(1000);

            while *s_run_flag.lock().unwrap() {
                match stdin.read_line(&mut input_buffer) {
                    Ok(_n) => {
                        log.lock()
                            .unwrap()
                            .log("Succesfully got message from stdin".into());
                        let write_buff = input_buffer.clone().into_bytes();
                        sender
                            .write_all(write_buff.as_slice())
                            .expect("Failed to write to sender TcpStream");
                        sender.flush().unwrap();

                        log.lock().unwrap().log("Succesfully sent message".into());

                        input_buffer.clear();
                    }
                    Err(e) => {
                        *s_run_flag.lock().unwrap() = false;
                        println!("Stoping program error occured: {:?}", e);
                    }
                }
            }
        })
        .expect("Sender thread crashed");
    sender
}

fn main() {
    test();
}
