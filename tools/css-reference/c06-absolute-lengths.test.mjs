import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import test from 'node:test';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const files = {
  reference: 'tests/fixtures/css/references/c06-absolute-lengths-v1.json',
  inventory: 'tests/fixtures/css/c06/absolute-lengths-inventory.json',
  html: 'tests/fixtures/css/c06/absolute-lengths.html',
  capture: 'tools/css-reference/capture-c06-absolute-lengths.mjs',
  helper: 'tools/css-reference/chromium-session.mjs',
};
const read = async (relativePath) => readFile(join(repositoryRoot, relativePath));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [reference, inventory] = await Promise.all([
  read(files.reference).then((bytes) => JSON.parse(bytes.toString('utf8'))),
  read(files.inventory).then((bytes) => JSON.parse(bytes.toString('utf8'))),
]);
const nodes = new Map(reference.observation.nodes.map((node) => [node.id, node]));
const get = (id) => {
  const node = nodes.get(id);
  assert.ok(node, `reference에 ${id} 노드가 있어야 합니다`);
  return node;
};

test('C06.3 Chromium oracle는 고정 실행 파일·fixture·capture 입력에 묶여 있다', async () => {
  assert.equal(inventory.schema, 'spinon-css-c06-absolute-lengths-inventory/v1');
  assert.equal(inventory.fixtureId, 'C06.3-absolute-lengths-v1');
  assert.equal(reference.schema, 'spinon-css-c06-absolute-lengths-reference/v1');
  assert.equal(reference.fixture.id, inventory.fixtureId);
  assert.equal(reference.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(reference.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.equal(reference.observation.nodes.length, 26);
  assert.deepEqual(reference.observation.nodes.map(({ id }) => id), inventory.nodes.map(({ id }) => id));
  assert.deepEqual(reference.environment.viewportCssPx, inventory.viewport);
  assert.equal(reference.environment.viewportCssPx.deviceScaleFactor, 1);
  assert.match(inventory.comparison.unitEquivalence, /101\.6Q/);

  for (const [relativePath, expectedHash] of [
    [files.inventory, reference.fixture.inventorySha256],
    [files.html, reference.fixture.htmlSha256],
    [files.capture, reference.captureTool.sha256],
    [files.helper, reference.captureTool.dependencies[0].sha256],
  ]) {
    assert.equal(sha256(await read(relativePath)), expectedHash, `${relativePath} hash`);
  }
});

test('7개 절대 길이는 모든 runtime layout 경계에서 같은 CSS px로 계산된다', () => {
  const unitIds = ['px', 'in', 'cm', 'mm', 'q', 'pt', 'pc'];
  const baseline = get('probe-px');
  for (const unit of unitIds) {
    const row = get(`probe-${unit}`);
    assert.equal(row.properties.width, '280px', `${unit} row width`);
    assert.equal(row.properties.height, '8px', `${unit} row height`);
    assert.equal(row.properties['margin-left'], '6px', `${unit} margin`);
    assert.equal(row.properties['padding-left'], '3px', `${unit} padding`);
    assert.equal(row.properties['column-gap'], '3px', `${unit} gap`);
    assert.equal(row.rect.x, baseline.rect.x, `${unit} row x`);
    assert.equal(row.rect.width, baseline.rect.width, `${unit} row width geometry`);

    const width = get(`${unit}-a`);
    assert.equal(width.properties.width, '96px', `${unit} width`);
    assert.equal(width.properties.height, '6px', `${unit} height`);
    assert.equal(width.rect.x, 9, `${unit} child x`);
    assert.equal(width.rect.width, 96, `${unit} child width`);
    assert.equal(width.rect.height, 6, `${unit} child height`);

    const basis = get(`${unit}-b`);
    assert.equal(basis.properties['flex-basis'], '48px', `${unit} flex basis`);
    assert.equal(basis.properties.height, '6px', `${unit} flex child height`);
    assert.equal(basis.rect.x, 108, `${unit} basis child x`);
    assert.equal(basis.rect.width, 48, `${unit} basis child width`);
    assert.equal(basis.rect.height, 6, `${unit} basis child height`);
  }
});

test('negative absolute margin와 custom-property unit substitution은 px 의미를 보존한다', () => {
  assert.equal(get('probe-negative').properties['margin-left'], '-6px');
  assert.equal(get('probe-negative').rect.x, -6);
  assert.equal(get('negative-a').rect.x, 2);
  assert.equal(get('negative-a').rect.width, 96);
  assert.equal(get('variable-a').properties.width, '96px');
  assert.equal(get('variable-a').rect.width, 96);
});

test('reference의 모든 node는 유한하고 마지막 row까지 viewport 안에 있다', () => {
  assert.equal(reference.observation.viewport.width, 320);
  assert.equal(reference.observation.viewport.height, 800);
  assert.equal(get('root').rect.width, 300);
  assert.equal(get('root').rect.height, 100);
  assert.equal(get('probe-pc').rect.y + get('probe-pc').rect.height, 88);

  for (const node of reference.observation.nodes) {
    for (const field of ['x', 'y', 'width', 'height']) {
      assert.ok(Number.isFinite(node.rect[field]), `${node.id}.${field} must be finite`);
    }
  }
});
