import { base } from "$app/paths";
import thumb from './thumb.png'

let name = 'dottify'

export default {
    image: thumb,
    title: name,
    subtitle: "Wasm experiment to modifying an image",
    tags: ["rust", "wasm", "Generative Art"].toSorted(),
    link: `${base}/experiments/${name}`,
};
