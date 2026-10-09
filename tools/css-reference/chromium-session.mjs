import { createReadStream } from 'node:fs';
import { spawn, spawnSync } from 'node:child_process';
import { access, mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { isAbsolute, join, resolve } from 'node:path';
import { createHash } from 'node:crypto';

const defaultChromiumPaths = process.platform === 'darwin'
  ? ['/Applications/Google Chrome.app/Contents/MacOS/Google Chrome']
  : process.platform === 'linux'
    ? ['/usr/bin/chromium', '/usr/bin/chromium-browser', '/usr/bin/google-chrome']
    : [];
const launchFlags = [
  '--headless=new',
  '--no-first-run',
  '--no-default-browser-check',
  '--disable-background-networking',
  '--lang=en-US',
  '--remote-debugging-port=0',
  '--user-data-dir=<temporary-profile>',
];

function sleep(milliseconds) {
  return new Promise((resolveSleep) => setTimeout(resolveSleep, milliseconds));
}

export function waitForWebSocketOpen(socket, timeoutMs = 5_000) {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    throw new RangeError('WebSocket 연결 제한 시간은 양수여야 합니다.');
  }
  return new Promise((resolveOpen, rejectOpen) => {
    const cleanup = () => {
      clearTimeout(timeout);
      socket.removeEventListener('open', onOpen);
      socket.removeEventListener('error', onError);
      socket.removeEventListener('close', onClose);
    };
    const onOpen = () => {
      cleanup();
      resolveOpen();
    };
    const onError = () => {
      cleanup();
      rejectOpen(new Error('Chromium DevTools 연결에 실패했습니다.'));
    };
    const onClose = () => {
      cleanup();
      rejectOpen(new Error('Chromium DevTools가 연결을 열기 전에 닫혔습니다.'));
    };
    const timeout = setTimeout(() => {
      cleanup();
      rejectOpen(new Error('Chromium DevTools WebSocket 연결 시간이 초과됐습니다.'));
    }, timeoutMs);
    socket.addEventListener('open', onOpen);
    socket.addEventListener('error', onError);
    socket.addEventListener('close', onClose);
  });
}

export async function findChromiumExecutable() {
  const configuredPath = process.env.SPINON_CHROMIUM_BIN;
  if (configuredPath !== undefined) {
    if (!isAbsolute(configuredPath)) {
      throw new Error('SPINON_CHROMIUM_BIN은 절대 경로여야 합니다.');
    }
    try {
      await access(configuredPath);
      return resolve(configuredPath);
    } catch (error) {
      throw new Error(`SPINON_CHROMIUM_BIN 실행 파일을 읽을 수 없습니다: ${configuredPath}`, { cause: error });
    }
  }

  for (const candidate of defaultChromiumPaths) {
    try {
      await access(candidate);
      return candidate;
    } catch {
      // 표준 설치 경로를 순서대로 확인합니다.
    }
  }
  throw new Error('Chromium 실행 파일을 찾지 못했습니다. SPINON_CHROMIUM_BIN에 절대 경로를 지정하세요.');
}

export async function sha256File(path) {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest('hex');
}

