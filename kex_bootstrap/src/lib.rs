use std::{process::Command, sync::Arc};
use kex_domain::Entitys::{Envent::EventEmitter, Payload};
use kex_infra::{dto::ServerOptions::ServerOptions, server::{response, server}};

pub use kex_infra::dto;
pub use kex_domain;
pub use kex_infra;
pub enum Method {
    Echo = 0,
    Ls = 1,
}

pub fn echo(data: Vec<u8>, addr: std::net::Ipv4Addr) {
              
}

pub fn Ls(data: Vec<u8>, addr: std::net::Ipv4Addr) {    
    println!("Data: {:?}", data);
    println!("Address: {:?}", addr);
}
pub fn meterpreter_criptografado(data: Vec<u8>, addr: std::net::Ipv4Addr) {    
   
}
pub fn init(args: ServerOptions) {  
    if cfg!(debug_assertions) {
            
        let evet_emitter = Arc::new(EventEmitter::new());
        evet_emitter.on(Method::Echo as u8, Box::new(echo));
        evet_emitter.on(Method::Ls as u8, Box::new(Ls));
        evet_emitter.on(Method::Ls as u8, Box::new(meterpreter_criptografado));
        server(args, evet_emitter);
    }
}

pub fn list_ifaces() {
    kex_infra::server::iface_list()
}

pub fn request(options: dto::ClientOptions::ClientOptions) {        
    kex_infra::client::request(options)
}