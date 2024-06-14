pub mod server;
pub mod client;
pub mod dto;
pub mod native_exec;
use std::sync::Arc;


use dto::{client_options, server_options::ServerOptions};
use kex_domain::entitys::envent::EventEmitter;
pub use hex;

pub fn server(args: ServerOptions, event: Arc<EventEmitter>){
    server::server(args, event);
}
pub fn client(options: client_options::ClientOptions){
    client::client(options)  
}