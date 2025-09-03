uniform vec3 diffuse;
uniform vec2 resolution;

void main() {


    vec2 coord = gl_FragCoord.xy / resolution;

    vec3 color = vec3(1.0);
    //color = color * diffuse;

    vec3 tl = vec3(1,0,0);
    vec3 tr = vec3(0,1,0);
    vec3 bl = vec3(0,0,1);
    vec3 br = vec3(1,0,1);

    vec3 top =    mix(tl,tr,coord.x);
    vec3 bottom = mix(bl,br,coord.x);

    color = mix(top,bottom, coord.y);

    //color = fract(color*5.0);

    if((mod(gl_FragCoord.x,2.0)==0.0)) {
        color = vec3(0,0,0);
    }

    gl_FragColor = vec4( color, 1.0 );
}