import { base } from "$app/paths";
import SpinnableBadge from "./spinnable_badge.svelte";
import thumb from './pix_badge.png'
let name = 'spinnable_badge'

export default {
    id: "SPINNABLE_BADGE",
    image: thumb,
    title: "Spinnable Badge",
    subtitle: "3D spinnable object ",
    tags: ["threejs"],
    link: `${base}/experiments/${name}`,
    component: SpinnableBadge,
};

