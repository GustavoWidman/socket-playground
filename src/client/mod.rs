use std::{
    io::{self, Read, Write},
    net::TcpStream,
    rc::Rc,
};

use crate::{cli, utils};

pub struct Client {
    pub listener: TcpStream,
    pub args: Rc<cli::Args>,
}

impl Client {
    pub fn new(args: Rc<cli::Args>) -> io::Result<Self> {
        Ok(Self {
            listener: TcpStream::connect((args.addr, args.port))?,
            args,
        })
    }

    fn on_read(&mut self, n: usize, buf: Vec<u8>) -> std::io::Result<()> {
        log::debug!(
            "Read {} bytes from stream",
            utils::bytes_to_pretty_string(n as u64),
        );
        log::trace!("Incoming data:\n{:?}", buf);

        if let Some(path) = &self.args.file {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)?;

            file.set_len(0)?; // truncate file (wipe if any results)
            file.write_all(&buf)?;

            log::info!(
                "Wrote {} to file {}",
                utils::bytes_to_pretty_string(n as u64),
                path.display()
            );
        } else {
            let stringified = String::from_utf8_lossy(&buf);
            log::info!("Received data:\n{}", stringified);
        }

        Ok(())
    }

    pub fn run(mut self) -> io::Result<()> {
        loop {
            log::info!("Waiting for data...");
            self.listener.write_all(b"REQUEST_DATA")?;

            let mut buffer = Vec::new();
            let mut temp_buffer = [0; 1024]; // Temporary buffer for reading data
            let mut bytes_read = 0;

            loop {
                let n = self.listener.read(&mut temp_buffer);
                match n {
                    Ok(0) => break, // No more data to read
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buffer[..n]); // Append the read data to the buffer
                        bytes_read += n;
                    }
                    Err(e) => {
                        return Err(e); // Return any read error
                    }
                }
            }

            if bytes_read == 0 {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            return self.on_read(bytes_read, buffer);
        }
    }
}
