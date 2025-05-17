# Skinvas

A high-performance HTML5 Canvas API implementation for Node.js using the Rust language and the Skia rendering engine.

## Features

- Complete HTML5 Canvas API implementation
- High-performance Rust implementation under the hood
- Powered by Google's Skia rendering engine (same engine used in Chrome, Android, and Flutter)
- Cross-platform support (Windows, macOS, Linux)
- Zero JavaScript dependencies

## Installation

```bash
npm install skinvas
```

## Usage

```javascript
const { Canvas } = require('skinvas');

// Create a canvas
const canvas = new Canvas(600, 400);
const ctx = canvas.getContext2D();

// Draw a rectangle
ctx.fillStyle = 'blue';
ctx.fillRect(100, 100, 200, 200);

// Draw text
ctx.font = '30px Arial';
ctx.fillStyle = 'white';
ctx.textAlign = 'center';
ctx.fillText('Hello Skinvas!', 200, 200);

// Draw a path
ctx.beginPath();
ctx.moveTo(300, 100);
ctx.lineTo(500, 100);
ctx.lineTo(400, 300);
ctx.closePath();
ctx.fillStyle = 'red';
ctx.fill();
ctx.lineWidth = 5;
ctx.strokeStyle = 'black';
ctx.stroke();

// Export to a file
const fs = require('fs');
const buffer = canvas.toBuffer('image/png');
fs.writeFileSync('output.png', buffer);
```

## API

This library implements the standard [HTML5 Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API). Most methods and properties should work as expected.

### Canvas

```typescript
class Canvas {
  constructor(width: number, height: number);
  readonly width: number;
  readonly height: number;
  set width(value: number);
  set height(value: number);
  resize(width: number, height: number): void;
  getContext2D(): CanvasRenderingContext2D;
  toBuffer(mimeType?: string, quality?: number): Buffer;
}
```

### CanvasRenderingContext2D

The context provides all the standard Canvas 2D drawing methods:

- State management: `save()`, `restore()`
- Transformations: `scale()`, `rotate()`, `translate()`
- Styles: `fillStyle`, `strokeStyle`, `lineWidth`, etc.
- Path operations: `beginPath()`, `moveTo()`, `lineTo()`, etc.
- Drawing: `fillRect()`, `strokeRect()`, `fill()`, `stroke()`, etc.
- Text: `fillText()`, `strokeText()`, `measureText()`, etc.

## Advanced Usage

### Using Gradients

```javascript
const { Canvas } = require('skinvas');
const canvas = new Canvas(400, 200);
const ctx = canvas.getContext2D();

// Create a linear gradient
const linearGradient = ctx.createLinearGradient(0, 0, 400, 0);
linearGradient.addColorStop(0, 'red');
linearGradient.addColorStop(0.5, 'green');
linearGradient.addColorStop(1, 'blue');

ctx.fillStyle = linearGradient;
ctx.fillRect(0, 0, 400, 200);

// Create a radial gradient
const radialGradient = ctx.createRadialGradient(200, 100, 0, 200, 100, 100);
radialGradient.addColorStop(0, 'white');
radialGradient.addColorStop(1, 'transparent');

ctx.fillStyle = radialGradient;
ctx.fillRect(0, 0, 400, 200);
```

### Using Path2D

```javascript
const { Canvas, Path2D } = require('skinvas');
const canvas = new Canvas(400, 400);
const ctx = canvas.getContext2D();

const path = new Path2D();
path.rect(10, 10, 100, 100);

const path2 = new Path2D();
path2.arc(250, 100, 50, 0, 2 * Math.PI);

ctx.fillStyle = 'blue';
ctx.fill(path);

ctx.fillStyle = 'green';
ctx.fill(path2);
```

### Image Processing

```javascript
const { Canvas, ImageData } = require('skinvas');
const fs = require('fs');

// Create a canvas and draw something on it
const canvas = new Canvas(200, 200);
const ctx = canvas.getContext2D();
ctx.fillStyle = 'red';
ctx.fillRect(0, 0, 100, 100);
ctx.fillStyle = 'blue';
ctx.fillRect(100, 100, 100, 100);

// Get image data
const imageData = ctx.getImageData(0, 0, 200, 200);
const data = imageData.data;

// Modify image data (invert colors)
for (let i = 0; i < data.length; i += 4) {
  data[i] = 255 - data[i];         // red
  data[i + 1] = 255 - data[i + 1]; // green
  data[i + 2] = 255 - data[i + 2]; // blue
}

// Put the modified image data back on the canvas
ctx.putImageData(imageData, 0, 0);

// Save the result
fs.writeFileSync('inverted.png', canvas.toBuffer('image/png'));
```

## Building from Source

To build this library from source, you need:

1. Rust toolchain (1.65+)
2. Node.js (14.x+)
3. Python (for node-gyp)
4. C++ compiler

```bash
npm install
npm run build
```
