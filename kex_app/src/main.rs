mod args;
use std::str::FromStr;
use clap::Parser;
use kex_bootstrap::{hex, Payload};
use tokio::io;

fn main() -> io::Result<()> {
    let args = args::Cli::parse();
    match args.client {
        args::Commands::Client(client) => {
            let payload = hex::decode(client.payload.unwrap()).unwrap();
            let addr: std::net::Ipv4Addr = std::net::Ipv4Addr::from_str(&client.addr).unwrap();
            let sig: Vec<u8> = client.sig.into();  
            let chunk_size = client.chunk_size.parse().unwrap();            
            //
            let part = 1;
            let total = 1;            
            let method = client.method.parse().unwrap();
            let tp = client.tp.parse().unwrap();
            let options = kex_bootstrap::dto::client_options::ClientOptions {
                chunk_size,
                destination: addr.octets(),
                payload: Payload::new(sig, method, part, total, tp, payload),                
            };
            kex_bootstrap::client(options)
        }

        args::Commands::Server(server) => {
            let options = kex_bootstrap::dto::server_options::ServerOptions {
                interface_name: server.interface_name,
                sig: server.sig.into_bytes(),
                tp: server.tp.parse().unwrap(),
            };
            kex_bootstrap::server(options)
        }
        args::Commands::Interfaces => {
            kex_bootstrap::list_ifaces();
        }
    }
    Ok(())
}
