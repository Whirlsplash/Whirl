// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

//! The API, for external interaction.

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png",
  html_favicon_url = "https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use actix_web::web::{resource, scope};

mod routes;

pub struct Api;
impl Api {
  /// Begin handling connections on the web-server.
  ///
  /// # Errors
  /// - An error may arise if the web-server is unable to bind the specified
  ///   port.
  ///
  /// # Panics
  /// - A panic may occur if the mpsc sender is unable to send a clone of the
  ///   server.
  pub async fn listen(
    tx: std::sync::mpsc::Sender<actix_web::dev::Server>,
    address: &str,
  ) -> std::io::Result<()> {
    let mut sys = actix_web::rt::System::new("api");

    let server = actix_web::HttpServer::new(|| {
      actix_web::App::new()
        .wrap(actix_cors::Cors::default().allow_any_origin())
        .service(resource("/").to(|| async { "Whirlsplash" }))
        .service(
          scope("/api/v1")
            .service(resource("/statistics").to(routes::stats::statistics))
            .service(
              scope("/worlds")
                .service(resource("/vip").to(routes::worlds::vip::vip))
                .service(resource("/info").to(routes::worlds::info::info)),
            ),
        )
    })
    .bind(address)?
    .run();

    info!("http api now listening at {}", address);

    tx.send(server.clone()).unwrap();

    sys.block_on(server)
  }
}

/// Spawn and return a thread handle for the API — which
/// should be — instantiated by the `whirl_api` crate.
///
/// # Panics
/// - A panic may occur if the mpsc sender is unable to send a clone of the
///   server.
#[must_use]
pub fn make() -> tokio::task::JoinHandle<()> {
  // actix_web::rt::System::new("").block_on(rx.recv().unwrap().stop(true));

  tokio::spawn(async move {
    crate::Api::listen(
      std::sync::mpsc::channel().0,
      &*format!(
        "0.0.0.0:{}",
        whirl_config::Config::get().whirlsplash.api.port
      ),
    )
    .await
    .unwrap();
  })
}
