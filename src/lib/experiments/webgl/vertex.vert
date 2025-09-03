void main() {
    gl_Position = projectionMatrix * modelViewMatrix * vec4( cos(sin(position))+position, 1.0 );
}