import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const [preflightArg, targetArg, outputArg] = process.argv.slice(2);
const pass = process.env.SPINON_TAILNET_RUN;
if (!preflightArg || !targetArg || !outputArg || pass !== 'pass1') throw new Error('usage: SPINON_TAILNET_RUN=pass1 node capture-open-handles.mjs PREFLIGHT_JSON PREVIEW_ROOT OUTPUT_JSON');
const targetRoot = path.resolve(targetArg);
if (targetRoot !== path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview')) throw new Error('preview root does not match the approved SpinonPreview path');
const preflight = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
if (preflight.pass !== pass || preflight.preview_root_relative !== 'SpinonPreview') throw new Error('preflight identity does not match this Tailnet sync');
const replacements = preflight.files.filter((entry) => entry.status === 'replace');
const inspected = replacements.map((entry) => {
  const result = spawnSync('lsof', ['-nP', '-t', path.join(targetRoot, entry.path)], { encoding: 'utf8' });
  if (result.error) throw result.error;
  const pids = [...new Set((result.stdout ?? '').split(/\s+/).filter(Boolean))];
  if (result.status !== 0 && !(result.status === 1 && pids.length === 0)) throw new Error(`lsof failed for ${entry.path}: ${result.stderr ?? ''}`);
  return { path: entry.path, open_pids: pids };
});
const openFiles = inspected.filter((entry) => entry.open_pids.length > 0);
const report = { pass, preview_root_relative: preflight.preview_root_relative, replacement_count: replacements.length,
  inspected_count: inspected.length, open_file_count: openFiles.length, open_files: openFiles,
  atomic_rename_preserves_existing_open_descriptors: true, files: inspected };
const outputPath = path.resolve(repoRoot, outputArg);
fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
process.stdout.write(`${JSON.stringify({ pass, replacement_count: replacements.length, open_file_count: openFiles.length, open_files: openFiles })}\n`);
