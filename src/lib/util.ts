export function clamp(min: number, max: number, x: number) {
  return Math.min(max, Math.max(min, x));
}

export function removeAfterElement<T>(arr: T[], element: T) {
  const index = arr.indexOf(element);

  if (index !== -1) {
    return arr.slice(0, index);
  }

  return arr;
}
