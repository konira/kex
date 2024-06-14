use crate::Enums::tp_enum::TpEnum;

pub  struct Payload {  
    pub sig: Vec<u8>,
    pub method: u8,  
    pub part: u16,
    pub total: u16,
    pub tp: TpEnum,
    pub payload: Vec<u8>,
}
impl Payload {
    pub fn default()->Payload {
        Payload {
            sig: vec![],
            method: 0,
            part: 0,
            total: 0,
            tp: TpEnum::Request,
            payload: vec![],
        }
    }
    pub fn new( sig: Vec<u8> ,method: u8, part: u16, total: u16, tp: u8, payload: Vec<u8>) -> Payload {
        Payload {
            sig,
            method,
            part,
            total,
            tp: TpEnum::from(tp),
            payload,
        }
    }    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.sig);
        bytes.extend_from_slice(&self.method.to_be_bytes());
        bytes.extend_from_slice(&self.part.to_be_bytes());
        bytes.extend_from_slice(&self.total.to_be_bytes());
        bytes.push(self.tp.into());
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
            tp: TpEnum::from(tp),
            payload,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.sig.len() == 14 && self.payload.len() > 0
    }
    pub fn is_last(&self) -> bool {
        self.part == self.total
    }
    pub fn is_first(&self) -> bool {
        self.part == 1
    }
    pub fn is_middle(&self) -> bool {
        self.part > 1 && self.part < self.total
    }
    pub fn is_single(&self) -> bool {
        self.part == 1 && self.total == 1
    }
    pub fn is_multi(&self) -> bool {
        self.total > 1
    }
    pub fn is_complete(&self) -> bool {
        self.total as usize == self.payload.len()
    }
    pub fn is_valid_method(&self) -> bool {
        self.method == 0 || self.method == 1
    }
    pub fn is_valid_tp(&self) -> bool {
        self.tp == TpEnum::Request || self.tp == TpEnum::Response
    }
    pub fn is_valid_part(&self) -> bool {
        self.part > 0
    }
    pub fn is_valid_total(&self) -> bool {
        self.total > 0
    }
    pub fn is_valid_sig(&self) -> bool {
        self.sig.len() == 14
    }
   pub fn chunk(&self, size: usize) -> Vec<Payload> {
        let mut chunks = vec![];
        let mut payload = self.payload.clone();
        let total = (payload.len() as f64 / size as f64).ceil() as usize;
        for i in 1..=total {
            let part = i as u16;
            let total = total as u16;
            let mut chunk = vec![];
            if payload.len() > size {
                chunk.extend_from_slice(&payload[0..size]);
                payload = payload[size..].to_vec();
            } else {
                chunk.extend_from_slice(&payload);
            }
            let payload = Payload::new(self.sig.clone(), self.method, part, total, self.tp.into(), chunk);
            chunks.push(payload);
        }
        chunks
    }
   pub fn merge(chunks: Vec<Payload>) -> Payload {
        let mut payload = vec![];
        for chunk in &chunks {
            payload.extend_from_slice(&chunk.payload);
        }
        let sig = chunks[0].sig.clone();
        let method = chunks[0].method;
        let part = 1;
        let total = 1;
        let tp = chunks[0].tp;
        Payload::new(sig, method, part, total, tp.into(), payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_payload() {
        let sig = vec![0; 14];
        let method = 0;
        let part = 1;
        let total = 1;
        let tp = 0;        
        let payload = vec![0; 100];

        let payload = Payload::new(sig, method, part, total, tp, payload);
        
        let bytes = payload.to_bytes();
        let payload = Payload::from_bytes(&bytes);
        assert_eq!(payload.sig.len(), 14);
        assert_eq!(payload.method, 0);
        assert_eq!(payload.part, 1);
        assert_eq!(payload.total, 1);
        assert_eq!(payload.tp, 0.into());
        assert_eq!(payload.payload.len(), 100);
        assert_eq!(payload.is_valid(), true);
        assert_eq!(payload.is_last(), true);
        assert_eq!(payload.is_first(), true);
        assert_eq!(payload.is_middle(), false);
        assert_eq!(payload.is_single(), true);
        assert_eq!(payload.is_multi(), false);
        assert_eq!(payload.is_complete(), true);
        assert_eq!(payload.is_valid_method(), true);
        assert_eq!(payload.is_valid_tp(), true);
        assert_eq!(payload.is_valid_part(), true);
        assert_eq!(payload.is_valid_total(), true);
        assert_eq!(payload.is_valid_sig(), true);
        let chunks = payload.chunk(50);
        assert_eq!(chunks.len(), 1);
        let payload = Payload::merge(chunks);
        assert_eq!(payload.payload.len(), 100);
    }
}