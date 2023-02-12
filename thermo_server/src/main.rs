use rand::Rng;
use std::{net::UdpSocket, thread, time::Duration};

fn main() {
    let mut rng = rand::thread_rng();
    let bind_addr = "127.0.0.1:34254";
    let dest_addr = "127.0.0.1:34253";
    let socket = UdpSocket::bind(bind_addr).expect("couldn't bind to address");
    println!("Starting send temperature from {bind_addr} to {dest_addr}");

    loop {
        let temperature: f32 = rng.gen_range(0.0..=40.0);
        let bytes = ["t".as_bytes(), &temperature.to_be_bytes()].concat();
        socket
            .send_to(&bytes, dest_addr)
            .expect("can't send temperature");

        println!("Server: {:?}°", temperature.round());
        thread::sleep(Duration::from_secs(1))
    }
}
