const nodes = Object.create(null);

function c183Node(parent, tag, id, declarations, src) {
  if (nodes[id]) throw new Error('C18.3 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement(tag);
  element.setAttribute('id', id);
  element.setAttribute('data-c183-node', '');
  element.setAttribute('style', declarations);
  if (src !== undefined) element.setAttribute('src', src);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const body = document.body || document;
const root = c183Node(
  body,
  'div',
  'c18-object-fit-root',
  'display:block;position:relative;width:224px;height:72px;background-color:#112233',
);

const imageRows = [
  ['fill-center', 8, 8, 24, 24, 'fill', '50% 50%'],
  ['contain-center', 40, 8, 24, 24, 'contain', '50% 50%'],
  ['contain-quarter', 72, 8, 24, 24, 'contain', '25% 75%'],
  ['cover-center', 104, 8, 24, 24, 'cover', '50% 50%'],
  ['cover-start', 136, 8, 24, 24, 'cover', '0% 0%'],
  ['cover-end', 168, 8, 24, 24, 'cover', '100% 100%'],
  ['none-clip', 8, 40, 16, 16, 'none', '100% 100%'],
  ['scale-down-shrink', 40, 40, 16, 16, 'scale-down', '50% 50%'],
  ['scale-down-no-upscale', 72, 40, 32, 20, 'scale-down', '50% 50%'],
  ['initial-values', 112, 40, 24, 12, null, null],
  ['contain-extended-position', 144, 40, 24, 24, 'contain', '-25% 150%'],
  ['cover-extended-position', 176, 40, 24, 24, 'cover', '150% -25%'],
];

for (const [id, x, y, width, height, fit, position] of imageRows) {
  const objectStyle = fit === null ? '' : `;object-fit:${fit};object-position:${position}`;
  c183Node(
    root,
    'img',
    id,
    `display:block;position:absolute;left:${x}px;top:${y}px;width:${width}px;height:${height}px;background-color:#d20ab4;image-rendering:auto${objectStyle}`,
    'spinon-fixture:c18-3/object-fit-pattern',
  );
}

globalThis.spinonC183NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
