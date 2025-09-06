
import { elasticOut } from "svelte/easing";

export function stutterFade(node: HTMLElement, params: { delay?: number, duration?: number, easing?: (t: number) => number }) {
  return {
    delay: params.delay || 0,
    duration: params.duration || 400,
    css: (t, u) => `opacity: ${Math.round(t * 5) / 5};`
  }
}

