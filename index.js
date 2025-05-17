const { loadBinding } = require('@node-rs/helper');
const path = require('path');

/**
 * skia-canvas is an implementation of the HTML5 Canvas API using Rust and Skia
 * @type {import('.').Canvas}
 */
const binding = loadBinding(
  path.resolve(__dirname),
  'skia-canvas',
  '@skia-canvas/skia-canvas'
);

module.exports = binding;
