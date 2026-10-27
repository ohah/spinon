import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const reportPath = path.join(evidenceDirectory, 'selected-link-check-pass01.json');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const publicationRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite link-check report');
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const selectedRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'publication root is missing');
assert.ok(fs.lstatSync(publicationRoot).isDirectory(), 'existing preview root is missing');

const htmlRoutes = [...selectedBuild.selectedRoutes.map((entry) => entry.path).filter((route) => route.endsWith('.html')), 'roadmap.html'];
const checks = [];
for (const sourceRoute of htmlRoutes) {
  const html = fs.readFileSync(path.join(selectedRoot, sourceRoute), 'utf8');
  for (const match of html.matchAll(/(?:href|src)=["']([^"']+)["']/g)) {
    const url = new URL(match[1], 'https://spinon.local');
    if (url.origin !== 'https://spinon.local' || !url.pathname.startsWith('/spinon/')) continue;
    const relative = url.pathname.slice('/spinon/'.length).split('/').map(decodeURIComponent).join('/');
    assert.ok(relative.split('/').every((part) => part !== '..'), `unsafe internal route from ${sourceRoute}: ${url.pathname}`);
    const candidates = [path.join(selectedRoot, relative), path.join(publicationRoot, relative)];
    const found = candidates.find((candidate) => {
      try { return fs.lstatSync(candidate).isFile(); } catch { return false; }
    });
    checks.push({ source: sourceRoute, href: match[1], path: relative, found_in: found === candidates[0] ? 'selected-publication' : found ? 'existing-preview' : null, bytes: found ? fs.statSync(found).size : null, sha256: found ? sha256(fs.readFileSync(found)) : null });
  }
}
const missing = checks.filter((entry) => !entry.found_in);
const report = {
  pass: 'pass01', sourceHead: selectedBuild.sourceHead, sourceTree: selectedBuild.sourceTree,
  sourceHtmlCount: htmlRoutes.length, checkedInternalLinks: checks.length,
  selectedPublicationLinks: checks.filter((entry) => entry.found_in === 'selected-publication').length,
  existingPreviewLinks: checks.filter((entry) => entry.found_in === 'existing-preview').length,
  missingCount: missing.length, allInternalRoutesResolved: missing.length === 0, missing, checks,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, checks: undefined }, null, 2)}\n`);
assert.equal(missing.length, 0, 'an internal selected-document link does not resolve');
