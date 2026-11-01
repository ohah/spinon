import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-text-indent-status-sync-2026-11-01/pass01';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const delta = JSON.parse(fs.readFileSync(path.join(evidence, 'status-route-delta-pass02.json'), 'utf8'));
const [mode] = process.argv.slice(2);
const reports = {
  preflight: path.join(evidence, 'tailnet-preflight.json'),
  copy: path.join(evidence, 'tailnet-copy-and-backup.json'),
  verify: path.join(evidence, 'tailnet-served-verification.json'),
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const normalizeGeneratedScriptRefs = (bytes) => bytes.toString('utf8')
  .replace(/<link rel="preload" href="[^"]+" as="script">/g, '<link rel="preload" href="<build-hash>" as="script">')
  .replace(/<script defer src="\/spinon\/static\/js\/[^"]+"><\/script>/g, '<script defer src="/spinon/static/js/<build-hash>"></script>');

function assertDirectoryNoSymlink(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}

function sourceBytes(item) {
  const source = path.resolve(repo, item.source);
  assert.ok(source.startsWith(`${repo}${path.sep}`), `repo 밖 source 거부: ${item.route}`);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source가 regular file이 아님: ${item.route}`);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `source 크기 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `source hash 변경: ${item.route}`);
  return bytes;
}

function inspectPath(root, relative) {
  assertDirectoryNoSymlink(root);
  const parts = relative.split('/');
  assert.ok(parts.length > 0 && parts.every((part) => part && part !== '.' && part !== '..' && !part.includes('\\')), `잘못된 route: ${relative}`);
  let current = root;
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink 거부: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent가 디렉터리가 아님: ${relative}`);
      else {
        assert.ok(stat.isFile(), `target가 regular file이 아님: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current), stat };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function inspectServer() {
  const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
  assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
  assert.equal(tailscale.Self?.Online, true, 'Tailscale self가 online이어야 함');
  assert.equal(fs.realpathSync(previewRoot), previewRoot, '미리보기 root는 설정된 실제 디렉터리여야 함');
  const pids = [...new Set(execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim().split(/\s+/).filter(Boolean))];
  assert.equal(pids.length, 1, `8768 listener가 하나여야 함: ${pids.join(',')}`);
  const serverPid = Number(pids[0]);
  const args = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
  assert.ok(args.includes('tailnet-preview-server.mjs'), `예상하지 못한 preview process: ${args}`);
  const local = fetch('http://127.0.0.1:8768/spinon/STATUS.html');
  return { tailscale, serverPid, args, local };
}

function classify(item) {
  const target = inspectPath(previewRoot, item.route);
  const targetHash = target.exists ? sha256(target.bytes) : null;
  if (target.exists && targetHash === item.sha256) return { action: 'same', targetHash, target };
  if (item.previousSha256 !== null) {
    assert.ok(target.exists, `기존 route가 사라짐: ${item.route}`);
    assert.equal(targetHash, item.previousSha256, `target drift가 있어 보존 중단: ${item.route}`);
    return { action: 'replace', targetHash, target };
  }
  assert.ok(!target.exists, `새 route에 기존 파일이 있어 drift 확인 필요: ${item.route}`);
  return { action: 'create', targetHash: null, target };
}

