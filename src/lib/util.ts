export function clamp(min: number, max: number, x: number) {
  return Math.min(max, Math.max(min, x));
}