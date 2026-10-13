import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json';
const htmlPath = 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html';
const referencePath = 'tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json';
const outputPath = 'tests/fixtures/css/c11/grid-auto-placement-c1123-runtime-matrix-v1.json';
const generatorPath = 'tools/css-reference/build-c11-2-3-grid-auto-placement-runtime-matrix.mjs';
const wrapperStyle = 'display:block;position:relative;width:360px;height:800px';
const expectedRejections = new Map([
  ['row-dense-fixed-row', {
    code: 'unsupported_grid_auto_columns',
    nodeIdSuffix: 'grid',
    property: 'grid-auto-columns',
  }],
  ['reject-repeated-auto-track', {
    code: 'unsupported_computed_value',
    nodeIdSuffix: 'grid',
    property: 'grid-auto-rows',
  }],
  ['reject-negative-implicit-line', {
    code: 'invalid_grid_placement',
    nodeIdSuffix: 'negative-line',
    property: 'grid-column-start',
  }],
  ['reject-nonzero-order', {
    code: 'unsupported_computed_value',
    nodeIdSuffix: 'order-two',
    property: 'order',
  }],
  ['reject-indefinite-axis', {
    code: 'unsupported_computed_value',
    nodeIdSuffix: 'grid',
    property: 'height',
  }],
  ['auto-span-over-track-limit', {
    code: 'grid_track_limit_exceeded',
    nodeIdSuffix: 'grid',
    property: 'grid-template-columns',
  }],
]);
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

