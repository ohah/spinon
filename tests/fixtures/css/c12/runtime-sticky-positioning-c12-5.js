const c125Nodes = Object.create(null);

function c125Node(parent, id, style, tagName = 'div', includeMarker = true) {
  if (c125Nodes[id]) throw new Error('C12.5 fixture node ID가 중복되었습니다: ' + id);
  const element = document.createElement(tagName);
  element.setAttribute('id', id);
  if (includeMarker) element.setAttribute('data-c12-5-node', '');
  element.setAttribute('style', 'box-sizing:border-box;' + style);
  parent.appendChild(element);
  if (includeMarker) c125Nodes[id] = element;
  return element;
}

const html = c125Node(
  document,
  'html-root',
  'display:block;width:360px;height:260px;margin:0;overflow:hidden',
  'html',
  false,
);
const body = c125Node(
  html,
  'body-root',
  'display:block;width:360px;height:260px;margin:0;overflow:hidden',
  'body',
  false,
);

const outer = c125Node(
  body,
  'outer',
  'position:absolute;left:16px;top:16px;width:328px;height:228px;overflow:auto;border:2px solid #111;padding:8px',
);
const outerContent = c125Node(
  outer,
  'outer-content',
  'position:relative;width:288px;height:560px',
);
c125Node(outerContent, 'before-inner', 'height:48px;background-color:#ddd');
const inner = c125Node(
  outerContent,
  'inner',
  'width:260px;height:148px;overflow:auto;border:2px solid #333;padding:6px',
);
const innerContent = c125Node(
  inner,
  'inner-content',
  'position:relative;width:224px;height:360px',
);
c125Node(innerContent, 'before-section', 'height:36px;background-color:#bbb');
const section = c125Node(
  innerContent,
  'section',
  'position:relative;width:224px;height:176px;background-color:#f5f5f5',
);
c125Node(
  section,
  'sticky',
  'position:sticky;top:8px;right:auto;bottom:auto;left:auto;width:180px;height:28px;background-color:#db3344',
);
c125Node(innerContent, 'section-after', 'height:132px;background-color:#cde');
c125Node(innerContent, 'inner-tail', 'height:148px;background-color:#edf');
c125Node(outerContent, 'outer-tail', 'height:240px;background-color:#def');

globalThis.spinonC125Nodes = c125Nodes;
globalThis.spinonC125SetParentContentEdge = function () {
  c125Nodes.section.setAttribute(
    'style',
    'box-sizing:border-box;position:relative;width:224px;height:176px;padding-bottom:12px;border-bottom:2px solid #333;background-color:#f5f5f5',
  );
  return { parent: 'section', paddingBottom: 12, borderBottom: 2 };
};
({ nodeCount: Object.keys(c125Nodes).length, stickyId: c125Nodes.sticky.id });
