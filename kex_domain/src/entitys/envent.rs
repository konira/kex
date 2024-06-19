use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Mutex;

pub struct EventEmitter {
    listeners: Mutex<HashMap<u8, Vec<Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>>>>,
    all_listeners: Mutex<Vec<Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
            all_listeners: Mutex::new(Vec::new()),
        }
    }
    pub fn on(&self, event: u8,callback: Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(event).or_insert_with(Vec::new).push(callback);
    }
    pub fn on_all(&self, callback: Box<dyn Fn(Vec<u8>,Ipv4Addr) + Send + 'static>) {
        let mut all_listeners = self.all_listeners.lock().unwrap();
        all_listeners.push(callback);
    }
    pub fn emit(&self, event: &u8, data: Vec<u8>,ip: Ipv4Addr) {
        let listeners = self.listeners.lock().unwrap();
        if let Some(callbacks) = listeners.get(event) {
            for callback in callbacks {
                callback(data.clone(),ip);
            }
        }

        let all_listeners = self.all_listeners.lock().unwrap();
        for callback in all_listeners.iter() {
            callback(data.clone(), ip);
        }
    }
    pub fn remove_listener(&self, event: &u8) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(event);
    }
    pub fn remove_all_listeners(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();

        let mut all_listeners = self.all_listeners.lock().unwrap();
        all_listeners.clear();
    }
}