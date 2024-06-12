use etherparse::err::packet;
use etherparse::{icmpv4, Icmpv4Type, PacketBuilder};
use kex_domain::Entitys::Envent::EventEmitter;
use kex_domain::Entitys::Payload::Payload;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::fs::write;
use std::net::UdpSocket;
use std::path::Path;
use std::process::Termination;
use std::sync::{Arc, Mutex};

use crate::dto::ClientOptions::ClientOptions;

pub fn request(options: ClientOptions) {
    let dst_ip = options.destination.clone();
    let chunk_size = options.chunk_size.clone();

    let chunks = options.payload.chunk(chunk_size);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let mut seq = 0;    
    for chunk in chunks {
        let mut packet = Vec::<u8>::with_capacity(chunk.to_bytes().len()+28);
        PacketBuilder::ipv4([0, 0, 0, 0], dst_ip, 60).icmpv4(Icmpv4Type::EchoReply(
            etherparse::IcmpEchoHeader {
                id: 1,
                seq: seq,
            },
        )).write(&mut packet, &chunk.to_bytes()).unwrap();
        
        socket
            .send_to(
                &packet,
                format!("{}.{}.{}.{}:0", dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3]),
            )
            .expect("couldn't send data");
        seq += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request() {
        let msg = "Hello, world!".as_bytes().to_vec();
        let sig: Vec<u8> = "abcdskluipuyyt".as_bytes().to_vec();
        let options = ClientOptions {
            destination: [192, 168, 18, 6],
            chunk_size: 60,
            payload: Payload::new(sig, 0, 1, 1, 0, msg),
        };
        request(options);
    }
}
