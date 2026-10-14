const nodes = Object.create(null);

// C13.2 app verification first runs a real html/body owner fixture. Replace that
// single-root scene in the same DOM transaction before constructing this matrix.
if (globalThis.spinonC132RootBodyNodes) {
  const oldHtml = globalThis.spinonC132RootBodyNodes.html;
  if (oldHtml && oldHtml.parentNode) oldHtml.parentNode.removeChild(oldHtml);
  delete globalThis.spinonC132RootBodyNodes;
}

function c132Node(parent, id, declarations) {
  if (nodes[id]) throw new Error('C13.2 node ID가 중복되었습니다: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c132-scroll-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const root = c132Node(
  document.body || document,
  'c132-root',
  'display:flow-root;position:relative;box-sizing:border-box;width:380px;height:180px;background-color:#101827',
);
const visible = c132Node(
  root,
  'c132-visible-port',
  'position:absolute;left:10px;top:10px;box-sizing:border-box;width:120px;height:82px;border:3px solid #f0c674;overflow:visible;background-color:#344256',
);
c132Node(
  visible,
  'c132-visible-escape',
  'position:absolute;left:130px;top:8px;width:30px;height:24px;background-color:#e3544e',
);
const scrollport = c132Node(
  root,
  'c132-scrollport',
  'position:absolute;left:170px;top:10px;box-sizing:border-box;width:120px;height:82px;padding:0;border:3px solid #f0c674;overflow:auto;background-color:#ffffff',
);
const content = c132Node(
  scrollport,
  'c132-scroll-content',
  'position:relative;box-sizing:border-box;width:220px;height:160px;background-color:#57a6d9',
);
c132Node(
  content,
  'c132-scroll-marker',
  'position:absolute;left:90px;top:60px;width:24px;height:20px;background-color:#e3544e',
);
c132Node(
  root,
  'c132-fixed-control',
  'position:fixed;left:340px;top:120px;width:32px;height:24px;background-color:#ed8b36',
);

// 화면 밖에서도 동일한 HostDocument → Stylo → Taffy → render 경로로
// Chrome 사전 비교의 owner/range 경계들을 평가한다.
function c132OffscreenOwner(parent, id, left, overflow) {
  return c132Node(
    parent,
    id,
    `position:absolute;left:${left}px;top:0;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow:${overflow};background-color:#ffffff`,
  );
}

const noOverflow = c132OffscreenOwner(root, 'c132-no-overflow', 410, 'auto');
c132Node(noOverflow, 'c132-no-overflow-fill', 'width:80px;height:45px;background-color:#57a6d9');

const oneAxisX = c132Node(root, 'c132-one-axis-x',
  'position:absolute;left:540px;top:0;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow-x:auto;overflow-y:hidden;background-color:#ffffff');
c132Node(oneAxisX, 'c132-one-axis-x-fill', 'width:200px;height:36px;background-color:#57a6d9');

const oneAxisY = c132Node(root, 'c132-one-axis-y',
  'position:absolute;left:670px;top:0;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow-x:hidden;overflow-y:auto;background-color:#ffffff');
c132Node(oneAxisY, 'c132-one-axis-y-fill', 'width:36px;height:150px;background-color:#57a6d9');

const hidden = c132OffscreenOwner(root, 'c132-hidden', 800, 'hidden');
c132Node(hidden, 'c132-hidden-fill', 'width:200px;height:150px;background-color:#57a6d9');

const clipped = c132OffscreenOwner(root, 'c132-clip', 930, 'clip');
c132Node(clipped, 'c132-clip-fill', 'width:200px;height:150px;background-color:#57a6d9');

const absolute = c132OffscreenOwner(root, 'c132-absolute', 1190, 'auto');
c132Node(absolute, 'c132-absolute-fill', 'width:48px;height:40px;background-color:#57a6d9');
c132Node(absolute, 'c132-absolute-escape',
  'position:absolute;left:150px;top:8px;width:28px;height:24px;background-color:#ed8b36');

const nestedOuter = c132Node(root, 'c132-nested-outer',
  'position:absolute;left:1320px;top:0;box-sizing:border-box;width:150px;height:104px;border:3px solid;overflow:auto;background-color:#ffffff');
const nestedWrapper = c132Node(nestedOuter, 'c132-nested-wrapper',
  'box-sizing:border-box;width:232px;height:176px;padding:6px;background-color:#344256');
const nestedInner = c132Node(nestedWrapper, 'c132-nested-inner',
  'box-sizing:border-box;width:126px;height:88px;border:2px solid;overflow:auto;background-color:#ffffff');
c132Node(nestedInner, 'c132-nested-fill', 'width:188px;height:138px;background-color:#57a6d9');

const forcedScroll = c132OffscreenOwner(root, 'c132-forced-scroll', 1500, 'scroll');
c132Node(forcedScroll, 'c132-forced-scroll-fill', 'width:80px;height:45px;background-color:#57a6d9');

globalThis.spinonC132ScrollNodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, scrollportId: 'c132-scrollport' });
