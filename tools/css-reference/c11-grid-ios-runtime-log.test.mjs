import assert from 'node:assert/strict';
import test from 'node:test';

import { hasC11GridDrawn, parseC11GridFrames } from './c11-grid-ios-runtime-log.mjs';

test('iOS frame parser preserves preorder IDs and fractional CSS coordinates', () => {
  const log = [
    'SPINON_C111_GRID_NODE_FRAME_SWIFT 0:node=1,x=0,y=0,width=360,height=800',
    'SPINON_C111_GRID_NODE_FRAME_SWIFT 1:node=2,x=-0.5,y=12.25,width=30.5,height=18',
  ].join('\n');

  assert.deepEqual(parseC11GridFrames(log), [
    { nodeId: 1, rect: { x: 0, y: 0, width: 360, height: 800 } },
    { nodeId: 2, rect: { x: -0.5, y: 12.25, width: 30.5, height: 18 } },
  ]);
});

test('draw detector accepts generation metadata before the success status', () => {
  const log = 'SPINON_C0410_DRAW generation=1 status=0 presented boxes=6 viewport_css_px=320x240';

  assert.equal(hasC11GridDrawn(log, 6), true);
  assert.equal(hasC11GridDrawn(log, 5), false);
});

test('draw detector rejects failed, missing, and stale-sized presentations', () => {
  assert.equal(hasC11GridDrawn('SPINON_C0410_DRAW generation=1 status=-1 boxes=6', 6), false);
  assert.equal(hasC11GridDrawn('SPINON_C0410_DRAW generation=1 status=0 presented boxes=5', 6), false);
  assert.equal(hasC11GridDrawn('SPINON_C0410_RENDERER status=0 boxes=6', 6), false);
});
