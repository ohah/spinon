const nodes = Object.create(null);

function c184Node(parent, tag, id, declarations, src) {
  if (nodes[id]) throw new Error('C18.4 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement(tag);
  element.setAttribute('id', id);
  element.setAttribute('data-c184-node', '');
  element.setAttribute('style', declarations);
  if (src !== undefined) element.setAttribute('src', src);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const body = document.body || document;
const root = c184Node(
  body,
  'div',
  'c18-async-image-root',
  'display:block;position:relative;width:224px;height:88px;background-color:#112233',
);

const images = [
  ['current-b', 8, 8, 'spinon-fixture:c18-4/b-fast'],
  ['pending-c', 64, 8, 'spinon-fixture:c18-4/c-slow'],
  ['c-success', 120, 8, 'spinon-fixture:c18-4/c-slow'],
  ['decode-failure', 176, 8, 'spinon-fixture:c18-4/broken'],
  ['recovery-d', 8, 48, 'spinon-fixture:c18-4/d-fast'],
];

for (const [id, x, y, src] of images) {
  c184Node(
    root,
    'img',
    id,
    `display:block;position:absolute;left:${x}px;top:${y}px;width:48px;height:32px;background-color:#d20ab4;object-fit:fill`,
    src,
  );
}

globalThis.spinonC184NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
