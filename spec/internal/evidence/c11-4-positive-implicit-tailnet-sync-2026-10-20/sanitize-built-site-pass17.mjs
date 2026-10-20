import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (run !== 'pass17') throw new Error('usage: node sanitize-built-site-pass17.mjs pass17');

const outputRoot = path.join(repoRoot, 'build/spinon', `c11-4-positive-implicit-tailnet-site-${run}`);
const textExtensions = new Set(['.css', '.html', '.js', '.json', '.map', '.svg', '.txt']);
const redacted = [];
const remaining = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      visit(fullPath);
      continue;
    }
    if (!entry.isFile() || !textExtensions.has(path.extname(entry.name))) continue;
    const before = fs.readFileSync(fullPath, 'utf8');
    const after = before
      .replace(/\/Users\/[^/\s"'<>:,)}\]]+(?:\/[^\s"'<>:,)}\]]*)*/g, '/host-path-redacted')
      .replace(/\/private\/tmp(?:\/[^\s"'<>:,)}\]]*)*/g, '/worktree-path-redacted');
    if (after !== before) {
      fs.writeFileSync(fullPath, after, 'utf8');
      redacted.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
    }
    if (/\/Users\/[^/\s"'<>:,)}\]]|\/private\/tmp(?:\/|$)/.test(after)) {
      remaining.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
    }
  }
}
visit(outputRoot);
if (remaining.length > 0) throw new Error(`host path remains in ${remaining.join(', ')}`);
process.stdout.write(`${JSON.stringify({
  run,
  output_root_relative: path.relative(repoRoot, outputRoot).split(path.sep).join('/'),
  redacted_file_count: redacted.length,
  redacted_files: redacted.sort(),
  remaining_host_path_files: remaining,
}, null, 2)}\n`);
