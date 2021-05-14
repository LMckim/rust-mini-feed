use super::parser::Parser;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub struct Ingestor {
    pub addr: String,
    pub port: u16,
    pub point_storage: Arc<Mutex<Parser>>,
}
impl Ingestor {
    pub fn listen(&mut self) -> std::io::Result<()> {
        let bind_to: String = self.addr.to_owned() + ":" + &self.port.to_string();
        let listener = UdpSocket::bind(bind_to)?;
        let mut buf = [0; 125];
        loop {
            // println!("recieved");
            let _ = listener.recv(&mut buf)?;
            let mut p = self.point_storage.lock().unwrap();
            p.add_message(buf);
            buf = [0; 125];
            drop(p);
        }
    }
}
