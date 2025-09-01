import { base } from "$app/paths";
import thumb from './thumb.png'

let name = 'window_manager'

export default {
    image: thumb,
    title: "Window Manager",
    subtitle: "",
    tags: ["UI", "web-gpu"].toSorted(),
    link: `${base}/experiments/${name}`,
};
