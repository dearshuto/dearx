pub trait IDevice {}

pub trait IBuffer {}

pub trait IDepthStencilView {}

pub trait IShader {}

pub trait ITexture {}

pub trait IVertexState {}

pub trait IApi {
    type Device: IDevice;
    type DepthStencilView: IDepthStencilView;
    type Buffer: IBuffer;
    type Shader: IShader;
    type Texture: ITexture;
    type VertexState: IVertexState;
}
