import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { spawn, spawnSync } from 'node:child_process';
import { access, mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { release as osRelease, tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixtureRelative = 'tests/fixtures/css/c04/runtime-style-layout.html';
const inventoryRelative = 'tests/fixtures/css/c04/runtime-style-layout-inventory.json';
const captureRelative = 'tools/css-reference/capture-runtime-style-layout.mjs';
const fixturePath = join(root, fixtureRelative);
const inventoryPath = join(root, inventoryRelative);
const versionRequired = '154.0.8037.98';
const revisionRequired = '@b859317bf11f6be47f9b7799ec690a0a42a1fb33';
const binaryHashRequired = 'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954';
const viewportRequired = { width: 800, height: 600, deviceScaleFactor: 1 };
const chromiumCandidates = process.platform === 'darwin'
  ? ['/Applications/Google Chrome.app/Contents/MacOS/Google Chrome']
  : process.platform === 'linux'
    ? ['/usr/bin/chromium', '/usr/bin/chromium-browser', '/usr/bin/google-chrome']
    : [];

function connectDevTools(url) {
  const socket = new WebSocket(url);
  const pending = new Map();
  let nextId = 1;
  const opened = new Promise((resolve, reject) => {
    socket.addEventListener('open', resolve, { once: true });
    socket.addEventListener('error', () => reject(new Error('Chromium DevTools 연결에 실패했습니다.')), { once: true });
  });
  socket.addEventListener('message', (event) => {
    const message = JSON.parse(String(event.data));
    if (message.id === undefined) return;
    const request = pending.get(message.id);
    if (!request) return;
    clearTimeout(request.timeout);
    pending.delete(message.id);
    if (message.error) request.reject(new Error(`DevTools ${request.method}: ${message.error.message}`));
    else request.resolve(message.result ?? {});
  });
  socket.addEventListener('close', () => {
    for (const request of pending.values()) {
      clearTimeout(request.timeout);
      request.reject(new Error('Chromium DevTools 연결이 닫혔습니다.'));
    }
    pending.clear();
  });
  return {
    opened,
    send(method, params = {}) {
      return new Promise((resolve, reject) => {
        const id = nextId++;
        const timeout = setTimeout(() => {
          pending.delete(id);
          reject(new Error(`DevTools 명령 시간이 초과됐습니다: ${method}`));
        }, 10_000);
        pending.set(id, { method, resolve, reject, timeout });
        socket.send(JSON.stringify({ id, method, params }));
      });
    },
    waitForEvent(method) {
      return new Promise((resolve) => {
        const listener = (event) => {
          socket.removeEventListener('message', listener);
          const message = JSON.parse(String(event.data));
          if (message.method === method) resolve(message.params ?? {});
          else socket.addEventListener('message', listener);
        };
        socket.addEventListener('message', listener);
      });
    },
    close() { socket.close(); },
  };
}

async function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function sha256File(path) {
  return sha256(await readFile(path));
}

async function waitForPort(profilePath, child) {
  const portFile = join(profilePath, 'DevToolsActivePort');
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (child.exitCode !== null) throw new Error(`Chromium이 일찍 종료됐습니다: ${child.exitCode}`);
    try {
      const [port] = (await readFile(portFile, 'utf8')).trim().split('\n');
      if (port) return port;
    } catch {
      // Chromium이 임시 DevTools 포트를 열 때까지 기다립니다.
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error('Chromium DevTools 포트를 열지 못했습니다.');
}

function stopProcessGroup(child, signal) {
  if (child?.pid === undefined || child.exitCode !== null) return;
  try {
    process.kill(-child.pid, signal);
  } catch (error) {
    if (error?.code !== 'ESRCH') throw error;
  }
}

async function waitForExit(child, timeoutMs) {
  if (child.exitCode !== null) return;
  await Promise.race([
    new Promise((resolve) => child.once('exit', resolve)),
    new Promise((resolve) => setTimeout(resolve, timeoutMs)),
  ]);
}

function validateObservation(observation, inventory) {
  if (observation.fixtureId !== inventory.fixtureId
    || JSON.stringify(observation.viewport) !== JSON.stringify(viewportRequired)) {
    throw new Error('Chromium fixture ID 또는 viewport가 고정 입력과 다릅니다.');
  }
  if (observation.environment?.locale !== 'en-US' || observation.environment?.intlLocale !== 'en-US'
    || observation.environment?.timeZone !== 'UTC' || observation.environment?.dark !== false
    || observation.environment?.reducedMotion !== false || observation.environment?.forcedColors !== false) {
    throw new Error(`Chromium 환경이 고정 입력과 다릅니다: ${JSON.stringify(observation.environment)}`);
  }
  if (!Array.isArray(observation.nodes) || observation.nodes.length !== inventory.nodes.length) {
    throw new Error('Chromium node 수가 inventory와 다릅니다.');
  }
  for (const [index, expected] of inventory.nodes.entries()) {
    const observed = observation.nodes[index];
    if (!observed || !observed.properties || observed.id !== expected.id
      || Object.keys(observed.properties).length !== expected.properties.length
      || expected.properties.some((property) => typeof observed.properties[property] !== 'string'
        || observed.properties[property].length === 0)) {
      throw new Error(`Chromium computed property가 inventory와 다릅니다: ${expected.id}`);
    }
    if (!['x', 'y', 'width', 'height'].every((key) => Number.isFinite(observed.rect?.[key]))) {
      throw new Error(`Chromium geometry가 유한값이 아닙니다: ${expected.id}`);
    }
  }
}

const chromiumPath = process.env.SPINON_CHROMIUM_BIN ?? chromiumCandidates[0];
if (!chromiumPath) throw new Error('Chromium 실행 파일을 찾지 못했습니다. SPINON_CHROMIUM_BIN을 설정하세요.');
const [fixtureBytes, inventoryBytes] = await Promise.all([readFile(fixturePath), readFile(inventoryPath)]);
const fixtureSha256 = await sha256(fixtureBytes);
const inventorySha256 = await sha256(inventoryBytes);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
if (inventory.schema !== 'spinon-css-runtime-layout-inventory'
  || inventory.fixtureId !== 'C04-runtime-style-layout'
  || JSON.stringify(inventory.viewport) !== JSON.stringify(viewportRequired)
  || !Array.isArray(inventory.nodes) || inventory.nodes.length !== 5
  || new Set(inventory.nodes.map((node) => node.id)).size !== inventory.nodes.length
  || inventory.nodes.some((node) => !node || typeof node.id !== 'string' || !Array.isArray(node.properties)
    || new Set(node.properties).size !== node.properties.length
    || node.properties.some((property) => typeof property !== 'string' || property.length === 0))) {
  throw new Error('Runtime layout inventory의 schema·범위·viewport가 예상과 다릅니다.');
}

const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8' });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const version = versionResult.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
const executableSha256 = await sha256File(chromiumPath);
if (version !== versionRequired || executableSha256 !== binaryHashRequired) {
  throw new Error(`Chromium 기준이 고정값과 다릅니다: ${version ?? 'unknown'} ${executableSha256}`);
}

const profilePath = await mkdtemp(join(tmpdir(), 'spinon-runtime-css-layout-'));
const flags = [
  '--headless=new', '--no-first-run', '--no-default-browser-check', '--disable-background-networking',
  '--lang=en-US', '--remote-debugging-port=0', '--user-data-dir=<temporary-profile>',
];
let browserProcess;
let browserDevTools;
let pageDevTools;

try {
  browserProcess = spawn(chromiumPath, [
    ...flags.filter((flag) => !flag.startsWith('--user-data-dir=')),
    `--user-data-dir=${profilePath}`, 'about:blank',
  ], { detached: true, env: { ...process.env, TZ: 'UTC' }, stdio: 'ignore' });
  browserProcess.once('error', (error) => console.error('Chromium 실행 오류:', error.message));

  const port = await waitForPort(profilePath, browserProcess);
  const browserInfo = await fetch(`http://127.0.0.1:${port}/json/version`).then((response) => response.json());
  browserDevTools = connectDevTools(browserInfo.webSocketDebuggerUrl);
  await browserDevTools.opened;
  const browserVersion = await browserDevTools.send('Browser.getVersion');
  if (browserVersion.product.match(/\d+\.\d+\.\d+\.\d+/)?.[0] !== versionRequired
    || browserVersion.revision !== revisionRequired) {
    throw new Error(`Chromium CDP 기준이 다릅니다: ${browserVersion.product} ${browserVersion.revision}`);
  }

  const pageTarget = (await fetch(`http://127.0.0.1:${port}/json/list`).then((response) => response.json()))
    .find((target) => target.type === 'page');
  if (!pageTarget) throw new Error('Chromium page target을 찾지 못했습니다.');
  pageDevTools = connectDevTools(pageTarget.webSocketDebuggerUrl);
  await pageDevTools.opened;
  await pageDevTools.send('Page.enable');
  await pageDevTools.send('Runtime.enable');
  await pageDevTools.send('Page.addScriptToEvaluateOnNewDocument', {
    source: `Object.defineProperty(globalThis, '__SPINON_C04_RUNTIME_LAYOUT_INVENTORY__', {
      configurable: false, enumerable: false, writable: false,
      value: Object.freeze(${JSON.stringify(inventory)}),
    });`,
  });
  await pageDevTools.send('Emulation.setDeviceMetricsOverride', {
    width: viewportRequired.width, height: viewportRequired.height,
    deviceScaleFactor: viewportRequired.deviceScaleFactor, mobile: false,
    screenWidth: viewportRequired.width, screenHeight: viewportRequired.height,
  });
  await pageDevTools.send('Emulation.setLocaleOverride', { locale: 'en-US' });
  await pageDevTools.send('Emulation.setUserAgentOverride', {
    userAgent: browserVersion.userAgent, acceptLanguage: 'en-US',
  });
  await pageDevTools.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
  await pageDevTools.send('Emulation.setEmulatedMedia', {
    features: [
      { name: 'prefers-color-scheme', value: 'light' },
      { name: 'prefers-reduced-motion', value: 'no-preference' },
      { name: 'forced-colors', value: 'none' },
    ],
  });
  const pageLoaded = pageDevTools.waitForEvent('Page.loadEventFired');
  await pageDevTools.send('Page.navigate', { url: pathToFileURL(fixturePath).href });
  await pageLoaded;
  const evaluation = await pageDevTools.send('Runtime.evaluate', {
    expression: 'document.querySelector("#reference-result")?.textContent ?? ""', returnByValue: true,
  });
  const encoded = evaluation.result?.value;
  if (evaluation.exceptionDetails || typeof encoded !== 'string' || !encoded) {
    throw new Error(`Chromium fixture 캡처 실패: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
  }
  const observation = JSON.parse(Buffer.from(encoded, 'base64').toString('utf8'));
  validateObservation(observation, inventory);

  const platform = process.platform === 'darwin' ? 'macos' : process.platform;
  let osVersion = `${platform} ${osRelease()}`;
  let osBuild = null;
  if (process.platform === 'darwin') {
    const product = spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8' });
    const build = spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8' });
    if (product.status === 0 && build.status === 0) {
      osVersion = `macOS ${product.stdout.trim()}`;
      osBuild = build.stdout.trim();
    }
  }
  const captureSha256 = await sha256File(fileURLToPath(import.meta.url));
  const osId = [osVersion, osBuild].filter(Boolean).join('-').replace(/[^a-zA-Z0-9.-]/g, '-').toLowerCase();
  const referenceId = `chromium-${platform}-${process.arch}-${osId}-c04-runtime-layout-${fixtureSha256.slice(0, 12)}-${inventorySha256.slice(0, 12)}-${captureSha256.slice(0, 12)}-${executableSha256.slice(0, 12)}`;
  const outputDirectory = join(root, 'tests/fixtures/css/references', referenceId);
  const outputPath = join(outputDirectory, 'runtime-style-layout.json');
  try {
    await access(outputPath);
    throw new Error(`기준 snapshot은 덮어쓰지 않습니다: ${outputPath}`);
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
  const reference = {
    schema: 'spinon-css-reference/v1', referenceId,
    captureTool: { path: captureRelative, sha256: captureSha256, inventorySha256 },
    oracle: {
      name: 'Chromium', product: versionResult.stdout.trim(), version,
      revision: browserVersion.revision, executable: chromiumPath, executableSha256,
    },
    environment: {
      os: osVersion, osBuild, architecture: process.arch, nodeVersion: process.version,
      flags, emulation: {
        cssViewportPx: { width: 800, height: 600 }, deviceScaleFactor: 1,
        locale: 'en-US', timeZone: 'UTC', media: inventory.environment,
      },
    },
    fixture: {
      id: inventory.fixtureId, path: fixtureRelative, sha256: fixtureSha256,
      inventoryPath: inventoryRelative, inventorySha256, comparison: inventory.comparison,
    },
    observations: observation,
  };
  await mkdir(outputDirectory, { recursive: true });
  await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`, { flag: 'wx' });
  console.log(`저장: ${outputPath}`);
  console.log(`Chromium: ${version} ${browserVersion.revision}`);
  console.log(`node ${observation.nodes.length}개, computed property ${observation.nodes.reduce((sum, node) => sum + Object.keys(node.properties).length, 0)}개`);
} finally {
  pageDevTools?.close();
  browserDevTools?.close();
  if (browserProcess) {
    stopProcessGroup(browserProcess, 'SIGTERM');
    await waitForExit(browserProcess, 2_000);
    stopProcessGroup(browserProcess, 'SIGKILL');
  }
  await rm(profilePath, { recursive: true, force: true });
}
