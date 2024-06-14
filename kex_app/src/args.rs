use clap::{ builder::Str, Args, Parser, Subcommand};


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
    Client(ArgsKexClient),
    /// monitor data from client
    Server(ArgsKexServer),
    /// list all interfaces
    Interfaces,
}

#[derive(Debug,Args)]
pub struct ArgsKexClient {

    #[arg(short, long, default_value = "abcdefghijklkj")]
    pub sig: String,
    #[arg(short, long)]
    pub payload: Option<String>,
    #[arg(short, long, default_value = "64")]
    pub chunk_size: String,
    #[arg(short, long)]
    pub addr: String,
    #[arg(short, long, default_value = "0")]
    pub tp: String,
    #[arg(short, long, default_value = "0")]
    pub method: String,
}

#[derive(Debug,Args)]
pub struct ArgsKexServer {

    #[arg(short, long, default_value = "abcdefghijklkj")]
    pub sig: String,
    #[arg(short, long)]
    pub interface_name: String,    
    #[arg(short, long, default_value = "0")]
    pub tp: String,
}