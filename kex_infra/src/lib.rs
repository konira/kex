pub mod server;
pub mod client;
pub mod dto;
use std::sync::Arc;
use dto::ServerOptions::ServerOptions;
use kex_domain::Entitys::{Envent::EventEmitter, Payload};
pub use hex;

pub fn server(args: ServerOptions, event: Arc<EventEmitter>){
    server::server(args, event);
}
pub fn response(dst_ip: [u8; 4], data: Vec<u8>,chunk_size: usize){
   server::response(dst_ip, &Payload::Payload::from_bytes(&data), chunk_size);
}