function connectDevTools(url) {
  const socket = new WebSocket(url);
  const pending = new Map();
  const eventListeners = new Map();
  let nextId = 1;
  const opened = waitForWebSocketOpen(socket);

  socket.addEventListener('message', (event) => {
    const message = JSON.parse(String(event.data));
    if (message.id !== undefined) {
      const request = pending.get(message.id);
      if (!request) return;
      clearTimeout(request.timeout);
      pending.delete(message.id);
      if (message.error) request.reject(new Error(`DevTools ${request.method}: ${message.error.message}`));
      else request.resolve(message.result ?? {});
      return;
    }
    const listeners = eventListeners.get(message.method);
    if (!listeners) return;
    eventListeners.delete(message.method);
    for (const listener of listeners) listener(message.params ?? {});
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
      return new Promise((resolveResult, rejectResult) => {
        const id = nextId++;
        const timeout = setTimeout(() => {
          pending.delete(id);
          rejectResult(new Error(`Chromium DevTools 명령 시간이 초과됐습니다: ${method}`));
        }, 10_000);
        pending.set(id, { method, resolve: resolveResult, reject: rejectResult, timeout });
        socket.send(JSON.stringify({ id, method, params }));
      });
    },
    waitForEvent(method) {
      return new Promise((resolveEvent, rejectEvent) => {
        const listener = (params) => {
          clearTimeout(timeout);
          resolveEvent(params);
        };
        const listeners = eventListeners.get(method) ?? [];
        listeners.push(listener);
        eventListeners.set(method, listeners);
        const timeout = setTimeout(() => {
          const remaining = (eventListeners.get(method) ?? []).filter((candidate) => candidate !== listener);
          if (remaining.length === 0) eventListeners.delete(method);
          else eventListeners.set(method, remaining);
          rejectEvent(new Error(`Chromium DevTools event 시간이 초과됐습니다: ${method}`));
        }, 10_000);
      });
    },
    close() {
      socket.close();
    },
  };
}

async function waitForDevToolsPort(profilePath, child, getSpawnError) {
  const activePortPath = join(profilePath, 'DevToolsActivePort');
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const spawnError = getSpawnError();
    if (spawnError) throw new Error(`Chromium 프로세스를 실행하지 못했습니다: ${spawnError.message}`, { cause: spawnError });
    if (child.exitCode !== null) throw new Error(`Chromium이 DevTools 시작 전에 종료됐습니다: ${child.exitCode}`);
    try {
      const [port, browserPath] = (await readFile(activePortPath, 'utf8')).trim().split('\n');
      if (port && browserPath) return port;
    } catch {
      // Chromium이 디버깅 포트를 준비할 때까지 기다립니다.
    }
    await sleep(100);
  }
  throw new Error('Chromium이 제한 시간 안에 DevTools 포트를 열지 못했습니다.');
}

function processGroupExists(processGroupId) {
  try {
    process.kill(-processGroupId, 0);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') return false;
    if (error?.code === 'EPERM') return true;
    throw error;
  }
}

export function assertProcessGroupOwnership(processList, processGroupId, profilePath) {
  const members = processList.split('\n').flatMap((line) => {
    const match = line.match(/^\s*(\d+)\s+(\d+)\s+(.+?)\s*$/);
    return match && Number(match[2]) === processGroupId
      ? [{ pid: Number(match[1]), command: match[3] }]
      : [];
  });
  if (members.length === 0) {
    throw new Error(`Chromium process group 멤버를 확인하지 못했습니다: ${processGroupId}`);
  }
  const unrelated = members.filter(({ command }) => !command.includes(profilePath));
  if (unrelated.length > 0) {
    throw new Error(`Chromium process group에 임시 profile을 확인할 수 없는 프로세스가 있습니다: ${processGroupId}`);
  }
  return members;
}

function assertOwnedProcessGroup(processGroupId, profilePath) {
  const result = spawnSync('ps', ['-ww', '-Ao', 'pid=,pgid=,command='], {
    encoding: 'utf8',
    timeout: 5_000,
  });
  if (result.error || result.status !== 0) {
    throw new Error(`Chromium process group 소유자를 확인하지 못했습니다: ${result.error?.message ?? result.stderr}`);
  }
  assertProcessGroupOwnership(result.stdout, processGroupId, profilePath);
}

async function waitForProcessGroupExit(processGroupId, timeoutMs) {
  const deadline = performance.now() + timeoutMs;
  while (performance.now() < deadline) {
    if (!processGroupExists(processGroupId)) return true;
    await sleep(50);
  }
  return !processGroupExists(processGroupId);
}

