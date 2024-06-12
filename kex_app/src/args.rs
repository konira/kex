use clap::{Args, Parser, Subcommand};


#[derive(Debug,Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub client: Commands,
}

#[derive(Debug,Subcommand)]
pub enum Commands {
    /// send data to server
    Client(ArgsKexRequest),
    /// monitor data from client
    Server(ArgsKexServer),
    /// list all interfaces
    Interfaces,
}

#[derive(Debug,Args)]
pub struct ArgsKexRequest {

    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long)]
    pub method: u8,
    #[arg(short, long)]
    pub payload: Option<String>,
    #[arg(short, long)]
    pub chunk_size: u8,
    #[arg(short, long)]
    pub addr: String,
    #[arg(short, long)]
    pub interface_name: Option<String>,

}

#[derive(Debug,Args)]
pub struct ArgsKexServer {

    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long)]
    pub interface_name: String,
}
fn parse_hex(hex_str: &str) -> Result<Vec<u8>, kex_bootstrap::kex_infra::hex::FromHexError> {
    kex_bootstrap::kex_infra::hex::decode(hex_str)
}