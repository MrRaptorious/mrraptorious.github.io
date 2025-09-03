import { base } from "$app/paths";
import thumb from './pix_cellular.png'
import Cellautomaton from "./cellautomaton.svelte";

let name = 'Cellular automaton'

export default {
    id: "CELLULARAUTOMATON",
    image: thumb,
    title: name,
    subtitle: "little cellular action!",
    tags: ["wasm", "Generative Art"].toSorted(),
    link: `${base}/experiments/cellautomaton`,
    component: Cellautomaton,
};