async function stopChromiumProcessGroup(child, profilePath) {
  if (child?.pid === undefined) return;
  for (const signal of ['SIGTERM', 'SIGKILL']) {
    if (!processGroupExists(child.pid)) return;
    assertOwnedProcessGroup(child.pid, profilePath);
    try {
      process.kill(-child.pid, signal);
    } catch (error) {
      if (error?.code === 'ESRCH') return;
      throw error;
    }
    if (await waitForProcessGroupExit(child.pid, 1_500)) return;
  }
  throw new Error(`이 실행에서 시작한 Chromium process group을 종료하지 못했습니다: ${child.pid}`);
}

export async function runChromiumPage({ chromiumPath, onPage }) {
  const profilePath = await mkdtemp(join(tmpdir(), 'spinon-css-property-surface-'));
  const browserFlags = launchFlags;
  const browserArguments = [
    ...launchFlags.filter((flag) => !flag.startsWith('--user-data-dir=')),
    `--user-data-dir=${profilePath}`,
    'about:blank',
  ];
  const browserProcess = spawn(chromiumPath, browserArguments, {
    detached: true,
    env: { ...process.env, TZ: 'UTC' },
    stdio: 'ignore',
  });
  let spawnError = null;
  browserProcess.once('error', (error) => { spawnError = error; });
  let browserDevTools;
  let pageDevTools;
  let captureResult;
  let captureError = null;
  let cleanupError = null;
  let processGroupStopped = false;

  try {
    const port = await waitForDevToolsPort(profilePath, browserProcess, () => spawnError);
    const versionResponse = await fetch(`http://127.0.0.1:${port}/json/version`, {
      signal: AbortSignal.timeout(5_000),
    });
    if (!versionResponse.ok) throw new Error(`Chromium DevTools version 조회 실패: HTTP ${versionResponse.status}`);
    const browserInformation = await versionResponse.json();
    if (typeof browserInformation.webSocketDebuggerUrl !== 'string') {
      throw new Error('Chromium DevTools version 응답에 browser WebSocket 주소가 없습니다.');
    }
    browserDevTools = connectDevTools(browserInformation.webSocketDebuggerUrl);
    await browserDevTools.opened;
    const browserVersion = await browserDevTools.send('Browser.getVersion');

    const targetsResponse = await fetch(`http://127.0.0.1:${port}/json/list`, {
      signal: AbortSignal.timeout(5_000),
    });
    if (!targetsResponse.ok) throw new Error(`Chromium page target 조회 실패: HTTP ${targetsResponse.status}`);
    const targets = await targetsResponse.json();
    const pageTarget = targets.find((target) => target.type === 'page');
    if (typeof pageTarget?.webSocketDebuggerUrl !== 'string') {
      throw new Error('Chromium page target을 찾지 못했습니다.');
    }
    pageDevTools = connectDevTools(pageTarget.webSocketDebuggerUrl);
    await pageDevTools.opened;
    captureResult = await onPage({
      page: pageDevTools,
      browserVersion,
      browserFlags,
    });
  } catch (error) {
    captureError = error;
  } finally {
    pageDevTools?.close();
    browserDevTools?.close();
    try {
      await stopChromiumProcessGroup(browserProcess, profilePath);
      processGroupStopped = true;
    } catch (error) {
      cleanupError = error;
    }
    if (processGroupStopped) {
      try {
        await rm(profilePath, { recursive: true });
      } catch (error) {
        cleanupError ??= new Error(`임시 Chromium profile을 정리하지 못했습니다: ${profilePath}`, { cause: error });
      }
    } else {
      cleanupError ??= new Error(`Chromium 종료를 확인할 수 없어 임시 profile을 보존했습니다: ${profilePath}`);
    }
  }

  if (captureError && cleanupError) {
    throw new AggregateError([captureError, cleanupError], 'Chromium 캡처와 종료 정리가 모두 실패했습니다.');
  }
  if (captureError) throw captureError;
  if (cleanupError) throw cleanupError;
  return { captureResult, browserFlags };
}
