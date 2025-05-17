const { Canvas } = require('../');
const fs = require('fs');
const path = require('path');

describe('Canvas API', () => {
  const outputDir = path.join(__dirname, 'output');

  // Create output directory if it doesn't exist
  beforeAll(() => {
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }
  });

  test('create canvas with dimensions', () => {
    const canvas = new Canvas(200, 100);
    expect(canvas.width).toBe(200);
    expect(canvas.height).toBe(100);
  });

  test('resize canvas', () => {
    const canvas = new Canvas(200, 100);
    canvas.resize(300, 150);
    expect(canvas.width).toBe(300);
    expect(canvas.height).toBe(150);
  });

  test('draw rectangle', () => {
    const canvas = new Canvas(200, 100);
    const ctx = canvas.getContext2D();

    ctx.fillStyle = 'blue';
    ctx.fillRect(10, 10, 50, 50);

    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(outputDir, 'rectangle.png'), buffer);

    expect(buffer.length).toBeGreaterThan(0);
  });

  test('draw path', () => {
    const canvas = new Canvas(200, 200);
    const ctx = canvas.getContext2D();

    ctx.beginPath();
    ctx.moveTo(50, 50);
    ctx.lineTo(150, 50);
    ctx.lineTo(100, 150);
    ctx.closePath();

    ctx.fillStyle = 'red';
    ctx.fill();

    ctx.strokeStyle = 'black';
    ctx.lineWidth = 5;
    ctx.stroke();

    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(outputDir, 'path.png'), buffer);

    expect(buffer.length).toBeGreaterThan(0);
  });

  test('draw text', () => {
    const canvas = new Canvas(300, 100);
    const ctx = canvas.getContext2D();

    ctx.fillStyle = 'black';
    ctx.font = '30px Arial';
    ctx.fillText('Hello Skinvas!', 10, 50);

    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(outputDir, 'text.png'), buffer);

    expect(buffer.length).toBeGreaterThan(0);
  });

  test('draw circle', () => {
    const canvas = new Canvas(200, 200);
    const ctx = canvas.getContext2D();

    ctx.beginPath();
    ctx.arc(100, 100, 50, 0, Math.PI * 2);
    ctx.fillStyle = 'green';
    ctx.fill();
    ctx.lineWidth = 5;
    ctx.strokeStyle = 'black';
    ctx.stroke();

    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(outputDir, 'circle.png'), buffer);

    expect(buffer.length).toBeGreaterThan(0);
  });

  test('gradient', () => {
    const canvas = new Canvas(200, 100);
    const ctx = canvas.getContext2D();

    // Create gradient
    const gradient = ctx.createLinearGradient(0, 0, 200, 0);
    gradient.addColorStop(0, 'red');
    gradient.addColorStop(0.5, 'yellow');
    gradient.addColorStop(1, 'blue');

    // Fill with gradient
    ctx.setLinearGradientFillStyle(gradient);
    ctx.fillRect(0, 0, 200, 100);

    const buffer = canvas.toBuffer('image/png');
    fs.writeFileSync(path.join(outputDir, 'gradient.png'), buffer);

    expect(buffer.length).toBeGreaterThan(0);
  });
});
