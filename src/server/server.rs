// Copyleft (ɔ) 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use std::{error::Error, fmt, net::SocketAddr, sync::Arc};

use tokio::{
  net::{TcpListener, TcpStream},
  sync::Mutex,
};

use crate::server::interaction::shared::Shared;

#[derive(Debug)]
pub enum ServerType {
  AnonRoomServer,
  AnonUserServer,
  AutoServer,
  RoomServer,
  UserServer,
}
// https://stackoverflow.com/a/32712140/14452787
impl fmt::Display for ServerType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

#[async_trait::async_trait]
pub trait Server {
  async fn listen(address: &str, server_type: ServerType) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    let state = Arc::new(Mutex::new(Shared::new()));
    let mut counter = 0;

    info!(
      "server of type {} now listening at {}",
      server_type.to_string(),
      address
    );

    loop {
      let (stream, address) = listener.accept().await?;
      counter += 1;
      let state = Arc::clone(&state);

      trace!("accepted client at {}", address);

      tokio::spawn(async move {
        if let Err(e) = Self::handle(state, stream, address, counter).await {
          error!("an error occurred: {}", e);
        }
      });
    }
  }

  async fn handle(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    _address: SocketAddr,
    count: usize,
  ) -> Result<(), Box<dyn Error>>;
}
