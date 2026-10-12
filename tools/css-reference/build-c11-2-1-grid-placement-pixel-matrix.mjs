import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-placement-inventory.json',
  html: 'tests/fixtures/css/c11/grid-placement.html',
  runtimeMatrix: 'tests/fixtures/css/c11/grid-placement-runtime-matrix.json',
  reference: 'tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json',
  generator: 'tools/css-reference/build-c11-2-1-grid-placement-pixel-matrix.mjs',
};
const outputPath = 'tests/fixtures/css/c11/grid-placement-mobile-surface-pixel-matrix.json';
const runtimeViewportCssPx = { width: 320, height: 240 };
const expectedColors = new Map([
  ['surface', '#edf1f7'], ['blue', '#2866f6'], ['red', '#e5484d'], ['green', '#19a974'],
  ['yellow', '#ffc857'], ['purple', '#8758ff'], ['cyan', '#0ea5e9'], ['orange', '#f97316'],
  ['pink', '#ec4899'],
]);
const intentionallyOccludedNodes = new Set(['c1121-overlap-first']);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

async function inputBytes(path, overrides) {
  return overrides?.[path] ?? readFile(resolve(repositoryRoot, path));
}

function attributes(node) {
  return new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
}

function descendants(root, predicate) {
  const result = [];
  const visit = (node) => {
    if (predicate(node)) result.push(node);
    for (const child of node.childNodes ?? []) visit(child);
  };
  visit(root);
  return result;
}

function textContent(node) {
  return node.nodeName === '#text'
    ? node.value
    : (node.childNodes ?? []).map(textContent).join('');
}

