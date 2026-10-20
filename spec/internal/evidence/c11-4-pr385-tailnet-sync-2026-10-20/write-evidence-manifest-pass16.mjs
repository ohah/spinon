import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const root = path.dirname(fileURLToPath(import.meta.url));
const output = path.join(root, 'SHA256SUMS-evidence-pass16.txt');
const files = [];

function walk(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error('symlink in evidence tree: ' + fullPath);
    if (entry.isDirectory()) walk(fullPath);
    else if (entry.isFile() && fullPath !== output) files.push(fullPath);
    else if (!entry.isFile()) throw new Error('unsupported evidence entry: ' + fullPath);
  }
}

walk(root);
files.sort();
const lines = files.map((fullPath) => {
  const relative = path.relative(root, fullPath).split(path.sep).join('/');
  const digest = createHash('sha256').update(fs.readFileSync(fullPath)).digest('hex');
  return digest + '  ' + relative;
});
fs.writeFileSync(output, lines.join('\n') + '\n');
process.stdout.write(JSON.stringify({ output, file_count: files.length }, null, 2) + '\n');
