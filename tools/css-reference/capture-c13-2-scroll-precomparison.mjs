import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const evidenceDirectory = path.join(root, 'spec/internal/evidence/c13-2-scroll-container-precomparison-2026-10-15');
const referencePath = path.join(root, 'tests/fixtures/css/references/c13-2-scroll-container-precomparison-v1.json');
const viewport = { width: 640, height: 480, deviceScaleFactor: 1 };
const fixtureDefinitions = [
  {
    id: 'c13-2-scroll-containers-v1',
    path: 'tests/fixtures/css/c13/c13-2-scroll-containers.html',
    screenshot: 'spec/internal/evidence/c13-2-scroll-container-precomparison-2026-10-15/c13-2-scroll-containers.png',
  },
  {
    id: 'c13-2-root-scroll-owner-v1',
    path: 'tests/fixtures/css/c13/c13-2-root-scroll-owner.html',
    screenshot: 'spec/internal/evidence/c13-2-scroll-container-precomparison-2026-10-15/c13-2-root-scroll-owner.png',
  },
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

const scrollContainerCapture = String.raw`
const metrics = (element) => {
  const style = getComputedStyle(element);
  const rect = element.getBoundingClientRect();
  return {
    overflow: [style.overflowX, style.overflowY],
    rect: [rect.x, rect.y, rect.width, rect.height],
    client: [element.clientWidth, element.clientHeight],
    scroll: [element.scrollWidth, element.scrollHeight],
    offset: [element.scrollLeft, element.scrollTop],
    max: [Math.max(0, element.scrollWidth - element.clientWidth), Math.max(0, element.scrollHeight - element.clientHeight)],
  };
};
const port = (id) => document.querySelector('[id="' + id + '"] .scrollport');
const result = { viewport: [innerWidth, innerHeight, devicePixelRatio], cases: {} };
for (const id of ['no-overflow', 'one-axis-x', 'one-axis-y', 'hidden', 'clip', 'ink', 'absolute', 'visible-overflow', 'forced-scroll']) {
  result.cases[id] = metrics(port(id));
}
port('one-axis-x').scrollLeft = -20;
result.cases.negativeXAfterWrite = metrics(port('one-axis-x'));
port('one-axis-y').scrollTop = -20;
result.cases.negativeYAfterWrite = metrics(port('one-axis-y'));
port('forced-scroll').scrollLeft = 9999;
port('forced-scroll').scrollTop = 9999;
result.cases.forcedScrollAfterWrite = metrics(port('forced-scroll'));
port('no-overflow').scrollLeft = 30;
port('no-overflow').scrollTop = 30;
result.cases.noOverflowAfterWrite = metrics(port('no-overflow'));
const hiddenPort = port('hidden');
result.cases.fractionalOffsetInput = [55.25, 55.49, 55.5, 55.51, 55.75].map((requested) => {
  hiddenPort.scrollLeft = requested;
  hiddenPort.scrollTop = requested;
  return { requested, actual: [hiddenPort.scrollLeft, hiddenPort.scrollTop] };
});
hiddenPort.scrollLeft = 55;
hiddenPort.scrollTop = 44;
result.cases.hiddenAfterScriptedScroll = metrics(port('hidden'));
port('clip').scrollLeft = 55;
port('clip').scrollTop = 44;
result.cases.clipAfterScriptedScroll = metrics(port('clip'));
port('visible-overflow').scrollLeft = 30; port('visible-overflow').scrollTop = 30;
result.cases.visibleAfterScriptedWrite = metrics(port('visible-overflow'));
const outer = document.querySelector('#nested .outer');
const inner = document.querySelector('#nested .inner-scrollport');
outer.scrollLeft = 22; outer.scrollTop = 17;
inner.scrollLeft = 42; inner.scrollTop = 29;
result.cases.nested = { outer: metrics(outer), inner: metrics(inner) };
const clampPort = port('clamp');
const clampFill = clampPort.querySelector('.fill');
clampPort.scrollLeft = 30; clampPort.scrollTop = 20;
clampFill.style.width = '260px'; clampFill.style.height = '200px';
clampPort.getBoundingClientRect();
result.cases.retainAfterGrowth = metrics(clampPort);
const reparentTarget = clampPort.querySelector('.content-reparent-target');
reparentTarget.append(clampFill);
clampPort.getBoundingClientRect();
result.cases.retainAfterDescendantMove = metrics(clampPort);
clampPort.scrollLeft = 9999; clampPort.scrollTop = 9999;
result.cases.clampBeforeShrink = metrics(clampPort);
clampFill.style.width = '40px'; clampFill.style.height = '30px';
clampPort.getBoundingClientRect();
result.cases.clampAfterShrink = metrics(clampPort);
const moved = port('move');
moved.scrollLeft = 37; moved.scrollTop = 28;
result.cases.moveBeforeDetach = metrics(moved);
moved.remove();
result.cases.moveDetached = metrics(moved);
document.querySelector('#move-target').append(moved);
result.cases.moveAfterReattach = metrics(moved);
JSON.stringify(result)
`;

const rootOwnerCapture = String.raw`
function sample(name, htmlOverflow, bodyOverflow) {
  const html = document.documentElement;
  const body = document.body;
  html.style.overflow = htmlOverflow;
  body.style.overflow = bodyOverflow;
  const writes = {};
  const fixed = document.querySelector('#fixed');
  const fixedRect = () => { const rect = fixed.getBoundingClientRect(); return [rect.x, rect.y, rect.width, rect.height]; };
  for (const [owner, write] of [
    ['window.scrollTo', () => window.scrollTo(71, 83)],
    ['html.scroll', () => { html.scrollLeft = 71; html.scrollTop = 83; }],
    ['body.scroll', () => { body.scrollLeft = 71; body.scrollTop = 83; }],
  ]) {
    html.scrollLeft = html.scrollTop = 0;
    body.scrollLeft = body.scrollTop = 0;
    window.scrollTo(0, 0);
    write();
    writes[owner] = {
      window: [window.scrollX, window.scrollY],
      html: [html.scrollLeft, html.scrollTop],
      body: [body.scrollLeft, body.scrollTop],
      fixedRect: fixedRect(),
    };
  }
  return {
    name,
    scrollingElement: document.scrollingElement?.tagName ?? null,
    computed: [getComputedStyle(html).overflowX, getComputedStyle(html).overflowY, getComputedStyle(body).overflowX, getComputedStyle(body).overflowY],
    dims: {
      viewport: [innerWidth, innerHeight],
      htmlClient: [html.clientWidth, html.clientHeight],
      htmlScroll: [html.scrollWidth, html.scrollHeight],
      bodyClient: [body.clientWidth, body.clientHeight],
      bodyScroll: [body.scrollWidth, body.scrollHeight],
    },
    writes,
  };
}
JSON.stringify({
  viewport: [innerWidth, innerHeight, devicePixelRatio],
  cases: [
    sample('default-visible', 'visible', 'visible'),
    sample('body-auto-propagates', 'visible', 'auto'),
    sample('html-auto', 'auto', 'visible'),
    sample('html-hidden-body-auto', 'hidden', 'auto'),
  ],
})
`;

const sessionId = run('agent-browser', ['session', 'id', '--scope', 'worktree', '--prefix', 'c13-2-scroll-oracle']);
let browserVersion;
try {
  const toolVersion = run('agent-browser', ['--version']);
  for (const fixture of fixtureDefinitions) {
    const absoluteFixture = path.join(root, fixture.path);
    const absoluteScreenshot = path.join(root, fixture.screenshot);
    run('agent-browser', ['--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height)]);
    run('agent-browser', ['--session', sessionId, 'open', pathToFileURL(absoluteFixture).href]);
    run('agent-browser', ['--session', sessionId, 'screenshot', absoluteScreenshot]);
    fixture.observation = fixture.id === 'c13-2-scroll-containers-v1'
      ? evaluateJson(sessionId, scrollContainerCapture)
      : evaluateJson(sessionId, rootOwnerCapture);
    fixture.sha256 = sha256(await readFile(absoluteFixture));
    fixture.screenshotSha256 = sha256(await readFile(absoluteScreenshot));
  }

  const cdpUrl = run('agent-browser', ['--session', sessionId, 'get', 'cdp-url']);
  const cdp = new URL(cdpUrl);
  const versionUrl = new URL('/json/version', cdp);
  versionUrl.protocol = 'http:';
  browserVersion = await fetch(versionUrl).then((response) => {
    assert.equal(response.ok, true, 'agent-browser DevTools version endpoint must respond');
    return response.json();
  });
  assert.equal(browserVersion.Browser, 'Chrome/155.0.8059.40');
  assert.match(browserVersion['WebKit-Version'], /cfaadc5a132d78e1828635aa8405a499f3e14864/);

  const sourceCommit = run('git', ['rev-parse', 'HEAD']);
  const observation = {
    schema: 'spinon-css-c13-2-scroll-container-precomparison/v1',
    capturedAt: new Date().toISOString(),
    sourceCommit,
    platform: { os: process.platform, arch: process.arch, node: process.version },
    captureTool: { name: 'agent-browser', version: toolVersion },
    oracle: {
      browser: browserVersion.Browser,
      protocolVersion: browserVersion['Protocol-Version'],
      userAgent: browserVersion['User-Agent'],
      v8Version: browserVersion['V8-Version'],
      webkitVersion: browserVersion['WebKit-Version'],
      viewport,
    },
    fixtures: fixtureDefinitions,
  };
  await writeFile(referencePath, `${JSON.stringify(observation, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ reference: path.relative(root, referencePath), sha256: sha256(await readFile(referencePath)), fixtures: fixtureDefinitions.map(({ id, sha256: fixtureSha, screenshotSha256 }) => ({ id, sha256: fixtureSha, screenshotSha256 })) }, null, 2)}\n`);
} finally {
  try {
    run('agent-browser', ['--session', sessionId, 'close']);
  } catch {
    // Closing the dedicated capture session is best-effort after a failed capture.
  }
}
