import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const evidencePath = process.argv[2];
if (!evidencePath) {
  throw new Error('Android 실행 로그 경로를 인자로 지정하세요.');
}

const [log, inventory, reference] = await Promise.all([
  readFile(resolve(evidencePath), 'utf8'),
  readFile(join(repositoryRoot, 'tests/fixtures/css/c12/position-absolute-block-inventory.json'), 'utf8')
    .then(JSON.parse),
  readFile(join(repositoryRoot, 'tests/fixtures/css/references/c12-2-position-absolute-block-v1.json'), 'utf8')
    .then(JSON.parse),
]);

assert.match(log, /SPINON_C122_SUMMARY status=0 layout=ready boxes=80\b/);
assert.match(log, /SPINON_C122_FRAME_SUMMARY (?:state=initial )?frames=83 marker=present/);
assert.equal(inventory.nodes.length, 82);
assert.equal(reference.observations[0].nodes.length, inventory.nodes.length);

const platform = log.includes('state=initial') ? 'Android physical device' : 'iOS Simulator';
assert.match(log, /SPINON_C0410_DRAW .*status=0 presented boxes=80 .*viewport_css_px=360x800/);
if (platform === 'Android physical device') {
  assert.match(log, /SPINON_C0410_RENDERER=backend=Vulkan/);
} else {
  assert.match(log, /SPINON_C0410_ENVIRONMENT viewport=360\.0x800\.0 .*render_viewport_css_px=360x800/);
}

const frames = [];
for (const line of log.split(/\r?\n/)) {
  if (!line.includes('SPINON_C122_NODE_FRAME ')) continue;
  const match = line.match(/SPINON_C122_NODE_FRAME (?:state=initial )?(\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,\s]+)/);
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
  frames.push(frame);
}

assert.equal(frames.length, inventory.nodes.length + 1);
assert.deepEqual(frames.map(({ index }) => index), frames.map((_, index) => index));

// Runtime fixture의 첫 자식은 화면 상자가 아닌 author <style> 요소이며, frame 출력에도 0 크기로 남는다.
const authorStyleFrame = frames[1];
assert.equal(authorStyleFrame.nodeId, 83);
assert.deepEqual(
  [authorStyleFrame.x, authorStyleFrame.y, authorStyleFrame.width, authorStyleFrame.height],
  [0, 0, 0, 0],
);
const productFrames = [frames[0], ...frames.slice(2)];
const tolerance = reference.comparison.maximumAbsoluteRectErrorCssPx;
let maximumAbsoluteErrorCssPx = 0;
let worstField = null;

for (const [index, expectedNode, observedNode, actual] of inventory.nodes
  .map((node, index) => [index, node, reference.observations[0].nodes[index], productFrames[index]])) {
  assert.equal(observedNode.id, expectedNode.id, `기준 순서 ${index}`);
  assert.equal(actual.index, index === 0 ? 0 : index + 1, `${expectedNode.id} runtime preorder`);
  assert.equal(actual.nodeId, index + 1, `${expectedNode.id} runtime node ID`);
  for (const field of ['x', 'y', 'width', 'height']) {
    const error = Math.abs(actual[field] - observedNode.rect[field]);
    if (error > maximumAbsoluteErrorCssPx) {
      maximumAbsoluteErrorCssPx = error;
      worstField = { index, node: expectedNode.id, field, actual: actual[field], expected: observedNode.rect[field] };
    }
    assert.ok(error <= tolerance, `${expectedNode.id}.${field}: ${actual[field]} != ${observedNode.rect[field]} (허용 ${tolerance})`);
  }
}

process.stdout.write(`${JSON.stringify({
  status: 'matched',
  platform,
  fixtureId: inventory.fixtureId,
  matchedNodes: productFrames.length,
  skippedZeroBoxStyleNodes: 1,
  chromiumVersion: reference.chromium.version,
  maximumAbsoluteErrorCssPx,
  toleranceCssPx: tolerance,
  worstField,
}, null, 2)}\n`);
