import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const inventoryPath = join(repositoryRoot, 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json');
const htmlPath = join(repositoryRoot, 'tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html');
const mode = process.argv.includes('--write') ? 'write' : 'check';

const child = (name, color, width, height, style = '', children = []) => ({
  name, color, width, height, style, children,
});
const scene = (id, title, columns, rows, children, options = {}) => ({
  id,
  title,
  scope: options.scope ?? 'implementation-candidate',
  rejectionReason: options.rejectionReason,
  expectedRuntimeResult: options.expectedRuntimeResult,
  gridStyle: [
    'position:absolute', 'left:20px', 'top:20px', 'display:grid',
    ...(options.width === null ? [] : [`width:${options.width ?? 184}px`]),
    ...(options.height === null ? [] : [`height:${options.height ?? 88}px`]),
    `grid-template-columns:${columns}`, `grid-template-rows:${rows}`,
    `grid-auto-flow:${options.flow ?? 'row'}`,
    `grid-auto-rows:${options.autoRows ?? 'auto'}`,
    `grid-auto-columns:${options.autoColumns ?? 'auto'}`,
    `column-gap:${options.columnGap ?? 8}px`, `row-gap:${options.rowGap ?? 8}px`,
  ].join(';'),
  children,
});

const scenes = [
  scene('row-sparse-baseline', 'C11.2.2 sparse 회귀 기준', '40px 40px 40px', '20px', [
    child('auto-a', 'blue', 18, 18),
    child('span-two', 'purple', 84, 18, 'grid-column:span 2'),
    child('fixed-cell', 'red', 18, 18, 'grid-column:2 / 3;grid-row:1 / 2'),
    child('auto-b', 'green', 18, 18),
  ], { width: 128, height: 20, autoRows: '25px', columnGap: 4, rowGap: 4 }),
  scene('row-dense-hole-fill', 'row dense가 앞선 hole을 채움', '40px 40px 40px', '20px 20px', [
    child('span-a', 'blue', 84, 18, 'grid-column:span 2'),
    child('span-b', 'purple', 84, 18, 'grid-column:span 2'),
    child('small-c', 'green', 18, 18),
  ], { width: 128, height: 44, flow: 'row dense', autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('row-dense-span-collision', 'row dense span 충돌과 definite 선점', '40px 40px 40px', '20px 20px', [
    child('span-a', 'blue', 84, 18, 'grid-column:span 2'),
    child('fixed-c', 'red', 18, 18, 'grid-column:3 / 4;grid-row:1 / 2'),
    child('span-b', 'purple', 84, 18, 'grid-column:span 2'),
    child('small-d', 'green', 18, 18),
  ], { width: 128, height: 44, flow: 'row dense', autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('row-sparse-hole-control', '동일 source order sparse 대조군', '40px 40px 40px', '20px 20px', [
    child('span-a', 'blue', 84, 18, 'grid-column:span 2'),
    child('span-b', 'purple', 84, 18, 'grid-column:span 2'),
    child('small-c', 'green', 18, 18),
  ], { width: 128, height: 44, autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('row-dense-fixed-row', 'row dense definite row 잠금', '40px 40px 40px', '20px 20px', [
    child('locked-a', 'blue', 18, 18, 'grid-row:1'),
    child('locked-span', 'purple', 84, 18, 'grid-row:1;grid-column:span 2'),
    child('locked-c', 'green', 18, 18, 'grid-row:1'),
  ], { width: 128, height: 44, flow: 'row dense', autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('row-dense-fixed-column', 'row dense definite column 잠금', '40px 40px 40px', '20px 20px', [
    child('locked-a', 'blue', 18, 18, 'grid-column:2'),
    child('locked-b', 'red', 18, 18, 'grid-column:2'),
    child('locked-c', 'green', 18, 18, 'grid-column:2'),
  ], { width: 128, height: 44, flow: 'row dense', autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('row-dense-positive-implicit-row', 'row dense 양의 implicit row와 고정 px', '40px 40px', '20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
    child('item-c', 'green', 18, 18), child('item-d', 'purple', 18, 18),
    child('item-e', 'orange', 18, 18),
  ], { width: 88, height: 73, flow: 'row dense', autoRows: '25px', columnGap: 8, rowGap: 8 }),
  scene('column-sparse-explicit-fill', 'column sparse 명시 track 채우기', '40px 40px', '20px 20px 20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
    child('item-c', 'green', 18, 18), child('item-d', 'purple', 18, 18),
  ], { width: 88, height: 76, flow: 'column', autoRows: '20px', columnGap: 8, rowGap: 8 }),
  scene('column-sparse-positive-implicit-column', 'column sparse 양의 implicit column', '40px', '20px 20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
    child('item-c', 'green', 18, 18), child('item-d', 'purple', 18, 18),
    child('item-e', 'orange', 18, 18),
  ], { width: 124, height: 48, flow: 'column', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-sparse-fixed-row', 'column sparse definite row 잠금', '40px 40px', '20px 20px 20px', [
    child('locked-a', 'blue', 18, 18, 'grid-row:2'),
    child('locked-b', 'red', 18, 18, 'grid-row:2'),
    child('locked-c', 'green', 18, 18, 'grid-row:2'),
  ], { width: 88, height: 76, flow: 'column', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-sparse-fixed-column', 'column sparse definite column 잠금', '40px 40px', '20px 20px 20px', [
    child('locked-a', 'blue', 18, 18, 'grid-column:1'),
    child('locked-b', 'red', 18, 18, 'grid-column:1'),
    child('locked-c', 'green', 18, 18, 'grid-column:1'),
  ], { width: 88, height: 76, flow: 'column', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-dense-hole-fill', 'column dense가 앞선 hole을 채움', '40px 40px', '20px 20px 20px', [
    child('span-a', 'blue', 18, 44, 'grid-row:span 2'),
    child('span-b', 'purple', 18, 44, 'grid-row:span 2'),
    child('small-c', 'green', 18, 18),
  ], { width: 88, height: 76, flow: 'column dense', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-sparse-hole-control', '동일 source order column sparse 대조군', '40px 40px', '20px 20px 20px', [
    child('span-a', 'blue', 18, 44, 'grid-row:span 2'),
    child('span-b', 'purple', 18, 44, 'grid-row:span 2'),
    child('small-c', 'green', 18, 18),
  ], { width: 88, height: 76, flow: 'column', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-dense-span-collision', 'column dense span과 definite 선점', '40px 40px', '20px 20px 20px', [
    child('span-a', 'blue', 18, 44, 'grid-row:span 2'),
    child('fixed-c', 'red', 18, 18, 'grid-row:3 / 4;grid-column:1 / 2'),
    child('span-b', 'purple', 18, 44, 'grid-row:span 2'),
    child('small-d', 'green', 18, 18),
  ], { width: 88, height: 76, flow: 'column dense', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('column-dense-positive-implicit-column', 'column dense 양의 implicit column', '40px', '20px 20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
    child('item-c', 'green', 18, 18), child('item-d', 'purple', 18, 18),
    child('item-e', 'orange', 18, 18),
  ], { width: 124, height: 48, flow: 'column dense', autoRows: '20px', autoColumns: '28px', columnGap: 8, rowGap: 8 }),
  scene('definite-first-dense', 'DOM 뒤 definite item의 선점', '40px 40px 40px', '20px 20px', [
    child('auto-a', 'blue', 18, 18),
    child('fixed-late', 'red', 18, 18, 'grid-column:1 / 2;grid-row:1 / 2'),
    child('span-b', 'purple', 84, 18, 'grid-column:span 2'),
    child('auto-c', 'green', 18, 18),
  ], { width: 128, height: 44, flow: 'row dense', autoRows: '20px', columnGap: 4, rowGap: 4 }),
  scene('hidden-item-excluded', 'display none child 제외', '40px 40px', '20px', [
    child('hidden', 'yellow', 18, 18, 'display:none;grid-column:1 / 2;grid-row:1 / 2'),
    child('visible-a', 'blue', 18, 18), child('visible-b', 'red', 18, 18),
  ], { width: 88, height: 20, flow: 'row dense', columnGap: 8, rowGap: 8 }),
  scene('nested-mixed-flow', '중첩 container 독립 flow', '88px 88px', '40px', [
    child('nested-row', 'blue', 88, 40, 'display:grid;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-flow:row dense;grid-auto-rows:18px;column-gap:4px;row-gap:4px;grid-column:1 / 2;grid-row:1 / 2', [
      child('inner-span-a', 'yellow', 36, 18, 'grid-column:span 2'),
      child('inner-span-b', 'orange', 36, 18, 'grid-column:span 2'),
      child('inner-small', 'green', 18, 18),
    ]),
    child('outer-item', 'red', 88, 40, 'grid-column:2 / 3;grid-row:1 / 2'),
  ], { width: 184, height: 40, flow: 'row', columnGap: 8, rowGap: 8 }),
  scene('auto-track-no-implicit', 'implicit track 없이 auto sizing', '40px 40px', '20px 20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
  ], { width: 88, height: 48, flow: 'row dense', autoRows: 'auto', autoColumns: 'auto', columnGap: 8, rowGap: 8 }),
  scene('reject-repeated-auto-track', '반복 auto track pattern oracle', '40px 40px', '20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
    child('item-c', 'green', 18, 18), child('item-d', 'yellow', 18, 18),
    child('item-e', 'purple', 18, 18), child('item-f', 'cyan', 18, 18),
  ], { width: 88, height: 100, autoRows: '16px 24px', columnGap: 8, rowGap: 4,
    scope: 'chromium-oracle-only', rejectionReason: '반복 grid-auto-rows pattern은 현재 profile 지원 범위 밖입니다.' }),
  scene('reject-negative-implicit-line', '음수 implicit line oracle', '40px 40px', '20px', [
    child('negative-line', 'orange', 18, 18, 'grid-column:-4 / -3;grid-row:1 / 2'),
  ], { width: 88, height: 20, scope: 'chromium-oracle-only',
    rejectionReason: '명시 Grid 앞의 음수 implicit column이 필요합니다.' }),
  scene('reject-nonzero-order', 'nonzero order oracle', '40px 40px', '20px', [
    child('order-two', 'blue', 18, 18, 'order:2'), child('order-zero', 'red', 18, 18),
  ], { width: 88, height: 20, scope: 'chromium-oracle-only',
    rejectionReason: 'order가 0이 아니어서 현재 source-order placement profile에서 제외합니다.' }),
  scene('reject-indefinite-axis', 'indefinite container axis oracle', '40px 40px', '20px', [
    child('item-a', 'blue', 18, 18), child('item-b', 'red', 18, 18),
  ], { width: 88, height: null, scope: 'chromium-oracle-only',
    rejectionReason: 'indefinite block axis 기반 auto track sizing은 현재 profile 범위 밖입니다.' }),
  scene('auto-span-at-track-limit', '자동 배치 10,000 implicit column 경계', '1px', '1px', [
    child('span-at-maximum', 'blue', 10000, 1, 'grid-column:span 10000;grid-row:1 / 2'),
  ], { width: 360, height: 20, flow: 'row dense', autoColumns: '1px', columnGap: 0, rowGap: 0,
    expectedRuntimeResult: { status: 'success', computedColumns: 10000, maximum: 10000 } }),
  scene('auto-span-over-track-limit', '자동 배치 10,001 implicit column 초과', '1px', '1px', [
    child('span-over-maximum', 'red', 10001, 1, 'grid-column:span 10001;grid-row:1 / 2'),
  ], { width: 360, height: 20, flow: 'row dense', autoColumns: '1px', columnGap: 0, rowGap: 0,
    expectedRuntimeResult: { status: 'reject', errorCode: 'GridTrackExpansionOverflow', maximum: 10000 } }),
];

function nodeId(sceneId, name) {
  return `c1123-${sceneId}-${name}`;
}

function flattenNodes(sceneValue) {
  const result = [];
  const visit = (value) => {
    result.push(nodeId(sceneValue.id, value.name));
    for (const nested of value.children ?? []) visit(nested);
  };
  visit({ name: 'grid', children: sceneValue.children });
  return result;
}

const inventory = {
  schema: 'spinon-css-c11-2-3-grid-auto-placement-inventory/v6',
  fixtureId: 'C11-2-3-grid-auto-placement-precomparison-v6',
  profileId: 'RuntimeBlockFlexGridAutoPlacementDenseV1',
  viewport: { width: 360, height: 800, deviceScaleFactors: [1, 2] },
  environment: {
    locale: 'en-US', timeZone: 'UTC', colorScheme: 'light', forcedColors: 'none',
    pointer: 'coarse', hover: 'none', direction: 'ltr', writingMode: 'horizontal-tb',
  },
  comparison: {
    rectFields: ['x', 'y', 'width', 'height'],
    maximumAbsoluteRectErrorCssPx: 0.5,
    computedProperties: [
      'display', 'grid-template-columns', 'grid-template-rows', 'grid-auto-flow',
      'grid-auto-columns', 'grid-auto-rows', 'grid-column-gap', 'grid-row-gap',
      'grid-row-start', 'grid-row-end', 'grid-column-start', 'grid-column-end', 'order',
    ],
  },
  wpt: {
    revision: 'c999c58338ee1d223df5ad62e48be1202ced0d37',
    chrome: { product: 'Chrome/154.0.8037.98', revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33' },
    supportCss: { path: 'css/support/grid.css', sha256: 'b2f35862e7b8fa9dcae8f4f8d25add78d93f3e7993e08a4337030a4dd2d4425e' },
    candidates: [
      ['css/css-grid/placement/grid-auto-flow-sparse-001.html', '9d30cb69227ab5bf80fda3f557b03eb89be7c9d5171de73ef7db8ecc5f80c0cf', 'column sparse 자동 배치 포함'],
      ['css/css-grid/placement/grid-auto-placement-fixed-row-column-001.html', '14c4a5c29bfd093473c7a5794637ea8d5aea44f4298c7a4f17df360d373762a2', '고정 secondary 축에서 sparse/dense span 배치 포함'],
      ['css/css-grid/placement/grid-auto-placement-implicit-tracks-001.html', '785012f37a838995bdd6491bd9d27034d21e2441152a067f050bc82fd993ad53', 'column 및 row/column dense와 implicit track 포함'],
    ].map(([path, sha256, coverage]) => ({
      path, sha256, coverage, status: 'chrome-wpt-run-pass',
      sourceUrl: `https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/${path}`,
    })),
    runnerResult: { path: 'spec/internal/evidence/c11-2-3-dense-column-placement-precomparison-2026-10-13/wpt-chrome-154-rerun.log', summary: '3 ran as expected; 0 skipped' },
  },
  cases: scenes.map((value) => ({
    id: value.id,
    title: value.title,
    scope: value.scope,
    ...(value.rejectionReason ? { rejectionReason: value.rejectionReason } : {}),
    ...(value.expectedRuntimeResult ? { expectedRuntimeResult: value.expectedRuntimeResult } : {}),
    rootId: nodeId(value.id, 'grid'),
    nodeIds: flattenNodes(value),
  })),
};

function renderNode(sceneValue, value) {
  const id = nodeId(sceneValue.id, value.name);
  const isGrid = value.name === 'grid' || Boolean(value.children?.length);
  const style = value.name === 'grid' ? sceneValue.gridStyle : [
    `width:${value.width}px`, `height:${value.height}px`, value.style,
  ].filter(Boolean).join(';');
  const classes = value.name === 'grid' ? 'surface' : value.color;
  const descendants = value.children ?? [];
  return `<div id="${id}" data-node${isGrid ? ' data-grid' : ''} class="${classes}" style="${style}">${descendants.map((item) => renderNode(sceneValue, item)).join('')}</div>`;
}

const html = [
  '<!doctype html>', '<html lang="en"><head><meta charset="utf-8">',
  '<meta name="viewport" content="width=device-width, initial-scale=1">',
  '<title>Spinon C11.2.3 dense·column Grid 기준 장면</title>', '<style>',
  'html,body{margin:0;padding:0}',
  'body{direction:ltr;writing-mode:horizontal-tb;color-scheme:light}',
  '*,*::before,*::after{box-sizing:content-box}',
  '#stage{position:absolute;top:0;left:0;width:360px;height:800px}',
  '.scene{position:absolute;top:0;left:0;display:none;width:360px;height:800px}',
  '.scene.active{display:block}',
  '.surface{background:#edf1f7}', '.blue{background:#2866f6}', '.red{background:#e5484d}',
  '.green{background:#19a974}', '.yellow{background:#ffc857}', '.purple{background:#8758ff}',
  '.cyan{background:#0ea5e9}', '.orange{background:#f97316}', '.pink{background:#ec4899}',
  '</style></head><body><div id="stage">',
  ...scenes.map((value) => `<section class="scene" data-scene="${value.id}"><div class="scene-root">${renderNode(value, { name: 'grid', children: value.children })}</div></section>`),
  '</div></body></html>',
].join('\n') + '\n';

assert.equal(scenes.length, 25);
assert.equal(scenes.filter(({ scope }) => scope === 'implementation-candidate').length, 21);
assert.equal(scenes.filter(({ scope }) => scope === 'chromium-oracle-only').length, 4);
assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size,
  inventory.cases.flatMap(({ nodeIds }) => nodeIds).length);

const outputs = [[inventoryPath, `${JSON.stringify(inventory, null, 2)}\n`], [htmlPath, html]];
if (mode === 'write') {
  const existing = [];
  for (const [path, contents] of outputs) {
    try {
      const current = await readFile(path, 'utf8');
      assert.equal(current, contents,
        `${path.slice(repositoryRoot.length + 1)}은 이미 고정된 fixture입니다. 변경이 필요하면 새 vN 경로를 추가하세요.`);
      existing.push(path);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
    }
  }
  for (const [path, contents] of outputs) {
    if (!existing.includes(path)) await writeFile(path, contents, { flag: 'wx' });
  }
} else {
  for (const [path, expected] of outputs) {
    const actual = await readFile(path, 'utf8');
    assert.equal(actual, expected, `${path.slice(repositoryRoot.length + 1)} 생성물이 최신 상태가 아닙니다.`);
  }
}