function htmlPaintColors(htmlBytes) {
  const errors = [];
  const document = parse(htmlBytes.toString('utf8'), {
    onParseError: (error) => errors.push(error.code),
  });
  assert.deepEqual(errors, [], 'C11.2.3 HTML parse error');
  assert.equal(document.mode, 'no-quirks', 'C11.2.3 HTML standards mode');
  const attributes = (node) => new Map((node.attrs ?? []).map(({ name, value }) => [name, value]));
  const styleElements = descendants(document, (node) => node.tagName === 'style');
  assert.equal(styleElements.length, 1, '고정 HTML style 요소 수');
  const text = (node) => node.nodeName === '#text'
    ? node.value
    : (node.childNodes ?? []).map(text).join('');
  const palette = new Map([...text(styleElements[0])
    .matchAll(/\.([a-z][a-z0-9-]*)\s*\{\s*background(?:-color)?:\s*(#[0-9a-f]{6})\s*;?\s*\}/gi)]
    .map(([, name, color]) => [name, color.toLowerCase()]));
  assert.deepEqual(palette, new Map([
    ['surface', '#edf1f7'], ['blue', '#2866f6'], ['red', '#e5484d'],
    ['green', '#19a974'], ['yellow', '#ffc857'], ['purple', '#8758ff'],
    ['cyan', '#0ea5e9'], ['orange', '#f97316'], ['pink', '#ec4899'],
  ]), '고정 HTML paint palette가 바뀌었습니다.');
  const colors = new Map();
  for (const node of descendants(document, (candidate) => candidate.tagName
    && attributes(candidate).has('data-node'))) {
    const attrs = attributes(node);
    const id = attrs.get('id');
    assert.ok(id, 'data-node id 누락');
    assert.equal(colors.has(id), false, `중복 data-node id: ${id}`);
    const colorClass = (attrs.get('class') ?? '').split(/\s+/)
      .find((name) => palette.has(name));
    assert.ok(colorClass, `${id}: 고정 HTML paint color class 없음`);
    colors.set(id, palette.get(colorClass));
  }
  return colors;
}

function runtimeSource(scene, paintColors) {
  const lines = [
    '(() => {',
    'const refs = Object.create(null);',
    'const nodeIds = [];',
    'const sceneRoot = document.createElement("div");',
    'sceneRoot.setAttribute("id", "c1123-runtime-scene-root");',
    `sceneRoot.setAttribute("style", ${JSON.stringify(wrapperStyle)});`,
    'document.appendChild(sceneRoot);',
  ];
  for (const [nodeIndex, node] of scene.nodes.entries()) {
    const variable = `node_${nodeIndex}`;
    const color = paintColors.get(node.id);
    assert.ok(color, `${scene.id}/${node.id}: 고정 HTML paint color 없음`);
    const style = node.authoredStyle
      ? `${node.authoredStyle};background-color:${color}`
      : `background-color:${color}`;
    lines.push(`const ${variable} = document.createElement("div");`);
    lines.push(`${variable}.setAttribute("id", ${JSON.stringify(node.id)});`);
    lines.push(`${variable}.setAttribute("style", ${JSON.stringify(style)});`);
    const parent = node.parentId === null ? 'sceneRoot' : `refs[${JSON.stringify(node.parentId)}]`;
    if (node.parentId !== null) {
      assert.ok(scene.nodes.slice(0, nodeIndex).some(({ id }) => id === node.parentId),
        `${scene.id}/${node.id}: parent가 preorder 앞에 있어야 합니다.`);
    }
    lines.push(`${parent}.appendChild(${variable});`);
    lines.push(`refs[${JSON.stringify(node.id)}] = ${variable};`);
    lines.push(`nodeIds.push(${JSON.stringify(node.id)});`);
  }
  lines.push(`return { sceneId:${JSON.stringify(scene.id)}, rootId:${JSON.stringify(scene.rootId)}, nodeIds };`);
  lines.push('})()');
  const source = lines.join('\n');
  assert.ok(source.length > 0 && source.length <= 64 * 1024, `${scene.id}: JS source 길이`);
  return source;
}

function expected(scene) {
  const rejected = expectedRejections.get(scene.id);
  if (!rejected) return { outcome: 'success' };
  const nodeId = scene.nodes.find(({ id }) => id.endsWith(rejected.nodeIdSuffix))?.id;
  assert.ok(nodeId, `${scene.id}: 예상 오류 node를 찾지 못했습니다.`);
  return {
    outcome: 'expected-rejection',
    code: rejected.code,
    nodeId,
    property: rejected.property,
    runtimeNodeId: scene.nodes.findIndex(({ id }) => id === nodeId) + 2,
  };
}

export async function buildC1123GridAutoPlacementRuntimeMatrix() {
  const inventoryBytes = await readFile(resolve(repositoryRoot, inventoryPath));
  const htmlBytes = await readFile(resolve(repositoryRoot, htmlPath));
  const referenceBytes = await readFile(resolve(repositoryRoot, referencePath));
  const generatorBytes = await readFile(resolve(repositoryRoot, generatorPath));
  const inventory = JSON.parse(inventoryBytes.toString('utf8'));
  const reference = JSON.parse(referenceBytes.toString('utf8'));
  assert.equal(inventory.schema, 'spinon-css-c11-2-3-grid-auto-placement-inventory/v6');
  assert.equal(inventory.fixtureId, 'C11-2-3-grid-auto-placement-precomparison-v6');
  assert.equal(inventory.profileId, 'RuntimeBlockFlexGridAutoPlacementDenseV1');
  assert.equal(reference.schema, 'spinon-css-c11-2-3-grid-auto-placement-precomparison/v7');
  assert.equal(reference.fixture.inventorySha256, sha256(inventoryBytes));
  assert.equal(reference.fixture.htmlPath, htmlPath);
  assert.equal(reference.fixture.htmlSha256, sha256(htmlBytes));
  const paintColors = htmlPaintColors(htmlBytes);
  assert.equal(reference.observations.length, 2);
  const chromeScenes = reference.observations
    .find(({ deviceScaleFactor }) => deviceScaleFactor === 1)?.cases;
  assert.ok(chromeScenes);
  assert.equal(chromeScenes.length, 25);
  assert.deepEqual(chromeScenes.map(({ id }) => id), inventory.cases.map(({ id }) => id));

  const cases = chromeScenes.map((scene, index) => {
    assert.equal(scene.scope, inventory.cases[index].scope);
    assert.equal(scene.rootId, inventory.cases[index].rootId);
    assert.deepEqual(scene.nodes.map(({ id }) => id), inventory.cases[index].nodeIds);
    const source = runtimeSource(scene, paintColors);
    return {
      index,
      id: scene.id,
      scope: scene.scope,
      rootId: scene.rootId,
      nodeIds: scene.nodes.map(({ id }) => id),
      parentIds: scene.nodes.map(({ parentId }) => parentId),
      expected: expected(scene),
      source,
      sourceSha256: sha256(source),
    };
  });
  const nodeCount = cases.reduce((count, scene) => count + scene.nodeIds.length, 0);
  const successCount = cases.filter(({ expected: result }) => result.outcome === 'success').length;
  const expectedRejectCount = cases.length - successCount;
  assert.equal(nodeCount, 107);
  assert.equal(successCount, 19);
  assert.equal(expectedRejectCount, 6);

  return {
    schema: 'spinon-css-c11-2-3-grid-auto-placement-runtime-matrix/v1',
    profileId: inventory.profileId,
    inputs: {
      inventory: { path: inventoryPath, sha256: sha256(inventoryBytes) },
      html: { path: htmlPath, sha256: sha256(htmlBytes) },
      chromiumReference: {
        path: referencePath,
        sha256: sha256(referenceBytes),
        referenceId: reference.referenceId,
      },
      generator: { path: generatorPath, sha256: sha256(generatorBytes) },
    },
    viewportCssPx: inventory.viewport,
    wrapperStyle,
    cases,
    summary: {
      sceneCount: 25,
      implementationCandidateCount: 21,
      oracleOnlyCount: 4,
      successCount,
      expectedRejectCount,
      nodeCount,
    },
  };
}

async function main() {
  const matrix = await buildC1123GridAutoPlacementRuntimeMatrix();
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output,
      `${outputPath}가 고정 inventory와 Chromium reference에서 재생성한 결과와 다릅니다.`);
  } else {
    await writeFile(target, output);
  }
  process.stdout.write(`C11.2.3 runtime matrix: 25 scene · 19 success · 6 expected reject · 107 node · ${process.argv.includes('--check') ? '확인' : '생성'}\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    process.stderr.write(`${error.stack ?? error}\n`);
    process.exitCode = 1;
  });
}
