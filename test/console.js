'use strict';

const assert = require('assert');
console.log(process);
console.log({
  foo: 'bar',
  fee: {
    loop: 100
  }
});

assert.ok(typeof console.log === 'function');
assert.ok(typeof console.info === 'function');
assert.ok(typeof console.warn === 'function');
assert.ok(typeof console.error === 'function');
console.info('test/console.js is done');
