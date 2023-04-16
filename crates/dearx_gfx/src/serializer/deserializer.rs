use crate::IApi;
use crate::Scene;

#[cfg(not(target_arch = "wasm32"))]
pub fn deserialize<T: IApi>(data: &[u8]) -> Scene<T> {
    let mut stream_reader = usd_rs::StreamReader::new(data);
    let _reader = usd_rs::AsciiReader::new(&mut stream_reader);
    todo!()
}
