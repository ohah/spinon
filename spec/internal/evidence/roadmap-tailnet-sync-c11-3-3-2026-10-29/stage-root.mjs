import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = '900404c95c57b60c3cc3e3306f2c488c2abc3586';
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-3-3-pr792-2026-10-29');
const root = path.join(buildBase, 'root-pass03');
const site = path.join(buildBase, 'site-pass03');
const publication = path.join(buildBase, 'publication-pass03-staging');
const configPath = path.join(evidence, 'rspress-pass03.config.ts');
const reportPath = path.join(evidence, 'staged-root-pass03.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 32 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
assert.equal(git(['rev-parse', 'HEAD']).toString().trim(), head);
assert.equal(git(['rev-parse', 'HEAD^{tree}']).toString().trim(), git(['rev-parse', `${head}^{tree}`]).toString().trim());
for (const candidate of [buildBase, root, site, publication, configPath, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `preserving existing work; refusing to overwrite ${candidate}`);
}

const prefixes = [
  'spec/internal/evidence/c11-3-3-leading-named-implicit-lines-precomparison-2026-10-29',
  'spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-29',
  'spec/internal/evidence/c11-3-3-leading-named-implicit-lines-implementation-2026-10-29',
];
const sources = new Set([
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'plan/c11-css-grid.md',
  'plan/c11-3-3-leading-named-implicit-lines.md',
  'spec/internal/evidence/c11-3-3-leading-named-implicit-lines-plan-review-20-2026-10-29.md',
  'spec/internal/evidence/c11-3-3-leading-named-implicit-lines-plan-pr-diff-review-30-2026-10-29.md',
  'tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133.html',
  'tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133-inventory.json',
  'tests/fixtures/css/c11/grid-leading-named-implicit-lines-mobile-pixels-v1.json',
  'tests/fixtures/css/references/c11-3-3-leading-named-implicit-lines-precomparison-v1.json',
]);
for (const prefix of prefixes) {
  const paths = git(['ls-tree', '-r', '--name-only', head, prefix]).toString().trim().split('\n');
  for (const source of paths) if (source) sources.add(source);
}
const sourceTree = git(['rev-parse', `${head}^{tree}`]).toString().trim();
const staged = [];
fs.mkdirSync(buildBase, { recursive: false });
fs.mkdirSync(root, { recursive: false });

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copySource(source, route) {
  safeRoute(route);
  const sourcePath = path.join(repo, source);
  const sourceBytes = fs.readFileSync(sourcePath);
  assert.deepEqual(sourceBytes, git(['show', `${head}:${source}`]), `working source differs from merged main: ${source}`);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, sourceBytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(hash(copied), hash(sourceBytes), `staged hash mismatch: ${route}`);
  staged.push({ source, route, bytes: copied.length, sha256: hash(copied) });
}

for (const source of [...sources].sort()) {
  if (source.startsWith('plan/')) {
    copySource(source, source);
    copySource(source, `spec/${source}`);
  } else if (source.startsWith('tests/')) {
    copySource(source, source);
  } else if (source === 'spec/public/roadmap.html') {
    copySource(source, 'roadmap.html');
    copySource(source, 'public/roadmap.html');
    copySource(source, source);
  } else if (source.startsWith('spec/public/')) {
    const basename = path.basename(source);
    copySource(source, `public/${basename}`);
    copySource(source, source);
  } else if (source.startsWith('spec/')) {
    copySource(source, source.slice('spec/'.length));
    copySource(source, source);
  } else {
    throw new Error(`unexpected selected source: ${source}`);
  }
}
const index = Buffer.from('# Spinon 문서 미리보기\n\n[공개 로드맵](/spinon/roadmap.html) · [공식 구현 상태](/spinon/STATUS.html)\n');
fs.writeFileSync(path.join(root, 'index.md'), index, { flag: 'wx' });
staged.push({ source: 'generated:index.md', route: 'index.md', bytes: index.length, sha256: hash(index) });

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-3-3-pr792-2026-10-29/root-pass03'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-3-3-pr792-2026-10-29/site-pass03'),
  builderConfig: { ...baseConfig.builderConfig, server: { ...baseConfig.builderConfig?.server, publicDir: false }, output: { ...baseConfig.builderConfig?.output, cleanDistPath: true } },
  markdown: { ...baseConfig.markdown, link: { ...baseConfig.markdown?.link, checkDeadLinks: false }, image: { ...baseConfig.markdown?.image, checkDeadImages: false } },
};
`;
fs.writeFileSync(configPath, config, { flag: 'wx' });
const report = {
  sourceHead: head, sourceTree,
  selectedRoot: path.relative(repo, root), siteRoot: path.relative(repo, site),
  sourceFileCount: sources.size, stagedRouteCount: staged.length,
  fullDocsLinkValidation: false, staged,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
