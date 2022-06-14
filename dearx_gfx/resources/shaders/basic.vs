#version 450

layout(location = 0) out vec3 v_WorldPosition;
layout(location = 1) out vec3 v_Normal;

layout(location = 0) in vec3 i_Position;
layout(location = 1) in vec3 i_Normal;

layout(binding = 0) uniform View {
  mat4 u_ModelMatrix;
  mat4 u_ProjectionViewMatrix;
};

void main() {
  vec4 worldPosition = u_ModelMatrix * vec4(i_Position, 1.0);
  vec4 position = u_ProjectionViewMatrix * worldPosition;

  v_WorldPosition = worldPosition.xyz;
  v_Normal = i_Normal;
}
