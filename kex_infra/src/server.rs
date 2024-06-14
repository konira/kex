extern crate pnet;
use kex_domain::Entitys::Envent::EventEmitter;
use kex_domain::Entitys::Payload::Payload;
use kex_domain::Enums::tp_enum::TpEnum;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::sync::{Arc, Mutex};

use crate::dto::server_options::ServerOptions;


pub fn server(args: ServerOptions, event: Arc<EventEmitter>) {
    let sig = args.sig.clone();
    let sig_len = sig.len();
    let tp = args.tp.clone();
    let _ = event;
    let interface_name = args.interface_name.clone();

    let shared_data: Arc<Mutex<Vec<Payload>>> = Arc::new(Mutex::new(Vec::new()));   
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
                            
                            let contains_sequence = payload_raw.windows(sig_len).any(|window| window == sig);

                            if !contains_sequence {
                                continue;
                            }                  
                            let payload = Payload::from_bytes(&payload_raw[4..]);
                            if payload.tp != TpEnum::from(tp) {
                                continue;
                            }
                            if !payload.is_valid(){
                                continue;
                            }    
                            if payload.is_single() {
                                event.emit(&payload.method, payload.payload, ip_packet.get_source());
                                continue;
                            } else {
                                let data_clone = Arc::clone(&shared_data);
                                let mut data = data_clone.lock().unwrap();
                                data.push(payload);
                                
                                if kex_domain::Entitys::Payload::Payload::is_complete(&mut data) {
                                    let mut payload = vec![];
                                    for i in 0..data.len() {
                                        payload.extend_from_slice(&data[i].payload);
                                    }
                                    event.emit(&data[0].method, payload, ip_packet.get_source());
                                    data.clear();
                                }
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

pub fn iface_list() {
    let ifaces = datalink::interfaces();
    for iface in ifaces {
        println!("{}", iface.to_string());
        println!("\tdescription: {}", iface.description);
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;
    #[test]
    fn test_iface_list() {
        iface_list();
    }
}
