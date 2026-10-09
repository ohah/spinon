import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixturePath = 'tests/fixtures/css/c05/runtime-registered-properties.html';
const html = await readFile(join(repositoryRoot, fixturePath), 'utf8');
const chromiumPath = await findChromiumExecutable();
const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    assert.equal(browserVersion.product, 'Chrome/154.0.8037.98');
    assert.equal(browserVersion.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: 301,
      height: 100,
      deviceScaleFactor: 1,
      mobile: false,
    });
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    const pageLoaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', {
      url: `data:text/html,${encodeURIComponent(html)}`,
    });
    await pageLoaded;
    const evaluated = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const readConnected = () => Object.fromEntries(['app', 'declared', 'inherited'].map((id) => {
          const node = document.getElementById(id);
          const style = getComputedStyle(node);
          const rect = node.getBoundingClientRect();
          return [id, {
            width: style.width,
            rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
          }];
        }));
        const connectedBefore = readConnected();
        const detached = document.createElement('div');
        detached.setAttribute('style', 'width:113px;height:17px;--probe:7px');
        detached.setAttribute('style', 'width:227px;height:29px;--probe:11px');
        const rect = detached.getBoundingClientRect();
        return JSON.stringify({
          connectedBefore,
          connectedAfter: readConnected(),
          detachedConnected: detached.isConnected,
          detachedStyle: detached.getAttribute('style'),
          detachedRect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        });
      })()`,
      returnByValue: true,
    });
    if (evaluated.exceptionDetails) {
      throw new Error(`Chromium 비교를 실행하지 못했습니다: ${evaluated.exceptionDetails.text}`);
    }
    return JSON.parse(evaluated.result.value);
  },
});

assert.deepEqual(captureResult.connectedAfter, captureResult.connectedBefore);
assert.equal(captureResult.detachedConnected, false);
assert.deepEqual(captureResult.detachedRect, { x: 0, y: 0, width: 0, height: 0 });
assert.equal(captureResult.detachedStyle, 'width:227px;height:29px;--probe:11px');
process.stdout.write(`${JSON.stringify(captureResult, null, 2)}\n`);
