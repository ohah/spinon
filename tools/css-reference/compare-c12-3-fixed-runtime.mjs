import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const runtimeCaseIds = new Set([
  'viewport-insets',
  'unpositioned-ancestor',
  'positioned-ancestors',
  'fixed-nesting',
  'hidden-subtree',
]);

function parseFrames(log) {
  const framesByState = new Map();
  const pattern = /SPINON_C123_NODE_FRAME state=([^\s]+) (\d+):node=(\d+),x=([^,]+),y=([^,]+),width=([^,]+),height=([^,\s]+)/;
  for (const line of log.split(/\r?\n/)) {
    const match = line.match(pattern);
    if (!match) continue;
    const [, state, index, nodeId, x, y, width, height] = match;
    const frames = framesByState.get(state) ?? [];
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
    framesByState.set(state, frames);
  }
  return framesByState;
}

function parseSummaries(log) {
  const summaries = new Map();
  for (const line of log.split(/\r?\n/)) {
    const marker = 'SPINON_C123_SUMMARY state=';
    const markerIndex = line.indexOf(marker);
    if (markerIndex < 0) continue;
    const value = line.slice(markerIndex + marker.length);
    const separator = value.indexOf(' ');
    assert.ok(separator > 0, `summary 형식이 잘못되었습니다: ${line}`);
    const state = value.slice(0, separator);
    assert.ok(!summaries.has(state), `중복 summary state: ${state}`);
    summaries.set(state, value.slice(separator + 1));
  }
  return summaries;
}

function statePlan(reference) {
  const states = reference.observations.map(observation => ({
    id: observation.id,
    observation,
  }));
  states.push(...reference.resizeObservations.map(observation => ({
    id: observation.id,
    observation,
  })));
  states.push({
    id: 'device-final',
    observation: reference.observations.find(({ id }) => id === 'matrix-360x800-dpr-1'),
  });
  return states;
}

