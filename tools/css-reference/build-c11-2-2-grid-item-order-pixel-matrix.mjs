import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const runtimeMatrixPath = 'tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json';
const outputPath = 'tests/fixtures/css/c11/grid-item-order-mobile-surface-pixel-matrix-v1.json';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

export async function buildC1122GridItemOrderPixelMatrix() {
  const runtimeBytes = await readFile(resolve(repositoryRoot, runtimeMatrixPath));
  const runtimeMatrix = JSON.parse(runtimeBytes.toString('utf8'));
  assert.equal(runtimeMatrix.schema, 'spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1');
  const cases = runtimeMatrix.cases.map((fixtureCase) => {
    const nodes = new Map(fixtureCase.nodes.map((node) => [node.id, node]));
    const samples = fixtureCase.expectedRects
      .filter(({ id }) => id !== fixtureCase.rootId)
      .map(({ id, rect }) => {
        const node = nodes.get(id);
        assert.ok(node, `${fixtureCase.id}/${id}: runtime matrix node 누락`);
        const color = node.authoredStyle.match(/(?:^|;)background-color:(#[0-9a-f]{6})(?:;|$)/i)?.[1];
        assert.ok(color, `${fixtureCase.id}/${id}: 고정 배경색 누락`);
        const channels = color.slice(1).match(/../g).map((channel) => Number.parseInt(channel, 16));
        return {
          id: `item-${id}`,
          nodeId: id,
          kind: 'item',
          xCssPx: Math.floor(rect.x + rect.width / 2),
          yCssPx: Math.floor(rect.y + rect.height / 2),
          rgba: [...channels, 255],
        };
      });
    return {
      id: fixtureCase.id,
      index: fixtureCase.index,
      runtimeMatrixSourceSha256: fixtureCase.sourceSha256,
      outcome: 'surface-pixel-compare',
      nodeIds: fixtureCase.nodeIds,
      samples,
    };
  });
  return {
    schema: 'spinon-css-c11-2-2-grid-item-order-mobile-surface-pixel-matrix/v1',
    fixtureId: 'c11-2-2-grid-item-order-mobile-surface-pixels',
    runtime: {
      viewportCssPx: { width: 320, height: 240 },
      surface: 'Android SurfaceView PixelCopy 또는 iOS WGPU Metal drawable, compositor 제출 전',
      devicePixelFormula: 'floor((cssPx + 0.5) * drawablePixels / runtimeViewportCssPx)',
      maximumChannelError: 0,
    },
    inputs: {
      runtimeMatrix: { path: runtimeMatrixPath, sha256: sha256(runtimeBytes) },
      generator: { path: 'tools/css-reference/build-c11-2-2-grid-item-order-pixel-matrix.mjs' },
    },
    cases,
    summary: {
      sceneCount: cases.length,
      sampleCount: cases.reduce((count, fixtureCase) => count + fixtureCase.samples.length, 0),
    },
  };
}

async function main() {
  const matrix = await buildC1122GridItemOrderPixelMatrix();
  const generatorPath = resolve(repositoryRoot, 'tools/css-reference/build-c11-2-2-grid-item-order-pixel-matrix.mjs');
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
