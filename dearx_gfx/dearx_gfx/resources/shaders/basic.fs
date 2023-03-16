#version 450

layout(location = 0) out vec4 o_Color;

layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_Normal;

layout(binding = 2) uniform Light {
  vec4 u_LightDirection;
  vec4 u_Color;
};

void main() {
  float diffuse = max(0.0, dot(u_LightDirection.xyz, v_Normal));
  // o_Color = vec4(vec3(diffuse) + vec3(0.1), 1.0);
  o_Color = vec4(v_Normal + vec3(0.1), 1.0);
}
