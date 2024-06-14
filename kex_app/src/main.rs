mod args;
use std::{env, str::FromStr};

use clap::Parser;
use kex_bootstrap::{init, kex_domain::{Entitys::Payload::Payload, Enums::tp_enum::TpEnum}, list_ifaces};
use tokio::io;

fn main() -> io::Result<()>{
    let args = args::Cli::parse();
    match args.client {
        args::Commands::Client(client) => {            
            let payload = kex_bootstrap::kex_infra::hex::decode(client.payload.unwrap()).unwrap();
            let addr:std::net::Ipv4Addr = std::net::Ipv4Addr::from_str(&client.addr).unwrap();
            let sig: Vec<u8> = client.sig.into();
            let method = client.method;
            let chunk_size = client.chunk_size as usize;
            //
            let part = 1;
            let total = 1;
            let tp = TpEnum::Request as u8;
            //
            let payload: Payload = Payload::new(sig, method, part, total, tp, payload);            
            let options = kex_bootstrap::dto::ClientOptions::ClientOptions{                
                chunk_size: chunk_size,                            
                destination: addr.octets(),
                payload,
            };
            kex_bootstrap::request(options)
        }
        
        args::Commands::Server(server) => {            
            let options = kex_bootstrap::dto::ServerOptions::ServerOptions{
                interface_name: server.interface_name,
                sig: server.sig.into_bytes(),
                tp: TpEnum::Request as u8,                
            };
            kex_bootstrap::init(options)
        }
        args::Commands::Interfaces => {
            kex_bootstrap::list_ifaces();
        }
    }
    Ok(())
}