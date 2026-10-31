import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const referenceHtmlPath = 'spec/internal/evidence/c11-2-2-grid-item-order-precomparison-2026-10-31/chrome-order.html';
const referenceObservationPath = 'spec/internal/evidence/c11-2-2-grid-item-order-precomparison-2026-10-31/chrome-observation.json';
const outputPath = 'tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json';
const colors = { a: '#2866f6', b: '#e5484d', c: '#2b9a66', d: '#f0ae00' };
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function caseDefinition(id, top, letters) {
  const rootId = `c1122-order-${id}-grid`;
  const items = letters.map(({ letter, order }) => ({
    id: `c1122-order-${id}-${letter}`,
    letter,
    order,
    style: `display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0;border:0;order:${order};background-color:${colors[letter]}`,
  }));
  const rootStyle = `position:absolute;left:0;top:${top}px;display:grid;box-sizing:border-box;width:30px;height:10px;grid-template-columns:10px 10px 10px;grid-template-rows:10px;grid-auto-rows:10px;grid-auto-flow:row;column-gap:0;row-gap:0;margin:0;padding:0;border:0;background-color:#edf1f7`;
  return {
    id: `order-${id}`,
    rootId,
    nodes: [
      { id: rootId, parentId: null, style: rootStyle },
      ...items.map(({ id: nodeId, style }) => ({ id: nodeId, parentId: rootId, style })),
    ],
    items,
  };
}

function makeSource(scene) {
  const lines = [
    '(() => {',
    'const refs = Object.create(null);',
    'const nodeIds = [];',
    'const sceneRoot = document.createElement("div");',
    'sceneRoot.setAttribute("id", "c1122-runtime-scene-root");',
    'sceneRoot.setAttribute("style", "display:flex;width:360px;height:800px");',
    'sceneRoot.setAttribute("data-c1122-grid-wrapper", "");',
    'document.appendChild(sceneRoot);',
  ];
  for (const [index, node] of scene.nodes.entries()) {
    const variable = `node_${index}`;
    lines.push(`const ${variable} = document.createElement("div");`);
    lines.push(`${variable}.setAttribute("id", ${JSON.stringify(node.id)});`);
    lines.push(`${variable}.setAttribute("data-c1122-grid-node", "");`);
    lines.push(`${variable}.setAttribute("style", ${JSON.stringify(node.style)});`);
    const parent = node.parentId === null ? 'sceneRoot' : `refs[${JSON.stringify(node.parentId)}]`;
    lines.push(`${parent}.appendChild(${variable});`);
    lines.push(`refs[${JSON.stringify(node.id)}] = ${variable};`);
    lines.push(`nodeIds.push(${JSON.stringify(node.id)});`);
  }
  lines.push(`return { sceneId:${JSON.stringify(scene.id)}, rootId:${JSON.stringify(scene.rootId)}, nodeIds };`);
  lines.push('})()');
  return lines.join('\n');
}

export async function buildC1122GridItemOrderRuntimeMatrix() {
  const [htmlBytes, observationBytes] = await Promise.all([
    readFile(resolve(repositoryRoot, referenceHtmlPath)),
    readFile(resolve(repositoryRoot, referenceObservationPath)),
  ]);
  const observation = JSON.parse(observationBytes.toString('utf8'));
  assert.equal(observation.browser, 'HeadlessChrome/156.0.0.0');
  assert.equal(sha256(htmlBytes), observation.input.sha256);
  assert.deepEqual(observation.cases.map(({ id }) => id), ['mixed', 'ties']);

  const definitions = [
    caseDefinition('mixed', 0, [
      { letter: 'a', order: 2 }, { letter: 'b', order: -1 }, { letter: 'c', order: 0 },
    ]),
    caseDefinition('ties', 20, [
      { letter: 'a', order: 0 }, { letter: 'b', order: -1 },
      { letter: 'c', order: -1 }, { letter: 'd', order: 0 },
    ]),
  ];
  const cases = definitions.map((scene, index) => {
    const oracleCase = observation.cases[index];
    assert.equal(oracleCase.id, scene.id.slice('order-'.length));
    const expectedItems = oracleCase.itemsInDomOrder.map((item) => {
      const letter = item.className.split(' ').at(-1);
      const node = scene.items.find(({ letter: candidate }) => candidate === letter);
      assert.ok(node, `${scene.id}/${letter}: DOM item node가 없습니다.`);
      assert.equal(item.order, String(node.order));
      return { id: node.id, rect: { x: item.x, y: item.y, width: item.width, height: item.height } };
    });
    return {
      index,
      id: scene.id,
      rootId: scene.rootId,
      nodes: scene.nodes.map(({ id, parentId, style }) => ({ id, parentId, authoredStyle: style })),
      nodeIds: scene.nodes.map(({ id }) => id),
      parentIds: scene.nodes.map(({ parentId }) => parentId),
      domItemOrder: scene.items.map(({ id }) => id),
      computedOrders: Object.fromEntries(scene.items.map(({ id, order }) => [id, String(order)])),
      expectedRects: [
        { id: scene.rootId, rect: { x: 0, y: scene.id === 'order-ties' ? 20 : 0, width: 30, height: 10 } },
        ...expectedItems,
      ],
      expected: { outcome: 'success' },
      computedOrders: Object.fromEntries(scene.items.map(({ id, order }) => [id, String(order)])),
      source: makeSource(scene),
      sourceSha256: sha256(makeSource(scene)),
    };
  });
  return {
    schema: 'spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1',
    profileId: 'RuntimeBlockFlexGridAutoPlacementV1',
    inputs: {
      agentBrowserHtml: { path: referenceHtmlPath, sha256: sha256(htmlBytes) },
      chromeObservation: { path: referenceObservationPath, sha256: sha256(observationBytes) },
      generator: { path: 'tools/css-reference/build-c11-2-2-grid-item-order-runtime-matrix.mjs' },
    },
    viewportCssPx: { width: 360, height: 800 },
    wrapperStyle: 'display:flex;width:360px;height:800px',
    cases,
    summary: { sceneCount: 2, nodeCount: 9, itemCount: 7 },
  };
}

async function main() {
  const matrix = await buildC1122GridItemOrderRuntimeMatrix();
  const generatorPath = resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-item-order-runtime-matrix.mjs');
  matrix.inputs.generator.sha256 = sha256(await readFile(generatorPath));
  const output = `${JSON.stringify(matrix, null, 2)}\n`;
  const target = resolve(repositoryRoot, outputPath);
  if (process.argv.includes('--check')) {
    assert.equal(await readFile(target, 'utf8'), output, `${outputPath}가 생성기 결과와 다릅니다.`);
    process.stdout.write(`PASS ${outputPath}\n`);
  } else {
    await writeFile(target, output);
    process.stdout.write(`WROTE ${outputPath}\n`);
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main();
}
