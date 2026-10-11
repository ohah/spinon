import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = 'tests/fixtures/css/c12/stacking-context-inventory.json';
const outputPath = 'build/spinon/bootstrap/c12-4-runtime-matrix.json';

const expectedRejections = {
  'display-none-visibility-hidden': {
    kind: 'css-rejection',
    property: 'visibility',
    diagnostic: 'visibility',
  },
  'visibility-collapse-flex': {
    kind: 'css-rejection',
    property: 'visibility',
    diagnostic: 'visibility',
  },
  'absolute-owner-vs-opacity-context': {
    kind: 'css-rejection',
    property: 'opacity',
    diagnostic: 'opacity',
  },
  'top-layer-dialog': {
    kind: 'api-rejection',
    api: 'showModal',
    diagnostic: 'showModal',
  },
  'pseudo-element-paint': {
    kind: 'css-rejection',
    property: 'pseudo-element',
    diagnostic: 'content',
  },
};

function cssRulesForCase(fixtureCase, stylesheetRules) {
  const classNames = new Set(fixtureCase.nodes.map((node) => node.className).filter(Boolean));
  if (classNames.size === 0) return [];

  const hasClassSelector = (rule) => [...classNames].some((className) =>
    new RegExp(`\\.${className.replace(/[.*+?^${}()|[\\]\\\\]/g, '\\\\$&')}(?![a-zA-Z0-9_-])`).test(rule));
  const matchingRules = stylesheetRules.filter(hasClassSelector);
  if (matchingRules.some((rule) => rule.startsWith('@layer c124-base {'))
      || matchingRules.some((rule) => rule.startsWith('@layer c124-overrides {'))) {
    const layerOrder = stylesheetRules.find((rule) => rule.startsWith('@layer c124-base,'));
    if (layerOrder) matchingRules.unshift(layerOrder);
  }
  if (classNames.has('c124-registered')) {
    const registration = stylesheetRules.find((rule) => rule.startsWith('@property --c124-registered-z'));
    if (registration) matchingRules.unshift(registration);
  }
  return [...new Set(matchingRules)];
}

function javascriptSource(fixtureCase, stage, inventoryStylesheetRules) {
  const stylesheetRules = cssRulesForCase(fixtureCase, inventoryStylesheetRules);
  const lines = [
    '(() => {',
    `const root = document.createElement('div');`,
    `root.setAttribute('id', 'c124-matrix-root-${fixtureCase.id}');`,
    `root.setAttribute('style', 'display:block;position:relative;box-sizing:border-box;width:320px;height:240px;background-color:#112233');`,
    `const stage = document.createElement('div');`,
    `stage.setAttribute('id', 'c124-matrix-stage-${fixtureCase.id}');`,
    `stage.setAttribute('style', 'display:block;position:relative;left:${stage.x}px;top:${stage.y}px;width:${stage.width}px;height:${stage.height}px;--tile-x:${stage.x}px;--tile-y:${stage.y}px');`,
    'root.appendChild(stage);',
    'const refs = Object.create(null);',
  ];

  for (const node of fixtureCase.nodes) {
    const tagName = node.tag ?? 'div';
    lines.push(`const element_${node.id.replaceAll('-', '_')} = document.createElement(${JSON.stringify(tagName)});`);
    lines.push(`element_${node.id.replaceAll('-', '_')}.setAttribute('id', ${JSON.stringify(node.id)});`);
    if (node.className) {
      lines.push(`element_${node.id.replaceAll('-', '_')}.setAttribute('class', ${JSON.stringify(node.className)});`);
    }
    lines.push(`element_${node.id.replaceAll('-', '_')}.setAttribute('style', ${JSON.stringify(node.style)});`);
    lines.push(`element_${node.id.replaceAll('-', '_')}.setAttribute('data-c124-matrix-node', '');`);
    if (node.paint) lines.push(`element_${node.id.replaceAll('-', '_')}.setAttribute('data-c124-paint', '');`);
    const parent = node.parentId ? `refs[${JSON.stringify(node.parentId)}]` : 'stage';
    lines.push(`${parent}.appendChild(element_${node.id.replaceAll('-', '_')});`);
    lines.push(`refs[${JSON.stringify(node.id)}] = element_${node.id.replaceAll('-', '_')};`);
  }

  if (stylesheetRules.length > 0) {
    lines.push('const matrixStyle = document.createElement(\'style\');');
    lines.push("matrixStyle.setAttribute('type', 'text/css');");
    lines.push(`matrixStyle.appendChild(document.createTextNode(${JSON.stringify(stylesheetRules.join('\n'))}));`);
    lines.push('root.appendChild(matrixStyle);');
  }

  if (fixtureCase.activateTopLayer?.length) {
    for (const nodeId of fixtureCase.activateTopLayer) {
      lines.push(`refs[${JSON.stringify(nodeId)}].showModal();`);
    }
  }
  lines.push('document.appendChild(root);');
  lines.push(`return { caseId:${JSON.stringify(fixtureCase.id)}, nodeCount:${fixtureCase.nodes.length} };`);
  lines.push('})()');
  return lines.join('\n');
}

