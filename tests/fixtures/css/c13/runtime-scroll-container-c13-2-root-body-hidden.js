(() => {
const previousNodes = globalThis.spinonC132RootBodyNodes;
if (previousNodes && previousNodes.html && previousNodes.html.parentNode) {
  previousNodes.html.parentNode.removeChild(previousNodes.html);
}

const nodes = Object.create(null);
function c132RootBodyNode(parent, id, tagName, declarations) {
  const element = document.createElement(tagName);
  element.setAttribute('id', id);
  element.setAttribute('data-c132-root-body-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const html = c132RootBodyNode(
  document,
  'c132-html-root-hidden',
  'html',
  'display:block;box-sizing:border-box;width:400px;height:240px;margin:0;overflow:hidden;background-color:#ffffff',
);
nodes.html = html;
const body = c132RootBodyNode(
  html,
  'c132-html-body-hidden',
  'body',
  'display:block;box-sizing:border-box;width:400px;height:240px;margin:0;overflow:auto;background-color:#f4f7fa',
);
nodes.body = body;
const content = c132RootBodyNode(
  body,
  'c132-root-content-hidden',
  'main',
  'position:relative;display:block;box-sizing:border-box;width:464px;height:324px;background-color:#57a6d9',
);
c132RootBodyNode(
  content,
  'c132-root-marker-hidden',
  'div',
  'position:absolute;left:120px;top:120px;width:24px;height:20px;background-color:#e3544e',
);
c132RootBodyNode(
  body,
  'c132-root-fixed-hidden',
  'div',
  'position:fixed;right:8px;bottom:8px;width:52px;height:28px;background-color:#ed8b36',
);
globalThis.spinonC132RootBodyNodes = nodes;
return { nodeCount: Object.keys(nodes).length, viewport: [400, 240], rootOverflow: 'hidden' };
})();
