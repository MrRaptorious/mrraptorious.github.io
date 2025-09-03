varying vec3 vNormal; // NEU: Normale im Weltraum
varying vec3 vWorldPosition; 

void main() {
    gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
    vNormal = normalize(normalMatrix * normal);
}