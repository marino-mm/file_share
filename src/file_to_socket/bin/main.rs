use std::{
    fs::File,
    io::{BufReader, Read, Write, stdout},
    net::{Ipv4Addr, UdpSocket},
    thread::{self, JoinHandle},
    time::Duration,
};

const PORT: u16 = 6767;

fn sender_thread() -> JoinHandle<()> {
    let socket = UdpSocket::bind((Ipv4Addr::BROADCAST, 0)).expect("Could not bind sender thread");
    socket
        .set_broadcast(true)
        .expect("Could not set broadcast in sender thread");

    let file_path = File::open("/home/matej/Projects/file_share/test_files/2_kb.txt").expect("Couldn't open file");
    let mut buf_reader = BufReader::new(file_path);
    let mut buf = [0; 4096];
    let join_handle = thread::spawn(move || {
        socket
            .connect((Ipv4Addr::BROADCAST, PORT))
            .expect("Sender socket could not connect to UNSPECIFIED adress");

        while let Ok(n) = buf_reader.read(&mut buf) {
            if n > 0 {
                let _ = socket
                    .send(&mut buf[..n])
                    .expect("Could not send read buffer contet to sender socket");
            } else {
                // socket
                //     .send(String::from("CLOSE CONNECTION!!!").as_bytes())
                //     .unwrap();
                break;
            }
        }
    });
    join_handle
}

fn reader_thread() -> JoinHandle<()> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT)).expect("Could not bind reader thread");
    // socket
    //     .connect((Ipv4Addr::UNSPECIFIED, PORT))
    //     .expect("Reader socket could not connect to UNSPECIFIED adress");
    //

    let mut stdout = stdout();
    let mut stdout_buff = [0; 4096];
    let mut fuck_you_count = 0;
    let join_handle = thread::spawn(move || {
        while let Ok((n, addr)) = socket.recv_from(&mut stdout_buff) {
            if n > 0 {
                if &stdout_buff[..n] == String::from("CLOSE CONNECTION!!!").as_bytes() {
                    break;
                }
                stdout
                    .write(format!("n: {}add: {:?}\n", n, addr).as_bytes())
                    .unwrap();
                stdout
                    .write(&stdout_buff[..n])
                    .unwrap();
                stdout.flush().unwrap();
                socket
                    .send_to(b"Fuck you", addr)
                    .unwrap();
                fuck_you_count += 1;
                if fuck_you_count >= 4 {
                    break;
                }
            } else {
                break;
            }
        }
    });
    join_handle
}

fn main() {
    let sender_count = 4;
    let mut sender_list = vec![];
    let listener = reader_thread();

    for _ in 0..sender_count {
        let sender = sender_thread();
        sender_list.push(sender);
    }

    for sender in sender_list {
        sender.join().unwrap();
    }

    // let _ = sender.join();
    let _ = listener.join();
}
