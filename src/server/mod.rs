use std::{
    io::{self, Read, Write},
    net::TcpListener,
    rc::Rc,
};

use crate::{cli, utils};

mod server;

pub struct Server {
    pub inner: server::BlockingTcpServer,
    pub args: Rc<cli::Args>,
}

impl Server {
    pub fn new(args: Rc<cli::Args>) -> io::Result<Self> {
        if args.file.is_none() {
            log::error!(
                "File path not specified, the argument \"file\" is required when in server mode"
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "File path not specified, the argument \"file\" is required when in server mode",
            ));
        }
        if !args.file.as_ref().unwrap().exists() {
            log::error!("Given file path does not exist");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Given file path does not exist",
            ));
        }
        if !args.file.as_ref().unwrap().is_file() {
            log::error!("Given file path is not a file");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Given file path is not a file",
            ));
        }

        let server: server::BlockingTcpServer = TcpListener::bind((args.addr, args.port))?.into();

        Ok(Self {
            inner: server,
            args,
        })
    }

    fn on_request(&mut self) -> std::io::Result<()> {
        log::debug!("Incoming request");

        let path = self.args.file.as_ref().unwrap();

        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        log::trace!("Outgoing data:\n{:?}", buf);
        self.inner.write(&buf)?;
        self.inner.shutdown();

        log::info!(
            "Wrote {} to stream from file {}",
            utils::bytes_to_pretty_string(file.metadata()?.len()),
            path.display()
        );

        Ok(())
    }

    pub fn run(mut self) -> io::Result<()> {
        loop {
            let mut buf = Vec::from([0; 1024]);
            log::info!("Waiting for requests...");
            let bytes_read = self.inner.read(&mut buf)?;

            if bytes_read == 0 {
                log::info!("No data received, waiting... Buf state:\n{:?}", buf);
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            let stringified = String::from_utf8(buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            if stringified.trim().contains("REQUEST_DATA") {
                self.on_request()?;
            } else {
                log::error!("Invalid request: {}", stringified);
            }
        }
    }
}
