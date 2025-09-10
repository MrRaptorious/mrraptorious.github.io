import { base } from "$app/paths";
import thumb from '../assets/forbidden.ico'
import ErrorMessage from "./errorMessage.svelte";

let name = 'errormessage'

export default {
	id: "errormessage",
	image: thumb,
	title: name,
	subtitle: "",
	tags: [].toSorted(),
	link: "",
	component: ErrorMessage,
};
