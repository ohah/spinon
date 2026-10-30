import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const head = 'e886dfebccc406ac6f7303a462b228ef3f258248';
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr837-2026-10-30/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const publication = path.join(base, 'publication');
const configPath = path.join(evidence, 'rspress-pass01.config.ts');
const reportPath = path.join(evidence, 'selected-root-pass01.json');
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, maxBuffer: 32 * 1024 * 1024 });

assert.equal(git(['rev-parse', 'HEAD']).toString().trim(), head);
assert.equal(git(['rev-parse', 'origin/main']).toString().trim(), head);
const sourceTree = git(['rev-parse', `${head}^{tree}`]).toString().trim();
for (const candidate of [base, root, site, publication, configPath, reportPath]) {
  assert.ok(!fs.existsSync(candidate), `기존 산출물을 보존하기 위해 중단: ${candidate}`);
}

const sources = new Set([
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-auto-repeat-pattern-ceiling.md',
  'plan/c11-4-auto-repeat-pattern-ceiling-plan-review-20.md',
]);
for (const prefix of [
  'spec/internal/evidence/c11-4-auto-repeat-pattern-ceiling-plan-2026-10-30',
  'spec/internal/evidence/c11-4-auto-repeat-pattern-ceiling-precomparison-2026-10-30',
  'spec/internal/evidence/c11-4-auto-repeat-pattern-ceiling-runtime-2026-10-30',
]) {
  for (const file of git(['ls-tree', '-r', '--name-only', head, prefix]).toString().trim().split('\n')) {
    if (file) sources.add(file);
  }
}

fs.mkdirSync(base, { recursive: true });
fs.mkdirSync(root, { recursive: false });
const staged = [];
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'));
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'));
}
function copySource(source, route) {
  safeRoute(route);
  const bytes = git(['show', `${head}:${source}`]);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(hash(fs.readFileSync(destination)), hash(bytes), `staged hash mismatch: ${route}`);
  staged.push({ source, route, bytes: bytes.length, sha256: hash(bytes) });
}

for (const source of [...sources].sort()) {
  if (source.startsWith('plan/')) {
    copySource(source, source);
    copySource(source, `spec/${source}`);
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
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr837-2026-10-30/pass01/root'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr837-2026-10-30/pass01/site'),
  builderConfig: { ...baseConfig.builderConfig, server: { ...baseConfig.builderConfig?.server, publicDir: false }, output: { ...baseConfig.builderConfig?.output, cleanDistPath: true } },
  markdown: { ...baseConfig.markdown, link: { ...baseConfig.markdown?.link, checkDeadLinks: false }, image: { ...baseConfig.markdown?.image, checkDeadImages: false } },
};
`;
fs.writeFileSync(configPath, config, { flag: 'wx' });
const report = {
  sourceHead: head,
  sourceTree,
  selectedRoot: path.relative(repo, root),
  siteRoot: path.relative(repo, site),
  sourceCount: sources.size,
  stagedRouteCount: staged.length,
  fullDocsLinkValidation: false,
  staged,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
