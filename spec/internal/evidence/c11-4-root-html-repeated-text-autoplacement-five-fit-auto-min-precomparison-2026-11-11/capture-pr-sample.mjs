import { execFileSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';

const session = 'spinon-c114-five-fit-pr-sample-20261111';
const here = dirname(fileURLToPath(import.meta.url));
const fixture = join(here, 'oracle-fixture.html');
const screenshot = join(here, 'chrome157-five-fit-auto-min-pr-sample.png');
const run = (args) => execFileSync('agent-browser', ['--session', session, ...args], {
  encoding: 'utf8', maxBuffer: 4 * 1024 * 1024,
}).trim();
let opened = false;
let closed = false;
try {
  opened = true;
  run(['open', pathToFileURL(fixture).href]);
  run(['set', 'viewport', '1280', '900']);
  run(['eval', `(() => {
    const style = document.createElement('style');
    style.textContent = '#sample{align-items:flex-start;gap:70px} #sample-grid{transform:scale(2.5);transform-origin:top left;margin-right:60px;margin-bottom:330px} #sample-summary{font-size:12px;line-height:1.45;background:#f8fafc;padding:18px;border:1px solid #cbd5e1}';
    document.head.append(style);
    window.scrollTo(0, 0);
  })()`]);
  run(['screenshot', screenshot]);
  process.stdout.write(`SCREENSHOT ${screenshot}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
