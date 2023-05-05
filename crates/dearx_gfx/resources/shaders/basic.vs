#version 450

layout(location = 0) out vec3 v_WorldPosition;
layout(location = 1) out vec3 v_Normal;

layout(location = 0) in vec3 i_Position;
layout(location = 1) in vec3 i_Normal;

layout(binding = 0) uniform ModelData {
  vec4 u_ModelMatrix[4];
};

layout(binding = 1) uniform ViewData {
  vec4 u_ProjectionViewMatrix[4];
};

void main() {
  vec4 worldPosition = vec4(
    dot(u_ModelMatrix[0], vec4(i_Position, 1.0)),
    dot(u_ModelMatrix[1], vec4(i_Position, 1.0)),
    dot(u_ModelMatrix[2], vec4(i_Position, 1.0)),
    dot(u_ModelMatrix[3], vec4(i_Position, 1.0)));
  vec4 position = vec4(
    dot(u_ProjectionViewMatrix[0], worldPosition),
    dot(u_ProjectionViewMatrix[1], worldPosition),
    dot(u_ProjectionViewMatrix[2], worldPosition),
    dot(u_ProjectionViewMatrix[3], worldPosition));
  gl_Position = position;

  v_WorldPosition = worldPosition.xyz;
  v_Normal = i_Normal;
}
