attribute vec3 a_position;
varying vec4 VertexColor;

void main() {
  gl_Position = vec4(a_position.x, a_position.y, a_position.z, 1.0);
  VertexColor = vec4(0.5, 0.0, 0.0, 1.0);
}
