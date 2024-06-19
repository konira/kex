use std::sync::MutexGuard;

use crate::enums::tp_enum::TpEnum;

#[derive(Debug, PartialEq)]
pub struct Payload {
    pub sig: Vec<u8>,
    pub method: u8,
    pub part: u16,
    pub total: u16,
    pub tp: TpEnum,
    pub len: u8,
    pub payload: Vec<u8>,
}
impl Payload {
    pub fn default() -> Payload {
        Payload {
            sig: vec![],
            method: 0,
            part: 0,
            total: 0,
            tp: TpEnum::Request,
            len: 0,
            payload: vec![],
        }
    }
    pub fn new(
        sig: Vec<u8>,
        method: u8,
        part: u16,
        total: u16,
        tp: u8,
        payload: Vec<u8>,
    ) -> Payload {
        let len = payload.len() as u8;
        let sig_normalized = Self::validate_and_adjust_sig(&mut sig.clone());
        Payload {
            sig: sig_normalized,
            method,
            part,
            total,
            tp: TpEnum::from(tp),
            len,
            payload,
        }
    }
    pub fn validate_and_adjust_sig(sig: &mut Vec<u8>) -> Vec<u8> {
        let len = sig.len();
        if len < 14 {
            // Se sig for menor que 14, preencha com números crescentes a partir de 1
            for i in len..14 {
                sig.push(i as u8 + 1);
            }
        } else if len > 14 {
            // Se sig for maior que 14, trunque para os primeiros 14 elementos
            sig.truncate(14);
        }
        sig.to_vec()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.sig);
        bytes.extend_from_slice(&self.method.to_be_bytes());
        bytes.extend_from_slice(&self.part.to_be_bytes());
        bytes.extend_from_slice(&self.total.to_be_bytes());
        bytes.push(self.tp.into());
        bytes.push(self.len);
        bytes.extend_from_slice(&self.payload);
        bytes
    }
    pub fn from_bytes(bytes: &[u8]) -> Payload {
        let sig = bytes[0..14].to_vec();
        let method = u8::from_be_bytes([bytes[14]]);
        let part = u16::from_be_bytes([bytes[15], bytes[16]]);
        let total = u16::from_be_bytes([bytes[17], bytes[18]]);
        let tp = bytes[19];
        let len = bytes[20];
        let payload = bytes[21..21 + len as usize].to_vec();
        Payload {
            sig,
            method,
            part,
            total,
            tp: TpEnum::from(tp),
            len,
            payload,
        }
    }
    pub fn is_valid(&self) -> bool {
        if self.is_valid_part()
            && self.is_valid_total()
            && self.is_valid_sig()
            && self.is_valid_tp()
            && self.is_valid_method()
        {
            return true;
        }
        return false;
    }
    pub fn is_single(&self) -> bool {
        self.part == 1 && self.total == 1
    }
    pub fn is_complete(payloads: &mut MutexGuard<Vec<Payload>>) -> bool {
        let total = payloads[0].total;
        payloads.sort_by(|a, b| a.part.cmp(&b.part));
        payloads.len() == (total as usize)
    }
    pub fn is_multi(&self) -> bool {
        self.total > 1
    }
    pub fn is_valid_method(&self) -> bool {
        self.method >= u8::MIN && self.method <= u8::MAX
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
            let payload = Payload::new(
                self.sig.clone(),
                self.method,
                part,
                total,
                self.tp.into(),
                chunk,
            );
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
    fn test_default() {
        let payload = Payload::default();
        assert_eq!(payload.sig, vec![]);
        assert_eq!(payload.method, 0);
        assert_eq!(payload.part, 0);
        assert_eq!(payload.total, 0);
        assert_eq!(payload.tp, TpEnum::Request);
        assert_eq!(payload.len, 0);
        assert_eq!(payload.payload, vec![]);
    }

    #[test]
    fn test_new() {
        let sig = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, b'a', b'b', b'c', b'd', b'e'];
        let _payload = vec![5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd];
        let payload = Payload::new(sig.clone(), 1, 1, 1, 1, _payload.clone());

        assert_eq!(payload.sig, sig);
        assert_eq!(payload.method, 1);
        assert_eq!(payload.part, 1);
        assert_eq!(payload.total, 1);
        assert_eq!(payload.tp, TpEnum::from(1));
        assert_eq!(payload.len, 9);
        assert_eq!(payload.payload, _payload);
    }

    #[test]
    fn test_to_bytes_and_from_bytes() {
        let payload = Payload::new(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, b'a', b'b', b'c', b'd', b'e'],
            1,
            1,
            1,
            1,
            vec![5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd],
        );
        let bytes = payload.to_bytes();
        let payload_from_bytes = Payload::from_bytes(&bytes);
        assert_eq!(payload, payload_from_bytes);
    }

    #[test]
    fn test_is_valid() {
        let payload = Payload::new(vec![1; 14], 1, 2, 3, TpEnum::Response as u8, vec![5, 6, 7]);
        assert!(payload.is_valid());
    }

    #[test]
    fn test_is_single() {
        let payload = Payload::new(vec![1; 14], 1, 1, 1, TpEnum::Response as u8, vec![5, 6, 7]);
        assert!(payload.is_single());
    }

    // ... continue with other tests
}
