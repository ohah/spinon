import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (run !== 'pass1') throw new Error('usage: node sanitize-built-output.mjs pass1');
const outputRoot = path.join(repoRoot, 'build/spinon', `c11-3-3-pr393-tailnet-site-${run}`);
const workerSource = path.join(repoRoot, 'node_modules/.bun/flexsearch@0.8.212/node_modules/flexsearch/dist/flexsearch.bundle.module.min.mjs');
if (!fs.existsSync(workerSource)) throw new Error(`locked FlexSearch 0.8.212 worker is missing: ${workerSource}`);
const workerBytes = fs.readFileSync(workerSource);
const workerDestination = path.join(outputRoot, 'static/js/flexsearch.bundle.module.min.mjs');
fs.mkdirSync(path.dirname(workerDestination), { recursive: true });
fs.writeFileSync(workerDestination, workerBytes);

const patchedFiles = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) { visit(fullPath); continue; }
    if (!entry.isFile() || !/\.m?js$/.test(entry.name)) continue;
    const before = fs.readFileSync(fullPath, 'utf8');
    const after = before.replace(/file:\/\/\/Users\/[^"'\s]+\/flexsearch\.bundle\.module\.min\.mjs/g, '/spinon/static/js/flexsearch.bundle.module.min.mjs');
    if (after !== before) { fs.writeFileSync(fullPath, after); patchedFiles.push(path.relative(outputRoot, fullPath).split(path.sep).join('/')); }
  }
}
visit(outputRoot);
if (patchedFiles.length === 0) throw new Error('expected the Rspress FlexSearch worker URL to be present');
process.stdout.write(`${JSON.stringify({ run, worker_source_sha256: await import('node:crypto').then(({ createHash }) => createHash('sha256').update(workerBytes).digest('hex')),
  worker_destination: path.relative(outputRoot, workerDestination).split(path.sep).join('/'), worker_bytes: workerBytes.byteLength, patched_files: patchedFiles }, null, 2)}\n`);
