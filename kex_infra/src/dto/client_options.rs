use kex_domain::entitys::payload::Payload;

pub struct ClientOptions{
    pub destination: [u8; 4],        
    pub chunk_size: usize,
    pub payload: Payload,    
}