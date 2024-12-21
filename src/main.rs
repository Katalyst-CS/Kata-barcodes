extern crate barcoders;

mod core;
mod domain;
mod infrastructure;
mod shared;

use core::generators::barcodes::proxy::BarcodeGeneratorProxy;
use infrastructure::http::cors::generate_cors;
use infrastructure::http::routes::index::main_router;
use log::info;
use salvo::conn::tcp::TcpAcceptor;
use salvo::conn::TcpListener;
use salvo::Listener;
use salvo::Router;
use salvo::Server;
use salvo::Service;
use shared::crypto::check_sign;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

#[tokio::main]
async fn main() {
    colog::init();
    info!("Logger init");
    let router: Router = main_router();
    let address: String = env::var("APP_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: String = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let service: Service = Service::new(router).hoop(generate_cors());
    let acceptor: TcpAcceptor = TcpListener::new(format!("{}:{}", address, port))
        .bind()
        .await;
    info!("Server started in http://{}:{}", address, port);
    Server::new(acceptor).serve(service).await
}

fn mn_sinlge() {
    let barcode_bytes = match BarcodeGeneratorProxy::new("ean13").generate("435443", 100, &"png") {
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Fin de programa")
        }
        Ok(data) => data,
    };
    let file = File::create(&Path::new("barcode.png")).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&barcode_bytes[..]).unwrap();
}
