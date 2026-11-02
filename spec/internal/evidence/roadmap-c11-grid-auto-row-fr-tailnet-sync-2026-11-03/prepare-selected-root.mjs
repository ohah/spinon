import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const root = path.join(repo, 'build/spinon/c11-root-html-grid-auto-row-fr-roadmap-sync-2026-11-03/root');
assert.ok(!fs.existsSync(root), `기존 selected root 보존: ${root}`);
fs.mkdirSync(path.dirname(root), { recursive: true });
fs.mkdirSync(root, { recursive: false });

function copy(sourceRelative, route) {
  const source = path.join(repo, sourceRelative);
  let current = repo;
  for (const part of sourceRelative.split('/')) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `source symlink 거부: ${sourceRelative}`);
  }
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile(), `source가 일반 파일이 아님: ${sourceRelative}`);
  const destination = path.join(root, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination, fs.constants.COPYFILE_EXCL);
}
function trackedFiles(prefix) {
  return execFileSync('git', ['ls-files', '-z', '--', prefix], { cwd: repo })
    .toString('utf8').split('\0').filter(Boolean);
}
function copyTrackedTree(prefix, destinationPrefix) {
  const paths = trackedFiles(prefix);
  assert.ok(paths.length > 0, `tracked file이 없음: ${prefix}`);
  for (const source of paths) {
    const suffix = source.slice(prefix.length).replace(/^\//, '');
    copy(source, path.posix.join(destinationPrefix, suffix));
  }
}

fs.writeFileSync(path.join(root, 'index.md'), '# C11 Root HTML grid-auto-rows roadmap preview\n');
copy('spec/STATUS.md', 'STATUS.md');
copy('plan/c11-root-html-grid-auto-row-fr.md', 'plan/c11-root-html-grid-auto-row-fr.md');
copy('spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md');
copy('spec/public/icon.svg', 'public/icon.svg');
copy('spec/public/logo.svg', 'public/logo.svg');
copy('spec/public/roadmap.svg', 'public/roadmap.svg');
copyTrackedTree(
  'spec/internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03',
  'internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03',
);
copyTrackedTree(
  'spec/internal/evidence/c11-root-html-grid-auto-row-fr-simulators-2026-11-03',
  'internal/evidence/c11-root-html-grid-auto-row-fr-simulators-2026-11-03',
);
copy('tests/fixtures/css/c11/root-html-grid-auto-row-fr-runtime.html', 'tests/fixtures/css/c11/root-html-grid-auto-row-fr-runtime.html');
process.stdout.write(`${JSON.stringify({ root: path.relative(repo, root), selectedRootFiles: trackedFiles('spec/internal/evidence/c11-root-html-grid-auto-row-fr-simulators-2026-11-03').length + trackedFiles('spec/internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03').length + 7 }, null, 2)}\n`);
