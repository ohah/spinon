import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { parse } from 'parse5';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-auto-placement-inventory-v3.json',
  html: 'tests/fixtures/css/c11/grid-auto-placement-v3.html',
  runtimeMatrix: 'tests/fixtures/css/c11/grid-auto-placement-runtime-matrix.json',
  reference: 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json',
  generator: 'tools/css-reference/build-c11-2-2-grid-auto-placement-pixel-matrix.mjs',
};
const outputPath = resolve(repositoryRoot,
  'tests/fixtures/css/c11/grid-auto-placement-mobile-surface-pixel-matrix.json');
const runtimeViewportCssPx = { width: 320, height: 240 };
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function rgbaFromHex(value) {
  assert.match(value, /^#[0-9a-f]{6}$/i);
  return [...value.slice(1).matchAll(/../g)].map(([channel]) => Number.parseInt(channel, 16)).concat(255);
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

function attributes(node) {
  return new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
}

function textContent(node) {
  return node.nodeName === '#text'
    ? node.value
    : (node.childNodes ?? []).map(textContent).join('');
}

function parseFixtureHtml(htmlBytes) {
  const errors = [];
  const document = parse(htmlBytes.toString('utf8'), {
    sourceCodeLocationInfo: true,
    onParseError: (error) => errors.push(error.code),
  });
  assert.deepEqual(errors, [], 'C11.2.2 HTML parse error');
  assert.equal(document.mode, 'no-quirks', 'C11.2.2 HTML standards mode');
  const styleNodes = descendants(document, (node) => node.tagName === 'style');
  assert.equal(styleNodes.length, 1, '고정 HTML style 요소 수');
  const css = textContent(styleNodes[0]);
  const palette = new Map([...css.matchAll(/\.([a-z][a-z0-9-]*)\s*\{\s*background-color:\s*(#[0-9a-f]{6})\s*;?\s*\}/gi)]
    .map(([, name, color]) => [name, color.toLowerCase()]));
  assert.deepEqual(palette, new Map([
    ['surface', '#edf1f7'], ['blue', '#2866f6'], ['red', '#e5484d'],
    ['green', '#19a974'], ['yellow', '#ffc857'], ['purple', '#8758ff'],
    ['cyan', '#0ea5e9'], ['orange', '#f97316'], ['pink', '#ec4899'],
  ]), '고정 HTML paint palette가 바뀌었습니다.');
  const nodes = new Map();
  for (const node of descendants(document,
    (candidate) => candidate.tagName && attributes(candidate).has('data-node'))) {
    const attrs = attributes(node);
    const id = attrs.get('id');
    assert.ok(id, 'data-node id 누락');
    assert.equal(nodes.has(id), false, `중복 data-node id: ${id}`);
    nodes.set(id, {
      classes: (attrs.get('class') ?? '').split(/\s+/).filter(Boolean),
      style: attrs.get('style') ?? '',
    });
  }
  return { nodes, palette };
}

function rectContains(rect, x, y) {
  return x + 0.5 >= rect.x && x + 0.5 < rect.x + rect.width
    && y + 0.5 >= rect.y && y + 0.5 < rect.y + rect.height;
}

function isInterior(image, x, y, rgba) {
  if (x < 1 || y < 1 || x + 1 >= image.width || y + 1 >= image.height) return false;
  for (let dy = -1; dy <= 1; dy += 1) {
    for (let dx = -1; dx <= 1; dx += 1) {
      if (!image.getPixel(x + dx, y + dy).every((channel, index) => channel === rgba[index])) return false;
    }
  }
  return true;
}

function nearestInteriorPoint(image, rect, rgba, usedCoordinates, context) {
  const left = Math.max(1, Math.ceil(rect.x));
  const top = Math.max(1, Math.ceil(rect.y));
  const right = Math.min(runtimeViewportCssPx.width - 1, image.width - 2,
    Math.floor(rect.x + rect.width) - 1);
  const bottom = Math.min(runtimeViewportCssPx.height - 1, image.height - 2,
    Math.floor(rect.y + rect.height) - 1);
  const centerX = rect.x + rect.width / 2;
  const centerY = rect.y + rect.height / 2;
  const candidates = [];
  for (let y = top; y <= bottom; y += 1) {
    for (let x = left; x <= right; x += 1) {
      if (usedCoordinates.has(`${x},${y}`) || !rectContains(rect, x, y)) continue;
      if (image.getPixel(x, y).every((channel, index) => channel === rgba[index])
          && isInterior(image, x, y, rgba)) {
        candidates.push({ x, y,
          distance: (x + 0.5 - centerX) ** 2 + (y + 0.5 - centerY) ** 2 });
      }
    }
  }
  candidates.sort((leftPoint, rightPoint) => leftPoint.distance - rightPoint.distance
    || leftPoint.y - rightPoint.y || leftPoint.x - rightPoint.x);
  assert.ok(candidates.length > 0, `${context}: Chrome 기준에서 3×3 내부 표본을 찾지 못했습니다.`);
  const point = candidates[0];
  usedCoordinates.add(`${point.x},${point.y}`);
  return { xCssPx: point.x, yCssPx: point.y };
}

function pxList(value, context) {
  if (typeof value !== 'string' || value === 'none') return [];
  const tokens = value.trim().split(/\s+/);
  const numbers = tokens.map((token) => {
    const match = token.match(/^(\d+(?:\.\d+)?)px$/);
    assert.ok(match, `${context}: 지원하지 않는 track 값 ${token}`);
    return Number(match[1]);
  });
  assert.ok(numbers.length > 0, `${context}: track이 비었습니다.`);
  return numbers;
}

function gapSamples({ scene, chromeCase, htmlNodes, palette, screenshot, usedCoordinates }) {
  const samples = [];
  for (const grid of chromeCase.gridContainers ?? []) {
    const htmlNode = htmlNodes.get(grid.id);
    const backgroundClass = htmlNode?.classes.find((name) => palette.has(name));
    assert.ok(backgroundClass, `${scene.id}/${grid.id}: Grid 배경 색 없음`);
    const rgba = rgbaFromHex(palette.get(backgroundClass));
    const node = chromeCase.nodes.find(({ id }) => id === grid.id);
    assert.ok(node, `${scene.id}/${grid.id}: Chrome Grid rect 없음`);
    const rect = node.rect;
    const columnTracks = pxList(grid.properties['grid-template-columns'],
      `${scene.id}/${grid.id}/grid-template-columns`);
    const rowTracks = pxList(grid.properties['grid-template-rows'],
      `${scene.id}/${grid.id}/grid-template-rows`);
    const columnGap = node.properties['grid-column-gap'] === 'normal'
      ? 0 : Number.parseFloat(node.properties['grid-column-gap']);
    const rowGap = node.properties['grid-row-gap'] === 'normal'
      ? 0 : Number.parseFloat(node.properties['grid-row-gap']);
    assert.ok(Number.isFinite(columnGap) && columnGap >= 0, `${scene.id}/${grid.id}: column gap`);
    assert.ok(Number.isFinite(rowGap) && rowGap >= 0, `${scene.id}/${grid.id}: row gap`);

    if (columnGap > 0 && columnTracks.length > 1 && rowTracks.length > 0) {
      const gapRect = {
        x: rect.x + columnTracks[0],
        y: rect.y + 1,
        width: columnGap,
        height: Math.max(1, rowTracks[0] - 2),
      };
      try {
        const point = nearestInteriorPoint(screenshot, gapRect, rgba, usedCoordinates,
          `${scene.id}/${grid.id}/column-gap`);
        samples.push({ id: `gap-column-${grid.id}`, nodeId: grid.id, kind: 'column-gap', ...point, rgba });
      } catch {
        // Span item이 gap을 덮는 고정 장면은 아래의 보이는 container 표본으로 확인한다.
      }
    }
    if (rowGap > 0 && rowTracks.length > 1 && columnTracks.length > 0) {
      const gapRect = {
        x: rect.x + 1,
        y: rect.y + rowTracks[0],
        width: Math.max(1, columnTracks[0] - 2),
        height: rowGap,
      };
      try {
        const point = nearestInteriorPoint(screenshot, gapRect, rgba, usedCoordinates,
          `${scene.id}/${grid.id}/row-gap`);
        samples.push({ id: `gap-row-${grid.id}`, nodeId: grid.id, kind: 'row-gap', ...point, rgba });
      } catch {
        // Span item이 gap을 덮는 고정 장면은 아래의 보이는 container 표본으로 확인한다.
      }
    }
    if (!samples.some((sample) => sample.nodeId === grid.id)) {
      const point = nearestInteriorPoint(screenshot, rect, rgba, usedCoordinates,
        `${scene.id}/${grid.id}/surface`);
      samples.push({ id: `surface-${grid.id}`, nodeId: grid.id, kind: 'grid-surface', ...point, rgba });
    }
  }
  return samples;
}

function createSceneSamples({ scene, chromeCase, screenshot, htmlNodes, palette }) {
  assert.deepEqual(chromeCase.nodes.map(({ id }) => id), scene.nodeIds,
    `${scene.id}: Chrome node order`);
  const usedCoordinates = new Set();
  const samples = [];
  for (const nodeID of scene.nodeIds) {
    const node = htmlNodes.get(nodeID);
    assert.ok(node, `${scene.id}/${nodeID}: HTML data-node 없음`);
    const chromeNode = chromeCase.nodes.find(({ id }) => id === nodeID);
    assert.ok(chromeNode, `${scene.id}/${nodeID}: Chrome node 없음`);
    if (chromeNode.properties?.display === 'none'
        || chromeNode.rect.width <= 0 || chromeNode.rect.height <= 0) continue;
    const colorClass = node.classes.find((name) => palette.has(name));
    assert.ok(colorClass, `${scene.id}/${nodeID}: paint palette class 없음`);
    if (colorClass === 'surface') continue;
    const rgba = rgbaFromHex(palette.get(colorClass));
    const point = nearestInteriorPoint(screenshot, chromeNode.rect, rgba,
      usedCoordinates, `${scene.id}/${nodeID}/item`);
    samples.push({ id: `item-${nodeID}`, nodeId: nodeID, kind: 'item', ...point, rgba });
  }
  samples.push(...gapSamples({ scene, chromeCase, htmlNodes, palette, screenshot, usedCoordinates }));
  assert.ok(samples.length >= 2 && samples.length <= 64,
    `${scene.id}: item/gap 표본 수가 2–64 범위 밖입니다.`);
  assert.equal(new Set(samples.map(({ id }) => id)).size, samples.length, `${scene.id}: sample ID 중복`);
  assert.equal(new Set(samples.map(({ xCssPx, yCssPx }) => `${xCssPx},${yCssPx}`)).size,
    samples.length, `${scene.id}: sample 좌표 중복`);
  for (const sample of samples) {
    assert.deepEqual(screenshot.getPixel(sample.xCssPx, sample.yCssPx), sample.rgba,
      `${scene.id}/${sample.id}: 저장할 Chrome 기준 RGBA`);
  }
  return samples;
}

export async function buildC1122GridAutoPlacementPixelMatrix(overrides = {}) {
  const inputPaths = {
    inventory: paths.inventory,
    html: paths.html,
    runtimeMatrix: paths.runtimeMatrix,
    reference: paths.reference,
    generator: paths.generator,
  };
  const pairs = await Promise.all(Object.entries(inputPaths).map(async ([key, path]) => [
    key, overrides[path] ?? await readFile(resolve(repositoryRoot, path)),
  ]));
  const bytes = Object.fromEntries(pairs);
  const inventory = JSON.parse(bytes.inventory.toString('utf8'));
  const runtimeMatrix = JSON.parse(bytes.runtimeMatrix.toString('utf8'));
  const reference = JSON.parse(bytes.reference.toString('utf8'));
  const { nodes: htmlNodes, palette } = parseFixtureHtml(bytes.html);
  assert.equal(inventory.schema, 'spinon-css-c11-2-2-grid-auto-placement-inventory/v3');
  assert.equal(runtimeMatrix.schema, 'spinon-css-c11-2-2-grid-auto-placement-runtime-matrix/v1');
  assert.equal(reference.schema, 'spinon-css-c11-2-2-grid-auto-placement-precomparison/v4');
  assert.equal(runtimeMatrix.inputs.inventory.sha256, sha256(bytes.inventory));
  assert.equal(runtimeMatrix.inputs.html.sha256, sha256(bytes.html));
  assert.equal(runtimeMatrix.inputs.chromiumReference.sha256, sha256(bytes.reference));
  assert.equal(reference.fixture.inventorySha256, sha256(bytes.inventory));
  assert.equal(reference.fixture.htmlSha256, sha256(bytes.html));
  assert.equal(runtimeMatrix.summary.sceneCount, 16);
  const dprOne = reference.observations.find(({ deviceScaleFactor }) => deviceScaleFactor === 1);
  assert.ok(dprOne, 'Chrome 154 DPR 1 기준이 없습니다.');
  assert.deepEqual(runtimeMatrix.cases.map(({ id }) => id), dprOne.cases.map(({ id }) => id));

  const cases = [];
  let sampleCount = 0;
  for (const fixtureCase of runtimeMatrix.cases) {
    const chromeCase = dprOne.cases.find(({ id }) => id === fixtureCase.id);
    const screenshotRecord = reference.screenshots.find(({ caseId }) => caseId === fixtureCase.id);
    assert.ok(chromeCase && screenshotRecord, `${fixtureCase.id}: Chrome oracle scene 없음`);
    const screenshotBytes = overrides[screenshotRecord.path]
      ?? await readFile(resolve(repositoryRoot, screenshotRecord.path));
    assert.equal(sha256(screenshotBytes), screenshotRecord.sha256,
      `${fixtureCase.id}: Chrome screenshot hash`);
    const screenshot = decodeRgbaPng(screenshotBytes);
    assert.deepEqual([screenshot.width, screenshot.height], [360, 800],
      `${fixtureCase.id}: Chrome screenshot dimension`);
    const base = {
      id: fixtureCase.id,
      index: fixtureCase.index,
      sourceSha256: fixtureCase.sourceSha256,
      chromeScreenshot: { path: screenshotRecord.path, sha256: screenshotRecord.sha256 },
    };
    if (fixtureCase.expected.outcome === 'expected-rejection') {
      cases.push({ ...base, outcome: 'expected-rejection', expected: fixtureCase.expected });
      continue;
    }
    assert.equal(fixtureCase.expected.outcome, 'success');
    const samples = createSceneSamples({
      scene: fixtureCase, chromeCase, screenshot, htmlNodes, palette,
    });
    sampleCount += samples.length;
    cases.push({ ...base, outcome: 'surface-pixel-compare', nodeIds: fixtureCase.nodeIds, samples });
  }
  const matrix = {
    schema: 'spinon-css-c11-2-2-grid-auto-placement-mobile-surface-pixel-matrix/v1',
    fixtureId: 'c11-2-2-grid-auto-placement-mobile-surface-pixels',
    runtime: {
      viewportCssPx: runtimeViewportCssPx,
      surface: 'Android SurfaceView PixelCopy 또는 iOS WGPU Metal drawable, compositor 제출 전',
      devicePixelFormula: 'floor((cssPx + 0.5) * drawablePixels / runtimeViewportCssPx)',
      maximumChannelError: 0,
    },
    inputs: {
      inventory: { path: paths.inventory, sha256: sha256(bytes.inventory) },
      html: { path: paths.html, sha256: sha256(bytes.html) },
      runtimeMatrix: { path: paths.runtimeMatrix, sha256: sha256(bytes.runtimeMatrix) },
      generator: { path: paths.generator, sha256: sha256(bytes.generator) },
      chromiumReference: {
        path: paths.reference,
        sha256: sha256(bytes.reference),
        referenceId: reference.referenceId,
        product: reference.oracle.product,
      },
    },
    cases,
    summary: {
      sceneCount: 16,
      pixelSceneCount: cases.filter(({ outcome }) => outcome === 'surface-pixel-compare').length,
      expectedRejectCount: cases.filter(({ outcome }) => outcome === 'expected-rejection').length,
      sampleCount,
      hiddenNodesAreNotSampled: true,
      screenshotsArePixelParity: false,
    },
  };
  assert.equal(matrix.summary.pixelSceneCount, 11);
  assert.equal(matrix.summary.expectedRejectCount, 5);
  return matrix;
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? '').href) {
  const generated = await buildC1122GridAutoPlacementPixelMatrix();
  const serialized = `${JSON.stringify(generated, null, 2)}\n`;
  if (process.argv.includes('--write')) {
    await writeFile(outputPath, serialized);
    process.stdout.write(`C11.2.2 Pixel matrix 저장: ${outputPath}\n`);
  } else if (process.argv.includes('--check')) {
    const current = await readFile(outputPath, 'utf8');
    assert.equal(current, serialized, 'C11.2.2 Pixel matrix가 입력 fixture와 다릅니다.');
    process.stdout.write('C11.2.2 Pixel matrix hash·scene·sample 검사 통과\n');
  } else {
    process.stdout.write(`${JSON.stringify(generated.summary)}\n`);
  }
}
