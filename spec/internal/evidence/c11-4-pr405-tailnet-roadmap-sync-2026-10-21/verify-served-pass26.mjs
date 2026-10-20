import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const siteRoot = path.join(repoRoot, 'build/spinon/c11-4-pr405-tailnet-site-pass26');
const baseUrl = process.env.SPINON_TAILNET_BASE ?? 'https://macstudio.tailed42f2.ts.net/spinon/';
const outputPath = path.join(evidenceDir, 'served-verification-pass26.json');

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

const routes = [
  { path: 'roadmap.html', requiredText: ['PR #405', 'SM-S731N'] },
  { path: 'STATUS.html', requiredText: ['PR #405', '53/53'] },
  { path: 'spec/STATUS.html', requiredText: ['PR #405', '53/53'] },
  { path: 'plan/c11-4-auto-fit-axis-physical-android.html', requiredText: ['PR #405', '53/53'] },
  { path: 'spec/plan/c11-4-auto-fit-axis-physical-android.html', requiredText: ['PR #405', '53/53'] },
  { path: 'internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21/README.html', requiredText: ['53/53', 'SM-S731N'] },
  { path: 'spec/internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21/README.html', requiredText: ['53/53', 'SM-S731N'] },
  { path: 'plan/c13-3-ios-recognizer-event-pairing.html', requiredText: ['recognizer', 'event'] },
  { path: 'internal/evidence/c13-3-android-physical-2026-10-21/README.html', requiredText: ['SM-S731N', 'ACTION_DOWN'] },
  { path: 'internal/evidence/c13-3-ios-recognizer-event-pairing-2026-10-21/README.html', requiredText: ['iPad Pro 13-inch', '2,384'] },
];

for (const page of [
  'internal/evidence/c11-4-auto-fit-min-size-axis-physical-android-2026-10-21/README.html',
  'internal/evidence/c13-3-android-physical-2026-10-21/README.html',
  'internal/evidence/c13-3-ios-recognizer-event-pairing-2026-10-21/README.html',
]) {
  const html = fs.readFileSync(path.join(siteRoot, page), 'utf8');
  const imageRoutes = [...html.matchAll(/(?:href|src)="\/spinon\/(static\/image\/[^" ]+\.png)"/g)].map((match) => match[1]);
  if (imageRoutes.length === 0) throw new Error(`no rendered image routes found in ${page}`);
  for (const imageRoute of new Set(imageRoutes)) routes.push({ path: imageRoute, requiredText: [] });
}

const results = [];
for (const route of routes) {
  const localPath = path.join(siteRoot, route.path);
  const localBytes = fs.readFileSync(localPath);
  const response = await fetch(new URL(route.path, baseUrl.endsWith('/') ? baseUrl : `${baseUrl}/`));
  const servedBytes = Buffer.from(await response.arrayBuffer());
  const contentType = response.headers.get('content-type');
  const localSha256 = sha256(localBytes);
  const servedSha256 = sha256(servedBytes);
  const textChecks = route.requiredText.map((text) => ({ text, present: servedBytes.toString('utf8').includes(text) }));
  const result = {
    route: route.path,
    status: response.status,
    final_url: response.url,
    content_type: contentType,
    local_bytes: localBytes.byteLength,
    served_bytes: servedBytes.byteLength,
    local_sha256: localSha256,
    served_sha256: servedSha256,
    matched: response.status === 200 && localBytes.byteLength === servedBytes.byteLength && localSha256 === servedSha256,
    text_checks: textChecks,
  };
  results.push(result);
  if (!result.matched || textChecks.some((check) => !check.present)) {
    throw new Error(`served verification failed for ${route.path}: ${JSON.stringify(result)}`);
  }
}

const summary = {
  base_url: baseUrl,
  verified_route_count: results.length,
  all_routes_verified: results.every((result) => result.matched && result.text_checks.every((check) => check.present)),
  routes: results,
};
fs.writeFileSync(outputPath, `${JSON.stringify(summary, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  base_url: baseUrl,
  verified_route_count: summary.verified_route_count,
  all_routes_verified: summary.all_routes_verified,
  routes: results.map(({ route, status, content_type, local_bytes, local_sha256, matched }) => ({ route, status, content_type, local_bytes, local_sha256, matched })),
  output: path.relative(repoRoot, outputPath).split(path.sep).join('/'),
}, null, 2)}\n`);
