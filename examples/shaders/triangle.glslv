#version 330 core

in vec3 a_Pos;

void main() {
  gl_Position.xyz = a_Pos;
  gl_Position.w = 1.0;
}
