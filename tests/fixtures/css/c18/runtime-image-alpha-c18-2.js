const nodes = Object.create(null);

function c182Node(parent, tag, id, declarations, src) {
  if (nodes[id]) throw new Error('C18.2 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement(tag);
  element.setAttribute('id', id);
  element.setAttribute('data-c182-node', '');
  element.setAttribute('style', declarations);
  if (src !== undefined) element.setAttribute('src', src);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const body = document.body || document;
const root = c182Node(
  body,
  'div',
  'c18-alpha-root',
  'display:flow-root;position:relative;box-sizing:border-box;width:80px;height:40px;background-color:#112233',
);
c182Node(
  root,
  'img',
  'c18-alpha-image',
  'position:absolute;left:8px;top:12px;display:block;box-sizing:border-box;width:24px;height:16px;padding:0;border:0',
  'spinon-fixture:c18-2/alpha-source-over',
);

globalThis.spinonC182NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
