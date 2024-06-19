#[derive(PartialEq, Debug,Clone, Copy)]
pub enum TpEnum {
    Request = 0,
    Response = 1,
}
impl From<u8> for TpEnum {
    fn from(item: u8) -> Self {
        match item {
            0 => TpEnum::Request,
            1 => TpEnum::Response,
            // adicione mais correspondências conforme necessário
            _ => panic!("Valor {} não suportado", item),
        }
    }
}
impl Into<u8> for TpEnum {
    fn into(self) -> u8 {
        match self {
            TpEnum::Request => 0,
            TpEnum::Response => 1,
            // adicione mais correspondências conforme necessário
        }
    }    
}