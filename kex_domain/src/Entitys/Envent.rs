use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Mutex;







pub struct EventEmitter {
    listeners: Mutex<HashMap<u8, Vec<Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
        }
    }
    pub fn on(&self, event: u8,callback: Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(event).or_insert_with(Vec::new).push(callback);
    }
    pub fn emit(&self, event: &u8, data: Vec<u8>,ip: Ipv4Addr) {
        let listeners = self.listeners.lock().unwrap();
        if let Some(callbacks) = listeners.get(event) {
            for callback in callbacks {
                callback(data.clone(),ip);
            }
        }
    }
    pub fn remove_listener(&self, event: &u8) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(event);
    }
    pub fn remove_all_listeners(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();
    }
}