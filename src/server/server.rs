use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct BlockingTcpServer {
    pub listener: TcpListener,
    pub current_client: Option<TcpStream>,
}

impl BlockingTcpServer {
    pub fn shutdown(&mut self) {
        log::trace!("Shutting down client...");
        drop(self.current_client.take());
    }
}

impl Read for BlockingTcpServer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let client = self
            .current_client
            .as_mut()
            .filter(|client| client.peek(&mut [0; 1]).is_ok());

        let client = match client {
            Some(client) => client,
            None => {
                let (client, _) = self.listener.accept()?;
                self.current_client = Some(client);

                self.current_client.as_mut().unwrap() // safe unwrap
            }
        };

        client.read(buf)
    }
}
impl Write for BlockingTcpServer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let client = match self.current_client.as_mut() {
            Some(client) => client,
            None => {
                let (client, _) = self.listener.accept()?;
                self.current_client = Some(client);

                self.current_client.as_mut().unwrap() // safe unwrap
            }
        };

        client.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let client = self
            .current_client
            .as_mut()
            .filter(|client| client.peek(&mut [0; 1]).is_ok());

        let client = match client {
            Some(client) => client,
            None => {
                let (client, _) = self.listener.accept()?;
                self.current_client = Some(client);

                self.current_client.as_mut().unwrap() // safe unwrap
            }
        };

        client.flush()
    }
}
impl From<TcpListener> for BlockingTcpServer {
    fn from(listener: TcpListener) -> Self {
        BlockingTcpServer {
            listener,
            current_client: None,
        }
    }
}
