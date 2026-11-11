import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1351-2026-11-11');
const site = path.join(buildRoot, 'site');
const publication = path.join(buildRoot, 'publication');
const manifestPath = path.join(evidence, 'selected-publication.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(sourceHead, mainHead, 'publication은 현재 origin/main에서만 만든다');
assert.ok(!fs.existsSync(publication), `기존 publication 보존: ${publication}`);
assert.ok(!fs.existsSync(manifestPath), `기존 manifest 보존: ${manifestPath}`);
for (const directory of [site, path.join(buildRoot, 'root')]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `선택 build 경로가 안전한 디렉터리가 아님: ${directory}`);
}

fs.mkdirSync(publication, { recursive: false });
const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `위험한 route: ${route}`);
}
function add(route, bytes, source) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `publication route 내용 충돌: ${route}`);
    return;
  }
  const destination = path.join(publication, route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(destination)), digest, `publication hash 불일치: ${route}`);
  selected.set(route, { path: route, source, bytes: bytes.length, sha256: digest });
}
function addSiteFile(route) {
  safeRoute(route);
  const filename = path.join(site, route);
  const stat = fs.lstatSync(filename);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `build source가 일반 파일이 아님: ${route}`);
  add(route, fs.readFileSync(filename), `rspress:${route}`);
}
function addTree(relative) {
  safeRoute(relative);
  const directory = path.join(site, relative);
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `build tree가 안전한 디렉터리가 아님: ${relative}`);
  function visit(current, route) {
    for (const entry of fs.readdirSync(current, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const filename = path.join(current, entry.name);
      const childRoute = `${route}/${entry.name}`;
      assert.ok(!entry.isSymbolicLink(), `publication symlink 거부: ${childRoute}`);
      if (entry.isDirectory()) visit(filename, childRoute);
      else if (entry.isFile()) add(childRoute, fs.readFileSync(filename), `rspress:${childRoute}`);
      else throw new Error(`지원하지 않는 build 항목: ${childRoute}`);
    }
  }
  visit(directory, relative);
}

const roadmap = execFileSync('git', ['show', `${sourceHead}:spec/public/roadmap.html`], { cwd: repo, maxBuffer: 8 * 1024 * 1024 });
add('roadmap.html', roadmap, 'git:spec/public/roadmap.html');
const status = execFileSync('git', ['show', `${sourceHead}:spec/STATUS.md`], { cwd: repo, maxBuffer: 8 * 1024 * 1024 });
add('STATUS.md', status, 'git:spec/STATUS.md');
add('spec/STATUS.md', status, 'git:spec/STATUS.md');
add('logo.svg', execFileSync('git', ['show', `${sourceHead}:spec/public/logo.svg`], { cwd: repo }), 'git:spec/public/logo.svg');

for (const route of [
  'index.html',
  'STATUS.html',
  'spec/STATUS.html',
  'icon.svg',
  'internal/0059-c11-grid-layout.html',
  'spec/internal/0059-c11-grid-layout.html',
  'plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.html',
  'plan/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum.html',
  'plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-plan-review-20.html',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.html',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum.html',
  'spec/plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-plan-review-20.html',
]) addSiteFile(route);

for (const alias of ['internal/evidence', 'spec/internal/evidence']) {
  addTree(`${alias}/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11`);
  addTree(`${alias}/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-runtime-2026-11-11`);
  addTree(`${alias}/roadmap-tailnet-sync-pr1350-2026-11-11`);
}
addTree('static');

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const manifest = {
  sourceHead,
  sourceTree: execFileSync('git', ['rev-parse', `${sourceHead}^{tree}`], { cwd: repo, encoding: 'utf8' }).trim(),
  selectedRootManifest: 'spec/internal/evidence/roadmap-tailnet-sync-pr1351-2026-11-11/selected-root.json',
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, routes: undefined }, null, 2)}\n`);
