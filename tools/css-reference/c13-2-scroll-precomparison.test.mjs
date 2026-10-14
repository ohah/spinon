import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const referencePath = path.join(root, 'tests/fixtures/css/references/c13-2-scroll-container-precomparison-v1.json');
const reference = JSON.parse(readFileSync(referencePath, 'utf8'));
const byId = (id) => {
  const fixture = reference.fixtures.find((entry) => entry.id === id);
  assert.ok(fixture, `missing fixture observation: ${id}`);
  return fixture;
};
const cases = byId('c13-2-scroll-containers-v1').observation.cases;
const rootCases = byId('c13-2-root-scroll-owner-v1').observation.cases;
const rootCase = (name) => {
  const item = rootCases.find((entry) => entry.name === name);
  assert.ok(item, `missing root case: ${name}`);
  return item;
};

function sha256(pathname) {
  return createHash('sha256').update(readFileSync(pathname)).digest('hex');
}

test('pins the C13.2 Chrome 155 agent-browser oracle and its fixture images', () => {
  assert.equal(reference.schema, 'spinon-css-c13-2-scroll-container-precomparison/v1');
  assert.equal(reference.oracle.browser, 'Chrome/155.0.8059.40');
  assert.match(reference.oracle.webkitVersion, /cfaadc5a132d78e1828635aa8405a499f3e14864/);
  assert.equal(reference.captureTool.name, 'agent-browser');
  assert.deepEqual(reference.oracle.viewport, { width: 640, height: 480, deviceScaleFactor: 1 });
  assert.equal(reference.fixtures.length, 2);

  for (const fixture of reference.fixtures) {
    const fixturePath = path.join(root, fixture.path);
    const screenshotPath = path.join(root, fixture.screenshot);
    assert.equal(sha256(fixturePath), fixture.sha256, `${fixture.id} source hash`);
    assert.equal(sha256(screenshotPath), fixture.screenshotSha256, `${fixture.id} screenshot hash`);
  }
});

test('separates visible paint, out-of-flow scrollable overflow, and ink-only overflow', () => {
  const visible = cases['visible-overflow'];
  assert.deepEqual(visible.overflow, ['visible', 'visible']);
  assert.ok(visible.scroll[0] > visible.client[0]);
  assert.deepEqual(cases.visibleAfterScriptedWrite.offset, [0, 0]);

  const escaped = cases.absolute;
  assert.deepEqual(escaped.overflow, ['auto', 'auto']);
  assert.equal(escaped.scroll[0], 178);
  assert.equal(escaped.client[0], 114);
  assert.equal(escaped.max[0], 64);

  const inkOnly = cases.ink;
  assert.deepEqual(inkOnly.scroll, inkOnly.client);
  assert.deepEqual(inkOnly.max, [0, 0]);
});

test('checks axis ranges, hidden programmatic scrolling, and clip no-offset behavior', () => {
  assert.deepEqual(cases['one-axis-x'].max, [86, 0]);
  assert.deepEqual(cases['one-axis-y'].max, [0, 74]);
  assert.deepEqual(cases.noOverflowAfterWrite.offset, [0, 0]);
  assert.deepEqual(cases['forced-scroll'].overflow, ['scroll', 'scroll']);
  assert.deepEqual(cases['forced-scroll'].max, [0, 0]);
  assert.deepEqual(cases.forcedScrollAfterWrite.offset, [0, 0]);
  assert.deepEqual(cases.negativeXAfterWrite.offset, [0, 0]);
  assert.deepEqual(cases.negativeYAfterWrite.offset, [0, 0]);
  assert.deepEqual(cases.hiddenAfterScriptedScroll.offset, [55, 44]);
  assert.deepEqual(cases.fractionalOffsetInput, [
    { requested: 55.25, actual: [55, 55] },
    { requested: 55.49, actual: [55, 55] },
    { requested: 55.5, actual: [56, 56] },
    { requested: 55.51, actual: [56, 56] },
    { requested: 55.75, actual: [56, 56] },
  ]);
  assert.deepEqual(cases.clipAfterScriptedScroll.offset, [0, 0]);
});

test('keeps nested offsets independent and preserves or clamps them after layout changes', () => {
  assert.deepEqual(cases.nested.outer.offset, [22, 17]);
  assert.deepEqual(cases.nested.inner.offset, [42, 29]);
  assert.deepEqual(cases.retainAfterGrowth.offset, [30, 20]);
  assert.deepEqual(cases.retainAfterDescendantMove.offset, [30, 20]);
  assert.deepEqual(cases.clampBeforeShrink.offset, cases.clampBeforeShrink.max);
  assert.deepEqual(cases.clampAfterShrink.max, [0, 0]);
  assert.deepEqual(cases.clampAfterShrink.offset, [0, 0]);
  assert.deepEqual(cases.moveBeforeDetach.offset, [37, 28]);
  assert.deepEqual(cases.moveAfterReattach.offset, [0, 0]);
});

test('distinguishes viewport, propagated body overflow, and an independent body scroller', () => {
  const defaultVisible = rootCase('default-visible');
  const bodyPropagates = rootCase('body-auto-propagates');
  const htmlAuto = rootCase('html-auto');
  const bodyOwns = rootCase('html-hidden-body-auto');

  for (const item of [defaultVisible, bodyPropagates, htmlAuto, bodyOwns]) {
    assert.equal(item.scrollingElement, 'HTML');
  }
  assert.deepEqual(bodyPropagates.writes['window.scrollTo'].html, [64, 83]);
  assert.deepEqual(bodyPropagates.writes['body.scroll'].body, [0, 0]);
  assert.deepEqual(bodyOwns.writes['window.scrollTo'].window, [0, 0]);
  assert.deepEqual(bodyOwns.writes['body.scroll'].body, [64, 83]);

  const fixedRects = rootCases.map((item) => item.writes['window.scrollTo'].fixedRect);
  for (const rect of fixedRects.slice(1)) assert.deepEqual(rect, fixedRects[0]);
});
