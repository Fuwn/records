// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

// #[macro_use]
// extern crate actix_web;
// #[macro_use]
// extern crate log;

use std::env;

const INDEX: &str = r#"The Fuwn Records
================

Last updated: 2021. 05. 18.
"#;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Environment
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=info");

  // Logging
  pretty_env_logger::init();

  // Web-server
  actix_web::HttpServer::new(|| {
    actix_web::App::new()
      .wrap(actix_cors::Cors::default().allow_any_origin())
      .wrap(actix_web::middleware::Logger::default())
      .service(actix_web::web::resource("/").to(|| async { INDEX }))
      .service(actix_files::Files::new("/records", "./records/_/").show_files_listing())
  })
  .bind(format!(
    "0.0.0.0:{}",
    env::var("PORT").unwrap_or_else(|_| "80".to_string())
  ))?
  .run()
  .await
}
