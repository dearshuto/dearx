#version 450

layout(location = 0) in vec3 i_Position;

layout(binding = 0) uniform View
{
  mat4 u_ProjectionViewMatrix;
};

void main()
{
  vec4 position = u_ProjectionViewMatrix * vec4(i_Position, 1.0);
  gl_Position = position;
}
