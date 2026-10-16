import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const fixturePath = 'tests/fixtures/css/c18/image-alpha-source-over-c18-2.html';
const pngPath = 'tests/fixtures/resources/c18-2-alpha-source-over.png';
const screenshotRelative = 'spec/internal/evidence/c18-2-alpha-precomparison-2026-10-16/chrome155.png';
const observationRelative = 'spec/internal/evidence/c18-2-alpha-precomparison-2026-10-16/chrome155-observation.json';
const viewport = { width: 80, height: 40, deviceScaleFactor: 1 };
const samplePoints = [
  { id: 'background-anchor', x: 2, y: 2 },
  { id: 'transparent-interior', x: 12, y: 20 },
  { id: 'transparent-right-edge', x: 15, y: 20 },
  { id: 'partial-left-edge', x: 16, y: 20 },
  { id: 'partial-interior', x: 20, y: 20 },
  { id: 'partial-right-edge', x: 23, y: 20 },
  { id: 'opaque-left-edge', x: 24, y: 20 },
  { id: 'opaque-interior', x: 28, y: 20 },
  { id: 'opaque-right-edge', x: 31, y: 20 },
  { id: 'after-image-anchor', x: 34, y: 20 },
];

function run(command, args, input) {
  return execFileSync(command, args, {
    cwd: root,
    encoding: 'utf8',
    input,
    stdio: ['pipe', 'pipe', 'pipe'],
  }).trim();
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function evaluateJson(sessionId, source) {
  const output = run('agent-browser', ['--session', sessionId, 'eval', '--stdin'], source);
  return JSON.parse(JSON.parse(output));
}

const sessionId = run('agent-browser', ['session', 'id', '--scope', 'worktree', '--prefix', 'c18-2-alpha-oracle']);
let sessionStarted = true;
try {
  const toolVersion = run('agent-browser', ['--version']);
  run('agent-browser', ['--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height)]);
  run('agent-browser', ['--session', sessionId, 'open', pathToFileURL(path.join(root, fixturePath)).href]);
  run('agent-browser', ['--session', sessionId, 'wait', '--load', 'networkidle']);
  const screenshotPath = path.join(root, screenshotRelative);
  const observationPath = path.join(root, observationRelative);
  await mkdir(path.dirname(screenshotPath), { recursive: true });
  const page = evaluateJson(sessionId, 'JSON.stringify(window.read_c18_2_oracle())');
  const cdpUrl = run('agent-browser', ['--session', sessionId, 'get', 'cdp-url']);
  const versionUrl = new URL('/json/version', cdpUrl);
  versionUrl.protocol = 'http:';
  const browserInfo = await fetch(versionUrl).then((response) => {
    assert.equal(response.ok, true, 'Agent Browser DevTools version endpoint 응답이 필요합니다');
    return response.json();
  });
  const screenshotBytes = Buffer.from(run('agent-browser', ['--session', sessionId, 'screenshot', screenshotPath]));
  const screenshotFile = await readFile(screenshotPath);
  const screenshotPng = decodeRgbaPng(screenshotFile);
  const imageBytes = await readFile(path.join(root, pngPath));
  const fixtureBytes = await readFile(path.join(root, fixturePath));
  assert.deepEqual(page.viewport, { width: viewport.width, height: viewport.height, dpr: viewport.deviceScaleFactor });
  assert.equal(page.image.loaded, true);
  assert.deepEqual(page.image.natural, { width: 24, height: 16 });
  assert.deepEqual(page.image.frame, { x: 8, y: 12, width: 24, height: 16 });
  assert.equal(screenshotPng.width, viewport.width);
  assert.equal(screenshotPng.height, viewport.height);
  assert.ok(screenshotBytes.length > 0);

  const samples = samplePoints.map((sample) => ({
    ...sample,
    rgba: screenshotPng.getPixel(sample.x, sample.y),
  }));
  assert.match(browserInfo.Browser, /^Chrome\/155\./, '고정 Chrome 155 oracle이 필요합니다');
  const observation = {
    schemaVersion: 1,
    browser: {
      product: browserInfo.Browser,
      revision: browserInfo['WebKit-Version'],
      platform: process.platform,
      userAgent: page.browser,
      captureTool: { name: 'agent-browser', version: toolVersion },
    },
    fixture: {
      path: fixturePath,
      sha256: sha256(fixtureBytes),
      imagePath: pngPath,
      imageSha256: sha256(imageBytes),
    },
    viewport,
    image: page.image,
    samples,
    screenshot: {
      path: path.basename(screenshotRelative),
      dimensions: [screenshotPng.width, screenshotPng.height],
      sha256: sha256(screenshotFile),
    },
  };
  await writeFile(observationPath, `${JSON.stringify(observation, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ observation: observationRelative, screenshot: screenshotRelative, sha256: sha256(screenshotFile), samples }, null, 2)}\n`);
} finally {
  if (sessionStarted) {
    try {
      run('agent-browser', ['--session', sessionId, 'close']);
    } catch {
      // 실패한 캡처도 전용 Agent Browser 세션 정리를 시도한다.
    }
  }
}
