extern crate pnet;
use kex_domain::Entitys::Envent::EventEmitter;
use kex_domain::Entitys::Payload::Payload;
use kex_domain::Enums::tp_enum::TpEnum;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::icmp::{self, IcmpCode, IcmpPacket};
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::transport::{self, TransportChannelType::Layer4, TransportProtocol::Ipv4};
use std::fs::write;
use std::net::Ipv4Addr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use crate::dto::ServerOptions::ServerOptions;

pub fn server(args: ServerOptions, event: Arc<EventEmitter>) {
    let sig = args.sig.clone();
    let tp = args.tp.clone();
    let _ = event;
    let interface_name = args.interface_name.clone();

    let shared_data: Arc<Mutex<Vec<Payload>>> = Arc::new(Mutex::new(Vec::new()));
    for iface in datalink::interfaces() {
        println!("{:?}", iface.name);
    }

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name.contains(&interface_name))
        .unwrap();

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                if let Some(ip_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    if ip_packet.get_next_level_protocol()
                        == pnet::packet::ip::IpNextHeaderProtocols::Icmp
                    {
                        if let Some(icmp_packet) = IcmpPacket::new(ip_packet.payload()) {
                            let payload_raw = icmp_packet.payload();
                            let payload_sig = &payload_raw[0..sig.len()];

                            if payload_sig != sig {
                                continue;
                            }
                            let payload = Payload::from_bytes(&payload_raw);
                            if payload.tp == TpEnum::Request {
                                continue;
                            }

                            let data_clone = Arc::clone(&shared_data);
                            let mut data = data_clone.lock().unwrap();

                            if data.len() == data[0].total as usize {
                                let mut payload = vec![];
                                for i in 0..data.len() {
                                    payload.extend_from_slice(&data[i].payload);
                                }
                                write(Path::new("output"), &payload).expect("Unable to write file");
                                event.emit(&data[0].method, payload, ip_packet.get_source());
                                data.clear();
                            }
                        }
                    }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
pub fn response(dst_ip: [u8; 4], payload: &Payload, chunk_size: usize) {
    let protocol = Layer4(transport::TransportProtocol::Ipv4(pnet::packet::ip::IpNextHeaderProtocols::Icmp));
    let destination_addr = Ipv4Addr::new(dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3]).into();
    let (mut tx, _) = transport::transport_channel(4096,protocol )
        .expect("Error creating transport channel");
    let chunks = payload.chunk(chunk_size);
    let mut sequenc = 1;
    for chunk in chunks {
        let chunk_bytes = chunk.to_bytes();
        let mut buffer = vec![0u8; chunk_bytes.len() + 32];
        
        let mut packet = pnet::packet::icmp::echo_request::MutableEchoRequestPacket::new(&mut buffer).unwrap();                      
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
        }else {
            sequenc += 1;
        }
        let ten_millis = std::time::Duration::from_millis(20);        
        std::thread::sleep(ten_millis);
    }
}

pub fn iface_list() {
    let ifaces = datalink::interfaces();
    for iface in ifaces {
        println!("{}", iface.to_string());
        println!("\tdescription: {}", iface.description);
    }
}

#[cfg(test)]
mod tests {
    use kex_domain::Enums::tp_enum;

    use super::*;
    #[test]
    fn test_iface_list() {
        iface_list();
    }
    #[test]
    fn test_response() {      
        let value = r#"        
Inspiração dos meus sonhos, não quero acordar
Quero ficar só contigo, não vou poder voar
Por que parar pra refletir se meu reflexo é você?
Aprendendo uma só vida, compartilhando prazer

Por que parece que na hora eu não vou aguentar
Se eu sempre tive força e nunca parei de lutar?
Como num filme, no final tudo vai dar certo
Quem foi que disse que pra tá junto precisa tá perto?

Pense em mim, que eu tô pensando em você
E me diz o que eu quero te dizer
Vem pra cá, pra ver que juntos estamos
E te falar mais uma vez que te amo

O tempo que passamos juntos vai ficar pra sempre
Intimidade, brincadeiras, só a gente entende
Pra quem fala que namorar é perder tempo eu digo
Há muito tempo não crescia o que eu cresci contigo

Juntos no balanço da rede, sob o céu estrelado
Sempre acontece, o tempo para quando eu tô do seu lado
A noite chega, eu fecho os olhos, é você que eu vejo
Como eu queria estar contigo, eu paro e faço um desejo

Pense em mim, que eu tô pensando em você
E me diz o que eu quero te dizer
Vem pra cá, pra ver que juntos estamos
E te falar mais uma vez que te amo
        "#.as_bytes().to_vec(); 
        let sig = "abcdefghijkmln".as_bytes().to_vec();   
        let payload = Payload::new(sig, 0, 1, 1, tp_enum::TpEnum::Response as u8, value);
        response([192, 168, 18, 6], &payload, 22);
    }
}
