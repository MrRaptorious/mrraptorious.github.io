import { base } from "$app/paths";
import thumb from './pix_mobility.png'
import Mobility from "./mobility.svelte";

let name = 'Mobility Models'

export default {
    id: "MOBILITYMODELS",
    image: thumb,
    title: name,
    subtitle: "Some interactive mobility Models",
    tags: ["sim", "rust", "wasm"].toSorted(),
    link: `${base}/experiments/mobility_models`,
    component: Mobility,
};
