use std::io::prelude::*;
use std::time::Duration;
use std::io::ErrorKind;

mod stats;

fn read_all(stream: &mut std::net::TcpStream,
    stats: &mut stats::Stats){
    let mut rcv_buf = vec![0; 10_000_000];
    loop {
        let bytes_rcvd = stream.read(&mut rcv_buf);
        match bytes_rcvd {
            Ok(n_rcvd)=>{
                stats.receiving.add_value(n_rcvd as u32);
                if n_rcvd == 0 {
                    break;
                }
                },
            Err(e)=>{
                match e.kind() {
                ErrorKind::Interrupted | ErrorKind::TimedOut =>{
                    break;
                    },
                e =>{
                    println!("some error {:?}", e);
                    break;
                    },
                };
            }
        }
    }
}

fn write_message(stream: &mut std::net::TcpStream,
    stats: &mut stats::Stats){
    let mut msg: std::vec::Vec<u8> = std::vec::Vec::new();
    for _ in  1..100_000_000 {
        msg.push(1);
    }

    loop {
        let bytes_sent = stream.write(msg.as_slice());
        match bytes_sent{
            Ok(n_sent)=>{
                stats.sending.add_value(n_sent as u32);
                read_all(stream, stats);
            },
            Err(_)=>{

            },
        };

        let now = chrono::Utc::now();
        println!("Stats: {:?} {}", stats, now );
    }
}

fn main() {

    let mut stats = stats::Stats{
        sending: stats::Statistics::new(),
        receiving: stats::Statistics::new(),
    };



    let s = std::net::TcpStream::connect("localhost:8080");
    match s{
        Ok(mut stream)=>{
            let _ = stream.set_read_timeout(Some(Duration::from_nanos(1)));
            write_message(&mut stream, &mut stats);
        },
        Err(_) => {
            s.expect("Unable to connect to server");
        },
    };
}
