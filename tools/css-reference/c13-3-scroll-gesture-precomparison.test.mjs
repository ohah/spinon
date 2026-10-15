import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const referencePath = path.join(root, 'tests/fixtures/css/references/c13-3-scroll-gesture-precomparison-v1.json');
const reference = JSON.parse(readFileSync(referencePath, 'utf8'));
const fixture = JSON.parse(JSON.stringify(reference.cases));

function sha256(relativePath) {
  return createHash('sha256').update(readFileSync(path.join(root, relativePath))).digest('hex');
}

function ownerOffset(value, id) {
  return value.finalOffsets[id];
}

test('pins the C13.3 Chrome 155 agent-browser oracle, fixture, and screenshot', () => {
  assert.equal(reference.schema, 'spinon-css-c13-3-scroll-gesture-precomparison/v1');
  assert.equal(reference.scope, 'Chrome oracle observation only; not Spinon runtime or app verification.');
  assert.equal(reference.oracle.browser, 'Chrome/155.0.8059.40');
  assert.match(reference.oracle.webkitVersion, /cfaadc5a132d78e1828635aa8405a499f3e14864/);
  assert.equal(reference.oracle.agentBrowserVersion, 'agent-browser 0.31.1');
  assert.deepEqual(reference.oracle.viewport, { width: 400, height: 240, deviceScaleFactor: 1 });
  assert.deepEqual(reference.screenshot.dimensions, [400, 240]);
  assert.equal(sha256(reference.fixture.path), reference.fixture.sha256);
  assert.equal(sha256(reference.screenshot.path), reference.screenshot.sha256);
});

test('separates direct pan, nested boundary start, and mid-gesture boundary behavior', () => {
  assert.deepEqual(ownerOffset(fixture.touchVertical60, 'inner'), [0, 45]);
  assert.deepEqual(ownerOffset(fixture.touchHorizontal60, 'inner'), [45, 0]);
  assert.deepEqual(ownerOffset(fixture.touchDiagonal60, 'inner'), [49, 49]);
  assert.deepEqual(ownerOffset(fixture.touchReachBoundary, 'inner'), [0, 106]);
  assert.deepEqual(ownerOffset(fixture.touchReachBoundary, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.touchBoundaryAuto, 'outer'), [0, 45]);
  assert.deepEqual(ownerOffset(fixture.touchBoundaryContain, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.touchBoundaryNone, 'outer'), [0, 0]);
  assert.deepEqual(fixture.touchBoundaryContain.finalOverscroll, ['contain', 'contain']);
  assert.deepEqual(fixture.touchBoundaryContain.initialOwners.outer.overscroll, ['auto', 'auto']);
  assert.deepEqual(fixture.touchBoundaryNone.finalOverscroll, ['none', 'none']);
  assert.deepEqual(fixture.touchBoundaryNone.initialOwners.outer.overscroll, ['auto', 'auto']);
  for (const id of ['touchOverflowHidden', 'touchOverflowClip', 'touchOverflowVisible']) {
    assert.deepEqual(ownerOffset(fixture[id], 'inner'), [0, 0]);
    assert.deepEqual(ownerOffset(fixture[id], 'outer'), [0, 45]);
  }
  assert.ok(fixture.touchBoundaryAuto.pointerEvents.some((event) => event.type === 'pointercancel'));
  assert.match(reference.interpretations.overflowValues, /hidden does not consume direct touch/);
});

