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
const longText = findById(root, "long-text");
const shortText = findById(root, "short-text");
const describeText = (element) => {
  if (element === null) return null;
  const node = element.firstChild;
  return {
    text: element.textContent,
    firstNodeType: node == null ? null : node.nodeType,
    hasNextSibling: node == null ? null : node.nextSibling != null,
  };
};
const longTextSource = describeText(longText);
const shortTextSource = describeText(shortText);
const hasOneTextNode = (source) =>
  source !== null && source.firstNodeType === 3 && source.hasNextSibling === false;
if (root === null || grid === null || stage === null || longText === null || shortText === null ||
    actualOrder.join(",") !== expectedOrder.join(",") ||
    !hasOneTextNode(longTextSource) || !hasOneTextNode(shortTextSource) ||
    longTextSource.text !== " \tA\t A\nA \r\n A \tA\nA  " ||
    shortTextSource.text !== " \tA\r\n ") {
  throw new Error(`normal-whitespace Root HTML fixture DOM mismatch: ${JSON.stringify({
    actualOrder,
    longTextSource,
    shortTextSource,
  })}`);
}
stage.setAttribute("data-runtime", "ready");
