use std::{error::Error, net::UdpSocket, sync::Arc, thread, time::Duration};
use thermo_client::Temperature;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:34253";
    let socket = UdpSocket::bind(addr).map_err(|e| format!("Error binding socket: {}", e))?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    let temperature = Arc::new(Temperature::default());
    let temperature_clone = temperature.clone();

    thread::spawn(move || loop {
        let mut buf = [0; 10];
        match socket.recv_from(&mut buf) {
            Ok((_amt, src)) => {
                let char_symbol = buf[0];

                match char::from(char_symbol) {
                    't' => {
                        let arr = [buf[1], buf[2], buf[3], buf[4]];
                        let val = f32::from_be_bytes(arr);
                        temperature_clone.set(val);
                    }

                    _ => {
                        println!(
                            "incorrect data received from {}. Expected 't' but got {}",
                            src,
                            char::from(char_symbol)
                        );
                    }
                }
            }
            Err(e) => println!("Error receiving data: {}", e),
        }
    });

    for _ in 0..30 {
        println!("Client: {:?}°", temperature.get().round());
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
