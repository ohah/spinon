import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';

const artifact = resolve(process.argv[2] ?? 'target/release/libspinon_ffi.a');
const mode = process.argv[3] ?? 'default';
assert.ok(['default', 'fixture'].includes(mode), `Unknown artifact mode: ${mode}`);
const bytes = readFileSync(artifact);
const source = bytes.toString('latin1');
const expectedValuesPath = resolve(
  'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/expected-values.json',
);
const expectedValues = JSON.parse(readFileSync(expectedValuesPath, 'utf8'));
const markers = expectedValues.cases.map(({ id }) => `text-indent-flags-${id}`);
assert.equal(markers.length, 20, 'Expected the fixed 20-scene flags matrix');

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
const sceneMarkers = Object.fromEntries(markers.map((marker) => [marker, source.includes(marker)]));
const expectedPresent = mode === 'fixture';
assert.equal(hostSymbols.length, expectedPresent ? 2 : 0,
  `${mode}: unexpected C11.4 host FFI symbol count`);
assert.equal(Object.values(sceneMarkers).filter(Boolean).length, expectedPresent ? 20 : 0,
  `${mode}: unexpected text-indent flags scene payload count`);

console.log(JSON.stringify({
  mode,
  artifact,
  bytes: bytes.length,
  sha256: createHash('sha256').update(bytes).digest('hex'),
  sceneMarkerCount: Object.values(sceneMarkers).filter(Boolean).length,
  sceneMarkers,
  c114HostFfiSymbols: hostSymbols,
  rustc: version.stdout.trim().split('\n').slice(0, 6),
  llvmNm: nmPath,
}, null, 2));