function parsePalette(document) {
  const styleElements = descendants(document, (node) => node.tagName === 'style');
  assert.equal(styleElements.length, 1, '고정 Grid HTML의 style 요소 수');
  const css = textContent(styleElements[0]);
  const palette = new Map([...css.matchAll(/\.([a-z][a-z0-9-]*)\s*\{\s*background-color:\s*(#[0-9a-f]{6})\s*;?\s*\}/gi)]
    .map(([, name, color]) => [name, color.toLowerCase()]));
  assert.deepEqual(palette, expectedColors, 'Grid HTML paint palette가 고정 계약과 다릅니다.');
  return palette;
}

function parseNodes(document) {
  const result = new Map();
  for (const node of descendants(document, (candidate) => candidate.tagName && attributes(candidate).has('data-node'))) {
    const attrs = attributes(node);
    const id = attrs.get('id');
    const classes = (attrs.get('class') ?? '').split(/\s+/).filter(Boolean);
    assert.ok(id && classes.length > 0, 'Grid data-node에 id 또는 class가 없습니다.');
    assert.equal(result.has(id), false, `중복 Grid data-node ID: ${id}`);
    result.set(id, { classes, style: attrs.get('style') ?? '' });
  }
  return result;
}

function rectContains(rect, x, y) {
  return x + 0.5 >= rect.x && x + 0.5 < rect.x + rect.width
    && y + 0.5 >= rect.y && y + 0.5 < rect.y + rect.height;
}

function interior(image, x, y, rgba) {
  if (x < 1 || y < 1 || x + 1 >= image.width || y + 1 >= image.height) return false;
  for (let dy = -1; dy <= 1; dy += 1) {
    for (let dx = -1; dx <= 1; dx += 1) {
      if (!image.getPixel(x + dx, y + dy).every((channel, index) => channel === rgba[index])) return false;
    }
  }
  return true;
}

function nearestInteriorPoint(image, rect, rgba, usedCoordinates, context) {
  const left = Math.max(1, Math.ceil(rect.x), 1);
  const top = Math.max(1, Math.ceil(rect.y), 1);
  const right = Math.min(runtimeViewportCssPx.width - 1, image.width - 2, Math.floor(rect.x + rect.width) - 1);
  const bottom = Math.min(runtimeViewportCssPx.height - 1, image.height - 2, Math.floor(rect.y + rect.height) - 1);
  const centerX = rect.x + rect.width / 2;
  const centerY = rect.y + rect.height / 2;
  const candidates = [];
  for (let y = top; y <= bottom; y += 1) {
    for (let x = left; x <= right; x += 1) {
      const key = `${x},${y}`;
      if (usedCoordinates.has(key) || !rectContains(rect, x, y)) continue;
      if (image.getPixel(x, y).every((channel, index) => channel === rgba[index])
          && interior(image, x, y, rgba)) {
        candidates.push({ x, y, distance: (x + 0.5 - centerX) ** 2 + (y + 0.5 - centerY) ** 2 });
      }
    }
  }
  candidates.sort((leftPoint, rightPoint) => leftPoint.distance - rightPoint.distance
    || leftPoint.y - rightPoint.y || leftPoint.x - rightPoint.x);
  const point = candidates[0];
  assert.ok(point, `${context}: 고정 Chrome screenshot에서 3×3 내부 sample을 찾지 못했습니다.`);
  return { xCssPx: point.x, yCssPx: point.y };
}

function rgbaFromHex(value) {
  return [...value.slice(1).matchAll(/../g)].map(([channel]) => Number.parseInt(channel, 16)).concat(255);
}

function createSceneSamples({ scene, chromeCase, screenshot, htmlNodes, palette }) {
  assert.deepEqual(chromeCase.nodes.map(({ id }) => id), scene.nodeIds, `${scene.id}: Chrome node preorder`);
  const parentById = new Map(scene.nodeIds.map((id, index) => [id, scene.parentIds[index]]));
  assert.deepEqual(chromeCase.nodes.map(({ id, parentId }) => ({ id, parentId })),
    scene.nodeIds.map((id) => ({ id, parentId: parentById.get(id) })), `${scene.id}: Chrome parent graph`);
  const rectById = new Map(chromeCase.nodes.map(({ id, documentRect }) => [id, documentRect]));
  const usedCoordinates = new Set();
  const samples = [];
  const occludedNodeIds = [];

  for (const nodeId of scene.nodeIds) {
    const node = htmlNodes.get(nodeId);
    assert.ok(node, `${scene.id}/${nodeId}: HTML data-node 없음`);
    const rect = rectById.get(nodeId);
    assert.ok(rect && [rect.x, rect.y, rect.width, rect.height].every(Number.isFinite),
      `${scene.id}/${nodeId}: Chrome rect가 유한하지 않습니다.`);
    if (rect.width <= 0 || rect.height <= 0) continue;
    const colorClass = node.classes.find((name) => palette.has(name));
    assert.ok(colorClass, `${scene.id}/${nodeId}: paint color class 없음`);
    if (colorClass === 'surface') continue;
    const rgba = rgbaFromHex(palette.get(colorClass));
    const hasVisibleColor = (() => {
      const left = Math.max(0, Math.ceil(rect.x));
      const top = Math.max(0, Math.ceil(rect.y));
      const right = Math.min(runtimeViewportCssPx.width, screenshot.width, Math.ceil(rect.x + rect.width));
      const bottom = Math.min(runtimeViewportCssPx.height, screenshot.height, Math.ceil(rect.y + rect.height));
      for (let y = top; y < bottom; y += 1) {
        for (let x = left; x < right; x += 1) {
          if (rectContains(rect, x, y)
              && screenshot.getPixel(x, y).every((channel, index) => channel === rgba[index])) return true;
        }
      }
      return false;
    })();
    if (!hasVisibleColor && intentionallyOccludedNodes.has(nodeId)) {
      occludedNodeIds.push(nodeId);
      continue;
    }
    assert.ok(hasVisibleColor, `${scene.id}/${nodeId}: 예상 paint color가 Chrome scene에 없습니다.`);
    const point = nearestInteriorPoint(screenshot, rect, rgba, usedCoordinates, `${scene.id}/${nodeId}`);
    const key = `${point.xCssPx},${point.yCssPx}`;
    usedCoordinates.add(key);
    samples.push({ id: `item-${nodeId}`, nodeId, ...point, rgba });
  }

  const surfaceNodes = scene.nodeIds.filter((nodeId) => htmlNodes.get(nodeId)?.classes.includes('surface'));
  for (const nodeId of surfaceNodes) {
    const rect = rectById.get(nodeId);
    if (!rect || rect.width <= 0 || rect.height <= 0) continue;
    const rgba = rgbaFromHex(palette.get('surface'));
    try {
      const point = nearestInteriorPoint(screenshot, rect, rgba, usedCoordinates, `${scene.id}/${nodeId}/surface`);
      usedCoordinates.add(`${point.xCssPx},${point.yCssPx}`);
      samples.push({ id: `surface-${nodeId}`, nodeId, ...point, rgba });
    } catch {
      // 덮인 container background에는 보이는 표면 sample이 없을 수 있다.
    }
  }

  assert.ok(samples.length >= 1 && samples.length <= 64, `${scene.id}: sample 수가 1–64 범위 밖입니다.`);
  assert.equal(new Set(samples.map(({ id }) => id)).size, samples.length, `${scene.id}: sample ID 중복`);
  assert.equal(new Set(samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
    samples.length, `${scene.id}: sample coordinate 중복`);
  assert.deepEqual(occludedNodeIds,
    scene.id === 'overlap-source-order' ? ['c1121-overlap-first'] : [],
    `${scene.id}: 가려진 node 집합이 고정 계약과 다릅니다.`);
  return { samples, occludedNodeIds };
}

export async function buildC1121GridPlacementPixelMatrix(overrides = {}) {
  const entries = await Promise.all(Object.entries(paths).map(async ([name, path]) => [
    name, await inputBytes(path, overrides),
  ]));
  const bytes = Object.fromEntries(entries);
  const inventory = JSON.parse(bytes.inventory.toString('utf8'));
  const htmlText = bytes.html.toString('utf8');
  const runtimeMatrix = JSON.parse(bytes.runtimeMatrix.toString('utf8'));
  const reference = JSON.parse(bytes.reference.toString('utf8'));
  const parseErrors = [];
  const document = parse(htmlText, { sourceCodeLocationInfo: true, onParseError: (error) => parseErrors.push(error) });
  assert.deepEqual(parseErrors, [], 'Grid HTML parse error');
  assert.equal(document.mode, 'no-quirks', 'Grid HTML이 standards mode가 아닙니다.');
  const palette = parsePalette(document);
  const htmlNodes = parseNodes(document);
  assert.equal(inventory.schema, 'spinon-css-c11-2-1-grid-placement-inventory/v1');
  assert.equal(runtimeMatrix.schema, 'spinon-css-c11-2-1-grid-placement-runtime-matrix/v1');
  assert.equal(reference.schema, 'spinon-css-c11-2-1-grid-placement-precomparison/v1');
  assert.equal(reference.fixture.inventorySha256, sha256(bytes.inventory));
  assert.equal(reference.fixture.htmlSha256, sha256(bytes.html));
  assert.equal(runtimeMatrix.inputs.inventory.sha256, sha256(bytes.inventory));
  assert.equal(runtimeMatrix.inputs.html.sha256, sha256(bytes.html));
  assert.equal(runtimeMatrix.inputs.chromiumReference.sha256, sha256(bytes.reference));
  assert.equal(runtimeMatrix.summary.sceneCount, 10);
  assert.equal(runtimeMatrix.summary.candidateCount, 6);
  assert.equal(runtimeMatrix.summary.expectedRejectCount, 4);
  const dprOne = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(dprOne, 'pinned Chrome DPR 1 observation이 없습니다.');
  assert.deepEqual(runtimeMatrix.cases.map(({ id }) => id), inventory.cases.map(({ id }) => id));
  assert.deepEqual(runtimeMatrix.cases.map(({ id }) => id), dprOne.cases.map(({ id }) => id));
  assert.equal(reference.environment.viewportCssPx.width, 360);
  assert.equal(reference.environment.viewportCssPx.height, 800);

  const htmlBytes = bytes.html;
  const cases = [];
  let sampleCount = 0;
  for (const scene of runtimeMatrix.cases) {
    const chromeCase = dprOne.cases.find(({ id }) => id === scene.id);
    const screenshotRecord = reference.screenshots.find(({ caseId }) => caseId === scene.id);
    assert.ok(chromeCase && screenshotRecord, `${scene.id}: Chrome oracle 연결 누락`);
    const screenshotBytes = await inputBytes(screenshotRecord.path, overrides);
    assert.equal(sha256(screenshotBytes), screenshotRecord.sha256, `${scene.id}: Chrome screenshot hash`);
    const { decodeRgbaPng } = await import('./read-png-rgba.mjs');
    const screenshot = decodeRgbaPng(screenshotBytes);
    assert.deepEqual([screenshot.width, screenshot.height], [360, 800], `${scene.id}: Chrome screenshot size`);
    const base = {
      id: scene.id,
      index: scene.index,
      sourceSha256: scene.sourceSha256,
      chromeScreenshot: { path: screenshotRecord.path, sha256: screenshotRecord.sha256 },
    };
    if (scene.expected.outcome === 'expected-rejection') {
      cases.push({ ...base, outcome: 'expected-rejection', expected: scene.expected });
      continue;
    }
    assert.equal(scene.expected.outcome, 'success', `${scene.id}: 알 수 없는 runtime outcome`);
    assert.ok(scene.nodeIds.every((nodeId) => htmlNodes.has(nodeId)), `${scene.id}: HTML node 누락`);
    const sampled = createSceneSamples({ scene, chromeCase, screenshot, htmlNodes, palette });
    sampleCount += sampled.samples.length;
    cases.push({ ...base, outcome: 'surface-pixel-compare', nodeIds: scene.nodeIds,
      ...sampled });
  }
  const generatorSha256 = sha256(bytes.generator);
  assert.equal(cases.length, 10);
  assert.equal(cases.filter(({ outcome }) => outcome === 'surface-pixel-compare').length, 6);
  assert.equal(cases.filter(({ outcome }) => outcome === 'expected-rejection').length, 4);
  return {
    schema: 'spinon-css-c11-2-1-mobile-surface-pixel-matrix/v1',
    fixtureId: 'c11-2-1-grid-placement-mobile-surface-pixels',
    runtime: {
      viewportCssPx: runtimeViewportCssPx,
      surface: 'Android SurfaceView PixelCopy 또는 iOS WGPU Metal drawable, compositor 제출 전',
      devicePixelFormula: 'floor((cssPx + 0.5) * drawablePixels / runtimeViewportCssPx)',
      maximumChannelError: 0,
    },
    inputs: {
      inventory: { path: paths.inventory, sha256: sha256(bytes.inventory) },
      html: { path: paths.html, sha256: sha256(htmlBytes) },
      runtimeMatrix: { path: paths.runtimeMatrix, sha256: sha256(bytes.runtimeMatrix) },
      generator: { path: paths.generator, sha256: generatorSha256 },
    },
    oracle: {
      name: reference.oracle.name,
      product: reference.oracle.product,
      referenceId: reference.referenceId,
      referencePath: paths.reference,
      referenceSha256: sha256(bytes.reference),
      viewportCssPx: reference.environment.viewportCssPx,
      deviceScaleFactor: 1,
    },
    summary: { sceneCount: cases.length, pixelSceneCount: 6, expectedRejectCount: 4, sampleCount },
    cases,
  };
}

const direct = process.argv[1] && pathToFileURL(resolve(process.argv[1])).href === import.meta.url;
if (direct) {
  const result = `${JSON.stringify(await buildC1121GridPlacementPixelMatrix(), null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--write')) {
    await writeFile(target, result);
  } else if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), result,
      `${outputPath}가 pinned Chrome/fixture 입력으로 재생성한 결과와 다릅니다.`);
  } else {
    throw new Error('사용법: node build-c11-2-1-grid-placement-pixel-matrix.mjs --write|--check');
  }
  process.stdout.write(`C11.2.1 pixel matrix ${process.argv.includes('--write') ? '작성' : '확인'}: 10 scene · 6 pixel scenes\n`);
}
