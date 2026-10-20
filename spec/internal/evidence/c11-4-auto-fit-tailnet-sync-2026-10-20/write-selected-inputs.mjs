import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const input = process.argv[2];
if (!input) throw new Error('usage: node write-selected-inputs.mjs PREPARE_ROOT_JSON');
const prepared = JSON.parse(fs.readFileSync(path.resolve(repoRoot, input), 'utf8'));
const sources = [...new Set(prepared.selected_sources.map((entry) => entry.source))].sort();
const files = [];
function walk(relative) {
  const full = path.join(repoRoot, relative);
  const stat = fs.lstatSync(full);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected inputs: ${relative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(full).sort()) walk(path.posix.join(relative, name));
  } else if (stat.isFile()) files.push(relative);
  else throw new Error(`unsupported selected input: ${relative}`);
}
for (const source of sources) walk(source);
const uniqueFiles = [...new Set(files)].sort();
const lines = uniqueFiles.map((relative) => {
  const digest = createHash('sha256').update(fs.readFileSync(path.join(repoRoot, relative))).digest('hex');
  return `${digest}  ${relative}`;
});
const output = 'spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/SHA256SUMS-preview-inputs.txt';
fs.writeFileSync(path.join(repoRoot, output), `${lines.join('\n')}\n`);
process.stdout.write(`${JSON.stringify({ output, source_file_count: uniqueFiles.length, selected_source_count: sources.length }, null, 2)}\n`);
