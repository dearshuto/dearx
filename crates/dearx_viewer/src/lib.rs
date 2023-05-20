pub mod http;
mod i_server_logic;
pub mod proto;
pub mod wgpu;

pub use i_server_logic::IServerLogic;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait IEditBuffer {
    fn write(&mut self, data: &CameraData);
}

pub struct CameraData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
