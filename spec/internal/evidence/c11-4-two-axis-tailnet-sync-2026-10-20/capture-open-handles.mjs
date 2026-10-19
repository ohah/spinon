import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const [preflightArg, targetArg, outputArg] = process.argv.slice(2);
if (!preflightArg || !targetArg || !outputArg) {
  throw new Error('usage: node capture-open-handles.mjs PREFLIGHT_JSON PREVIEW_ROOT OUTPUT_JSON');
}

const expectedTarget = path.resolve(os.homedir(), 'Library/Application Support/SpinonPreview');
const targetRoot = path.resolve(targetArg);
if (targetRoot !== expectedTarget) throw new Error('preview root does not match the approved SpinonPreview path');

const preflight = JSON.parse(fs.readFileSync(path.resolve(repoRoot, preflightArg), 'utf8'));
if (!['pass12', 'pass13'].includes(preflight.pass) || preflight.preview_root_relative !== 'SpinonPreview') {
  throw new Error('preflight identity does not match this Tailnet sync');
}

const replacements = preflight.files.filter((entry) => entry.status === 'replace');
const inspected = replacements.map((entry) => {
  const targetPath = path.join(targetRoot, entry.path);
  const result = spawnSync('lsof', ['-t', targetPath], { encoding: 'utf8' });
  if (result.error) throw result.error;
  const pids = [...new Set((result.stdout ?? '').split(/\s+/).filter(Boolean))];
  if (result.status !== 0 && !(result.status === 1 && pids.length === 0)) {
    throw new Error(`lsof failed for ${entry.path}: ${result.stderr ?? ''}`);
  }
  return { path: entry.path, open_pids: pids };
});

const openFiles = inspected.filter((entry) => entry.open_pids.length > 0);
const report = {
  pass: preflight.pass,
  preview_root_relative: preflight.preview_root_relative,
  replacement_count: replacements.length,
  inspected_count: inspected.length,
  open_file_count: openFiles.length,
  all_closed: openFiles.length === 0,
  files: inspected,
};
const outputPath = path.resolve(repoRoot, outputArg);
fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
process.stdout.write(`${JSON.stringify({ replacement_count: report.replacement_count, open_file_count: report.open_file_count, all_closed: report.all_closed })}\n`);
if (!report.all_closed) process.exitCode = 1;
