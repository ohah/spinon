import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const [prepareArg, outputArg] = process.argv.slice(2);
if (!prepareArg || !outputArg) throw new Error('usage: node write-preview-inputs-manifest.mjs PREPARE_ROOT_JSON OUTPUT_JSON');
const prep = JSON.parse(fs.readFileSync(path.resolve(repoRoot, prepareArg), 'utf8'));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const files = [];
function addSource(sourceRelative, destinationRelative) {
  const sourcePath = path.join(repoRoot, sourceRelative);
  const stat = fs.lstatSync(sourcePath);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected input: ${sourceRelative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(sourcePath).sort()) addSource(`${sourceRelative}/${name}`, `${destinationRelative}/${name}`);
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected input: ${sourceRelative}`);
  const bytes = fs.readFileSync(sourcePath);
  files.push({ source: sourceRelative, destination: destinationRelative, bytes: bytes.byteLength, sha256: hash(bytes) });
}
for (const mapping of prep.selected_sources) addSource(mapping.source, mapping.destination);
const outputPath = path.resolve(repoRoot, outputArg);
fs.writeFileSync(outputPath, `${JSON.stringify({ run: prep.run, source_branch: prep.source_branch, source_commit: prep.source_commit,
  selected_source_file_count: files.length, total_bytes: files.reduce((sum, file) => sum + file.bytes, 0), files }, null, 2)}\n`);
process.stdout.write(`${JSON.stringify({ run: prep.run, selected_source_file_count: files.length, total_bytes: files.reduce((sum, file) => sum + file.bytes, 0), output: path.relative(repoRoot, outputPath) })}\n`);
