import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const fixturePath = 'tests/fixtures/css/c18/image-object-fit-position-c18-3.html';
const pngPath = 'tests/fixtures/resources/c18-3-object-fit-pattern.png';
const screenshotRelative = 'spec/internal/evidence/c18-3-object-fit-precomparison/chrome155.png';
const observationRelative = 'spec/internal/evidence/c18-3-object-fit-precomparison/chrome155-observation.json';
const viewport = { width: 224, height: 72, deviceScaleFactor: 1 };
const samplePoints = [
  { id: 'root-background-anchor', x: 2, y: 2 },
  { id: 'fill-top-left', x: 9, y: 9 },
  { id: 'fill-bottom-right', x: 30, y: 30 },
  { id: 'contain-background-top', x: 42, y: 9 },
  { id: 'contain-image-top', x: 42, y: 15 },
  { id: 'contain-image-bottom', x: 42, y: 25 },
  { id: 'contain-quarter-image', x: 75, y: 18 },
  { id: 'cover-center-left', x: 105, y: 12 },
  { id: 'cover-center-right', x: 126, y: 12 },
  { id: 'cover-start-left', x: 137, y: 12 },
  { id: 'cover-end-left', x: 169, y: 12 },
  { id: 'none-clipped-object', x: 10, y: 45 },
  { id: 'none-outside-content-box', x: 25, y: 45 },
  { id: 'scale-down-shrink-top-bar', x: 42, y: 41 },
  { id: 'scale-down-shrink-image', x: 45, y: 46 },
  { id: 'scale-down-no-upscale-image', x: 82, y: 47 },
  { id: 'initial-values-natural', x: 113, y: 41 },
  { id: 'contain-extended-position-image', x: 145, y: 60 },
  { id: 'cover-extended-position-image', x: 177, y: 44 },
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

const sessionId = run('agent-browser', ['session', 'id', '--scope', 'worktree', '--prefix', 'c18-3-object-fit-oracle']);
try {
  const toolVersion = run('agent-browser', ['--version']);
  run('agent-browser', ['--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height)]);
  run('agent-browser', ['--session', sessionId, 'open', pathToFileURL(path.join(root, fixturePath)).href]);
  run('agent-browser', ['--session', sessionId, 'wait', '--load', 'networkidle']);
  const screenshotPath = path.join(root, screenshotRelative);
  const observationPath = path.join(root, observationRelative);
  await mkdir(path.dirname(screenshotPath), { recursive: true });
  const page = evaluateJson(sessionId, 'JSON.stringify(window.read_c18_3_oracle())');
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
  assert.equal(page.images.length, 12);
  assert.ok(page.images.every((image) => image.loaded));
  assert.ok(page.images.every((image) => image.natural.width === 24 && image.natural.height === 12));
  assert.ok(page.images.every((image) => image.imageRendering === 'auto'));
  assert.equal(screenshotPng.width, viewport.width);
  assert.equal(screenshotPng.height, viewport.height);
  assert.ok(screenshotBytes.length > 0);
  assert.match(browserInfo.Browser, /^Chrome\/155\./, '고정 Chrome 155 oracle이 필요합니다');

  const samples = samplePoints.map((sample) => ({
    ...sample,
    rgba: screenshotPng.getPixel(sample.x, sample.y),
  }));
  const observation = {
    schemaVersion: 1,
    agentBrowserSession: sessionId,
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
    images: page.images,
    samples,
    screenshot: {
      path: path.basename(screenshotRelative),
      dimensions: [screenshotPng.width, screenshotPng.height],
      sha256: sha256(screenshotFile),
    },
  };
  await writeFile(observationPath, `${JSON.stringify(observation, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ session: sessionId, observation: observationRelative, screenshot: screenshotRelative, sha256: sha256(screenshotFile), images: page.images, samples }, null, 2)}\n`);
} finally {
  try {
    run('agent-browser', ['--session', sessionId, 'close']);
  } catch {
    // 실패한 비교도 전용 Agent Browser 세션 정리를 시도한다.
  }
}
