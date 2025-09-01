import { base } from "$app/paths";

let name = 'Cellular automaton'

export default {
    id: "CELLULARAUTOMATON",
    image: `${base}/placeholder_image.svg`,
    title: name,
    subtitle: "little cellular action!",
    tags: ["wasm", "Generative Art"].toSorted(),
    link: `${base}/experiments/cellautomaton`,
};
