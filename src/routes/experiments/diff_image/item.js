import { base } from "$app/paths";
import thumb from './pix_diffImage.png'

let name = 'diff_image'

export default {
    id: "DIFFIMAGE",
    image: thumb,
    title: "Difference Image",
    subtitle: "Visualize motion between images",
    tags: ["CV"].toSorted(),
    link: `${base}/experiments/${name}`,
};
