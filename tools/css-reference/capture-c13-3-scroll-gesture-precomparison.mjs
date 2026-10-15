import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = fileURLToPath(new URL('../..', import.meta.url));
const fixturePath = 'tests/fixtures/css/c13/c13-3-scroll-gesture.html';
const screenshotPath = 'spec/internal/evidence/c13-3-scroll-gesture-precomparison-2026-10-15/chrome155-c13-3-scroll-gesture-400x240.png';
const evidenceDirectory = path.dirname(path.join(root, screenshotPath));
const referencePath = path.join(root, 'tests/fixtures/css/references/c13-3-scroll-gesture-precomparison-v1.json');
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

function offsets(snapshot) {
  return Object.fromEntries(Object.entries(snapshot.owners).map(([id, owner]) => [id, owner.offset]));
}

function pointerEvents(snapshot) {
  return snapshot.pointerEvents.map(({ type, id, pointerType, target, x, y, primary }) => ({
    type,
    id,
    pointerType,
    target,
    x,
    y,
    primary,
  }));
}

function scrollEvents(snapshot) {
  return snapshot.events.map(({ id, x, y }) => ({ owner: id, offset: [x, y] }));
}

function wheelEvents(snapshot) {
  return snapshot.wheelEvents.map(({ listener, target, deltaX, deltaY, deltaMode }) => ({
    listener,
    target,
    deltaX,
    deltaY,
    deltaMode,
  }));
}

const sessionId = run('agent-browser', ['session', 'id', '--scope', 'worktree', '--prefix', 'c13-3-scroll-oracle']);
let ws;

