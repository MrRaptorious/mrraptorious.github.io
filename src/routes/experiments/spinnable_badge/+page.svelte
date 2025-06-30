<script lang="ts">
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import * as THREE from "three";
    import fragmentShader from "./fragment.frag?raw";
    import vertexShader from "./vertex.vert?raw";
    import item from "./item.js";
    import { OBJLoader } from "three/examples/jsm/Addons.js";
    import cardObjUrl from "./card.obj?url";

    onMount(async () => {
        if (browser) {
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

            // --- OPTIONAL: Visueller Helper für das Punktlicht ---
            const lightSphereGeometry = new THREE.SphereGeometry(0.1, 16, 16);
            const lightSphereMaterial = new THREE.MeshBasicMaterial({ color: 0xffff00 }); // Gelbe Kugel
            const lightSphereMesh = new THREE.Mesh(lightSphereGeometry, lightSphereMaterial);
            lightSphereMesh.position.set(2,0,2,5); // Startposition des Lichts
            scene.add(lightSphereMesh);



            // ------- Shader ------
            // Fügen Sie die neuen Uniforms hinzu
            const material = new THREE.ShaderMaterial({
                uniforms: {
                    diffuse: { value: new THREE.Color(1, 1, 1) }, // Basisfarbe des Objekts
                    lightPosition: { value: lightSphereMesh.position }, // NEU: Position des Lichts
                    lightColor: { value: new THREE.Color(1.0, 1.0, 1.0) }, // Farbe des Lichts (weiß)
                    ambientColor: { value: (new THREE.Color(0.5,0.5,0.5))}, // Umgebungslichtfarbe
                    ambientIntensity: { value: 0.1 }, // Umgebungslichtintensität
                    lightRadius: { value: 5.0 }, // NEU: Radius für den Falloff. Spiel mit diesem Wert!
                },
                vertexShader: vertexShader,
                fragmentShader: fragmentShader,
            });

            // ------- Mesh ------
            material.uniforms.diffuse.value = new THREE.Color(0, 1, 0);

            let meshContainer: THREE.Group; // Dies ist unser Container-Objekt für Rotation

            // Load the OBJ model
            const loader = new OBJLoader();
            loader.load(
                cardObjUrl,
                function (object) {
                    meshContainer = new THREE.Group();

                    // --- NEU: Zentrieren und Anwenden des Materials auf jedes Mesh ---
                    object.traverse(function (child) {
                        if ((child as THREE.Mesh).isMesh) {
                            const mesh = child as THREE.Mesh;
                            mesh.material = material; // Material anwenden

                            // Zentriere die GEOMETRIE des Meshes
                            // Dadurch verschiebt sich der Inhalt des Meshes,
                            // aber der lokale Ursprung des Meshes bleibt an seiner Position.
                            mesh.geometry.computeBoundingBox();
                            const geomCenter =
                                mesh.geometry.boundingBox.getCenter(
                                    new THREE.Vector3(),
                                );
                            mesh.geometry.translate(
                                -geomCenter.x,
                                -geomCenter.y,
                                -geomCenter.z,
                            );

                            // Optional: Falls das Mesh selbst eine Verschiebung innerhalb der Gruppe hat,
                            // diese Verschiebung beibehalten oder auf (0,0,0) setzen, wenn gewünscht.
                            // mesh.position.set(0, 0, 0); // Dies würde die Position des Meshes in seiner Gruppe auf 0 setzen
                        }
                    });
                    // Füge das zentrierte Objekt dem Container hinzu

                    meshContainer.add(object);

                    // --- VISUELLER DEBUGGING-HELFER ---
                    const axesHelper = new THREE.AxesHelper(5); // 5 ist die Länge der Achsen
                    meshContainer.add(axesHelper);
                    // ------------------------------------

                    scene.add(meshContainer);

                    // --- Kameraanpassung ---
                    const newBox = new THREE.Box3().setFromObject(
                        meshContainer,
                        true,
                    );
                    const newSize = newBox.getSize(new THREE.Vector3());
                    const distance = newSize.length() * 0.75;
                    camera.position.z = distance;
                    camera.lookAt(0, 0, 0);
                },
                function (xhr) {
                    console.log((xhr.loaded / xhr.total) * 100 + "% loaded");
                },
                function (error) {
                    console.error(
                        "An error happened while loading the OBJ:",
                        error,
                    );
                },
            );

            function animate() {
                if (meshContainer) {
                    meshContainer.rotateY(0.005);
                }
                renderer.render(scene, camera);
            }

            animate();
        }
    });
</script>

<h1 class="text-4xl">{item.title}</h1>
<canvas id="canvas" width="1000" height="1000"></canvas>
