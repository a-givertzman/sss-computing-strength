use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time, io};
use log2::*;

use crate::Error;

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
//
pub fn get_args() -> Result<(String, String), Error> {
    let host: String;
    let port;
    let stdin_channel = spawn_stdin_channel();
    thread::sleep(time::Duration::from_millis(100));
    match stdin_channel.try_recv() {
        Ok(input) => {
            info!("read from stdin: {input}");
            let json_data: serde_json::Value = serde_json::from_str(&input)?;
            info!("io::stdin(): {}", json_data);
            host = json_data
                .get("api-host")
                .ok_or(Error::FromString(
                    "Parse param error: no api-host".to_owned(),
                ))?
                .to_string();
            port = json_data
                .get("api-port")
                .ok_or(Error::FromString(
                    "Parse param error: no api-host".to_owned(),
                ))?
                .to_string();            
        }
        Err(error) => {
            error!("error read from stdin!: {error}");
            info!("set default host:0.0.0.0, port:8080");
            host = "0.0.0.0".to_string();
            port = "8080".to_string();
        },
    }
   Ok((host, port))
}