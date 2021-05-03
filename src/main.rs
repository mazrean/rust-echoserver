use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address).context("failed to bind address")?;

    loop {
        let (mut stream, _) = listener.accept().context("failed to accept listener")?;
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