test('pins touch-action start-time behavior and restricted pan axes', () => {
  assert.deepEqual(ownerOffset(fixture.touchActionNone, 'inner'), [0, 0]);
  assert.ok(fixture.touchActionNone.pointerEvents.some((event) => event.type === 'pointerup'));
  assert.equal(fixture.touchActionAfterStart.touchAction, 'none');
  assert.deepEqual(ownerOffset(fixture.touchActionAfterStart, 'inner'), [0, 45]);
  assert.deepEqual(ownerOffset(fixture.touchActionPanXHorizontal, 'inner'), [45, 0]);
  assert.deepEqual(ownerOffset(fixture.touchActionPanXVertical, 'inner'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.touchActionPanYVertical, 'inner'), [0, 45]);
  assert.deepEqual(ownerOffset(fixture.touchActionPanYHorizontal, 'inner'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.parentTouchActionPanXVertical, 'inner'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.parentTouchActionPanXVertical, 'outer'), [0, 0]);
  assert.equal(fixture.parentTouchActionPanXVertical.touchActionPath['inner-content'], 'pan-x');
  assert.equal(fixture.parentTouchActionPanXVertical.touchActionPath.inner, 'auto');
  assert.ok(ownerOffset(fixture.parentTouchActionPanXHorizontal, 'inner')[0] > 0);
  assert.equal(fixture.touchActionAfterPanBegan.touchAction, 'none');
  assert.deepEqual(ownerOffset(fixture.touchActionAfterPanBegan, 'inner'), [0, 45]);
  assert.ok(fixture.touchActionAfterPanBegan.pointerEvents.some((event) => event.type === 'pointercancel'));
  assert.deepEqual(ownerOffset(fixture.wheelTouchActionNone, 'inner'), [0, 40]);
  assert.ok(ownerOffset(fixture.touchAxisDirectionChange, 'inner')[0] > 0);
  assert.equal(ownerOffset(fixture.touchAxisDirectionChange, 'inner')[1], 0);
});

test('cancel preserves offset, records CDP late-move rejection, and recovers for a new pointer', () => {
  assert.deepEqual(fixture.touchCancel.beforeLateMove.offsets, fixture.touchCancel.afterLateMove.offsets);
  assert.deepEqual(fixture.touchCancel.afterLateMove.offsets.inner, [0, 15]);
  assert.equal(fixture.touchCancel.lateMoveDispatch.accepted, false);
  assert.match(fixture.touchCancel.lateMoveDispatch.error, /TouchStart first/);
  assert.ok(fixture.touchCancel.afterLateMove.pointerEvents.some((event) => event.type === 'pointercancel'));
  const multi = fixture.multiPointer;
  assert.equal(multi.input.simultaneousStart, true);
  assert.deepEqual(multi.during.offsets.inner, [0, 0]);
  assert.equal(multi.visualViewportScale, 1);
  assert.equal(multi.afterCancel.pointerEvents.filter((event) => event.type === 'pointercancel').length, 2);
  assert.deepEqual(multi.afterCancel.offsets, multi.during.offsets);
  assert.deepEqual(multi.freshGesture.offsets.inner, [0, 30]);
  assert.deepEqual(ownerOffset(fixture.touchInnerOuterBoundaryRootAuto, 'inner'), [0, 106]);
  assert.deepEqual(ownerOffset(fixture.touchInnerOuterBoundaryRootAuto, 'outer'), [0, 186]);
  assert.ok(ownerOffset(fixture.touchInnerOuterBoundaryRootAuto, 'root')[1] > 0);
});

test('records the per-axis overscroll oracle discrepancy without treating it as the contract', () => {
  assert.deepEqual(ownerOffset(fixture.touchAxisContainXAutoYHorizontal, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.touchAxisContainXAutoYVertical, 'outer'), [0, 45]);
  assert.deepEqual(ownerOffset(fixture.touchAxisContainXAutoYDiagonal, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.wheelXContainYAutoX, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.wheelXContainYAutoY, 'outer'), [0, 40]);
  assert.deepEqual(ownerOffset(fixture.wheelXContainYAutoDiagonal, 'outer'), [0, 0]);
  assert.deepEqual(ownerOffset(fixture.wheelAutoDiagonal, 'outer'), [40, 40]);
  assert.match(reference.interpretations.mixedAxisOverscroll, /per-axis contract/);
});
