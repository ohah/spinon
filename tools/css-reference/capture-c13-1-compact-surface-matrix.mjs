import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { createServer } from 'node:http';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { decodeRgbaPng } from './read-png-rgba.mjs';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(scriptDirectory, '../..');
const evidenceDirectory = path.join(
  repositoryRoot,
  'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14',
);
const fixtureDirectory = path.join(repositoryRoot, 'tests/fixtures/css/c13');
const agentEvalPath = path.join(scriptDirectory, 'c13-1-compact-surface-matrix-agent-eval.js');
const caseIds = [
  'visible', 'clip-x', 'clip-y', 'clip-both', 'clip-margin-padding',
  'clip-margin-content', 'clip-margin-border', 'clip-margin-hidden',
  'mixed-visible-hidden', 'mixed-clip-auto', 'positioned-contained',
  'positioned-escape', 'nested-clip',
];
const sampleRoles = ['paint', 'target-beyond-frame', 'x-overflow', 'y-overflow'];
const palette = {
  blue: [87, 166, 217, 255],
  dark: [16, 24, 39, 255],
  box: [52, 66, 86, 255],
  red: [227, 84, 78, 255],
};

const expectedSamples = {
  visible: [['target', 'blue'], ['target', 'blue'], ['target', 'blue'], ['target', 'blue']],
  'clip-x': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['target', 'blue']],
  'clip-y': [['target', 'blue'], ['target', 'blue'], ['target', 'blue'], ['section', 'dark']],
  'clip-both': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['section', 'dark']],
  'clip-margin-padding': [['target', 'blue'], ['section', 'dark'], ['target', 'blue'], ['target', 'blue']],
  'clip-margin-content': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['target', 'blue']],
  'clip-margin-border': [['target', 'blue'], ['section', 'dark'], ['target', 'blue'], ['target', 'blue']],
  'clip-margin-hidden': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['section', 'dark']],
  'mixed-visible-hidden': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['section', 'dark']],
  'mixed-clip-auto': [['target', 'blue'], ['section', 'dark'], ['section', 'dark'], ['section', 'dark']],
  'positioned-contained': [['target', 'red'], ['section', 'dark'], ['section', 'dark'], ['section', 'dark']],
  'positioned-escape': [['target', 'red'], ['target', 'red'], ['target', 'red'], ['target', 'red']],
  'nested-clip': [['target', 'blue'], ['box', 'box'], ['target', 'blue'], ['target', 'blue']],
};

const expectedOverflow = {
  visible: ['visible', 'visible', '0px', 'absolute'],
  'clip-x': ['clip', 'visible', '0px', 'absolute'],
  'clip-y': ['visible', 'clip', '0px', 'absolute'],
  'clip-both': ['clip', 'clip', '0px', 'absolute'],
  'clip-margin-padding': ['clip', 'clip', '4px', 'absolute'],
  'clip-margin-content': ['clip', 'clip', 'content-box 4px', 'absolute'],
  'clip-margin-border': ['clip', 'clip', 'border-box 4px', 'absolute'],
  'clip-margin-hidden': ['hidden', 'hidden', 'border-box 4px', 'absolute'],
  'mixed-visible-hidden': ['auto', 'hidden', '0px', 'absolute'],
  'mixed-clip-auto': ['hidden', 'auto', 'border-box 4px', 'absolute'],
  'positioned-contained': ['clip', 'clip', '0px', 'relative'],
  'positioned-escape': ['clip', 'clip', '0px', 'static'],
  'nested-clip': ['visible', 'visible', '0px', 'absolute'],
};

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function rgbToRgba(value) {
  const channels = value.match(/^rgb\((\d+),\s*(\d+),\s*(\d+)\)$/);
  assert.ok(channels, `불투명 RGB 색이 아닙니다: ${value}`);
  return channels.slice(1).map(Number).concat(255);
}

function expectedPoint(caseId, index, role) {
  const x = (index % 3) * 100;
  const y = Math.floor(index / 3) * 20;
  const points = {
    'positioned-contained': [[x + 61, y + 12], [x + 70, y + 12], [x + 66, y + 12], [x + 61, y + 15]],
    'positioned-escape': [[x + 61, y + 12], [x + 70, y + 12], [x + 66, y + 12], [x + 61, y + 16]],
    'nested-clip': [[x + 25, y + 10], [x + 60, y + 10], [x + 52, y + 10], [x + 25, y + 16]],
  }[caseId] ?? [[x + 5, y + 6], [x + 70, y + 6], [x + 66, y + 6], [x + 5, y + 15]];
  return points[sampleRoles.indexOf(role)];
}

