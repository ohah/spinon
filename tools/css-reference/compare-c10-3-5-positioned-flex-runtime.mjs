import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const [platform, evidencePath] = process.argv.slice(2);
if (!['android-physical', 'ios-simulator'].includes(platform) || !evidencePath) {
  throw new Error('사용법: node compare-c10-3-5-positioned-flex-runtime.mjs <android-physical|ios-simulator> <로그 경로>');
}

const [log, runtimeInventory, fixtureInventory, reference] = await Promise.all([
  readFile(resolve(evidencePath), 'utf8'),
  readFile(join(repositoryRoot, 'tests/fixtures/css/c10/positioned-flex-runtime-inventory.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(repositoryRoot, 'tests/fixtures/css/c10/positioned-flex-inventory.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(repositoryRoot, 'tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json'), 'utf8')
    .then(JSON.parse),
]);

const expectedBoxCount = 16;
assert.match(log, new RegExp(`SPINON_C1035_SUMMARY status=0 layout=ready boxes=${expectedBoxCount}\\b`));
assert.match(log, /SPINON_C1035_FRAME_SUMMARY (?:state=initial )?frames=16 marker=present/);
assert.match(log, /SPINON_C0410_DRAW .*status=0 presented boxes=16 .*viewport_css_px=320x240/);
if (platform === 'android-physical') {
  assert.match(log, /SPINON_C0410_RENDERER=backend=Vulkan device=Samsung Xclipse 940/);
} else {
  assert.match(log, /SPINON_C0410_RENDERER status=0 backend=Metal/);
  assert.match(log, /SPINON_C0410_SURFACE_CONFIGURE thread=main status=0/);
}

const observedFrames = [];
for (const line of log.split(/\r?\n/)) {
  if (!line.includes('SPINON_C1035_NODE_FRAME ')) continue;
  const match = line.match(/SPINON_C1035_NODE_FRAME (?:state=initial )?(\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,\s]+)/);
  assert.ok(match, `프레임 로그 형식이 잘못되었습니다: ${line}`);
  const [, index, nodeId, x, y, width, height] = match;
  const frame = {
    index: Number(index),
    nodeId: Number(nodeId),
    x: Number(x),
    y: Number(y),
    width: Number(width),
    height: Number(height),
  };
  assert.ok(Object.values(frame).every(Number.isFinite), `유한하지 않은 frame: ${line}`);
  observedFrames.push(frame);
}

assert.equal(observedFrames.length, expectedBoxCount);
assert.deepEqual(observedFrames.map(({ index }) => index), observedFrames.map((_, index) => index));
assert.deepEqual(observedFrames.map(({ nodeId }) => nodeId), observedFrames.map((_, index) => index + 1));
assert.deepEqual(
  [observedFrames[0].x, observedFrames[0].y, observedFrames[0].width, observedFrames[0].height],
  [0, 0, runtimeInventory.viewport.width, runtimeInventory.viewport.height],
  'runtime root viewport frame',
);

let frameIndex = 1;
let matchedNodes = 0;
let maximumAbsoluteErrorCssPx = 0;
let worstField = null;
const tolerance = fixtureInventory.comparison.maximumAbsoluteRectErrorCssPx;
const referenceCases = reference.observations[0].cases;

for (const runtimeCase of runtimeInventory.cases) {
  const fixtureCase = fixtureInventory.cases.find(({ id }) => id === runtimeCase.id);
  const referenceCase = referenceCases.find(({ id }) => id === runtimeCase.id);
  assert.ok(fixtureCase && referenceCase, `고정 기준 case가 없습니다: ${runtimeCase.id}`);
  assert.equal(reference.observations[1].cases.find(({ id }) => id === runtimeCase.id)?.id, runtimeCase.id);
  assert.equal(referenceCase.nodes.length, fixtureCase.nodes.length);

  for (let nodeIndex = 0; nodeIndex < fixtureCase.nodes.length; nodeIndex += 1) {
    const expectedNode = fixtureCase.nodes[nodeIndex];
    const referenceNode = referenceCase.nodes[nodeIndex];
    const actual = observedFrames[frameIndex];
    assert.equal(referenceNode.id, expectedNode.id, `${runtimeCase.id} 기준 순서 ${nodeIndex}`);
    assert.equal(actual.nodeId, frameIndex + 1, `${runtimeCase.id}/${expectedNode.id} runtime NodeId`);

    for (const field of ['x', 'y', 'width', 'height']) {
      const expected = referenceNode.rect[field]
        + (field === 'x' ? runtimeCase.frameOffset.x : field === 'y' ? runtimeCase.frameOffset.y : 0);
      const error = Math.abs(actual[field] - expected);
      if (error > maximumAbsoluteErrorCssPx) {
        maximumAbsoluteErrorCssPx = error;
        worstField = { case: runtimeCase.id, node: expectedNode.id, field, actual: actual[field], expected };
      }
      assert.ok(error <= tolerance,
        `${runtimeCase.id}/${expectedNode.id}.${field}: ${actual[field]} != ${expected} (허용 ${tolerance})`);
    }

    frameIndex += 1;
    matchedNodes += 1;
  }
}

assert.equal(frameIndex, observedFrames.length);
process.stdout.write(`${JSON.stringify({
  status: 'matched',
  platform,
  fixtureId: runtimeInventory.fixtureId,
  chromiumVersion: reference.oracle.cliVersion,
  matchedNodes,
  maximumAbsoluteErrorCssPx,
  toleranceCssPx: tolerance,
  worstField,
}, null, 2)}\n`);