export function compareRuntimeLog({ log, platform, inventory, reference }) {
  assert.ok(['android-physical', 'ios-simulator'].includes(platform), `알 수 없는 플랫폼: ${platform}`);
  assert.equal(inventory.schema, 'spinon-css-c12-3-position-fixed-inventory/v1');
  assert.equal(reference.referenceId, 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-3');
  assert.equal(reference.comparison.maximumAbsoluteRectErrorCssPx, 0.5);
  assert.doesNotMatch(log, /SPINON_C123_MATRIX_FAILURE/);

  const expectedNodes = inventory.nodes.filter(node =>
    node.id === 'c12-root' || runtimeCaseIds.has(node.caseId));
  assert.equal(expectedNodes.length, 28);
  assert.equal(expectedNodes[0].id, 'c12-root');

  const summaries = parseSummaries(log);
  const framesByState = parseFrames(log);
  const plannedStates = statePlan(reference);
  assert.equal(summaries.size, plannedStates.length);
  assert.equal(framesByState.size, plannedStates.length);
  const expectedStateIds = plannedStates.map(({ id }) => id).sort();
  assert.deepEqual([...summaries.keys()].sort(), expectedStateIds);
  assert.deepEqual([...framesByState.keys()].sort(), expectedStateIds);

  let maximumAbsoluteErrorCssPx = 0;
  let worstField = null;
  let expectedEnvironmentRevision = 0;
  let matchedFrames = 0;
  for (let stateIndex = 0; stateIndex < plannedStates.length; stateIndex += 1) {
    const { id: state, observation } = plannedStates[stateIndex];
    if (stateIndex > 0 && state !== 'resize-noop') expectedEnvironmentRevision += 1;
    const summary = summaries.get(state);
    assert.match(summary, /\bstatus=0\b/, `${state} 성공 상태`);
    assert.match(summary, /\blayout=ready\b/, `${state} layout`);
    assert.match(summary, /\bboxes=26\b/, `${state} box 수`);
    assert.match(summary, new RegExp(`\\benvironment_revision=${expectedEnvironmentRevision}(?:\\s|$)`),
      `${state} environment revision`);

    const frames = framesByState.get(state);
    assert.equal(frames.length, expectedNodes.length, `${state} frame 수`);
    const byNodeId = new Map(frames.map(frame => [frame.nodeId, frame]));
    assert.equal(byNodeId.size, expectedNodes.length, `${state} 중복 NodeId`);
    const frameIndexes = frames.map(({ index }) => index).sort((left, right) => left - right);
    assert.deepEqual(frameIndexes, expectedNodes.map((_, index) => index), `${state} frame index`);

    const referenceNodes = new Map(observation.nodes.map(node => [node.id, node]));
    for (const [expectedIndex, expectedNode] of expectedNodes.entries()) {
      const actual = byNodeId.get(expectedIndex + 1);
      assert.ok(actual, `${state}/${expectedNode.id} runtime NodeId ${expectedIndex + 1} 누락`);
      const expected = referenceNodes.get(expectedNode.id);
      assert.ok(expected, `${state}/${expectedNode.id} Chromium node 누락`);
      assert.equal(expected.caseId, expectedNode.caseId, `${state}/${expectedNode.id} case`);
      assert.equal(expected.parentId, expectedNode.parentId, `${state}/${expectedNode.id} source parent`);
      assert.equal(expected.owner, expectedNode.expectedOwner, `${state}/${expectedNode.id} owner`);
      assert.equal(expected.properties.position, expectedNode.expectedPosition,
        `${state}/${expectedNode.id} position`);
      if (expectedNode.hasLayoutBox === false) {
        assert.deepEqual(
          [actual.x, actual.y, actual.width, actual.height],
          [0, 0, 0, 0],
          `${state}/${expectedNode.id} hidden frame`,
        );
      }

      for (const field of reference.comparison.rectFields) {
        const target = expected.rect[field];
        const error = Math.abs(actual[field] - target);
        if (error > maximumAbsoluteErrorCssPx) {
          maximumAbsoluteErrorCssPx = error;
          worstField = {
            state,
            node: expectedNode.id,
            field,
            actual: actual[field],
            expected: target,
          };
        }
        assert.ok(error <= reference.comparison.maximumAbsoluteRectErrorCssPx,
          `${state}/${expectedNode.id}.${field}: ${actual[field]} != ${target}`);
      }
      matchedFrames += 1;
    }
  }

  const finalRevision = expectedEnvironmentRevision;
  assert.match(log, new RegExp(`SPINON_C0410_DRAW .*status=0 presented boxes=26 environment_revision=${finalRevision} viewport_css_px=360x800`));
  if (platform === 'android-physical') {
    assert.match(log, /SPINON_C0410_RENDERER=backend=Vulkan device=Samsung Xclipse 940/);
  } else {
    assert.match(log, /SPINON_C0410_RENDERER status=0 backend=Metal/);
    assert.match(log, /SPINON_C0410_SURFACE_CONFIGURE thread=main status=0/);
  }

  return {
    status: 'matched',
    platform,
    fixtureId: inventory.fixtureId,
    chromiumVersion: reference.chromium.version,
    matchedStates: plannedStates.length,
    matchedFrames,
    finalEnvironmentRevision: finalRevision,
    maximumAbsoluteErrorCssPx,
    toleranceCssPx: reference.comparison.maximumAbsoluteRectErrorCssPx,
    worstField,
  };
}

async function main() {
  const [platform, logPath] = process.argv.slice(2);
  if (!['android-physical', 'ios-simulator'].includes(platform) || !logPath) {
    throw new Error('사용법: node compare-c12-3-fixed-runtime.mjs <android-physical|ios-simulator> <로그 경로>');
  }
  const [log, inventory, reference] = await Promise.all([
    readFile(resolve(logPath), 'utf8'),
    readFile(join(repositoryRoot, 'tests/fixtures/css/c12/position-fixed-inventory.json'), 'utf8')
      .then(JSON.parse),
    readFile(join(repositoryRoot, 'tests/fixtures/css/references/c12-3-position-fixed-v1.json'), 'utf8')
      .then(JSON.parse),
  ]);
  process.stdout.write(`${JSON.stringify(compareRuntimeLog({ log, platform, inventory, reference }), null, 2)}\n`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  await main();
}
