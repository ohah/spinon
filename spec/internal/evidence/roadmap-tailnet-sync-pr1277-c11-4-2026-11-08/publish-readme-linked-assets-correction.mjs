import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const localBase = 'http://127.0.0.1:8768/spinon/';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const reportPath = path.join(evidence, 'readme-links-route-correction-pass04.json');
const servedPath = path.join(evidence, 'readme-links-served-pass04.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const basename = path.basename(evidence);
for (const target of [reportPath, servedPath]) assert.ok(!fs.existsSync(target), `기존 evidence 보존: ${target}`);
const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale가 Running이어야 함');
const serverPid = Number(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim());
assert.ok(Number.isInteger(serverPid), 'Preview server PID를 확인할 수 없음');
const serverArgs = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
assert.ok(serverArgs.includes('tailnet-preview-server.mjs'), '예상한 Preview server가 아님');

const readme = fs.readFileSync(path.join(evidence, 'README.md'), 'utf8');
const linkedSources = new Map();
for (const [, rawHref] of readme.matchAll(/(?<!!)\[[^\]]*\]\(([^)]+)\)/g)) {
  const href = rawHref.split('#', 1)[0];
  if (!href || /^(?:https?:|mailto:|#)/i.test(href)) continue;
  const source = path.resolve(evidence, decodeURIComponent(href));
  const relative = path.relative(evidence, source);
  assert.ok(relative && !relative.startsWith('..') && !path.isAbsolute(relative), `README link escapes evidence directory: ${href}`);
  const extension = path.extname(source).toLowerCase();
  if (extension === '.md') continue;
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `README link source is not a regular file: ${href}`);
  const route = `internal/evidence/${basename}/${relative.split(path.sep).join('/')}`;
  const bytes = fs.readFileSync(source);
  const prior = linkedSources.get(route);
  if (prior) assert.equal(prior.sha256, sha256(bytes), `README source route conflict: ${route}`);
  else linkedSources.set(route, { route, source, bytes, sha256: sha256(bytes) });
}

function ensureDirectory(directory) {
  const relative = path.relative(previewRoot, directory);
  assert.ok(relative && !relative.startsWith('..') && !path.isAbsolute(relative), 'target directory escapes preview root');
  let current = previewRoot;
  for (const part of relative.split(path.sep)) {
    current = path.join(current, part);
    if (!fs.existsSync(current)) fs.mkdirSync(current);
    const stat = fs.lstatSync(current);
    assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe target directory: ${current}`);
  }
}

const items = [...linkedSources.values()].sort((a, b) => a.route.localeCompare(b.route));
const states = [];
for (const item of items) {
  const target = path.join(previewRoot, item.route);
  if (fs.existsSync(target)) {
    const stat = fs.lstatSync(target);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe existing target: ${item.route}`);
    const currentHash = sha256(fs.readFileSync(target));
    assert.equal(currentHash, item.sha256, `existing README link target differs: ${item.route}`);
    states.push({ route: item.route, source: path.relative(repo, item.source).split(path.sep).join('/'), state: 'already-identical', bytes: item.bytes.length, sha256: item.sha256 });
  } else {
    states.push({ route: item.route, source: path.relative(repo, item.source).split(path.sep).join('/'), state: 'add', bytes: item.bytes.length, sha256: item.sha256 });
  }
}

for (const item of items) {
  const target = path.join(previewRoot, item.route);
  const existing = states.find((state) => state.route === item.route);
  if (existing.state === 'already-identical') continue;
  assert.ok(!fs.existsSync(target), `target appeared after link preflight: ${item.route}`);
  ensureDirectory(path.dirname(target));
  const temporary = path.join(path.dirname(target), `.${path.basename(target)}.link-fix-${randomUUID()}.tmp`);
  const fd = fs.openSync(temporary, 'wx', 0o644);
  try { fs.writeFileSync(fd, item.bytes); fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
  fs.linkSync(temporary, target);
  fs.unlinkSync(temporary);
  const dirFd = fs.openSync(path.dirname(target), 'r');
  try { fs.fsyncSync(dirFd); } finally { fs.closeSync(dirFd); }
  assert.equal(sha256(fs.readFileSync(target)), item.sha256, `target hash mismatch: ${item.route}`);
}

const responses = [];
for (const base of [localBase, remoteBase]) {
  for (const item of items) {
    const response = await fetch(`${base}${encodeURI(item.route)}`, { cache: 'no-store', signal: AbortSignal.timeout(30000) });
    const bytes = Buffer.from(await response.arrayBuffer());
    assert.equal(response.status, 200, `${base}${item.route} returned ${response.status}`);
    assert.equal(sha256(bytes), item.sha256, `${base}${item.route} hash mismatch`);
    responses.push({ url: `${base}${item.route}`, status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
  }
}
const report = {
  readmeLinkCount: items.length,
  added: states.filter((item) => item.state === 'add').length,
  alreadyIdentical: states.filter((item) => item.state === 'already-identical').length,
  routes: states,
  initialEvidenceCopyHadWrongPrefix: `/${basename}/`,
  correctedPrefix: `/internal/evidence/${basename}/`,
  tailscaleBackendState: tailscale.BackendState,
  previewServerPid: serverPid,
  serverRestarted: false,
  noDeletes: true,
};
const served = { requests: responses.length, allHttp200: true, allHashesMatch: true, checks: responses };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(servedPath, `${JSON.stringify(served, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined, servedRequests: responses.length, allHttp200: true, allHashesMatch: true }, null, 2)}\n`);
