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
const hasOneTextNode = (element) =>
  element.firstChild !== null &&
  element.firstChild.nodeType === 3 &&
  element.firstChild.nextSibling === null;
if (root === null || grid === null || stage === null || longText === null || shortText === null ||
    actualOrder.join(",") !== expectedOrder.join(",") ||
    !hasOneTextNode(longText) || !hasOneTextNode(shortText) ||
    longText.textContent !== " \tA\t A\nA \n A \tA\nA  " ||
    shortText.textContent !== " \tA\n ") {
  throw new Error("normal-whitespace Root HTML fixture DOM did not preserve its source");
}
stage.setAttribute("data-runtime", "ready");
