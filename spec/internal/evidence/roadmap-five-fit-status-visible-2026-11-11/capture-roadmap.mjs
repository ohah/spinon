import { execFileSync } from 'node:child_process';
import { writeFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';

const session = 'spinon-roadmap-five-fit-status-pr1348-20261111';
const here = dirname(fileURLToPath(import.meta.url));
const roadmap = join(here, '../../../public/roadmap.html');
const desktopImage = join(here, 'roadmap-status-agent-browser-desktop.png');
const mobileImage = join(here, 'roadmap-status-agent-browser-mobile.png');
const reportPath = join(here, 'layout-verification.json');
const run = (args) => execFileSync('agent-browser', ['--session', session, ...args], {
  encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
}).trim();
const parse = (value) => {
  const decoded = JSON.parse(value);
  return typeof decoded === 'string' ? JSON.parse(decoded) : decoded;
};
let opened = false;
let closed = false;
try {
  opened = true;
  run(['open', pathToFileURL(roadmap).href]);
  run(['wait', '--load', 'domcontentloaded']);
  run(['wait', '--fn', 'document.getElementById("kanban-board")?.innerText.includes("C11.4 five-fit candidate")']);

  run(['set', 'viewport', '1440', '1000']);
  const desktop = parse(run(['eval', `JSON.stringify((() => {
    const note = document.querySelector('#kanban-board + .hero-progress');
    const board = document.getElementById('kanban-board');
    return {
      viewportWidth: innerWidth,
      documentWidth: document.documentElement.scrollWidth,
      noteText: note?.innerText ?? '',
      noteParentId: note?.parentElement?.id ?? '',
      noteVisible: !!note && getComputedStyle(note).display !== 'none',
      boardText: board?.innerText ?? '',
      officialStatusText: document.getElementById('official-status-note')?.innerText ?? ''
    };
  })())`]));
  if (!desktop.noteVisible || desktop.noteParentId === 'kanban-board'
      || !desktop.noteText.includes('17,280 scenes') || !desktop.boardText.includes('계획 완료')
      || desktop.documentWidth > desktop.viewportWidth) {
    throw new Error(`desktop assertion failed: ${JSON.stringify(desktop)}`);
  }
  run(['screenshot', desktopImage]);

  run(['set', 'viewport', '390', '844']);
  const mobile = parse(run(['eval', `JSON.stringify((() => {
    const note = document.querySelector('#kanban-board + .hero-progress');
    const board = document.getElementById('kanban-board');
    return {
      viewportWidth: innerWidth,
      documentWidth: document.documentElement.scrollWidth,
      noteVisible: !!note && getComputedStyle(note).display !== 'none',
      noteParentId: note?.parentElement?.id ?? '',
      boardClientWidth: board?.clientWidth ?? 0,
      boardScrollWidth: board?.scrollWidth ?? 0,
      boardOverflowX: board ? getComputedStyle(board).overflowX : ''
    };
  })())`]));
  if (!mobile.noteVisible || mobile.noteParentId === 'kanban-board'
      || mobile.documentWidth > mobile.viewportWidth || mobile.boardOverflowX !== 'auto'
      || mobile.boardScrollWidth <= mobile.boardClientWidth) {
    throw new Error(`mobile assertion failed: ${JSON.stringify(mobile)}`);
  }
  run(['screenshot', mobileImage]);
  writeFileSync(reportPath, `${JSON.stringify({ browser: 'Chrome 157.0.8103.40', agentBrowser: '0.31.1', desktop, mobile }, null, 2)}\n`);
  process.stdout.write(`LAYOUT_VERIFIED ${reportPath}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