export async function buildC124RuntimeMatrix() {
  const inventory = JSON.parse(await readFile(resolve(repositoryRoot, inventoryPath), 'utf8'));
  if (inventory.fixtureId !== 'C12.4-stacking-context-v1'
      || inventory.cases.length !== 26
      || inventory.cases.reduce((total, item) => total + item.nodes.length, 0) !== 77) {
    throw new Error('C12.4 모바일 matrix 생성 전 inventory의 case/node 수가 바뀌었습니다.');
  }

  const cases = inventory.cases.map((fixtureCase, index) => ({
    id: fixtureCase.id,
    expected: expectedRejections[fixtureCase.id] ?? { kind: 'supported' },
    nodeIds: fixtureCase.nodes.map((node) => node.id),
    samples: fixtureCase.samples.map(({ id, xCssPx, yCssPx, rgba }) => ({
      id,
      xCssPx,
      yCssPx,
      rgba,
    })),
    source: javascriptSource(fixtureCase, {
      x: inventory.layout.originCssPx.x,
      y: inventory.layout.originCssPx.y,
      width: inventory.layout.stageWidthCssPx,
      height: inventory.layout.stageHeightCssPx,
      index,
    }, inventory.stylesheetRules),
    stylesheetRules: cssRulesForCase(fixtureCase, inventory.stylesheetRules),
  }));
  const sourceHash = createHash('sha256')
    .update(await readFile(resolve(repositoryRoot, inventoryPath)))
    .digest('hex');
  return {
    schema: 'spinon-css-c12-4-runtime-matrix/v1',
    viewportCssPx: { width: 320, height: 240 },
    stage: {
      x: inventory.layout.originCssPx.x,
      y: inventory.layout.originCssPx.y,
      width: inventory.layout.stageWidthCssPx,
      height: inventory.layout.stageHeightCssPx,
    },
    inventory: { path: inventoryPath, sha256: sourceHash },
    cases,
    summary: {
      cases: cases.length,
      nodes: cases.reduce((total, item) => total + item.nodeIds.length, 0),
      expectedSupported: cases.filter((item) => item.expected.kind === 'supported').length,
      expectedRejected: cases.filter((item) => item.expected.kind !== 'supported').length,
    },
  };
}

async function main() {
  const matrix = await buildC124RuntimeMatrix();
  const target = resolve(repositoryRoot, outputPath);
  await mkdir(dirname(target), { recursive: true });
  await writeFile(target, `${JSON.stringify(matrix)}\n`);
  process.stdout.write(`C12.4 Android runtime matrix: ${matrix.summary.cases} cases, `
    + `${matrix.summary.nodes} nodes, ${matrix.summary.expectedSupported} expected ready, `
    + `${matrix.summary.expectedRejected} expected rejection\n`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main();
}