function expectedHitId(caseId, owner) {
  if (owner === 'target') return `c131-surface-target-${caseId}`;
  if (owner === 'box') return `c131-surface-box-${caseId}`;
  return `c131-surface-case-${caseId}`;
}

function assertFrame(item, index) {
  const x = (index % 3) * 100;
  const y = Math.floor(index / 3) * 20;
  assert.deepEqual(item.frame.section, { x, y, width: 96, height: 18 }, `${item.id} section`);
  const expectedBox = item.id === 'positioned-escape'
    ? { x, y, width: 64, height: 14 }
    : { x, y: y + 1, width: 64, height: 14 };
  assert.deepEqual(item.frame.box, expectedBox, `${item.id} box`);
  const expectedTarget = item.id === 'positioned-contained'
    ? { x: x + 59, y: y + 9, width: 14, height: 6 }
    : item.id === 'positioned-escape'
      ? { x: x + 58, y: y + 8, width: 30, height: 12 }
      : item.id === 'nested-clip'
        ? { x: x + 23, y: y + 7, width: 56, height: 12 }
        : { x: x + 3, y: y + 4, width: 88, height: 16 };
  assert.deepEqual(item.frame.target, expectedTarget, `${item.id} target`);
  assert.deepEqual(
    [item.frame.overflowX, item.frame.overflowY, item.frame.overflowClipMargin, item.frame.position],
    expectedOverflow[item.id],
    `${item.id} computed overflow and position`,
  );
  if (item.id === 'nested-clip') {
    assert.deepEqual(item.frame.inner, { x: x + 21, y: y + 5, width: 34, height: 10 });
    assert.deepEqual(
      [item.frame.innerOverflowX, item.frame.innerOverflowY, item.frame.innerOverflowClipMargin],
      ['clip', 'visible', '0px'],
    );
  } else {
    assert.equal(item.frame.inner, null);
  }
}

