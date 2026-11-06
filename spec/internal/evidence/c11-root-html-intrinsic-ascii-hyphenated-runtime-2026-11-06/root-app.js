const findById = (node, id) => {
  if (node === null) return null;
  if (node.nodeType === 1 && node.getAttribute("id") === id) return node;
  for (let child = node.firstChild; child !== null; child = child.nextSibling) {
    const found = findById(child, id);
    if (found !== null) return found;
  }
  return null;
};

const root = document.documentElement;
const grid = findById(root, "grid");
const stage = findById(root, "stage");
const expectedOrder = ["first", "long-text", "short-text", "last"];
const actualOrder = [];
if (grid !== null) {
  for (let child = grid.firstChild; child !== null; child = child.nextSibling) {
    if (child.nodeType === 1) actualOrder.push(child.getAttribute("id"));
  }
}
const describeText = (element) => {
  if (element === null) return null;
  const node = element.firstChild;
  return {
    text: element.textContent,
    firstNodeType: node == null ? null : node.nodeType,
    hasNextSibling: node == null ? null : node.nextSibling !== null,
  };
};
const longTextSource = describeText(findById(root, "long-text"));
const shortTextSource = describeText(findById(root, "short-text"));
const isSingleTextNode = (source) =>
  source !== null && source.firstNodeType === 3 && source.hasNextSibling === false;
if (root === null || grid === null || stage === null ||
    actualOrder.join(",") !== expectedOrder.join(",") ||
    !isSingleTextNode(longTextSource) || !isSingleTextNode(shortTextSource) ||
    longTextSource.text !== "12345678-12" || shortTextSource.text !== "A") {
  throw new Error("ASCII hyphenated Root HTML fixture DOM mismatch: " +
    JSON.stringify({ actualOrder, longTextSource, shortTextSource }));
}
stage.setAttribute("data-runtime", "ready");
