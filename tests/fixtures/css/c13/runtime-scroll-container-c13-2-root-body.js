(() => {
const rootBodyNodes = Object.create(null);

function c132RootBodyNode(parent, id, tagName, declarations) {
  const element = document.createElement(tagName);
  element.setAttribute('id', id);
  element.setAttribute('data-c132-root-body-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  rootBodyNodes[id] = element;
  return element;
}

const html = c132RootBodyNode(
  document,
  'c132-html-root',
  'html',
  'display:block;box-sizing:border-box;width:400px;height:240px;margin:0;overflow:visible;background-color:#ffffff',
);
rootBodyNodes.html = html;
const body = c132RootBodyNode(
  html,
  'c132-html-body',
  'body',
  'display:block;box-sizing:border-box;width:400px;height:240px;margin:0;overflow:auto;background-color:#f4f7fa',
);
rootBodyNodes.body = body;
c132RootBodyNode(
  body,
  'c132-root-content',
  'main',
  'position:relative;display:block;box-sizing:border-box;width:464px;height:324px;background-color:#57a6d9',
);
c132RootBodyNode(
  rootBodyNodes['c132-root-content'],
  'c132-root-marker',
  'div',
  'position:absolute;left:120px;top:120px;width:24px;height:20px;background-color:#e3544e',
);
c132RootBodyNode(
  body,
  'c132-root-fixed',
  'div',
  'position:fixed;right:8px;bottom:8px;width:52px;height:28px;background-color:#ed8b36',
);

globalThis.spinonC132RootBodyNodes = rootBodyNodes;
return { nodeCount: Object.keys(rootBodyNodes).length, viewport: [400, 240] };
})();
