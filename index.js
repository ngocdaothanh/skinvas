const path = require('path');

/**
 * skinvas is an implementation of the HTML5 Canvas API using Rust and Skia
 * @type {import('.').Canvas}
 */
// Try a direct require approach instead of using @node-rs/helper
const binding = require('./skinvas.node');

module.exports = binding;
