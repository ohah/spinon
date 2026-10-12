import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { parse } from 'parse5';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-layout-inventory.json';
const htmlPath = 'tests/fixtures/css/c11/grid-layout.html';
const referencePath = 'tests/fixtures/css/references/c11-grid-precomparison-v2.json';
const fixturePath = 'tests/fixtures/css/c11/mobile-surface-pixel-matrix.json';

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const bytesAt = async (path, overrides) => overrides?.[path] ?? readFile(resolve(repositoryRoot, path));

function assertInterior(image, x, y, rgba, context) {
  assert.ok(x >= 1 && y >= 1 && x + 1 < image.width && y + 1 < image.height,
    `${context}: 3x3 표본 영역이 PNG 경계 밖입니다.`);
  for (let dy = -1; dy <= 1; dy += 1) {
    for (let dx = -1; dx <= 1; dx += 1) {
      assert.deepEqual(image.getPixel(x + dx, y + dy), rgba,
        `${context}: 표본이 색상 경계 또는 anti-alias 영역에 있습니다.`);
    }
  }
}

function collectDataNodeAttributes(root) {
  const nodes = new Map();
  const visit = (node) => {
    const attributes = Object.fromEntries((node.attrs ?? []).map(({ name, value }) => [name, value]));
    if (Object.hasOwn(attributes, 'data-node')) {
      assert.ok(attributes.id, 'data-node element에 id가 없습니다.');
      assert.ok(!nodes.has(attributes.id), `중복 data-node ID입니다: ${attributes.id}`);
      nodes.set(attributes.id, {
        classes: new Set((attributes.class ?? '').split(/\s+/).filter(Boolean)),
        style: attributes.style ?? '',
      });
    }
    for (const child of node.childNodes ?? []) visit(child);
  };
  visit(root);
  return nodes;
}

function centerSample(image, rect, context, id, nodeId) {
  const x = Math.floor(rect.x + rect.width / 2);
  const y = Math.floor(rect.y + rect.height / 2);
  assert.ok(x >= 0 && x < 320 && y >= 0 && y < 240,
    `${context}: 표본이 공통 320x240 CSS 영역 밖입니다.`);
  const rgba = image.getPixel(x, y);
  assertInterior(image, x, y, rgba, context);
  return { id, nodeId, xCssPx: x, yCssPx: y, rgba };
}

function trackBackgroundSample(image, trackNodes, nodeById, rectById, context, usedCoordinates) {
  const expected = [237, 241, 247, 255];
  const candidates = trackNodes
    .map((nodeId) => ({ nodeId, rect: rectById.get(nodeId) }))
    .filter(({ rect }) => rect && rect.width > 0 && rect.height > 0)
    .sort((left, right) => right.rect.width * right.rect.height - left.rect.width * left.rect.height);
  for (const { nodeId, rect } of candidates) {
    const left = Math.max(1, Math.ceil(rect.x));
    const top = Math.max(1, Math.ceil(rect.y));
    const right = Math.min(319, Math.floor(rect.x + rect.width) - 1);
    const bottom = Math.min(239, Math.floor(rect.y + rect.height) - 1);
    const centerX = rect.x + rect.width / 2;
    const centerY = rect.y + rect.height / 2;
    const points = [];
    for (let y = top; y <= bottom; y += 1) {
      for (let x = left; x <= right; x += 1) {
        if (image.getPixel(x, y).every((channel, index) => channel === expected[index])) {
          points.push({ x, y, distance: (x + 0.5 - centerX) ** 2 + (y + 0.5 - centerY) ** 2 });
        }
      }
    }
    points.sort((a, b) => a.distance - b.distance);
    for (const { x, y } of points) {
      const key = `${x},${y}`;
      if (usedCoordinates.has(key)) continue;
      try {
        assertInterior(image, x, y, expected, `${context}/${nodeId}/track-background`);
        return { id: 'track-background', nodeId, xCssPx: x, yCssPx: y, rgba: expected };
      } catch {
        // 다음 내부 좌표를 확인한다. 경계 sample은 fixture에 저장하지 않는다.
      }
    }
  }
  const candidateIDs = candidates.map(({ nodeId }) => nodeId).join(',');
  const known = [...nodeById.keys()].join(',');
  throw new Error(`${context}: 내부 track 배경 표본을 찾지 못했습니다. tracks=${candidateIDs} nodes=${known}`);
}

