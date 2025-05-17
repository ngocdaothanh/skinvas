const { loadBinding } = require('@node-rs/helper');
const path = require('path');

/**
 * skinvas is an implementation of the HTML5 Canvas API using Rust and Skia
 * @type {import('.').Canvas}
 */
const binding = loadBinding(
  path.resolve(__dirname),
  'skinvas',
  '@skinvas/skinvas'
);

module.exports = binding;
