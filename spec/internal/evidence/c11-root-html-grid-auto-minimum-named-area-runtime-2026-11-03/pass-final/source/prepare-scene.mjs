import { readFileSync, writeFileSync } from 'node:fs';

const [scene, output] = process.argv.slice(2);
const columns = scene?.startsWith('columns-') ?? false;
const rows = scene?.startsWith('rows-') ?? false;
const zeroControl = scene?.endsWith('zero-one') ?? false;
const autoZero = scene?.endsWith('auto-zero') ?? false;
const autoOne = scene?.endsWith('auto-one') ?? false;
const validScene = columns || rows;

if (!validScene || !(zeroControl || autoZero || autoOne) || !output) {
  throw new Error('사용법: node prepare-scene.mjs <6개 고정 scene 이름> <출력 HTML 경로>');
}

const track = (autoZero ? 'minmax(auto,0fr)' : zeroControl ? 'minmax(0,1fr)' : 'minmax(auto,1fr)');
const gridStyle = [
  'display:grid',
  'box-sizing:content-box',
  `width:${columns ? '100px' : '80px'}`,
  `height:${columns ? '40px' : '100px'}`,
  `grid-template-columns:${columns ? `${track} 40px` : '40px 40px'}`,
  `grid-template-rows:${rows ? `${track} 40px` : '20px 20px'}`,
  "grid-template-areas:'header header' 'side content'",
  'gap:0',
  'margin:0',
  'padding:0',
  'border-width:2px',
  'border-style:solid',
  'overflow:visible',
  'background-color:#eeeeee',
  'min-width:auto',
  'min-height:auto',
  'max-width:none',
  'max-height:none',
].join(';');

const dimensions = columns
  ? [['auto', '20px'], ['80px', '20px'], ['40px', '20px']]
  : [['80px', '80px'], ['40px', '40px'], ['40px', '40px']];
const names = ['header', 'side', 'content'];
const colors = ['#f4c95d', '#78c59c', '#70a8d8'];
const itemStyles = dimensions.map(([width, height], index) => [
  'display:block',
  'box-sizing:content-box',
  `width:${width}`,
  `height:${height}`,
  'min-width:auto',
  'min-height:auto',
  'max-width:none',
  'max-height:none',
  `grid-area:${names[index]}`,
  'margin:0',
  'padding:0',
  'border-width:0',
  'overflow:visible',
  `background-color:${colors[index]}`,
].join(';'));

let html = readFileSync(new URL('./fixture.html', import.meta.url), 'utf8');
for (const [id, style] of [
  ['grid', gridStyle],
  ['header', itemStyles[0]],
  ['side', itemStyles[1]],
  ['content', itemStyles[2]],
]) {
  const marker = `id="${id}" style="`;
  const start = html.indexOf(marker);
  if (start < 0) throw new Error(`fixture에서 ${id} style을 찾지 못했습니다`);
  const valueStart = start + marker.length;
  const end = html.indexOf('"', valueStart);
  if (end < 0) throw new Error(`fixture의 ${id} style 속성이 닫히지 않았습니다`);
  html = `${html.slice(0, valueStart)}${style}${html.slice(end)}`;
}
writeFileSync(output, html);