try {
  await mkdir(evidenceDirectory, { recursive: true });
  const agentBrowserVersion = run('agent-browser', ['--version']);
  const hostOsVersion = run('sw_vers', ['-productVersion']);
  const fixtureUrl = pathToFileURL(path.join(root, fixturePath)).href;
  run('agent-browser', ['--session', sessionId, 'set', 'viewport', String(viewport.width), String(viewport.height)]);
  run('agent-browser', ['--session', sessionId, 'open', fixtureUrl]);
  run('agent-browser', ['--session', sessionId, 'wait', '--load', 'domcontentloaded']);
  run('agent-browser', ['--session', sessionId, 'screenshot', path.join(root, screenshotPath)]);
  const cdpUrl = run('agent-browser', ['--session', sessionId, 'get', 'cdp-url']);
  const cdpPort = new URL(cdpUrl).port;
  assert.ok(cdpPort, `agent-browser CDP URL has no port: ${cdpUrl}`);

  const browser = await fetch(`http://127.0.0.1:${cdpPort}/json/version`).then((response) => response.json());
  assert.equal(browser.Browser, 'Chrome/155.0.8059.40', 'C13.3 oracle must use the pinned Chrome build');
  assert.match(browser['WebKit-Version'], /cfaadc5a132d78e1828635aa8405a499f3e14864/);
  const targets = await fetch(`http://127.0.0.1:${cdpPort}/json/list`).then((response) => response.json());
  const page = targets.find((target) => target.type === 'page' && target.url.startsWith(fixtureUrl));
  assert.ok(page, 'agent-browser C13.3 fixture page target is missing');

  ws = new WebSocket(page.webSocketDebuggerUrl);
  let nextCommandId = 0;
  const pending = new Map();
  ws.addEventListener('message', (event) => {
    const message = JSON.parse(event.data);
    if (!message.id || !pending.has(message.id)) return;
    const waiter = pending.get(message.id);
    pending.delete(message.id);
    if (message.error) waiter.reject(new Error(`${waiter.method} ${JSON.stringify(waiter.params)}: ${message.error.message}`));
    else waiter.resolve(message.result ?? {});
  });
  await new Promise((resolve, reject) => {
    ws.addEventListener('open', resolve, { once: true });
    ws.addEventListener('error', reject, { once: true });
  });

  const cdp = (method, params = {}) => new Promise((resolve, reject) => {
    const id = ++nextCommandId;
    pending.set(id, { resolve, reject, method, params });
    ws.send(JSON.stringify({ id, method, params }));
  });
  const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));
  const evaluate = async (expression) => {
    const response = await cdp('Runtime.evaluate', {
      expression,
      returnByValue: true,
      awaitPromise: true,
    });
    if (response.exceptionDetails) throw new Error(response.exceptionDetails.text);
    return response.result?.value;
  };

  async function waitForFixture(expectedUrl) {
    const deadline = Date.now() + 5000;
    while (Date.now() < deadline) {
      const state = await evaluate('({ url: location.href, ready: document.readyState, harness: typeof window.c133 === "object" })');
      if (state.url === expectedUrl && state.ready === 'complete' && state.harness) return;
      await sleep(20);
    }
    const state = await evaluate('({ url: location.href, ready: document.readyState, harness: typeof window.c133 === "object" })');
    throw new Error(`C13.3 fixture navigation did not settle at ${expectedUrl}: ${JSON.stringify(state)}`);
  }

  await cdp('Page.enable');
  await cdp('Runtime.enable');
  await cdp('Emulation.setDeviceMetricsOverride', { ...viewport, mobile: true });
  await cdp('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 2 });
  await waitForFixture(fixtureUrl);

  async function navigate(touchAction = 'auto', overflow = 'auto') {
    const query = new URLSearchParams();
    if (touchAction !== 'auto') query.set('touch-action', touchAction);
    if (overflow !== 'auto') query.set('overflow', overflow);
    const suffix = query.size > 0 ? `?${query}` : '';
    const expectedUrl = fixtureUrl + suffix;
    await cdp('Page.navigate', { url: expectedUrl });
    await waitForFixture(expectedUrl);
  }

  async function prepare({ mode = 'auto', inner = [0, 0], outer = [0, 0], root = [0, 0], behavior } = {}) {
    await evaluate(`c133.prepare(${JSON.stringify(mode)}, ${JSON.stringify(inner)}, ${JSON.stringify(outer)}, ${JSON.stringify(root)})`);
    if (behavior) {
      await evaluate(`c133.setBehavior('inner', ${JSON.stringify(behavior[0])}, ${JSON.stringify(behavior[1])})`);
    }
    await evaluate('c133.settle()');
    await evaluate('c133.clearEvents()');
    return evaluate('c133.snapshot()');
  }

  async function centerOf(selector = '#inner') {
    return evaluate(`(() => {
      const rect = document.querySelector(${JSON.stringify(selector)}).getBoundingClientRect();
      return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
    })()`);
  }

  async function touchCase(id, options) {
    await navigate(options.touchAction ?? 'auto', options.overflow ?? 'auto');
    if (options.innerTop !== undefined) {
      await evaluate(`document.querySelector('#inner').style.top = ${JSON.stringify(`${options.innerTop}px`)}`);
    }
    const initial = await prepare(options);
    for (const [owner, value] of Object.entries(options.touchActions ?? {})) {
      await evaluate(`c133.setTouchAction(${JSON.stringify(owner)}, ${JSON.stringify(value)})`);
    }
    if (options.touchActions) {
      await evaluate('c133.settle()');
      await evaluate('c133.clearEvents()');
    }
    const start = await centerOf(options.targetSelector ?? '#inner');
    let current = { ...start };
    await cdp('Input.dispatchTouchEvent', {
      type: 'touchStart',
      touchPoints: [{ ...current, id: 1 }],
    });
    if (options.changeTouchActionAfterStart) {
      await evaluate("c133.setTouchAction('inner', 'none')");
    }

    for (const [index, move] of options.moves.entries()) {
      await sleep(move.wait ?? 50);
      current = { x: current.x + move.dx, y: current.y + move.dy };
      await cdp('Input.dispatchTouchEvent', {
        type: 'touchMove',
        touchPoints: [{ ...current, id: 1 }],
      });
      if (options.changeTouchActionAfterPanBegan && index === 0) {
        await evaluate("c133.setTouchAction('inner', 'none')");
      }
    }

    if (options.cancel) {
      await cdp('Input.dispatchTouchEvent', { type: 'touchCancel', touchPoints: [] });
      const beforeLateMove = await evaluate('c133.settle()');
      let lateMoveDispatch;
      try {
        await cdp('Input.dispatchTouchEvent', {
          type: 'touchMove',
          touchPoints: [{ x: current.x, y: current.y - 30, id: 1 }],
        });
        lateMoveDispatch = { accepted: true };
      } catch (error) {
        lateMoveDispatch = { accepted: false, error: error.message };
      }
      const afterLateMove = await evaluate('c133.settle()');
      return {
        id,
        lateMoveDispatch,
        touchAction: await evaluate("getComputedStyle(document.querySelector('#inner')).touchAction"),
        start,
        initialOffsets: offsets(initial),
        beforeLateMove: {
          offsets: offsets(beforeLateMove),
          scrollEvents: scrollEvents(beforeLateMove),
          pointerEvents: pointerEvents(beforeLateMove),
        },
        afterLateMove: {
          offsets: offsets(afterLateMove),
          scrollEvents: scrollEvents(afterLateMove),
          pointerEvents: pointerEvents(afterLateMove),
        },
      };
    }

    await cdp('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
    const final = await evaluate('c133.settle()');
    return {
      id,
      input: {
        type: 'direct-touch',
        configuredTouchAction: options.touchAction ?? 'auto',
        overflow: options.overflow ?? 'auto',
        targetSelector: options.targetSelector ?? '#inner',
        innerTop: options.innerTop ?? 20,
        initialOffsets: { inner: options.inner ?? [0, 0], outer: options.outer ?? [0, 0], root: options.root ?? [0, 0] },
        defaultOverscrollBehavior: options.mode ?? 'auto',
        innerOverscrollBehavior: options.behavior ?? null,
        touchActions: options.touchActions ?? null,
        moveDeltas: options.moves.map(({ dx, dy }) => [dx, dy]),
        moveWaitMs: options.moves.map(({ wait }) => wait ?? 50),
        touchActionChangedAfterStart: options.changeTouchActionAfterStart ?? false,
        touchActionChangedAfterPanBegan: options.changeTouchActionAfterPanBegan ?? false,
      },
      touchAction: await evaluate("getComputedStyle(document.querySelector('#inner')).touchAction"),
      touchActionPath: await evaluate(`Object.fromEntries(['inner-content', 'inner', 'outer', 'root'].map((id) => [id, getComputedStyle(document.getElementById(id)).touchAction]))`),
      start,
      initialOwners: initial.owners,
      finalOverscroll: final.owners.inner.overscroll,
      finalOffsets: offsets(final),
      scrollEvents: scrollEvents(final),
      pointerEvents: pointerEvents(final),
    };
  }

  async function wheelCase(id, options) {
    await navigate(options.touchAction ?? 'auto', options.overflow ?? 'auto');
    const initial = await prepare(options);
    const point = await centerOf(options.targetSelector ?? '#inner');
    await cdp('Input.dispatchMouseEvent', {
      type: 'mouseWheel',
      x: point.x,
      y: point.y,
      deltaX: options.deltaX,
      deltaY: options.deltaY,
    });
    const final = await evaluate('c133.settle()');
    return {
      id,
      input: {
        type: 'wheel',
        targetSelector: options.targetSelector ?? '#inner',
        point,
        initialOffsets: { inner: options.inner ?? [0, 0], outer: options.outer ?? [0, 0], root: options.root ?? [0, 0] },
        defaultOverscrollBehavior: options.mode ?? 'auto',
        innerOverscrollBehavior: options.behavior ?? null,
        touchAction: options.touchAction ?? 'auto',
        delta: [options.deltaX, options.deltaY],
      },
      point,
      delta: [options.deltaX, options.deltaY],
      behavior: final.owners.inner.overscroll,
      initialOffsets: offsets(initial),
      finalOffsets: offsets(final),
      scrollEvents: scrollEvents(final),
      wheelEvents: wheelEvents(final),
    };
  }

  const repeat = (dx, dy, count = 4, wait = 50) => Array.from({ length: count }, () => ({ dx, dy, wait }));
  const cases = {};

  cases.touchVertical60 = await touchCase('touch-vertical-60', { moves: repeat(0, -15) });
  cases.touchHorizontal60 = await touchCase('touch-horizontal-60', { moves: repeat(-15, 0) });
  cases.touchDiagonal60 = await touchCase('touch-diagonal-60-per-axis', { moves: repeat(-15, -15) });
  cases.touchReachBoundary = await touchCase('touch-reaches-inner-boundary-mid-gesture', {
    inner: [0, 86],
    moves: repeat(0, -15, 12, 35),
  });
  cases.touchBoundaryAuto = await touchCase('touch-starts-at-inner-y-boundary-auto', {
    inner: [0, 106],
    moves: repeat(0, -15),
  });
  cases.touchBoundaryContain = await touchCase('touch-starts-at-inner-y-boundary-contain', {
    inner: [0, 106],
    behavior: ['contain', 'contain'],
    moves: repeat(0, -15),
  });
  cases.touchBoundaryNone = await touchCase('touch-starts-at-inner-y-boundary-none', {
    inner: [0, 106],
    behavior: ['none', 'none'],
    moves: repeat(0, -15),
  });
  cases.touchOverflowHidden = await touchCase('touch-overflow-hidden', {
    overflow: 'hidden',
    moves: repeat(0, -15),
  });
  cases.touchOverflowClip = await touchCase('touch-overflow-clip', {
    overflow: 'clip',
    moves: repeat(0, -15),
  });
  cases.touchOverflowVisible = await touchCase('touch-overflow-visible', {
    overflow: 'visible',
    moves: repeat(0, -15),
  });
  cases.touchActionNone = await touchCase('touch-action-none-before-gesture', {
    touchAction: 'none',
    moves: repeat(0, -15),
  });
  cases.touchActionAfterStart = await touchCase('touch-action-changes-to-none-after-start', {
    changeTouchActionAfterStart: true,
    moves: repeat(0, -15),
  });
  cases.touchActionAfterPanBegan = await touchCase('touch-action-changes-after-pan-began', {
    changeTouchActionAfterPanBegan: true,
    moves: repeat(0, -15),
  });
  cases.parentTouchActionPanXVertical = await touchCase('outer-pan-x-restricts-inner-vertical-pan', {
    touchActions: { 'inner-content': 'pan-x' },
    moves: repeat(0, -15),
  });
  cases.parentTouchActionPanXHorizontal = await touchCase('outer-pan-x-allows-inner-horizontal-pan', {
    touchActions: { 'inner-content': 'pan-x' },
    moves: repeat(-15, 0),
  });
  cases.touchActionPanXHorizontal = await touchCase('touch-action-pan-x-horizontal', {
    touchAction: 'pan-x',
    moves: repeat(-15, 0),
  });
  cases.touchActionPanXVertical = await touchCase('touch-action-pan-x-vertical', {
    touchAction: 'pan-x',
    moves: repeat(0, -15),
  });
  cases.touchActionPanYVertical = await touchCase('touch-action-pan-y-vertical', {
    touchAction: 'pan-y',
    moves: repeat(0, -15),
  });
  cases.touchActionPanYHorizontal = await touchCase('touch-action-pan-y-horizontal', {
    touchAction: 'pan-y',
    moves: repeat(-15, 0),
  });
  cases.touchAxisDirectionChange = await touchCase('touch-axis-horizontal-then-vertical', {
    moves: [...repeat(-15, -2), ...repeat(-2, -15)],
  });
  cases.touchCancel = await touchCase('touch-cancel-late-move', {
    moves: repeat(0, -15, 2),
    cancel: true,
  });
  cases.touchAxisContainXAutoYHorizontal = await touchCase('touch-x-contain-y-auto-horizontal-at-boundary', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    moves: repeat(-15, 0),
  });
  cases.touchAxisContainXAutoYVertical = await touchCase('touch-x-contain-y-auto-vertical-at-boundary', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    moves: repeat(0, -15),
  });
  cases.touchAxisContainXAutoYDiagonal = await touchCase('touch-x-contain-y-auto-diagonal-at-boundary', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    moves: repeat(-15, -15),
  });
  cases.touchInnerOuterBoundaryRootAuto = await touchCase('touch-inner-and-outer-y-boundary-chains-to-root', {
    innerTop: 230,
    inner: [0, 106],
    outer: [0, 186],
    moves: repeat(0, -15),
  });
  cases.wheelXContainYAutoX = await wheelCase('wheel-x-contain-y-auto-x-only', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    deltaX: 40,
    deltaY: 0,
  });
  cases.wheelXContainYAutoY = await wheelCase('wheel-x-contain-y-auto-y-only', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    deltaX: 0,
    deltaY: 40,
  });
  cases.wheelXContainYAutoDiagonal = await wheelCase('wheel-x-contain-y-auto-diagonal', {
    inner: [86, 106],
    behavior: ['contain', 'auto'],
    deltaX: 40,
    deltaY: 40,
  });
  cases.wheelAutoDiagonal = await wheelCase('wheel-auto-auto-diagonal', {
    inner: [86, 106],
    behavior: ['auto', 'auto'],
    deltaX: 40,
    deltaY: 40,
  });
  cases.wheelTouchActionNone = await wheelCase('wheel-is-not-restricted-by-touch-action-none', {
    touchAction: 'none',
    deltaX: 0,
    deltaY: 40,
  });

  await navigate();
  await prepare();
  const multiPoint = await centerOf();
  await cdp('Input.dispatchTouchEvent', {
    type: 'touchStart',
    touchPoints: [
      { ...multiPoint, id: 51 },
      { x: multiPoint.x + 30, y: multiPoint.y, id: 52 },
    ],
  });
  await sleep(80);
  await cdp('Input.dispatchTouchEvent', {
    type: 'touchMove',
    touchPoints: [
      { x: multiPoint.x - 8, y: multiPoint.y - 8, id: 51 },
      { x: multiPoint.x + 38, y: multiPoint.y + 8, id: 52 },
    ],
  });
  const multiDuring = await evaluate('c133.settle()');
  await cdp('Input.dispatchTouchEvent', { type: 'touchCancel', touchPoints: [] });
  const multiAfterCancel = await evaluate('c133.settle()');
  await prepare();
  const recoveryPoint = await centerOf();
  await cdp('Input.dispatchTouchEvent', {
    type: 'touchStart',
    touchPoints: [{ ...recoveryPoint, id: 61 }],
  });
  for (let step = 1; step <= 3; step += 1) {
    await sleep(50);
    await cdp('Input.dispatchTouchEvent', {
      type: 'touchMove',
      touchPoints: [{ x: recoveryPoint.x, y: recoveryPoint.y - 15 * step, id: 61 }],
    });
  }
  await cdp('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  const multiRecovery = await evaluate('c133.settle()');
  cases.multiPointer = {
    id: 'two-pointers-cancel-then-single-pointer-recovery',
    input: {
      type: 'direct-touch',
      simultaneousStart: true,
      contacts: [
        { id: 51, start: { x: multiPoint.x, y: multiPoint.y }, move: [-8, -8] },
        { id: 52, start: { x: multiPoint.x + 30, y: multiPoint.y }, move: [8, 8] },
      ],
      terminal: 'touchCancel',
      freshGestureMoves: [[0, -15], [0, -15], [0, -15]],
    },
    visualViewportScale: await evaluate('visualViewport.scale'),
    during: { offsets: offsets(multiDuring), pointerEvents: pointerEvents(multiDuring) },
    afterCancel: { offsets: offsets(multiAfterCancel), pointerEvents: pointerEvents(multiAfterCancel) },
    freshGesture: { offsets: offsets(multiRecovery), scrollEvents: scrollEvents(multiRecovery), pointerEvents: pointerEvents(multiRecovery) },
  };

  const fixtureBytes = await readFile(path.join(root, fixturePath));
  const screenshotBytes = await readFile(path.join(root, screenshotPath));
  const reference = {
    schema: 'spinon-css-c13-3-scroll-gesture-precomparison/v1',
    capturedAt: new Date().toISOString(),
    scope: 'Chrome oracle observation only; not Spinon runtime or app verification.',
    oracle: {
      browser: browser.Browser,
      userAgent: browser['User-Agent'],
      webkitVersion: browser['WebKit-Version'],
      agentBrowserVersion,
      hostPlatform: `${process.platform}-${process.arch}`,
      hostOsVersion,
      viewport,
      touchEmulation: { enabled: true, maxTouchPoints: 2 },
    },
    fixture: { path: fixturePath, sha256: sha256(fixtureBytes) },
    screenshot: {
      path: screenshotPath,
      dimensions: [screenshotBytes.readUInt32BE(16), screenshotBytes.readUInt32BE(20)],
      sha256: sha256(screenshotBytes),
    },
    interpretations: {
      autoTouchMidGestureBoundary: 'The target inner owner reaches its maximum; Chrome does not hand the remaining direct-touch movement to outer in this trace.',
      autoTouchAtBoundaryStart: 'When the gesture starts at the inner y boundary, auto routes the movement to outer.',
      overflowValues: 'In this fixture, hidden does not consume direct touch and auto passes to outer; clip and visible have no inner scroll offset and also leave outer at y=45.',
      touchAction: 'A pre-gesture none value suppresses scrolling; changing auto to none after touchstart does not stop the active gesture. pan-x and pan-y restrict the axis.',
      axisDirectionChange: 'Chrome retains the first horizontal axis when the same auto gesture later turns vertical; this is an observed UA heuristic, not a CSS Overscroll requirement.',
      multiPointer: 'The two CDP touch contacts start together before pan recognition. This records the browser two-contact/cancel behavior and does not model adding a second contact after an active app pan.',
      postCancelInput: 'The browser-side CDP endpoint rejects a touchMove after touchCancel because its touch stream has ended; this harness rejection is not evidence that Spinon ignored a late queued Move.',
      mixedAxisOverscroll: 'The W3C Overscroll Behavior draft says axes should be considered independently. In this Chrome trace, a diagonal delta with inner x=contain and y=auto is blocked on both axes even though x-only/y-only traces are independent. Preserve this as an oracle discrepancy; implementation acceptance follows the per-axis contract in the C13.3 plan.',
      boundaryAffordance: 'The headless oracle records offsets and events only. It does not establish Android glow, iOS bounce, or any local boundary visual.',
      publicEvents: 'DOM PointerEvent, WheelEvent, and scroll-event logs belong to the Chrome oracle fixture, not Spinon public event support.',
    },
    cases,
  };
  await mkdir(evidenceDirectory, { recursive: true });
  await writeFile(referencePath, `${JSON.stringify(reference, null, 2)}\n`);
  console.log(`Captured C13.3 agent-browser oracle: ${referencePath}`);
} finally {
  ws?.close();
  run('agent-browser', ['--session', sessionId, 'close']);
}
