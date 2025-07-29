import"../chunks/CWj6FrbW.js";import"../chunks/Be5dWuTk.js";import{o as l}from"../chunks/ClWXu3MR.js";import{p as f,d as p,a as u,e as h}from"../chunks/CKsiWAQM.js";import{i as w}from"../chunks/DRl7fnVc.js";import{S as g,P as x,W as b,a as _,V as y,C as c,T as C,M}from"../chunks/CubGkJgd.js";const S=`uniform vec3 diffuse;
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
}`,F=`void main() {
    gl_Position = projectionMatrix * modelViewMatrix * vec4( cos(sin(position))+position, 1.0 );
}`;var P=p('<canvas id="canvas" width="1000" height="1000"></canvas>');function W(s,m){f(m,!1),l(async()=>{{let i=function(){n.rotateX(.005),n.rotateY(.005),e.render(r,t)};new FileReader;const o=window.document.getElementById("canvas"),r=new g,t=new x(75,o.width/o.height,.1,1e3),e=new b({canvas:o});e.setSize(o.width,o.height),e.setAnimationLoop(i);const a=new _({uniforms:{diffuse:{value:new c(16777215)},resolution:{value:new y(o.width,o.height)}},vertexShader:F,fragmentShader:S});a.uniforms.diffuse.value=new c(0,1,0);const v=new C(3,1,100,100),n=new M(v,a);r.add(n),t.position.z=10,i()}}),w();var d=P();u(s,d),h()}export{W as component};
