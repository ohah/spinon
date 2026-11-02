import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const priorRoot = path.join(repo, 'build/spinon/c11-roadmap-pr1042-pass04/root');
const base = path.join(repo, 'build/spinon/c11-roadmap-pr1047-pass01');
const root = path.join(base, 'root');
assert.ok(!fs.existsSync(base), `기존 build artifact 보존: ${base}`);
const priorStat = fs.lstatSync(priorRoot);
assert.ok(priorStat.isDirectory() && !priorStat.isSymbolicLink(), '안전하지 않은 선택 Rspress root');

function assertNoSymlinks(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const child = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `root 안 symlink 거부: ${path.relative(repo, child)}`);
    if (entry.isDirectory()) assertNoSymlinks(child);
  }
}
assertNoSymlinks(priorRoot);
fs.mkdirSync(base, { recursive: false });
fs.cpSync(priorRoot, root, { recursive: true, errorOnExist: true, force: false });

function copy(sourceRelative, routeRelative) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `안전하지 않은 source: ${sourceRelative}`);
  const destination = path.join(root, routeRelative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination);
  assert.deepEqual(fs.readFileSync(destination), fs.readFileSync(source), `copy 불일치: ${sourceRelative}`);
}

for (const [source, route] of [
  ['spec/STATUS.md', 'STATUS.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['plan/c11-root-html-grid-auto-row-list.md', 'plan/c11-root-html-grid-auto-row-list.md'],
  ['plan/c11-root-html-grid-auto-row-percentage-list.md', 'plan/c11-root-html-grid-auto-row-percentage-list.md'],
  ['plan/c11-root-html-grid-runtime.md', 'plan/c11-root-html-grid-runtime.md'],
  ['plan/c11-root-html-grid-auto-row-percentage-list-plan-review-20.md', 'plan/c11-root-html-grid-auto-row-percentage-list-plan-review-20.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/public/roadmap.svg', 'public/roadmap.svg'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/README.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/README.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/implementation-runtime-review-20.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/implementation-runtime-review-20.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/pr-diff-review-30.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/pr-diff-review-30.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/merge-PR1047.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/merge-PR1047.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/roadmap-postmerge-docs-review-30.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/roadmap-postmerge-docs-review-30.md'],
  ['spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/roadmap-source-pr-diff-review-30.md', 'internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/roadmap-source-pr-diff-review-30.md'],
]) copy(source, route);

const config = `import baseConfig from '../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');
const base = path.join(repo, 'build/spinon/c11-roadmap-pr1047-pass01');
export default {
  ...baseConfig,
  root: path.join(base, 'root'),
  outDir: path.join(base, 'site'),
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
fs.writeFileSync(path.join(base, 'rspress.config.ts'), config, { flag: 'wx' });

const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const main = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
const tree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(head, main, 'Tailnet publication source must be latest origin/main');
assert.equal(execFileSync('git', ['status', '--porcelain=v1', '--untracked-files=no'], { cwd: repo, encoding: 'utf8' }).trim(), '', 'tracked source tree must be clean');
const roadmap = fs.readFileSync(path.join(repo, 'spec/public/roadmap.html'), 'utf8');
assert.ok(roadmap.includes('PR #1047 Rebase and merge') && roadmap.includes('72/213'), 'roadmap card/count mismatch');
process.stdout.write(`${JSON.stringify({ sourceHead: head, sourceTree: tree, root: path.relative(repo, root), config: path.relative(repo, path.join(base, 'rspress.config.ts')), roadmapCard: 'PR #1047', officialCount: '72/213' }, null, 2)}\n`);
