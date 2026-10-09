import assert from 'node:assert/strict';
import test from 'node:test';
import { assertProcessGroupOwnership, waitForWebSocketOpen } from './chromium-session.mjs';

test('DevTools WebSocket 열림·오류·조기 종료·시간 초과를 구분한다', async () => {
  const connectedSocket = new EventTarget();
  const connected = waitForWebSocketOpen(connectedSocket, 1_000);
  connectedSocket.dispatchEvent(new Event('open'));
  await connected;

  const erroredSocket = new EventTarget();
  const errored = waitForWebSocketOpen(erroredSocket, 1_000);
  erroredSocket.dispatchEvent(new Event('error'));
  await assert.rejects(errored, /연결에 실패/);

  const closedSocket = new EventTarget();
  const closed = waitForWebSocketOpen(closedSocket, 1_000);
  closedSocket.dispatchEvent(new Event('close'));
  await assert.rejects(closed, /열기 전에 닫혔습니다/);

  await assert.rejects(waitForWebSocketOpen(new EventTarget(), 1), /시간이 초과됐습니다/);
  assert.throws(() => waitForWebSocketOpen(new EventTarget(), 0), RangeError);
});

test('Chromium 종료 전에 group 멤버 모두가 이 캡처의 임시 profile을 가리켜야 한다', () => {
  const output = [
    '1234 1234 /Applications/Google Chrome --user-data-dir=/tmp/spinon-css-property-surface-a1b2',
    '1235 1234 /Applications/Google Chrome Helper --type=renderer --user-data-dir=/tmp/spinon-css-property-surface-a1b2',
  ].join('\n');
  assert.equal(assertProcessGroupOwnership(output, 1234, '/tmp/spinon-css-property-surface-a1b2').length, 2);
  assert.throws(() => assertProcessGroupOwnership(output, 9999, '/tmp/spinon-css-property-surface-a1b2'), /멤버/);
  assert.throws(() => assertProcessGroupOwnership(
    `${output}\n1236 1234 /Applications/Google Chrome Helper --type=crashpad`,
    1234,
    '/tmp/spinon-css-property-surface-a1b2',
  ), /profile을 확인할 수 없는/);
});
