mod manipulator;
pub mod model;

pub use manipulator::{CameraData, Manipulator};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
