import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const fixturePath = 'tests/fixtures/css/c13/c13-3-scroll-gesture.html';
const evidenceDirectory = path.join(
  root,
  'spec/internal/evidence/c13-3-style-invalidation-precomparison-2026-10-15',
);
const screenshotPath = path.join(evidenceDirectory, 'chrome155-style-invalidation-400x240.png');
const referencePath = path.join(
  root,
  'tests/fixtures/css/references/c13-3-style-invalidation-precomparison-v1.json',
);
const viewport = { width: 400, height: 240, deviceScaleFactor: 1 };

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

const sessionId = run('agent-browser', [
  'session', 'id', '--scope', 'worktree', '--prefix', 'c13-3-style-invalidation-oracle',
]);
let ws;
let browserOpened = false;

try {
  await mkdir(evidenceDirectory, { recursive: true });
  const fixtureUrl = pathToFileURL(path.join(root, fixturePath)).href;
  run('agent-browser', [
    '--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height),
  ]);
  run('agent-browser', ['--session', sessionId, 'open', fixtureUrl]);
  browserOpened = true;
  run('agent-browser', ['--session', sessionId, 'wait', '--load', 'domcontentloaded']);
  run('agent-browser', ['--session', sessionId, 'screenshot', screenshotPath]);

  const agentBrowserVersion = run('agent-browser', ['--version']);
  const cdpUrl = run('agent-browser', ['--session', sessionId, 'get', 'cdp-url']);
  const cdpPort = new URL(cdpUrl).port;
  assert.ok(cdpPort, `Agent Browser CDP URL에 port가 없습니다: ${cdpUrl}`);
  const browser = await fetch(`http://127.0.0.1:${cdpPort}/json/version`).then((r) => r.json());
  assert.equal(browser.Browser, 'Chrome/155.0.8059.40');
  assert.match(browser['WebKit-Version'], /cfaadc5a132d78e1828635aa8405a499f3e14864/);
  const targets = await fetch(`http://127.0.0.1:${cdpPort}/json/list`).then((r) => r.json());
  const page = targets.find((target) => target.type === 'page' && target.url === fixtureUrl);
  assert.ok(page, 'Agent Browser C13.3 precomparison fixture tab이 없습니다');

  ws = new WebSocket(page.webSocketDebuggerUrl);
  let nextCommandId = 0;
  const pending = new Map();
  ws.addEventListener('message', (event) => {
    const message = JSON.parse(event.data);
    if (!message.id || !pending.has(message.id)) return;
    const waiter = pending.get(message.id);
    pending.delete(message.id);
    if (message.error) waiter.reject(new Error(`${waiter.method}: ${message.error.message}`));
    else waiter.resolve(message.result ?? {});
  });
  await new Promise((resolve, reject) => {
    ws.addEventListener('open', resolve, { once: true });
    ws.addEventListener('error', reject, { once: true });
  });

  const cdp = (method, params = {}) => new Promise((resolve, reject) => {
    const id = ++nextCommandId;
    pending.set(id, { resolve, reject, method });
    ws.send(JSON.stringify({ id, method, params }));
  });
  const evaluate = async (expression) => {
    const response = await cdp('Runtime.evaluate', {
      expression,
      returnByValue: true,
      awaitPromise: true,
    });
    if (response.exceptionDetails) throw new Error(response.exceptionDetails.text);
    return response.result?.value;
  };
  const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

  await cdp('Page.enable');
  await cdp('Runtime.enable');
  await cdp('Emulation.setDeviceMetricsOverride', { ...viewport, mobile: true });
  await cdp('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 2 });

  async function runTouchCase(changeDuringGesture) {
    const initial = await evaluate('c133.prepare("auto", [0, 106], [0, 0], [0, 0])');
    await evaluate("c133.setBehavior('inner', 'auto', 'auto')");
    await evaluate('c133.settle()');
    await evaluate('c133.clearEvents()');
    const start = await evaluate(`(() => {
      const rect = document.querySelector('#inner').getBoundingClientRect();
      return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
    })()`);

    await cdp('Input.dispatchTouchEvent', {
      type: 'touchStart',
      touchPoints: [{ ...start, id: 1 }],
    });
    await sleep(50);
    const firstPoint = { x: start.x, y: start.y - 30 };
    await cdp('Input.dispatchTouchEvent', {
      type: 'touchMove',
      touchPoints: [{ ...firstPoint, id: 1 }],
    });
    const afterFirstMove = await evaluate('c133.settle()');
    let computedAfterMutation = null;
    if (changeDuringGesture) {
      computedAfterMutation = await evaluate("c133.setBehavior('inner', 'auto', 'contain')");
    }
    const secondPoint = { x: start.x, y: start.y - 70 };
    await cdp('Input.dispatchTouchEvent', {
      type: 'touchMove',
      touchPoints: [{ ...secondPoint, id: 1 }],
    });
    await cdp('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
    const final = await evaluate('c133.settle()');
    return {
      changeDuringGesture,
      start,
      initialOffsets: Object.fromEntries(Object.entries(initial.owners).map(([id, owner]) => [id, owner.offset])),
      afterFirstMove: Object.fromEntries(Object.entries(afterFirstMove.owners).map(([id, owner]) => [id, owner.offset])),
      policyAfterMutation: computedAfterMutation?.overscroll ?? null,
      finalPolicy: final.owners.inner.overscroll,
      finalOffsets: Object.fromEntries(Object.entries(final.owners).map(([id, owner]) => [id, owner.offset])),
      scrollEvents: final.events.map(({ id, x, y }) => ({ owner: id, offset: [x, y] })),
      pointerEvents: final.pointerEvents.map(({ type, id, pointerType, target }) => ({ type, id, pointerType, target })),
    };
  }

  const control = await runTouchCase(false);
  const mutation = await runTouchCase(true);
  const fixtureBytes = await readFile(path.join(root, fixturePath));
  const screenshotBytes = await readFile(screenshotPath);
  const png = screenshotBytes.subarray(0, 24);
  assert.equal(png.toString('ascii', 1, 4), 'PNG');
  const reference = {
    schema: 'spinon-css-c13-3-style-invalidation-precomparison/v1',
    capturedAt: new Date().toISOString(),
    scope: 'Chrome touch oracle only; not Spinon runtime or app verification.',
    oracle: {
      browser: browser.Browser,
      webkitVersion: browser['WebKit-Version'],
      agentBrowserVersion,
      viewport,
      touchEmulation: { enabled: true, maxTouchPoints: 2 },
    },
    fixture: { path: fixturePath, sha256: sha256(fixtureBytes) },
    screenshot: {
      path: path.relative(root, screenshotPath).split(path.sep).join('/'),
      dimensions: [png.readUInt32BE(16), png.readUInt32BE(20)],
      sha256: sha256(screenshotBytes),
    },
    cases: { autoControl: control, activeOverscrollMutation: mutation },
    interpretation: {
      autoControl: 'Reference for chaining at the boundary across two ordered touchmove samples.',
      activeOverscrollMutation: 'Observed browser behavior after changing overscroll-behavior-y while the same touch gesture is active; use as a comparator, not as the Spinon queue contract.',
    },
  };
  await writeFile(referencePath, `${JSON.stringify(reference, null, 2)}\n`);
  console.log(JSON.stringify({
    reference: path.relative(root, referencePath),
    browser: browser.Browser,
    autoControl: control.finalOffsets,
    activeOverscrollMutation: mutation.finalOffsets,
    policyAfterMutation: mutation.policyAfterMutation,
  }, null, 2));
} finally {
  ws?.close();
  if (browserOpened) {
    try { run('agent-browser', ['--session', sessionId, 'close']); } catch {}
  }
}
