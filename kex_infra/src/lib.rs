use std::sync::Arc;
use kex_domain::Entitys::Envent::EventEmitter;
pub mod server;

pub fn server(args: Vec<String>, event: Arc<EventEmitter>){
    server::server(args, event);
}
pub fn response(dst_ip: [u8; 4], data: Vec<u8>,chunk_size: usize){
   server::response(dst_ip, data, chunk_size);
}