export async function buildC11MobileSurfacePixelMatrix(overrides = {}) {
  const [inventoryBytes, htmlBytes, referenceBytes] = await Promise.all([
    bytesAt(inventoryPath, overrides), bytesAt(htmlPath, overrides), bytesAt(referencePath, overrides),
  ]);
  const inventory = JSON.parse(inventoryBytes);
  const reference = JSON.parse(referenceBytes);
  const htmlNodes = collectDataNodeAttributes(parse(htmlBytes.toString('utf8')));
  assert.equal(inventory.schema, 'spinon-css-c11-grid-inventory/v1');
  assert.equal(reference.schema, 'spinon-css-c11-grid-precomparison/v1');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  assert.equal(inventory.cases.length, 10);

  const observations = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(observations, 'Chrome DPR 1 observation이 없습니다.');
  assert.deepEqual(observations.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  assert.deepEqual(reference.screenshots.map(({ caseId }) => caseId), inventory.cases.map(({ id }) => id));

  const scenes = [];
  let totalSamples = 0;
  for (let sceneIndex = 0; sceneIndex < inventory.cases.length; sceneIndex += 1) {
    const fixtureCase = inventory.cases[sceneIndex];
    const observation = observations.cases[sceneIndex];
    const screenshot = reference.screenshots[sceneIndex];
    assert.equal(observation.id, fixtureCase.id);
    assert.deepEqual(observation.nodes.map(({ id }) => id), fixtureCase.nodeIds,
      `${fixtureCase.id}: node preorder가 inventory와 다릅니다.`);
    const pngBytes = await bytesAt(screenshot.path, overrides);
    assert.equal(sha256(pngBytes), screenshot.sha256,
      `${fixtureCase.id}: Chrome screenshot SHA-256이 다릅니다.`);
    const image = decodeRgbaPng(pngBytes);
    assert.equal(image.width, inventory.viewport.width, `${fixtureCase.id}: PNG 너비가 다릅니다.`);
    assert.equal(image.height, inventory.viewport.height, `${fixtureCase.id}: PNG 높이가 다릅니다.`);

    const rectById = new Map(observation.nodes.map((node) => [node.id, node.documentRect]));
    const nodeById = new Map(fixtureCase.nodeIds.map((nodeId) => [nodeId, htmlNodes.get(nodeId)]));
    assert.ok([...nodeById.values()].every(Boolean), `${fixtureCase.id}: HTML data-node 속성이 없습니다.`);
    const samples = [];
    const usedCoordinates = new Set();
    for (const nodeId of fixtureCase.nodeIds) {
      const node = nodeById.get(nodeId);
      const rect = rectById.get(nodeId);
      assert.ok(Number.isFinite(rect.x) && Number.isFinite(rect.y)
        && Number.isFinite(rect.width) && Number.isFinite(rect.height),
      `${fixtureCase.id}/${nodeId}: geometry가 유한하지 않습니다.`);
      assert.ok(rect.width > 0 && rect.height > 0,
        `${fixtureCase.id}/${nodeId}: zero-size node는 표본 대상이 아닙니다.`);
      const colorClass = [...node.classes].find((name) =>
        ['blue', 'red', 'green', 'yellow', 'purple', 'cyan', 'orange', 'pink'].includes(name));
      if (colorClass) {
        const sample = centerSample(image, rect, `${fixtureCase.id}/${nodeId}`, `item-${nodeId}`, nodeId);
        const key = `${sample.xCssPx},${sample.yCssPx}`;
        assert.ok(!usedCoordinates.has(key), `${fixtureCase.id}: item 표본 좌표가 중복됩니다: ${key}`);
        usedCoordinates.add(key);
        samples.push(sample);
      }
    }
    const trackNodes = fixtureCase.nodeIds.filter((nodeId) => {
      const node = nodeById.get(nodeId);
      return node.classes.has('track-surface') || /(?:^|;)\s*background-color\s*:\s*#edf1f7\s*(?:;|$)/i.test(node.style);
    });
    assert.ok(trackNodes.length > 0, `${fixtureCase.id}: track 배경 node가 없습니다.`);
    const trackSample = trackBackgroundSample(
      image, trackNodes, nodeById, rectById, fixtureCase.id, usedCoordinates,
    );
    usedCoordinates.add(`${trackSample.xCssPx},${trackSample.yCssPx}`);
    samples.push(trackSample);
    assert.equal(new Set(samples.map(({ id }) => id)).size, samples.length,
      `${fixtureCase.id}: 중복 sample ID가 있습니다.`);
    const coordinates = samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`);
    assert.equal(new Set(coordinates).size, coordinates.length,
      `${fixtureCase.id}: 중복 sample 좌표가 있습니다.`);
    assert.ok(samples.length >= 1 && samples.length <= 64,
      `${fixtureCase.id}: sample 개수가 1–64 범위 밖입니다.`);
    totalSamples += samples.length;
    scenes.push({
      id: fixtureCase.id,
      index: sceneIndex,
      screenshot: { path: screenshot.path, sha256: screenshot.sha256 },
      samples,
    });
  }
  assert.equal(totalSamples, 36);
  return {
    schema: 'spinon-css-c11-mobile-surface-pixel-matrix/v1',
    fixtureId: inventory.fixtureId,
    viewportCssPx: { width: 320, height: 240 },
    oracle: {
      name: reference.oracle.name,
      product: reference.oracle.product,
      referenceId: reference.referenceId,
      referencePath,
      referenceSha256: sha256(referenceBytes),
      inventoryPath,
      inventorySha256: sha256(inventoryBytes),
      htmlPath,
      htmlSha256: sha256(htmlBytes),
      captureRunId: reference.runId,
      dpr: 1,
    },
    summary: { sceneCount: scenes.length, sampleCount: totalSamples },
    scenes,
  };
}

const isDirectExecution = process.argv[1]
  && pathToFileURL(resolve(process.argv[1])).href === import.meta.url;
if (isDirectExecution) {
  const generated = `${JSON.stringify(await buildC11MobileSurfacePixelMatrix(), null, 2)}\n`;
  if (process.argv.includes('--write')) {
    await writeFile(resolve(repositoryRoot, fixturePath), generated);
  } else if (process.argv.includes('--check')) {
    const existing = await readFile(resolve(repositoryRoot, fixturePath), 'utf8');
    assert.equal(existing, generated, 'C11 mobile surface pixel matrix fixture가 stale입니다.');
  } else {
    throw new Error('사용법: node build-c11-mobile-surface-pixel-matrix.mjs --write|--check');
  }
  process.stdout.write(`C11 mobile pixel fixture ${process.argv.includes('--write') ? '작성' : '확인'}: 10 scene · 36 sample\n`);
}
