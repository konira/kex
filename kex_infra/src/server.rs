extern crate pnet;
use kex_domain::Entitys::Envent::EventEmitter;
use kex_domain::Entitys::Payload::Payload;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::{ Packet};
use std::fs::write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::net::{UdpSocket};
use etherparse::{icmpv4, Icmpv4Type, PacketBuilder};


pub fn server(args: Vec<String>, event: Arc<EventEmitter>) {
    let sig = &args[2];
    let sig = sig.as_bytes();
    let _ = event;
    let interface_name = args[1].clone();
    
    let shared_data:Arc<Mutex<Vec<Payload>>> = Arc::new(Mutex::new(Vec::new()));
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
                            if payload_sig != sig || payload_raw.len() < 22{
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
                        }}
                    }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

pub fn response(dst_ip: [u8; 4], data: Vec<u8>,chunk_size: usize){    
    let chunks: Vec<_> = data.chunks(chunk_size).collect();
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    for chunk in chunks {
        let builder = PacketBuilder::ipv4(
            [127, 0, 0, 1], 
            dst_ip,
            60,
        )
        .icmpv4(Icmpv4Type::TimeExceeded(
            icmpv4::TimeExceededCode::TtlExceededInTransit,
        ));

        let mut result = Vec::<u8>::with_capacity(builder.size(chunk.len()));
        builder.write(&mut result, chunk).unwrap();
        socket.send_to(&result, format!("{}.{}.{}.{}:0", dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3])).expect("couldn't send data");
    }
}