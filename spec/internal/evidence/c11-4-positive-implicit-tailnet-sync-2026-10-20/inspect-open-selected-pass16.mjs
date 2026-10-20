import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const [preflightArg, targetArg] = process.argv.slice(2);
if (!preflightArg || !targetArg) {
  throw new Error('usage: node inspect-open-selected.mjs PREFLIGHT_JSON PREVIEW_ROOT');
}

const preflight = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
const targetRoot = path.resolve(targetArg);
const replacements = preflight.files.filter((entry) => entry.status === 'replace');
const expectedPaths = new Map(replacements.map((entry) => [
  path.resolve(targetRoot, entry.path), entry.path,
]));
let output = '';
let exitStatus = 0;
try {
  output = execFileSync('lsof', ['-Fpcfn', '--', ...expectedPaths.keys()], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
} catch (error) {
  output = error.stdout ?? '';
  exitStatus = error.status ?? 1;
  if (exitStatus !== 1) throw error;
}

const descriptors = [];
let current = {};
for (const line of output.split('\n')) {
  if (!line) continue;
  const field = line[0];
  const value = line.slice(1);
  if (field === 'p') current = { pid: value };
  else if (field === 'c') current.command = value;
  else if (field === 'f') current.fd = value;
  else if (field === 'n') {
    const relative = expectedPaths.get(path.resolve(value));
    if (relative) descriptors.push({ ...current, path: relative });
  }
}

const openPaths = [...new Set(descriptors.map((entry) => entry.path))].sort();
process.stdout.write(`${JSON.stringify({
  pass: preflight.pass,
  preview_root_relative: preflight.preview_root_relative,
  checked_path_count: expectedPaths.size,
  open_selected_path_count: openPaths.length,
  open_selected_paths: openPaths,
  descriptors,
  lsof_exit_status: exitStatus,
  replacement_method: 'write a same-directory temporary file and atomically rename it over the target',
}, null, 2)}\n`);
