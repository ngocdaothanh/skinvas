const { Canvas } = require('../');
const fs = require('fs');
const path = require('path');

// Create a canvas
const canvas = new Canvas(600, 400);
const ctx = canvas.getContext2d();

// Fill the background
ctx.fillStyle = 'white';
ctx.fillRect(0, 0, 600, 400);

// Draw a blue rectangle
ctx.fillStyle = 'blue';
ctx.fillRect(100, 100, 200, 200);

// Draw text
ctx.font = '30px Arial';
ctx.fillStyle = 'white';
ctx.textAlign = 'center';
ctx.fillText('Hello Skia Canvas!', 200, 200);

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

// Draw a circle
ctx.beginPath();
ctx.arc(450, 350, 40, 0, Math.PI * 2);
ctx.fillStyle = 'green';
ctx.fill();

// Save the canvas to a file
const outputDir = path.join(__dirname, 'output');
if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true });
}
const buffer = canvas.toBuffer('image/png');
fs.writeFileSync(path.join(outputDir, 'example.png'), buffer);

console.log('Image saved to', path.join(outputDir, 'example.png'));
