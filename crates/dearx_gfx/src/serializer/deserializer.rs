use crate::renderer::SceneObject;

pub trait IFactory {
    type TBuffer;

    fn create_buffer(&self, descriptor: &CreateBufferDescriptor) -> Self::TBuffer;
}

pub struct CreateBufferDescriptor<'a> {
    pub data: &'a [u8],
}

#[cfg(not(target_arch = "wasm32"))]
pub fn deserialize<TFactory: IFactory>(
    _data: &[u8],
    factory: &mut TFactory,
) -> SceneObject<TFactory::TBuffer> {
    // let mut stream_reader = usd_rs::StreamReader::new(data);
    // let _reader = usd_rs::AsciiReader::new(&mut stream_reader);

    let vertex_buffer_data0 =
        bytemuck::cast_slice(&[-0.40f32, -0.25, 0.0, 0.10, -0.25, 0.0, -0.15, 0.25, 0.0]);
    let vertex_buffer_data1 =
        bytemuck::cast_slice(&[-0.10f32, 0.25, 0.0, 0.40, 0.25, 0.0, 0.15, -0.25, 0.0]);

    let vertex_buffer0 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data0,
    });
    let vertex_buffer1 = factory.create_buffer(&CreateBufferDescriptor {
        data: vertex_buffer_data1,
    });

    SceneObject {
        vertex_buffers: vec![vertex_buffer0, vertex_buffer1],
    }
}
