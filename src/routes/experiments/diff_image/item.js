import { base } from "$app/paths";
import thumb from './thumb.png'

let name = 'diff_image'

export default {
    id: "DIFFIMAGE",
    image: thumb,
    title: "Difference Image",
    subtitle: "Visualize motion between images",
    tags: ["CV"].toSorted(),
    link: `${base}/experiments/${name}`,
};
