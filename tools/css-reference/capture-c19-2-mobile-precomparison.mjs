import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { decodeRgbaPng } from './read-png-rgba.mjs';

const root = fileURLToPath(new URL('../..', import.meta.url));
const fixturePath = 'tests/fixtures/css/c19/runtime-box-paint-opacity-c19-2.js';
const htmlPath = 'spec/internal/evidence/c19-2-box-paint-opacity-mobile-precomparison-2026-10-17/mobile-fixture.html';
const evidenceDirectory = 'spec/internal/evidence/c19-2-box-paint-opacity-mobile-precomparison-2026-10-17';
const screenshotRelative = `${evidenceDirectory}/chrome155-agent-browser.png`;
const observationRelative = `${evidenceDirectory}/observations.json`;
const viewport = { width: 360, height: 240, deviceScaleFactor: 1 };
const samples = [
  { id: 'rootBackdrop', x: 340, y: 232, rgba: [32, 49, 75, 255] },
  { id: 'sourceAlpha', x: 26, y: 26, rgba: [143, 25, 38, 255] },
  { id: 'groupSingle', x: 64, y: 26, rgba: [16, 89, 165, 255] },
  { id: 'groupOverlap', x: 80, y: 26, rgba: [143, 121, 38, 255] },
  { id: 'nestedOpacity', x: 128, y: 24, rgba: [24, 101, 56, 255] },
  { id: 'alphaTimesGroupOpacity', x: 168, y: 24, rgba: [88, 37, 56, 255] },
  { id: 'wideGamutSrgb', x: 208, y: 24, rgba: [0, 142, 200, 255] },
  { id: 'opacityZero', x: 16, y: 72, rgba: [32, 49, 75, 255] },
  { id: 'opacityHalfSingle', x: 84, y: 90, rgba: [143, 25, 38, 255] },
  { id: 'opacityHalfOverlap', x: 68, y: 68, rgba: [16, 25, 165, 255] },
  { id: 'opacityOneOverlap', x: 124, y: 68, rgba: [0, 0, 255, 255] },
  { id: 'clippedZOverlap', x: 202, y: 76, rgba: [16, 25, 165, 255] },
  { id: 'clippedRed', x: 212, y: 76, rgba: [143, 25, 38, 255] },
  { id: 'clippedAway', x: 220, y: 76, rgba: [32, 49, 75, 255] },
  { id: 'currentColorBorder', x: 234, y: 58, rgba: [28, 121, 102, 255] },
  { id: 'borderInterior', x: 256, y: 80, rgba: [32, 49, 75, 255] },
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

const sessionId = run('agent-browser', ['session', 'id', '--scope', 'worktree', '--prefix', 'c19-2-mobile-oracle']);
try {
  const toolVersion = run('agent-browser', ['--version']);
  run('agent-browser', ['--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height)]);
  run('agent-browser', ['--session', sessionId, 'open', pathToFileURL(path.join(root, htmlPath)).href]);
  run('agent-browser', ['--session', sessionId, 'wait', '--load', 'networkidle']);

  const report = evaluateJson(sessionId, `JSON.stringify((() => {
    const nodes = window.spinonC192Nodes;
    const computed = Object.fromEntries(Object.entries(nodes).map(([id, element]) => {
      const style = getComputedStyle(element);
      return [id, { color: style.color, backgroundColor: style.backgroundColor,
        borderTopColor: style.borderTopColor, opacity: style.opacity,
        overflow: style.overflow, zIndex: style.zIndex,
        rect: [element.offsetLeft, element.offsetTop, element.offsetWidth, element.offsetHeight] }];
    }));
    const hitPoints = ${JSON.stringify(Object.fromEntries(samples.map(({ id, x, y }) => [id, [x, y]])))};
    const hitTests = Object.fromEntries(Object.entries(hitPoints).map(([id, [x, y]]) => {
      const element = document.elementFromPoint(x, y);
      return [id, element?.getAttribute('data-c192-node') ?? element?.id ?? element?.tagName ?? null];
    }));
    return { viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio }, computed, hitTests };
  })())`);
  const cdpUrl = run('agent-browser', ['--session', sessionId, 'get', 'cdp-url']);
  const versionUrl = new URL('/json/version', cdpUrl);
  versionUrl.protocol = 'http:';
  const browserInfo = await fetch(versionUrl).then((response) => {
    assert.equal(response.ok, true, 'Agent Browser DevTools version endpoint 응답이 필요합니다');
    return response.json();
  });
  const screenshotPath = path.join(root, screenshotRelative);
  await mkdir(path.dirname(screenshotPath), { recursive: true });
  run('agent-browser', ['--session', sessionId, 'screenshot', screenshotPath]);
  const screenshotBytes = await readFile(screenshotPath);
  const image = decodeRgbaPng(screenshotBytes);
  const fixtureBytes = await readFile(path.join(root, fixturePath));
  const htmlBytes = await readFile(path.join(root, htmlPath));
  assert.match(browserInfo.Browser, /^Chrome\/155\./);
  assert.deepEqual(report.viewport, { width: 360, height: 240, dpr: 1 });
  assert.equal(image.width, viewport.width);
  assert.equal(image.height, viewport.height);
  const observedSamples = samples.map(({ id, x, y, rgba: expected }) => {
    const rgba = image.getPixel(x, y);
    assert.deepEqual(rgba, expected, `${id} screenshot sample`);
    return { id, x, y, rgba };
  });
  assert.equal(report.hitTests.opacityZero, 'c192-opacity-zero');
  assert.equal(report.hitTests.groupOverlap, 'c192-group-yellow');
  assert.equal(report.hitTests.clippedAway, 'c192-stage');
  assert.equal(report.hitTests.currentColorBorder, 'c192-current-color-border');

  const observation = {
    schema: 'spinon-c19-2-mobile-oracle/v1',
    browser: {
      product: browserInfo.Browser,
      revision: browserInfo['WebKit-Version'],
      protocolVersion: browserInfo['Protocol-Version'],
      userAgent: browserInfo.UserAgent,
      platform: process.platform,
      captureTool: { name: 'agent-browser', version: toolVersion },
    },
    fixture: {
      htmlPath,
      htmlSha256: sha256(htmlBytes),
      runtimeSourcePath: fixturePath,
      runtimeSourceSha256: sha256(fixtureBytes),
    },
    viewport,
    computed: report.computed,
    hitTests: report.hitTests,
    samples: observedSamples,
    screenshot: {
      path: path.basename(screenshotRelative),
      dimensions: [image.width, image.height],
      sha256: sha256(screenshotBytes),
    },
  };
  await writeFile(path.join(root, observationRelative), `${JSON.stringify(observation, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ observation: observationRelative, screenshot: screenshotRelative,
    browser: browserInfo.Browser, sampleCount: observedSamples.length,
    screenshotSha256: observation.screenshot.sha256 }, null, 2)}\n`);
} finally {
  try {
    run('agent-browser', ['--session', sessionId, 'close']);
  } catch {
    // 캡처가 실패해도 소유한 브라우저 세션 닫기를 시도한다.
  }
}
