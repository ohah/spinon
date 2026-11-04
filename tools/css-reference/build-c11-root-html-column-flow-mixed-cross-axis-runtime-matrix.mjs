import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const expectedPath = 'spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/expected.json';
const capturePath = 'spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/capture.json';
const screenshotPath = 'spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/chrome-156-agent-browser.png';
const runtimePath = 'tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime.html';
const rejectionRuntimePath = 'tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-implicit-boundary-runtime.html';
const outputPath = 'tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime-matrix-v1.json';
const generatorPath = 'tools/css-reference/build-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix.mjs';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

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
  return Object.fromEntries((node.attrs ?? []).map(({ name, value }) => [name, value]));
}

function inlineStyle(node) {
  return Object.fromEntries((attributes(node).style ?? '').split(';').flatMap((declaration) => {
    const separator = declaration.indexOf(':');
    return separator < 0 ? [] : [[declaration.slice(0, separator).trim().toLowerCase(),
      declaration.slice(separator + 1).trim().toLowerCase()]];
  }));
}

function colorRgba(value, context) {
  assert.match(value ?? '', /^#[0-9a-f]{6}$/i, `${context}: 고정 6자리 sRGB 색상이 아닙니다.`);
  return [1, 3, 5].map((offset) => Number.parseInt(value.slice(offset, offset + 2), 16))
    .concat(255);
}

function rectObject(rect) {
  assert.ok(Array.isArray(rect) && rect.length === 4 && rect.every(Number.isFinite));
  const [x, y, width, height] = rect;
  return { x, y, width, height };
}

export async function buildColumnFlowMixedCrossAxisRuntimeMatrix() {
  const [expectedBytes, captureBytes, screenshotBytes, runtimeBytes, rejectionRuntimeBytes, generatorBytes] = await Promise.all([
    readFile(resolve(repositoryRoot, expectedPath)),
    readFile(resolve(repositoryRoot, capturePath)),
    readFile(resolve(repositoryRoot, screenshotPath)),
    readFile(resolve(repositoryRoot, runtimePath)),
    readFile(resolve(repositoryRoot, rejectionRuntimePath)),
    readFile(resolve(repositoryRoot, generatorPath)),
  ]);
  const expected = JSON.parse(expectedBytes.toString('utf8'));
  const capture = JSON.parse(captureBytes.toString('utf8'));
  assert.equal(expected.spec.section, '8.5');
  assert.equal(expected.independentModel.flow, 'column');
  assert.equal(expected.independentModel.frameToleranceCssPx, 0);
  assert.match(capture.browser, /HeadlessChrome\/156\.0\.0\.0/);
  assert.deepEqual(capture.viewportCssPx, [1280, 633]);
  assert.equal(capture.devicePixelRatio, 1);
  assert.deepEqual([...screenshotBytes.subarray(0, 8)],
    [137, 80, 78, 71, 13, 10, 26, 10], 'Chrome oracle screenshot PNG signature');
  assert.equal(capture.scenes.length, 9);
  assert.equal(expected.scenes.length, capture.scenes.length);

  const parseErrors = [];
  const documents = [runtimeBytes, rejectionRuntimeBytes].map((bytes) => parse(bytes.toString('utf8'), {
    onParseError: (error) => parseErrors.push(error.code),
  }));
  assert.deepEqual(parseErrors, [], 'runtime fixture HTML parse errors');
  const gridsById = new Map(documents.flatMap((document) =>
    descendants(document, (node) => node.tagName === 'div'))
    .map((node) => [attributes(node).id, node]).filter(([id]) => id));

  const scenes = expected.scenes.map((expectedScene, index) => {
    const chromeScene = capture.scenes[index];
    assert.equal(chromeScene.id, expectedScene.id, `scene order ${index}`);
    assert.deepEqual(chromeScene.rootCssPx, expectedScene.rootCssPx,
      `${expectedScene.id}: Chrome/independent root size`);
    assert.equal(chromeScene.autoFlow, 'column');
    const grid = gridsById.get(expectedScene.productDisposition === 'positive'
      ? `grid-${expectedScene.id}` : expectedScene.id);
    assert.ok(grid, `${expectedScene.id}: runtime grid source missing`);
    const gridStyle = inlineStyle(grid);
    assert.equal(gridStyle.display, 'grid');
    assert.equal(gridStyle['grid-auto-flow'], 'column');
    const sourceItems = (grid.childNodes ?? []).filter((node) => node.tagName === 'div');
    assert.equal(sourceItems.length, chromeScene.items.length,
      `${expectedScene.id}: runtime/Chrome direct item count`);

    const expectedFrames = expectedScene.framesCssPx ?? {};
    const items = chromeScene.items.map((chromeItem, itemIndex) => {
      const source = sourceItems[itemIndex];
      const sourceAttributes = attributes(source);
      const sourceStyle = inlineStyle(source);
      const prefix = `${expectedScene.id}--`;
      assert.ok(sourceAttributes.id?.startsWith(prefix),
        `${expectedScene.id}/${chromeItem.id}: source DOM order/id`);
      const sourceID = sourceAttributes.id.slice(prefix.length);
      assert.equal(sourceID, chromeItem.id, `${expectedScene.id}: source/Chrome item order`);
      const display = sourceStyle.display ?? 'block';
      const visibility = sourceStyle.visibility ?? 'visible';
      assert.equal(display, chromeItem.display, `${expectedScene.id}/${sourceID}: display`);
      assert.equal(visibility, chromeItem.visibility, `${expectedScene.id}/${sourceID}: visibility`);
      const positive = expectedScene.productDisposition === 'positive';
      const expectedChromeRect = positive
        ? expectedFrames[sourceID] ?? null
        : expectedScene.chromeFramesCssPx?.[sourceID] ?? null;
      assert.deepEqual(chromeItem.frameCssPx, expectedChromeRect,
        `${expectedScene.id}/${sourceID}: Chrome/independent frame`);
      const expectedRect = positive ? expectedFrames[sourceID] ?? null : null;
      if (display === 'none') assert.equal(expectedRect, null,
        `${expectedScene.id}/${sourceID}: display:none frame`);
      else if (positive) assert.ok(expectedRect,
        `${expectedScene.id}/${sourceID}: participating item frame`);
      const item = {
        id: sourceID,
        htmlId: sourceAttributes.id,
        display,
        visibility,
        frameCssPx: expectedRect,
        chromeFrameCssPx: chromeItem.frameCssPx,
      };
      if (expectedScene.productDisposition === 'positive' && expectedRect) {
        const rect = rectObject(expectedRect);
        const rgba = visibility === 'hidden'
          ? [255, 255, 255, 255]
          : colorRgba(sourceStyle['background-color'], `${expectedScene.id}/${sourceID}`);
        const xCssPx = Math.floor(rect.x + rect.width / 2);
        const yCssPx = Math.floor(rect.y + rect.height / 2);
        assert.ok(xCssPx >= 0 && xCssPx < expectedScene.rootCssPx[0]
          && yCssPx >= 0 && yCssPx < expectedScene.rootCssPx[1],
        `${expectedScene.id}/${sourceID}: center sample outside viewport`);
        item.pixelSample = {
          id: `${expectedScene.id}-${sourceID}-center`,
          nodeId: sourceAttributes.id,
          xCssPx,
          yCssPx,
          rgba,
          visibilityExpectation: visibility === 'hidden'
            ? 'transparent-over-white-body' : 'authored-solid-srgb-background',
        };
      }
      return item;
    });

    const positive = expectedScene.productDisposition === 'positive';
    if (positive) {
      assert.deepEqual(Object.keys(expectedFrames).sort(),
        items.filter(({ frameCssPx }) => frameCssPx).map(({ id }) => id).sort(),
        `${expectedScene.id}: every and only participating item has a frame`);
    } else {
      assert.equal(expectedScene.id, 'implicit-boundary-control');
      assert.deepEqual(expectedScene.chromeResolvedTracks.columns, [40, 40, 40, 40, 40]);
      assert.deepEqual(expectedScene.chromeResolvedTracks.rows, [30, 30, 30, 30]);
    }
    return {
      id: expectedScene.id,
      outcome: positive ? 'success' : 'expected-rejection',
      viewportCssPx: { width: expectedScene.rootCssPx[0], height: expectedScene.rootCssPx[1] },
      expectedRootCssPx: expectedScene.rootCssPx,
      expectedTracks: expectedScene.tracks ?? expectedScene.explicitTracks,
      expectedFramesCssPx: expectedFrames,
      expectedNonParticipating: expectedScene.nonParticipating ?? [],
      expectedDiagnostic: positive ? null : {
        code: 'invalid_grid_placement',
        property: 'grid-row-start',
        nodeId: 16,
      },
      items,
      pixelSamples: items.flatMap(({ pixelSample }) => pixelSample ? [pixelSample] : []),
      chromeReason: expectedScene.reason,
    };
  });
  const successfulScenes = scenes.filter(({ outcome }) => outcome === 'success').length;
  const rejectedScenes = scenes.length - successfulScenes;
  const participatingItems = scenes.flatMap(({ items }) => items)
    .filter(({ frameCssPx }) => frameCssPx).length;
  const pixelSamples = scenes.reduce((sum, scene) => sum + scene.pixelSamples.length, 0);
  assert.equal(successfulScenes, 8);
  assert.equal(rejectedScenes, 1);
  assert.equal(participatingItems, 31);
  assert.equal(pixelSamples, 31);
  return {
    schema: 'spinon-css-c11-root-html-column-flow-mixed-cross-axis-runtime-matrix/v1',
    profileId: 'C11.1-RootHtmlColumnFlowMixedCrossAxisV1',
    inputs: {
      expected: { path: expectedPath, sha256: sha256(expectedBytes) },
      chromeCapture: { path: capturePath, sha256: sha256(captureBytes),
        browser: capture.browser, devicePixelRatio: capture.devicePixelRatio },
      chromeScreenshot: { path: screenshotPath, sha256: sha256(screenshotBytes) },
      runtimeFixture: { path: runtimePath, sha256: sha256(runtimeBytes) },
      rejectionFixture: { path: rejectionRuntimePath, sha256: sha256(rejectionRuntimeBytes) },
      generator: { path: generatorPath, sha256: sha256(generatorBytes) },
    },
    spec: expected.spec,
    viewportCssPx: { width: 160, height: 90 },
    scenes,
    summary: {
      sceneCount: scenes.length,
      successfulScenes,
      expectedRejectScenes: rejectedScenes,
      participatingItems,
      pixelSampleCount: pixelSamples,
      geometryToleranceCssPx: 0,
    },
  };
}

async function main() {
  const matrix = await buildColumnFlowMixedCrossAxisRuntimeMatrix();
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output,
      `${outputPath} differs from its pinned inputs.`);
  } else {
    await writeFile(target, output);
  }
  process.stdout.write(`Root HTML mixed column-flow matrix: 9 scenes · 8 success · 1 expected reject · 31 items · 31 pixel samples · ${process.argv.includes('--check') ? 'verified' : 'generated'}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
