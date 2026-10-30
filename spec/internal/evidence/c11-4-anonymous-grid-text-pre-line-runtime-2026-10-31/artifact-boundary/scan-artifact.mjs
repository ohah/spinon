import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';

const artifact = resolve(process.argv[2] ?? 'target/release/libspinon_ffi.a');
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
const nm = spawnSync(nmPath, [
  '--defined-only', '--extern-only', artifact,
], { encoding: 'utf8', maxBuffer: 128 * 1024 * 1024 });
assert.equal(nm.status, 0, nm.stderr ?? nm.error?.message ?? 'llvm-nm failed');

const hostSymbols = nm.stdout.split('\n').filter((line) => (
  /_?spinon_runtime_gpu_host_(?:new|eval)_c11_4_grid_track_sizing_fixture/.test(line)
));
const markerNames = [
  'anonymous-normal-newline-max-content',
  'anonymous-pre-line-max-content',
];

console.log(JSON.stringify({
  artifact,
  bytes: bytes.length,
  sha256: createHash('sha256').update(bytes).digest('hex'),
  sceneMarkers: Object.fromEntries(markerNames.map((marker) => [marker, source.includes(marker)])),
  c114HostFfiSymbols: hostSymbols,
}, null, 2));
