<script lang="ts">
  import { onMount } from "svelte";

  interface SpeechbubbleProps {
    right_offset: number;
    right_offset_unit: string;
    time?: number;
    close?: () => void;
  }

  let {
    right_offset,
    right_offset_unit,
    time = 0,
    close = () => {},
  }: SpeechbubbleProps = $props();

  onMount(() => {
    const interval = setInterval(() => {
      close();
    }, time);

    return () => clearInterval(interval);
  });
</script>

<div
  class="absolute bottom-13 text-black bg-wininfo-500 border rounded-md speech-bubble"
  style="right: {right_offset}{right_offset_unit};"
>
  <div
    style="
    content: '';
    position: absolute;
    right: {-right_offset + 5}{right_offset_unit};
    bottom: -14px;
    border-width: 13px 13px 0;
    border-style: solid;
    border-color: black transparent;
    display: block;"
  ></div>

  <slot />

  <div
    style="
    content: '';
    position: absolute;
    right: {-right_offset + 5}{right_offset_unit}; 
    bottom: -13px;
    border-width: 13px 13px 0;
    border-style: solid;
    border-color: var(--color-wininfo-500) transparent;
    display: block;"
  ></div>
</div>

<style>
</style>