export function validateObservation(observation, root = repositoryRoot) {
  assert.equal(observation.schemaVersion, 1);
  assert.deepEqual(observation.caseIds, caseIds);
  assert.equal(observation.caseCount, 13);
  assert.equal(observation.sampleCount, 52);
  assert.equal(observation.browser.agentBrowserVersion, '0.31.1');
  assert.equal(observation.browser.chromeVersion, 'Google Chrome 155.0.8059.40');
  assert.equal(observation.browser.revision, 'cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}');
  assert.equal(observation.browser.v8, '15.5.35.20');
  assert.match(observation.browser.userAgent, /HeadlessChrome\/155\.0\.0\.0/);
  assert.deepEqual(observation.viewport, { width: 300, height: 100, devicePixelRatio: 1, scrollX: 0, scrollY: 0 });
  assert.deepEqual(observation.root, { x: 0, y: 0, width: 300, height: 100 });
  assert.equal(observation.nodeCount, 41);

  const html = readFileSync(path.join(root, 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.html'));
  const fixture = readFileSync(path.join(root, 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.js'));
  const agentEval = readFileSync(path.join(root, 'tools/css-reference/c13-1-compact-surface-matrix-agent-eval.js'));
  const captureScript = readFileSync(path.join(root, 'tools/css-reference/capture-c13-1-compact-surface-matrix.mjs'));
  assert.equal(observation.sources.htmlSha256, sha256(html));
  assert.equal(observation.sources.fixtureSha256, sha256(fixture));
  assert.equal(observation.sources.agentEvalSha256, sha256(agentEval));
  assert.equal(observation.sources.captureScriptSha256, sha256(captureScript));
  const pngBytes = readFileSync(path.join(root, observation.screenshot.file));
  assert.equal(observation.screenshot.sha256, sha256(pngBytes));
  const png = decodeRgbaPng(pngBytes);
  assert.equal(png.width, 300);
  assert.equal(png.height, 100);
  assert.deepEqual([observation.screenshot.width, observation.screenshot.height], [png.width, png.height]);

  assert.equal(observation.cases.length, caseIds.length);
  const coordinates = new Set();
  for (const [index, item] of observation.cases.entries()) {
    assert.equal(item.id, caseIds[index]);
    assertFrame(item, index);
    assert.equal(item.samples.length, 4);
    for (const [sampleIndex, sample] of item.samples.entries()) {
      assert.equal(sample.role, sampleRoles[sampleIndex]);
      const [expectedX, expectedY] = expectedPoint(item.id, index, sample.role);
      assert.deepEqual([sample.x, sample.y], [expectedX, expectedY], `${item.id}/${sample.role} coordinate`);
      assert.ok(sample.x >= 0 && sample.x < 300 && sample.y >= 0 && sample.y < 100);
      const coordinateKey = `${sample.x},${sample.y}`;
      assert.equal(coordinates.has(coordinateKey), false, `중복 sample 좌표: ${coordinateKey}`);
      coordinates.add(coordinateKey);
      const [owner, color] = expectedSamples[item.id][sampleIndex];
      assert.equal(sample.hitId, expectedHitId(item.id, owner), `${item.id}/${sample.role} hit identity`);
      assert.equal(sample.hitNode, 'fixture-node');
      assert.deepEqual(rgbToRgba(sample.computedBackground), palette[color]);
      assert.deepEqual(sample.computedBackgroundRgba, palette[color]);
      assert.deepEqual(sample.rgba, palette[color], `${item.id}/${sample.role} PNG pixel`);
      assert.deepEqual(png.getPixel(sample.x, sample.y), sample.rgba);
    }
  }
  assert.equal(coordinates.size, 52);
  return true;
}

function normalizeCapture({ envelope, screenshotBytes, agentBrowserVersion, chromeVersion, revision, v8, now }) {
  assert.equal(envelope.success, true, 'agent-browser eval가 실패했습니다.');
  assert.equal(envelope.error, null);
  const browserData = envelope.data.result;
  const metadata = browserData.metadata;
  assert.equal(metadata.href.startsWith('http://127.0.0.1:'), true);
  assert.match(metadata.userAgent, /HeadlessChrome\/155\.0\.0\.0/);
  const viewport = {
    width: metadata.innerWidth,
    height: metadata.innerHeight,
    devicePixelRatio: metadata.devicePixelRatio,
    scrollX: metadata.scrollX,
    scrollY: metadata.scrollY,
  };
  const png = decodeRgbaPng(screenshotBytes);
  const cases = browserData.cases.map((item) => ({
    ...item,
    samples: item.samples.map((sample) => ({
      ...sample,
      computedBackgroundRgba: rgbToRgba(sample.computedBackground),
      rgba: png.getPixel(sample.x, sample.y),
    })),
  }));
  const html = readFileSync(path.join(fixtureDirectory, 'c13-1-compact-surface-matrix.html'));
  const fixture = readFileSync(path.join(fixtureDirectory, 'c13-1-compact-surface-matrix.js'));
  const agentEval = readFileSync(agentEvalPath);
  const captureScript = readFileSync(fileURLToPath(import.meta.url));
  const observation = {
    schemaVersion: 1,
    capturedAt: now,
    sourceUrl: metadata.href,
    sources: {
      html: 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.html',
      htmlSha256: sha256(html),
      fixture: 'tests/fixtures/css/c13/c13-1-compact-surface-matrix.js',
      fixtureSha256: sha256(fixture),
      agentEval: 'tools/css-reference/c13-1-compact-surface-matrix-agent-eval.js',
      agentEvalSha256: sha256(agentEval),
      captureScript: 'tools/css-reference/capture-c13-1-compact-surface-matrix.mjs',
      captureScriptSha256: sha256(captureScript),
    },
    browser: { agentBrowserVersion, chromeVersion, revision, v8, userAgent: metadata.userAgent },
    viewport,
    root: metadata.root,
    nodeCount: metadata.nodeCount,
    caseIds,
    caseCount: cases.length,
    sampleCount: cases.reduce((count, item) => count + item.samples.length, 0),
    cases,
    screenshot: {
      file: 'spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/chrome155-surface.png',
      sha256: sha256(screenshotBytes),
      width: png.width,
      height: png.height,
    },
  };
  validateObservation(observation);
  return observation;
}

async function cli(args, input) {
  return new Promise((resolve, reject) => {
    const child = spawn('agent-browser', args, { cwd: repositoryRoot, stdio: ['pipe', 'pipe', 'pipe'] });
    let stdout = '';
    let stderr = '';
    child.stdout.setEncoding('utf8').on('data', (chunk) => { stdout += chunk; });
    child.stderr.setEncoding('utf8').on('data', (chunk) => { stderr += chunk; });
    child.once('error', reject);
    child.once('close', (code, signal) => {
      if (code === 0) resolve(stdout);
      else reject(new Error(`agent-browser ${args.join(' ')} 실패 (${signal ?? code}): ${stderr}`));
    });
    child.stdin.end(input ?? '');
  });
}

async function capture() {
  await mkdir(evidenceDirectory, { recursive: true });
  const server = createServer((request, response) => {
    const pathname = decodeURIComponent(new URL(request.url, 'http://127.0.0.1').pathname);
    const filePath = path.resolve(repositoryRoot, `.${pathname}`);
    if (!filePath.startsWith(`${repositoryRoot}${path.sep}`)) {
      response.writeHead(403).end('forbidden');
      return;
    }
    readFile(filePath).then((contents) => {
      const contentType = filePath.endsWith('.html') ? 'text/html; charset=utf-8' : 'text/javascript; charset=utf-8';
      response.writeHead(200, { 'content-type': contentType, 'cache-control': 'no-store' });
      response.end(contents);
    }, () => response.writeHead(404).end('not found'));
  });
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  const address = server.address();
  assert.ok(address && typeof address !== 'string');
  const session = 'c13-1-compact-surface-matrix';
  let browserOpen = false;
  try {
    const agentBrowserVersion = (await cli(['--version'])).trim().replace(/^agent-browser\s+/, '');
    assert.equal(agentBrowserVersion, '0.31.1');

    await cli(['open', '--session', session]);
    browserOpen = true;
    await cli(['set', 'viewport', '300', '100', '1', '--session', session]);
    const fixtureUrl = `http://127.0.0.1:${address.port}/tests/fixtures/css/c13/c13-1-compact-surface-matrix.html`;
    await cli(['navigate', fixtureUrl, '--session', session]);
    const evalScript = readFileSync(agentEvalPath, 'utf8');
    const rawBrowserOutput = await cli(['eval', '--stdin', '--session', session, '--json'], evalScript);
    const envelope = JSON.parse(rawBrowserOutput);
    await writeFile(path.join(evidenceDirectory, 'chrome155-agent-browser-raw.json'), `${JSON.stringify(envelope, null, 2)}\n`);
    const screenshotPath = path.join(evidenceDirectory, 'chrome155-surface.png');
    await cli(['screenshot', '--session', session, screenshotPath]);
    const screenshotBytes = readFileSync(screenshotPath);
    await cli(['navigate', 'chrome://version', '--session', session]);
    const versionEnvelope = JSON.parse(await cli(['eval', '--session', session, '--json', 'document.body.innerText']));
    const versionText = versionEnvelope.data.result;
    const chromeVersionMatch = versionText.match(/Google Chrome\s+([0-9.]+)/);
    const chromeVersion = chromeVersionMatch ? `Google Chrome ${chromeVersionMatch[1]}` : null;
    const revision = versionText.match(/[a-f0-9]{40}-refs\/branch-heads\/\d+@\{#\d+\}/)?.[0];
    const v8 = versionText.match(/V8\s+([0-9.]+)/i)?.[1];
    assert.equal(chromeVersion, 'Google Chrome 155.0.8059.40');
    assert.ok(revision, 'chrome://version revision을 찾지 못했습니다.');
    assert.ok(v8, 'chrome://version V8 버전을 찾지 못했습니다.');
    const observation = normalizeCapture({
      envelope,
      screenshotBytes,
      agentBrowserVersion,
      chromeVersion,
      revision,
      v8,
      now: new Date().toISOString(),
    });
    await writeFile(
      path.join(evidenceDirectory, 'chrome155-observation.json'),
      `${JSON.stringify(observation, null, 2)}\n`,
    );
    console.log(JSON.stringify({
      status: 'PASS',
      agentBrowserVersion,
      chromeVersion,
      fixtureSha256: observation.sources.fixtureSha256,
      sampleCount: observation.sampleCount,
      screenshot: observation.screenshot,
    }, null, 2));
  } finally {
    if (browserOpen) {
      await cli(['close', '--session', session]);
    }
    await new Promise((resolve) => server.close(resolve));
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await capture();
}