function changedOpenHandles(changed) {
  const existing = changed.filter((item) => item.action === 'replace').map((item) => path.join(previewRoot, item.route));
  if (!existing.length) return [];
  const result = spawnSync('lsof', ['-nP', '--', ...existing], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split('\n').slice(1).filter(Boolean).map((row) => {
    const index = row.indexOf(previewRoot);
    assert.ok(index >= 0, `예상하지 못한 lsof 경로: ${row}`);
    const relative = path.relative(previewRoot, row.slice(index)).split(path.sep).join('/');
    const route = changed.find((item) => item.route === relative);
    assert.ok(route, `선택되지 않은 열린 route: ${relative}`);
    const fields = row.split(/\s+/);
    return { command: fields[0], pid: Number(fields[1]), fd: fields[3], route: relative, currentSha256: route.targetHash, nextSha256: route.sha256 };
  });
}

assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: bun sync-tailnet-delta.mjs preflight|copy|verify');
assert.equal(delta.noDeletes, true);
assertDirectoryNoSymlink(path.join(repo, 'build/spinon/c11-4-text-indent-status-sync-2026-11-01-pass02/site'));
assert.equal(delta.updateRouteCount, delta.routes.length);

if (mode === 'preflight') {
  assert.ok(!fs.existsSync(reports.preflight), '기존 preflight report를 덮어쓰지 않음');
  const server = inspectServer();
  const localResponse = await server.local;
  const localBytes = Buffer.from(await localResponse.arrayBuffer());
  assert.equal(localResponse.status, 200, 'local STATUS page가 HTTP 200이어야 함');
  const checked = delta.routes.map((item) => {
    sourceBytes(item);
    const target = classify(item);
    return { ...item, targetExists: target.target.exists, targetSha256: target.targetHash, target: target.target, action: target.action };
  });
  const changed = checked.filter((item) => item.action !== 'same');
  const openHandles = changedOpenHandles(changed);
  const preservedOpenHandleRoutes = [];
  for (const handle of openHandles) {
    const item = checked.find((entry) => entry.route === handle.route);
    assert.ok(item && item.action === 'replace', `열린 route가 예상한 replace 대상이어야 함: ${handle.route}`);
    assert.equal(
      normalizeGeneratedScriptRefs(item.target.bytes),
      normalizeGeneratedScriptRefs(sourceBytes(item)),
      `열린 route 본문도 바뀌므로 보존 처리 불가: ${handle.route}`,
    );
    preservedOpenHandleRoutes.push(handle.route);
  }
  const uniquePreservedOpenRoutes = [...new Set(preservedOpenHandleRoutes)].sort();
  const counts = Object.fromEntries(['create', 'replace', 'same'].map((action) => [action, checked.filter((item) => item.action === action).length]));
  const report = {
    sourceHead: delta.sourceHead,
    statusSha256: delta.statusSha256,
    tailscaleBackendState: server.tailscale.BackendState,
    selfOnline: server.tailscale.Self.Online,
    selfHostName: server.tailscale.Self.HostName,
    previewPid: server.serverPid,
    previewArgs: server.args,
    localStatusPage: { status: localResponse.status, bytes: localBytes.length, sha256: sha256(localBytes) },
    previewRoot,
    backupRoot,
    routeCount: checked.length,
    counts,
    openHandleCount: openHandles.length,
    preservedOpenHandleRoutes: uniquePreservedOpenRoutes,
    openHandles,
    changedRoutes: changed.map(({ route, source, bytes, sha256: digest, previousSha256, targetExists, targetSha256, action }) => ({ route, source, bytes, sha256: digest, previousSha256, targetExists, targetSha256, action: uniquePreservedOpenRoutes.includes(route) ? 'preserve-open' : action })),
    noDeletes: true,
    serverRestarted: false,
  };
  fs.writeFileSync(reports.preflight, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
} else if (mode === 'copy') {
  assert.ok(fs.existsSync(reports.preflight), 'copy 전에 preflight를 실행해야 함');
  assert.ok(!fs.existsSync(reports.copy), '기존 copy report를 덮어쓰지 않음');
  assert.ok(!fs.existsSync(backupRoot), `기존 backup 경로를 덮어쓰지 않음: ${backupRoot}`);
  const preflight = JSON.parse(fs.readFileSync(reports.preflight, 'utf8'));
  assert.equal(preflight.sourceHead, delta.sourceHead);
  assert.equal(preflight.statusSha256, delta.statusSha256);
  assert.equal(preflight.routeCount, delta.routes.length);
  const preservedOpenRoutes = new Set(preflight.preservedOpenHandleRoutes);
  const checked = delta.routes.map((item) => {
    sourceBytes(item);
    const before = preflight.changedRoutes.find((entry) => entry.route === item.route);
    if (before?.action === 'preserve-open') {
      const target = inspectPath(previewRoot, item.route);
      assert.ok(target.exists, `열린 파일을 유지할 target이 사라짐: ${item.route}`);
      assert.equal(sha256(target.bytes), item.previousSha256, `열린 파일 target이 preflight 이후 변경됨: ${item.route}`);
      assert.equal(normalizeGeneratedScriptRefs(target.bytes), normalizeGeneratedScriptRefs(sourceBytes(item)), `열린 파일 본문이 바뀜: ${item.route}`);
      return { item, target, targetHash: sha256(target.bytes), action: 'preserve-open' };
    }
    const target = classify(item);
    if (before) {
      assert.equal(before.action, target.action, `preflight 이후 action 변경: ${item.route}`);
      assert.equal(before.targetSha256, target.targetHash, `preflight 이후 target 변경: ${item.route}`);
    } else assert.equal(target.action, 'same', `preflight 이후 route drift: ${item.route}`);
    return { item, ...target };
  });
  const changed = checked.filter((item) => item.action !== 'same');
  assert.equal(changed.length, preflight.changedRoutes.length);
  assert.deepEqual([...preservedOpenRoutes].sort(), [...preflight.preservedOpenHandleRoutes].sort());
  const openNow = changedOpenHandles(checked.filter((item) => item.action === 'replace').map(({ item, ...rest }) => ({ ...item, ...rest })));
  assert.ok(openNow.every((handle) => preservedOpenRoutes.has(handle.route)), '새로 열린 교체 대상은 재검토 전 덮어쓰지 않음');
  const backupParent = path.dirname(path.dirname(backupRoot));
  assertDirectoryNoSymlink(backupParent);
  fs.mkdirSync(path.dirname(backupRoot), { recursive: false });
  fs.mkdirSync(backupRoot, { recursive: false });
  const created = [];
  const replaced = [];
  const identical = [];
  const preservedOpen = [];
  for (const current of checked) {
    const { item, action, target } = current;
    if (action === 'preserve-open') {
      preservedOpen.push({ route: item.route, bytes: target.bytes.length, sha256: item.previousSha256, sourceSha256: item.sha256, reason: 'open server handle; normalized HTML body unchanged' });
      continue;
    }
    if (action === 'same') {
      identical.push({ route: item.route, bytes: item.bytes, sha256: item.sha256 });
      continue;
    }
    const source = sourceBytes(item);
    const targetPath = path.join(previewRoot, item.route);
    if (action === 'replace') {
      const backupPath = path.join(backupRoot, item.route);
      fs.mkdirSync(path.dirname(backupPath), { recursive: true });
      fs.copyFileSync(targetPath, backupPath, fs.constants.COPYFILE_EXCL);
      const backup = fs.readFileSync(backupPath);
      assert.equal(sha256(backup), item.previousSha256, `backup hash 불일치: ${item.route}`);
    } else fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    const temporary = `${targetPath}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(temporary, source, { flag: 'wx', mode: target.exists ? target.stat.mode & 0o777 : 0o644 });
    if (target.exists) fs.chmodSync(temporary, target.stat.mode & 0o777);
    fs.renameSync(temporary, targetPath);
    const updated = fs.readFileSync(targetPath);
    assert.equal(sha256(updated), item.sha256, `Tailnet local target hash 불일치: ${item.route}`);
    if (action === 'replace') replaced.push({ route: item.route, bytes: item.bytes, sha256: item.sha256, previousBytes: target.bytes.length, previousSha256: item.previousSha256 });
    else created.push({ route: item.route, bytes: item.bytes, sha256: item.sha256 });
  }
  const report = {
    sourceHead: delta.sourceHead,
    statusSha256: delta.statusSha256,
    previewRoot,
    backupRoot,
    routeCount: checked.length,
    createdCount: created.length,
    replacedCount: replaced.length,
    identicalCount: identical.length,
    preservedOpenCount: preservedOpen.length,
    noDeletes: true,
    serverRestarted: false,
    created,
    replaced,
    identical,
    preservedOpen,
  };
  fs.writeFileSync(reports.copy, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
} else {
  assert.ok(fs.existsSync(reports.copy), 'verify 전에 copy를 실행해야 함');
  assert.ok(!fs.existsSync(reports.verify), '기존 verify report를 덮어쓰지 않음');
  const copied = JSON.parse(fs.readFileSync(reports.copy, 'utf8'));
  assert.equal(copied.sourceHead, delta.sourceHead);
  const preservedOpen = new Map(copied.preservedOpen.map((item) => [item.route, item]));
  const local = [];
  for (const item of delta.routes) {
    sourceBytes(item);
    const target = inspectPath(previewRoot, item.route);
    assert.ok(target.exists, `미리보기 route 누락: ${item.route}`);
    const preserved = preservedOpen.get(item.route);
    const expectedBytes = preserved ? item.previousBytes : item.bytes;
    const expectedHash = preserved ? item.previousSha256 : item.sha256;
    assert.equal(target.bytes.length, expectedBytes, `미리보기 크기 불일치: ${item.route}`);
    assert.equal(sha256(target.bytes), expectedHash, `미리보기 hash 불일치: ${item.route}`);
    if (preserved) assert.equal(normalizeGeneratedScriptRefs(target.bytes), normalizeGeneratedScriptRefs(sourceBytes(item)), `보존된 열린 route body mismatch: ${item.route}`);
    local.push({ route: item.route, bytes: expectedBytes, sha256: expectedHash, sourceSha256: item.sha256, preservedOpen: Boolean(preserved), verified: true });
  }
  const backups = [];
  for (const item of copied.replaced) {
    const backup = inspectPath(backupRoot, item.route);
    assert.ok(backup.exists, `backup 누락: ${item.route}`);
    assert.equal(backup.bytes.length, item.previousBytes, `backup size mismatch: ${item.route}`);
    assert.equal(sha256(backup.bytes), item.previousSha256, `backup hash mismatch: ${item.route}`);
    backups.push({ route: item.route, bytes: item.previousBytes, sha256: item.previousSha256, verified: true });
  }
  const remote = new Array(delta.routes.length);
  let next = 0;
  await Promise.all(Array.from({ length: 12 }, async () => {
    while (true) {
      const index = next++;
      if (index >= delta.routes.length) return;
      const item = delta.routes[index];
      const preserved = preservedOpen.get(item.route);
      const response = await fetch(new URL(item.route.split('/').map(encodeURIComponent).join('/'), remoteBase));
      const bytes = Buffer.from(await response.arrayBuffer());
      const expectedBytes = preserved ? item.previousBytes : item.bytes;
      const expectedHash = preserved ? item.previousSha256 : item.sha256;
      assert.equal(response.status, 200, `HTTPS status ${response.status}: ${item.route}`);
      assert.equal(bytes.length, expectedBytes, `HTTPS size mismatch: ${item.route}`);
      assert.equal(sha256(bytes), expectedHash, `HTTPS hash mismatch: ${item.route}`);
      if (preserved) assert.equal(normalizeGeneratedScriptRefs(bytes), normalizeGeneratedScriptRefs(sourceBytes(item)), `served preserved route body mismatch: ${item.route}`);
      remote[index] = { route: item.route, status: response.status, bytes: bytes.length, sha256: expectedHash, sourceSha256: item.sha256, preservedOpen: Boolean(preserved), verified: true };
    }
  }));
  const report = {
    sourceHead: delta.sourceHead,
    statusSha256: delta.statusSha256,
    previewRoot,
    backupRoot,
    remoteBase,
    routeCount: delta.routes.length,
    verifiedBytes: remote.reduce((sum, item) => sum + item.bytes, 0),
    localTargetsVerified: local.length,
    backupsVerified: backups.length,
    preservedOpenCount: preservedOpen.size,
    allHttp200: remote.every((item) => item.status === 200),
    allRemoteHashesMatch: remote.every((item) => item.verified),
    serverRestarted: false,
    noDeletes: true,
    local,
    backups,
    remote,
    preservedOpen: [...preservedOpen.values()],
  };
  fs.writeFileSync(reports.verify, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ ...report, local: undefined, backups: undefined, remote: undefined }, null, 2)}\n`);
}
