import type { SvelteComponent } from "svelte";

export type ItemData = {
    id: string,
    image: any,
    title: string; 
    subtitle: string; 
    tags: string[];
    link: string; 
    component: any 
}