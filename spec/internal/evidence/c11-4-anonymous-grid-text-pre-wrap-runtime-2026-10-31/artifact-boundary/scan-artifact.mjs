import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';

const artifact = resolve(process.argv[2] ?? 'target/release/libspinon_ffi.a');
const mode = process.argv[3];
assert.ok(mode === 'product' || mode === 'fixture', 'expected product or fixture scan mode');
const bytes = readFileSync(artifact);
const source = bytes.toString('latin1');
const sysroot = spawnSync('mise', ['exec', '--', 'rustc', '--print', 'sysroot'], {
  encoding: 'utf8',
});
assert.equal(sysroot.status, 0, sysroot.stderr);
const version = spawnSync('mise', ['exec', '--', 'rustc', '-vV'], { encoding: 'utf8' });
assert.equal(version.status, 0, version.stderr);
const host = version.stdout.match(/^host: (.+)$/m)?.[1];
assert.ok(host, 'rustc -vV must report the active host triple');
const nmPath = join(sysroot.stdout.trim(), 'lib/rustlib', host, 'bin/llvm-nm');
const nm = spawnSync(nmPath, ['--defined-only', '--extern-only', artifact], {
  encoding: 'utf8',
  maxBuffer: 128 * 1024 * 1024,
});
assert.equal(nm.status, 0, nm.stderr ?? nm.error?.message ?? 'llvm-nm failed');

const hostSymbols = nm.stdout.split('\n').filter((line) => (
  /_?spinon_runtime_gpu_host_(?:new|eval)_c11_4_grid_track_sizing_fixture/.test(line)
));
const markerNames = [
  'anonymous-normal-max-content-spaces',
  'anonymous-pre-wrap-max-content-spaces',
  'anonymous-pre-wrap-comment-split-max-content',
  'anonymous-pre-wrap-min-content-spaces',
  'anonymous-pre-wrap-soft-wrap',
  'anonymous-pre-wrap-forced-break',
  'anonymous-pre-wrap-whitespace-before-block',
];
const sceneMarkers = Object.fromEntries(markerNames.map((marker) => [marker, source.includes(marker)]));

if (mode === 'product') {
  assert.ok(Object.values(sceneMarkers).every((present) => !present),
    'product archive must not contain any new PreWrap scene marker');
  assert.equal(hostSymbols.length, 0, 'product archive must not expose C11.4 fixture host exports');
} else {
  assert.ok(Object.values(sceneMarkers).every(Boolean),
    'fixture archive must contain every new PreWrap scene marker');
  assert.equal(hostSymbols.length, 2, 'fixture archive must expose both C11.4 fixture host exports');
}

console.log(JSON.stringify({
  artifact,
  mode,
  bytes: bytes.length,
  sha256: createHash('sha256').update(bytes).digest('hex'),
  sceneMarkers,
  c114HostFfiSymbols: hostSymbols,
}, null, 2));
