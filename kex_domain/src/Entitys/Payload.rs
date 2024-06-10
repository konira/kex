pub  struct Payload {  
    pub sig: Vec<u8>,
    pub method: u8,  
    pub part: u16,
    pub total: u16,
    pub tp: u8,
    pub payload: Vec<u8>,
}
impl Payload {
    pub fn default()->Payload {
        Payload {
            sig: vec![],
            method: 0,
            part: 0,
            total: 0,
            tp: 0,
            payload: vec![],
        }
    }
    pub fn new( sig: Vec<u8> ,method: u8, part: u16, total: u16, tp: u8, payload: Vec<u8>) -> Payload {
        Payload {
            sig,
            method,
            part,
            total,
            tp,
            payload,
        }
    }    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.sig);
        bytes.extend_from_slice(&self.method.to_be_bytes());
        bytes.extend_from_slice(&self.part.to_be_bytes());
        bytes.extend_from_slice(&self.total.to_be_bytes());
        bytes.push(self.tp);
        bytes.extend_from_slice(&self.payload);
        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> Payload {
        let sig = bytes[0..14].to_vec();
        let method = u8::from_be_bytes([bytes[15]]);
        let part = u16::from_be_bytes([bytes[16], bytes[17]]);
        let total = u16::from_be_bytes([bytes[18], bytes[19]]);
        let tp = bytes[20];
        let payload = bytes[21..].to_vec();
        Payload {
            sig,
            method,
            part,
            total,
            tp,
            payload,
        }
    }
}