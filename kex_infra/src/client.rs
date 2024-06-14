extern crate pnet;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::icmp::{self, IcmpCode, IcmpPacket};
use pnet::packet::Packet;
use pnet::transport::{self, TransportChannelType::Layer4};
use std::net::Ipv4Addr;

use crate::dto::client_options::ClientOptions;

pub fn client(options: ClientOptions) {
    let dst_ip = options.destination;
    let chunk_size = options.chunk_size;
    let payload = options.payload;

    let protocol = Layer4(transport::TransportProtocol::Ipv4(
        pnet::packet::ip::IpNextHeaderProtocols::Icmp,
    ));
    let destination_addr = Ipv4Addr::new(dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3]).into();
    if let Ok((mut tx, _)) = transport::transport_channel(4096, protocol) {
        let chunks = payload.chunk(chunk_size);
        let mut sequenc = 1;
        for chunk in chunks {
            let chunk_bytes = chunk.to_bytes();
            let mut buffer = vec![0u8; chunk_bytes.len() + 32];

            let mut packet =
                pnet::packet::icmp::echo_request::MutableEchoRequestPacket::new(&mut buffer)
                    .unwrap();
            packet.set_icmp_type(IcmpTypes::EchoRequest);
            packet.set_icmp_code(IcmpCode::new(0));
            packet.set_sequence_number(sequenc);
            packet.set_identifier(1);
            let icmp_packet: IcmpPacket = IcmpPacket::new(packet.packet()).unwrap();
            let checksum = icmp::checksum(&icmp_packet);
            packet.set_checksum(checksum);
            packet.set_payload(&chunk_bytes);
            let r = tx.send_to(packet, destination_addr);
            if r.is_err() {
                println!("Error on send_to: {:?}", r.err());
            } else {
                sequenc += 1;
            }
            let ten_millis = std::time::Duration::from_millis(20);
            std::thread::sleep(ten_millis);
        }
    }
}

#[cfg(test)]
mod tests {
    use kex_domain::Enums::tp_enum::TpEnum;

    use super::*;
    #[test]
    fn test_request() {
        let msg = "Hello, world!".as_bytes().to_vec();
        let sig: Vec<u8> = "abcdskluipuyyt".as_bytes().to_vec();
        let options = ClientOptions {
            destination: [192, 168, 18, 6],
            chunk_size: 60,            
            payload: kex_domain::Entitys::Payload::Payload::new(sig, 0, 1, 1, TpEnum::Request as u8, msg),
        };
        client(options);
    }
}
