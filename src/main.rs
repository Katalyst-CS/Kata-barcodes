extern crate barcoders;

mod shared;
mod core;
mod domain;
mod infrastructure;


use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;
use core::generators::barcodes::proxy::BarcodeGeneratorProxy;
use std::env;
use infrastructure::http::cors::generate_cors;
use infrastructure::http::routes::index::main_router;
use salvo::conn::tcp::TcpAcceptor;
use salvo::conn::TcpListener;
use salvo::Listener;
use salvo::Router;
use salvo::Server;
use salvo::Service;

#[tokio::main]
async fn main() {
    let router: Router = main_router();
    let address: String = env::var("APP_ADRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: String = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let service:  Service = Service::new(router).hoop(generate_cors());
    let acceptor: TcpAcceptor = TcpListener::new(format!("{}:{}", address, port)).bind().await;
    println!("Server started in http://{}:{}", address, port);
    Server::new(acceptor).serve(service).await
}

fn mn_sinlge() {
    let barcode_bytes = match BarcodeGeneratorProxy::new("ean13")
   .generate("435443", 100, &"png") {
    Err(e) => { eprintln!("Error: {}", e); panic!("Fin de programa")} ,
    Ok(data) => data,
   };
   let file = File::create(&Path::new("barcode.png")).unwrap();
   let mut writer = BufWriter::new(file);
   writer.write(&barcode_bytes[..]).unwrap();
}
