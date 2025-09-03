<script lang="ts">
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import * as THREE from "three";
    import fragmentShader from './fragment.frag?raw'
    import vertexShader from './vertex.vert?raw'

    onMount(async () => {
        if (browser) {

            const reader = new FileReader();

            /**
             * @type{HTMLCanvasElement}
             */
            const canvas = window.document.getElementById(
                "canvas",
            ) as HTMLCanvasElement;
            const scene = new THREE.Scene();
            const camera = new THREE.PerspectiveCamera(
                75,
                canvas.width / canvas.height,
                0.1,
                1000,
            );

            const renderer = new THREE.WebGLRenderer({ canvas: canvas });
            renderer.setSize(canvas.width, canvas.height);
            renderer.setAnimationLoop(animate);

            // ------- Shader ------
            const material = new THREE.ShaderMaterial({
                uniforms: {
                    diffuse: { value: new THREE.Color(0xffffff) },
                    resolution: { value: new THREE.Vector2(canvas.width,canvas.height) },
                },
                vertexShader: vertexShader,
                fragmentShader: fragmentShader,
            });


            // ------- Mesh ------
            material.uniforms.diffuse.value = new THREE.Color(0,1,0);
            const geometry = new THREE.TorusGeometry(3,1,100,100);
            const mesh = new THREE.Mesh(geometry,material);

            scene.add(mesh);

            camera.position.z = 10;

            function animate() {
                mesh.rotateX(0.005);
                mesh.rotateY(0.005);
                renderer.render(scene, camera);
            }

            animate();
        }
    });
</script>

<canvas id="canvas" width="1000" height="1000"></canvas>
