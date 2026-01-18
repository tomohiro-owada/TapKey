export interface GridConfig {
  columns: number;
  rows: number;
}

export interface ButtonPosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ShortcutAction {
  type: 'shortcut';
  keys: string[];
}

export interface TextAndEnterAction {
  type: 'text_and_enter';
  text: string;
}

export type ButtonAction = ShortcutAction | TextAndEnterAction;

export interface RepeatConfig {
  enabled: boolean;
  interval_ms: number;
}

export interface ButtonConfig {
  id: string;
  label: string;
  position: ButtonPosition;
  action: ButtonAction;
  color?: string;
  repeat?: RepeatConfig;
}

export interface AppConfig {
  port: number;
  pin: string;
  auto_start: boolean;
  grid: GridConfig;
  buttons: ButtonConfig[];
}
