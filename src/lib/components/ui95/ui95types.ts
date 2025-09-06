
import type { Component } from "svelte";

export type Program = {
  id: number;
  image: any;
  title: string;
  component: Component;
  windowData: WindowData | null;
}

export type WindowData = {
  width: number;
  height: number;
  pos_x: number;
  pos_y: number;
  minimized: boolean;
  maximized: boolean;
  hasfocus: boolean;
}

export interface WindowEvents {
  onClose: (id: number) => void;
  onChangePosition: (id: number, newX: number, newY: number) => void;
  onChangeSize: (id: number, newWidth: number, newHeight: number) => void;
}

export interface DataManipulator {
  setWindowData: (id: number, windowData: WindowData) => void;
  setfocus: (id: number) => void;
}
