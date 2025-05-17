export interface TextMetrics {
  width: number;
  actualBoundingBoxLeft: number;
  actualBoundingBoxRight: number;
  fontBoundingBoxAscent: number;
  fontBoundingBoxDescent: number;
  actualBoundingBoxAscent: number;
  actualBoundingBoxDescent: number;
  emHeightAscent: number;
  emHeightDescent: number;
  hangingBaseline: number;
  alphabeticBaseline: number;
  ideographicBaseline: number;
}

export type CompositeOperation =
  | 'source-over'
  | 'source-in'
  | 'source-out'
  | 'source-atop'
  | 'destination-over'
  | 'destination-in'
  | 'destination-out'
  | 'destination-atop'
  | 'lighter'
  | 'copy'
  | 'xor'
  | 'multiply'
  | 'screen'
  | 'overlay'
  | 'darken'
  | 'lighten'
  | 'color-dodge'
  | 'color-burn'
  | 'hard-light'
  | 'soft-light'
  | 'difference'
  | 'exclusion';

export type LineCap = 'butt' | 'round' | 'square';
export type LineJoin = 'miter' | 'round' | 'bevel';
export type TextAlign = 'start' | 'end' | 'left' | 'right' | 'center';
export type TextBaseline = 'top' | 'hanging' | 'middle' | 'alphabetic' | 'ideographic' | 'bottom';
export type RepeatPattern = 'repeat' | 'repeat-x' | 'repeat-y' | 'no-repeat';

export interface ColorStop {
  offset: number;
  color: string;
}

export class Path2D {
  constructor(path?: Path2D);
  addPath(path: Path2D): void;
  closePath(): void;
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;
  quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;
  arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, counterclockwise?: boolean): void;
  rect(x: number, y: number, width: number, height: number): void;
  ellipse(
    x: number,
    y: number,
    radiusX: number,
    radiusY: number,
    rotation: number,
    startAngle: number,
    endAngle: number,
    counterclockwise?: boolean
  ): void;
}

export class CanvasPattern {
  constructor(imageData: ImageData, repeat: RepeatPattern);
}

export class LinearGradient {
  constructor(x0: number, y0: number, x1: number, y1: number);
  addColorStop(offset: number, color: string): void;
}

export class RadialGradient {
  constructor(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number);
  addColorStop(offset: number, color: string): void;
}

export class ImageData {
  constructor(width: number, height: number);
  static fromBuffer(data: Buffer, width: number, height?: number): ImageData;
  readonly width: number;
  readonly height: number;
  readonly data: Buffer;
}

export class CanvasRenderingContext2D {
  // State
  save(): void;
  restore(): void;

  // Transformations
  scale(x: number, y: number): void;
  rotate(angle: number): void;
  translate(x: number, y: number): void;

  // Fill and stroke styles
  fillStyle: string | CanvasPattern | LinearGradient | RadialGradient;
  strokeStyle: string | CanvasPattern | LinearGradient | RadialGradient;

  // Line styles
  lineWidth: number;
  lineCap: LineCap;
  lineJoin: LineJoin;
  miterLimit: number;

  // Shadows
  shadowBlur: number;
  shadowColor: string;
  shadowOffsetX: number;
  shadowOffsetY: number;

  // Rectangles
  clearRect(x: number, y: number, width: number, height: number): void;
  fillRect(x: number, y: number, width: number, height: number): void;
  strokeRect(x: number, y: number, width: number, height: number): void;

  // Paths
  beginPath(): void;
  closePath(): void;
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;
  quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void;
  arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, counterclockwise?: boolean): void;
  arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void;
  rect(x: number, y: number, width: number, height: number): void;
  fill(fillRule?: 'nonzero' | 'evenodd'): void;
  fill(path: Path2D, fillRule?: 'nonzero' | 'evenodd'): void;
  stroke(): void;
  stroke(path: Path2D): void;
  clip(fillRule?: 'nonzero' | 'evenodd'): void;
  clip(path: Path2D, fillRule?: 'nonzero' | 'evenodd'): void;
  isPointInPath(x: number, y: number, fillRule?: 'nonzero' | 'evenodd'): boolean;
  isPointInPath(path: Path2D, x: number, y: number, fillRule?: 'nonzero' | 'evenodd'): boolean;
  isPointInStroke(x: number, y: number): boolean;
  isPointInStroke(path: Path2D, x: number, y: number): boolean;

  // Text
  font: string;
  textAlign: TextAlign;
  textBaseline: TextBaseline;
  fillText(text: string, x: number, y: number, maxWidth?: number): void;
  strokeText(text: string, x: number, y: number, maxWidth?: number): void;
  measureText(text: string): TextMetrics;

  // Compositing
  globalAlpha: number;
  globalCompositeOperation: CompositeOperation;
}

export class Canvas {
  constructor(width: number, height: number);
  readonly width: number;
  readonly height: number;
  set width(value: number);
  set height(value: number);
  resize(width: number, height: number): void;
  getContext2d(): CanvasRenderingContext2D;
  toBuffer(mimeType?: string, quality?: number): Buffer;
}

export function version(): string;
