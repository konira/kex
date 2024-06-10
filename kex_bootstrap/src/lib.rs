use std::{process::Command, sync::Arc};

use kex_domain::Entitys::Envent::EventEmitter;
use kex_infra::{
    response,
    server
};

pub enum Method {
    Echo = 0,
    Ls = 1,
}

pub fn echo(data: Vec<u8>, addr: std::net::Ipv4Addr) {
                let output = Command::new("ls")
                    .output()
                    .expect("Falha ao executar o comando");

                if output.status.success() {
                    let files = String::from_utf8(output.stdout).unwrap();
                    response(addr.octets(), files.clone().into_bytes(), 22);
                    println!("{}", &files);
                } else {
                    let err = String::from_utf8(output.stderr).unwrap();
                    response(addr.octets(), err.clone().into_bytes(), 22);
                    println!("Erro: {}", err);
                }
}

pub fn Ls(data: Vec<u8>, addr: std::net::Ipv4Addr) {    
    println!("Data: {:?}", data);
    println!("Address: {:?}", addr);
}

pub fn init(args: Vec<String>) {  
    if cfg!(debug_assertions) {
        let evet_emitter = Arc::new(EventEmitter::new());
        evet_emitter.on(Method::Echo as u8, Box::new(echo));
        evet_emitter.on(Method::Ls as u8, Box::new(Ls));
        server(args.clone(), evet_emitter);
    }
}
