const nodes = Object.create(null);

function c181Node(parent, tag, id, declarations, src) {
  if (nodes[id]) throw new Error('C18.1 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement(tag);
  element.setAttribute('id', id);
  element.setAttribute('data-c181-node', '');
  element.setAttribute('style', declarations);
  if (src !== undefined) element.setAttribute('src', src);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const body = document.body || document;
const root = c181Node(
  body,
  'div',
  'c18-root',
  'display:flow-root;position:relative;box-sizing:border-box;width:80px;height:40px;background-color:#101827',
);
const visible = c181Node(
  root,
  'div',
  'c18-visible',
  'position:absolute;left:4px;top:4px;box-sizing:border-box;width:22px;height:26px;padding:4px;border:1px solid #f0c674;overflow:visible;background-color:transparent',
);
c181Node(
  visible,
  'img',
  'c18-visible-image',
  'display:block;width:24px;height:16px;padding:0;border:0',
  'spinon-fixture:c18-1/static-png-visible',
);
const hidden = c181Node(
  root,
  'div',
  'c18-hidden',
  'position:absolute;left:40px;top:4px;box-sizing:border-box;width:22px;height:26px;padding:4px;border:1px solid #f0c674;overflow:hidden;background-color:transparent',
);
c181Node(
  hidden,
  'img',
  'c18-hidden-image',
  'display:block;width:24px;height:16px;padding:0;border:0',
  'spinon-fixture:c18-1/static-png-hidden',
);

globalThis.spinonC181NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
