import { base } from "$app/paths";
import thumb from './pix_dottify.png'
import Dottify from "./dottify.svelte";

let name = 'dottify'

export default {
    id: "DOTTIFY",
    image: thumb,
    title: name,
    subtitle: "Transform an image into an dottified version with WASM",
    tags: ["rust", "wasm", "Generative Art"].toSorted(),
    link: `${base}/experiments/${name}`,
    component:Dottify,
};
