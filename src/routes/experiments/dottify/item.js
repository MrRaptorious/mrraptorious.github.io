import { base } from "$app/paths";
import thumb from './thumb.png'

let name = 'dottify'

export default {
    image: thumb,
    title: name,
    subtitle: "Transform an image into an dottified version with WASM",
    tags: ["rust", "wasm", "Generative Art"].toSorted(),
    link: `${base}/experiments/${name}`,
};
