import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-root');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-site');
const configPath = path.join(evidenceDirectory, 'rspress-selected.config.ts');
const reportPath = path.join(evidenceDirectory, 'selected-root.json');
const evidencePrefix = 'spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const candidate of [selectedRoot, outputRoot, configPath, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 경로 보존을 위해 중단: ${candidate}`);
}

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const fixedSources = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'plan/c11-4-generic-mixed-direct-text-image-intrinsic.md',
  'plan/c11-4-generic-mixed-direct-text-image-intrinsic-plan-review-20.md',
  'plan/c11-4-generic-mixed-direct-text-image-intrinsic-plan-pr-diff-review-20.md',
];
const evidenceSources = execFileSync('git', ['ls-files', '-z', '--', evidencePrefix], { cwd: repositoryRoot })
  .toString('utf8').split('\0').filter(Boolean);
const sourceFiles = [...new Set([...fixedSources, ...evidenceSources])].sort();
const trackedFiles = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', sourceHead, '--', ...sourceFiles], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));

function routesFor(source) {
  if (source === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html', source];
  if (source === 'spec/public/icon.svg' || source === 'spec/public/logo.svg') return [source.slice('spec/'.length), source];
  if (source === 'spec/STATUS.md') return ['STATUS.md', source];
  if (source.startsWith('plan/')) return [source, `spec/${source}`];
  if (source.startsWith('spec/internal/evidence/')) return [source, source.slice('spec/'.length)];
  throw new Error(`선택 publication에 예상하지 못한 source: ${source}`);
}

fs.mkdirSync(selectedRoot, { recursive: false });
const staged = [];
for (const source of sourceFiles) {
  assert.ok(trackedFiles.has(source), `HEAD에 없는 문서 source: ${source}`);
  const bytes = fs.readFileSync(path.join(repositoryRoot, source));
  const headBytes = execFileSync('git', ['show', `${sourceHead}:${source}`], { cwd: repositoryRoot });
  assert.deepEqual(bytes, headBytes, `작업 파일이 HEAD와 다름: ${source}`);
  for (const route of routesFor(source)) {
    assert.ok(!route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 route: ${route}`);
    const destination = path.join(selectedRoot, route);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(bytes), `staging hash 불일치: ${route}`);
    staged.push({ source, path: route, bytes: copied.length, sha256: sha256(copied) });
  }
}

const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html)\n');
fs.writeFileSync(path.join(selectedRoot, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', path: 'index.md', bytes: index.length, sha256: sha256(index) });

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-root'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr710-20261028-site'),
  builderConfig: {
    ...baseConfig.builderConfig,
    server: { ...baseConfig.builderConfig?.server, publicDir: false },
    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },
  },
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
`;
fs.writeFileSync(configPath, config, { flag: 'wx' });

const report = {
  pass: 'pass01',
  sourceHead,
  sourceTree,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  sourceCount: sourceFiles.length,
  routeCount: staged.length,
  stagedBytes: staged.reduce((sum, entry) => sum + entry.bytes, 0),
  staged,
  fullDocsLinkValidation: false,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
