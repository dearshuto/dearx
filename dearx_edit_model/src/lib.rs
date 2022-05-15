use std::sync::Arc;
use sje_immutable_macro::Immutable;

pub mod components;
mod float3;
pub use float3::Float3;

#[derive(Immutable)]
pub struct TestData
{
    pub value: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
