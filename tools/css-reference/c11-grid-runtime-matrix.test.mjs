import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';
import { buildC11GridRuntimeMatrix } from './build-c11-grid-runtime-matrix.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const paths = {
  inventory: 'tests/fixtures/css/c11/grid-layout-inventory.json',
  html: 'tests/fixtures/css/c11/grid-layout.html',
  reference: 'tests/fixtures/css/references/c11-grid-precomparison-v2.json',
  generated: 'tests/fixtures/css/c11/runtime-grid-matrix.json',
  generator: 'tools/css-reference/build-c11-grid-runtime-matrix.mjs',
};
const read = (path) => readFile(join(repositoryRoot, path));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [inventoryBytes, htmlBytes, referenceBytes, generatedBytes] = await Promise.all([
  read(paths.inventory), read(paths.html), read(paths.reference), read(paths.generated),
]);
const reference = JSON.parse(referenceBytes.toString('utf8'));

function inputsFor(html, { updateHtmlHash = false } = {}) {
  const htmlBytes = Buffer.from(html);
  const nextReference = structuredClone(reference);
  if (updateHtmlHash) nextReference.fixture.htmlSha256 = sha256(htmlBytes);
  return {
    inventory: inventoryBytes,
    html: htmlBytes,
    reference: Buffer.from(JSON.stringify(nextReference)),
  };
}

test('정적 runtime matrix는 기준 입력 hash와 10 scene·40 node source를 고정한다', async () => {
  const matrix = await buildC11GridRuntimeMatrix({
    inventory: inventoryBytes,
    html: htmlBytes,
    reference: referenceBytes,
  });
  assert.equal(matrix.schema, 'spinon-css-c11-grid-runtime-matrix/v1');
  assert.equal(matrix.summary.sceneCount, 10);
  assert.equal(matrix.summary.nodeCount, 40);
  assert.equal(matrix.runtimeSceneWrapperStyle, 'display:flex;width:360px;height:800px');
  assert.deepEqual(matrix.cases.map(({ id }) => id), reference.observations[0].cases.map(({ id }) => id));
  assert.equal(matrix.cases.reduce((sum, item) => sum + item.referenceNodeCount, 0), 40);
  for (const item of matrix.cases) {
    assert.equal(item.nodeIds.length, item.referenceNodeCount, item.id);
    assert.match(item.source, new RegExp(`sceneId:"${item.id}"`));
    assert.match(item.source, /data-c11-grid-wrapper/);
    assert.ok(item.source.includes(
      `sceneRoot.setAttribute("style", ${JSON.stringify(matrix.runtimeSceneWrapperStyle)});`,
    ));
    assert.equal(item.sourceSha256, sha256(Buffer.from(item.source)));
    assert.equal(new Set(item.nodeIds).size, item.nodeIds.length, item.id);
  }

  const generated = JSON.parse(generatedBytes.toString('utf8'));
  assert.equal(generated.inputs.generator.sha256, sha256(await read(paths.generator)));
  execFileSync('node', [join(repositoryRoot, paths.generator), '--check'], { cwd: repositoryRoot });
});

test('원본 hash가 어긋난 HTML은 matrix source 생성 전에 거부한다', async () => {
  const html = htmlBytes.toString('utf8').replace('title>Spinon C11 Grid', 'title>Changed C11 Grid');
  await assert.rejects(buildC11GridRuntimeMatrix(inputsFor(html)), /HTML fixture hash/);
});

test('parse5 복구가 필요한 malformed HTML은 거부한다', async () => {
  const html = htmlBytes.toString('utf8').replace('<body>', '<body><');
  await assert.rejects(buildC11GridRuntimeMatrix(inputsFor(html, { updateHtmlHash: true })), /parse error/);
});

test('fixture subset 밖 tag·attribute·CSS·style·class 변경은 거부한다', async (t) => {
  const original = htmlBytes.toString('utf8');
  const rejected = [
    ['unknown tag', original.replace('<div id="c11-fixed-a"', '<button id="c11-fixed-a"')],
    ['unknown attribute', original.replace('id="c11-fixed-a"', 'id="c11-fixed-a" onclick="alert(1)"')],
    ['unknown reset rule', original.replace('color-scheme: light', 'color-scheme: dark')],
    ['unknown inline property', original.replace('width:40px;height:20px', 'width:40px;height:20px;filter:blur(1px)')],
    ['unknown palette class', original.replace('class="blue"', 'class="brand-new"')],
  ];
  for (const [name, html] of rejected) {
    await t.test(name, async () => {
      await assert.rejects(
        buildC11GridRuntimeMatrix(inputsFor(html, { updateHtmlHash: true })),
      );
    });
  }
});

test('중복 NodeId와 Chromium reference tree drift를 거부한다', async (t) => {
  const original = htmlBytes.toString('utf8');
  await t.test('duplicate id', async () => {
    const html = original.replace('id="c11-fixed-a"', 'id="c11-fixed-grid"');
    await assert.rejects(
      buildC11GridRuntimeMatrix(inputsFor(html, { updateHtmlHash: true })), /중복/,
    );
  });
  await t.test('reference node drift', async () => {
    const html = original.replace('width:40px;height:20px', 'width:41px;height:20px');
    await assert.rejects(
      buildC11GridRuntimeMatrix(inputsFor(html, { updateHtmlHash: true })), /HTML fixture hash|NodeId|tree|inline width/,
    );
  });
});
