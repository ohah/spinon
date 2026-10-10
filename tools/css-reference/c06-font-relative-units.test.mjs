import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const files = {
  reference: 'tests/fixtures/css/references/c06-font-relative-units-v1.json',
  inventory: 'tests/fixtures/css/c06/font-relative-units-inventory.json',
  html: 'tests/fixtures/css/c06/font-relative-units.html',
  runtime: 'tests/fixtures/css/c06/runtime-font-relative-units.js',
  capture: 'tools/css-reference/capture-c06-font-relative-units.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
};
const read = async (relativePath) => readFile(join(repositoryRoot, relativePath));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [reference, inventory] = await Promise.all([
  read(files.reference).then((bytes) => JSON.parse(bytes.toString('utf8'))),
  read(files.inventory).then((bytes) => JSON.parse(bytes.toString('utf8'))),
]);
const byId = (observation) => new Map(observation.nodes.map((node) => [node.id, node]));
const get = (map, id) => {
  const node = map.get(id);
  assert.ok(node, 'reference에 ' + id + ' 노드가 있어야 합니다');
  return node;
};

test('C06.4 Chromium 기준은 고정 실행 파일·DPR 쌍·fixture 입력에 묶여 있다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c06-font-relative-units-inventory/v1');
  assert.equal(inventory.fixtureId, 'C06.4-font-relative-units-v1');
  assert.equal(reference.schema, 'spinon-css-c06-font-relative-units-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.observations.length, 2);
  assert.deepEqual(reference.observations.map(({ viewport }) => viewport.deviceScaleFactor), [1, 2]);
  assert.deepEqual(
    reference.observations[0].nodes.map(({ id }) => id),
    inventory.nodes.map(({ id }) => id),
  );

  for (const [relativePath, expectedHash] of [
    [files.inventory, reference.fixture.inventorySha256],
    [files.html, reference.fixture.htmlSha256],
    [files.capture, reference.captureTool.sha256],
    [files.helper, reference.captureTool.dependencies[0].sha256],
  ]) {
    assert.equal(sha256(await read(relativePath)), expectedHash, relativePath + ' hash');
  }
});

test('em, rem, 상속, font-size percentage 및 root rem 특례가 Chromium 계산값과 일치한다', () => {
  const nodes = byId(reference.observations[0]);
  assert.equal(get(nodes, 'html').properties['font-size'], '20px');
  assert.equal(get(nodes, 'body').properties['font-size'], '20px');
  assert.equal(get(nodes, 'mount').properties['font-size'], '10px');
  assert.equal(get(nodes, 'em-inherit').properties.width, '20px');
  assert.equal(get(nodes, 'em-parent').properties['font-size'], '12px');
  assert.equal(get(nodes, 'em-parent').properties.width, '24px');
  assert.equal(get(nodes, 'em-child').properties['font-size'], '18px');
  assert.equal(get(nodes, 'em-child').properties.width, '18px');
  assert.equal(get(nodes, 'rem-target').properties['font-size'], '25px');
  assert.equal(get(nodes, 'rem-target').properties.width, '40px');
  assert.equal(get(nodes, 'rem-target').properties.height, '20px');
  assert.equal(get(nodes, 'rem-target').properties['margin-left'], '5px');
  assert.equal(get(nodes, 'rem-target').properties['padding-left'], '10px');
  assert.equal(get(nodes, 'font-size-percent').properties['font-size'], '15px');
  assert.equal(get(nodes, 'font-size-percent').properties.width, '15px');
  assert.equal(get(nodes, 'zero').properties.width, '0px');
  assert.equal(get(nodes, 'var-em').properties.width, '28px');
  assert.equal(get(nodes, 'var-rem').properties.width, '40px');
});

test('em과 rem이 flex basis와 spacing에 CSS px로 반영되고 DPR과 독립적이다', () => {
  const one = byId(reference.observations[0]);
  const two = byId(reference.observations[1]);
  assert.equal(get(one, 'spacing').properties['row-gap'], '3px');
  assert.equal(get(one, 'spacing').properties['column-gap'], '6px');
  assert.equal(get(one, 'spacing').properties['margin-left'], '6px');
  assert.equal(get(one, 'spacing').properties['padding-left'], '3px');
  assert.equal(get(one, 'basis-a').properties['flex-basis'], '24px');
  assert.equal(get(one, 'basis-b').properties['flex-basis'], '20px');

  for (const { id } of inventory.nodes) {
    assert.deepEqual(get(one, id).properties, get(two, id).properties, id + ' computed CSS values');
    assert.deepEqual(get(one, id).rect, get(two, id).rect, id + ' CSS px geometry');
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Number.isFinite(get(one, id).rect[field]), id + '.' + field + ' is finite');
    }
  }
});

test('V8 runtime fixture는 Chromium HTML fixture와 동일한 author CSS와 노드 트리를 사용한다', async () => {
  const [htmlBytes, runtimeBytes] = await Promise.all([read(files.html), read(files.runtime)]);
  const html = htmlBytes.toString('utf8');
  const runtime = runtimeBytes.toString('utf8');
  const htmlCss = html.split('<style>')[1]?.split('</style>')[0];
  const runtimeCss = runtime.split('String.raw`')[1]?.split('`;')[0];
  assert.ok(htmlCss, 'Chromium fixture stylesheet must exist');
  assert.ok(runtimeCss, 'runtime fixture stylesheet must exist');
  const normalizeCss = (css) => css.trim().split('\n')
    .map((line) => line.trim()).filter(Boolean).join('\n');
  assert.equal(normalizeCss(runtimeCss), normalizeCss(htmlCss));
  for (const { id } of inventory.nodes.slice(2)) {
    assert.match(runtime, new RegExp('"' + id + '"'), 'runtime fixture must create ' + id);
  }
  assert.match(runtime, /document\.appendChild\(mount\)/);
  assert.match(runtime, /document\.createElement\("style"\)/);
  assert.match(runtime, /document\.createTextNode\(css\)/);
});
