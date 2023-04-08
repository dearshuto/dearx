use dearx_macro::Immutable;
use std::sync::Arc;

#[derive(Immutable)]
pub struct ModelContent {
    pub name: String,
}

#[derive(Default)]
pub struct Model<V, I>
where
    V: num_traits::Float,
    I: num_traits::Unsigned,
{
    pub vertices: Vec<V>,
    pub indices: Vec<I>,
}

impl<V, I> Model<V, I>
where
    V: num_traits::Float,
    I: num_traits::Unsigned,
{
    #[allow(dead_code)]
    pub fn get_vertices(&self) -> &[V] {
        &self.vertices
    }

    #[allow(dead_code)]
    pub fn get_indices(&self) -> &[I] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn float32_uint32() {
        let _ = Model::<f32, u32>::default();
    }

    #[test]
    fn float32_uint16() {
        let _ = Model::<f32, u16>::default();
    }

    #[test]
    fn float32_uint8() {
        let _ = Model::<f32, u8>::default();
    }
}
